// Copyright 2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Implementation of the Prospective Parachains subsystem - this tracks and handles
//! prospective parachain fragments and informs other backing-stage subsystems
//! of work to be done.
//!
//! This is the main coordinator of work within the node for the collation and
//! backing phases of parachain consensus.
//!
//! This is primarily an implementation of "Fragment Trees", as described in
//! [`polkadot_node_subsystem_util::inclusion_emulator::staging`].
//!
//! This also handles concerns such as the relay-chain being forkful,
//! session changes, predicting validator group assignments.

// TODO [now]: remove
#![allow(unused)]

use std::{
	collections::{hash_map::Entry as HEntry, HashMap, HashSet},
	sync::Arc,
};

use futures::{channel::oneshot, prelude::*};

use polkadot_node_subsystem::{
	overseer, ActiveLeavesUpdate, FromOverseer, OverseerSignal, SpawnedSubsystem, SubsystemContext,
	SubsystemError, SubsystemResult,
};
use polkadot_node_subsystem_util::{
	inclusion_emulator::staging::{
		ConstraintModifications, Constraints, Fragment, RelayChainBlockInfo,
	},
	metrics::{self, prometheus},
};
use polkadot_primitives::vstaging::{
	Block, BlockId, BlockNumber, CandidateDescriptor, CandidateHash, CommittedCandidateReceipt,
	GroupIndex, GroupRotationInfo, Hash, Header, Id as ParaId, PersistedValidationData,
	SessionIndex, ValidatorIndex,
};

use crate::{
	error::{Error, FatalError, FatalResult, JfyiError, JfyiErrorResult, Result},
	fragment_tree::{CandidateStorage, FragmentTree, Scope as TreeScope},
};

mod error;
mod fragment_tree;

const LOG_TARGET: &str = "parachain::prospective-parachains";

// The maximum depth the subsystem will allow. 'depth' is defined as the
// amount of blocks between the para head in a relay-chain block's state
// and a candidate with a particular relay-parent.
//
// This value is chosen mostly for reasons of resource-limitation.
// Without it, a malicious validator group could create arbitrarily long,
// useless prospective parachains and DoS honest nodes.
const MAX_DEPTH: usize = 4;

// The maximum ancestry we support.
const MAX_ANCESTRY: usize = 5;

/// The Prospective Parachains Subsystem.
pub struct ProspectiveParachainsSubsystems {
	metrics: Metrics,
}

// TODO [now]: add this enum to the broader subsystem types.
pub enum ProspectiveParachainsMessage {
	// TODO [now] : docs
	CandidateSeconded(ParaId, CommittedCandidateReceipt, PersistedValidationData),
	// TODO [now]: docs
	CandidateBacked(ParaId, CandidateHash),
	// TODO [now]: docs
	GetBackableCandidate(Hash, ParaId, Vec<CandidateHash>, oneshot::Sender<Option<CandidateHash>>),
}

struct ScheduledPara {
	para: ParaId,
	base_constraints: Constraints,
	validator_group: GroupIndex,
}

struct RelayBlockViewData {
	// Scheduling info for paras and upcoming paras.
	fragment_trees: HashMap<ParaId, FragmentTree>,
	block_info: RelayChainBlockInfo,
	// TODO [now]: other stuff
	// e.g. ancestors in same session
}

struct View {
	// Active or recent relay-chain blocks by block hash.
	active_leaves: HashMap<Hash, RelayBlockViewData>,
	candidate_storage: HashMap<ParaId, CandidateStorage>,
}

impl View {
	fn new() -> Self {
		View { active_leaves: HashMap::new(), candidate_storage: HashMap::new() }
	}
}

async fn run<Context>(mut ctx: Context) -> Result<()>
where
	Context: SubsystemContext<Message = ProspectiveParachainsMessage>,
	Context: overseer::SubsystemContext<Message = ProspectiveParachainsMessage>,
{
	// TODO [now]: run_until_error where view is preserved
	let mut view = View::new();
	loop {
		match ctx.recv().await.map_err(FatalError::SubsystemReceive)? {
			FromOverseer::Signal(OverseerSignal::Conclude) => return Ok(()),
			FromOverseer::Signal(OverseerSignal::ActiveLeaves(update)) => {
				view = handle_active_leaves_update(&mut ctx, view, update).await?;
			},
			FromOverseer::Signal(OverseerSignal::BlockFinalized(..)) => {},
			FromOverseer::Communication { msg } => match msg {
				ProspectiveParachainsMessage::CandidateSeconded(para, candidate, pvd) =>
					handle_candidate_seconded(&mut ctx, &mut view, para, candidate, pvd).await?,
				ProspectiveParachainsMessage::CandidateBacked(para, candidate_hash) =>
					handle_candidate_backed(&mut ctx, &mut view, para, candidate_hash).await?,
				ProspectiveParachainsMessage::GetBackableCandidate(
					relay_parent,
					para,
					required_path,
					tx,
				) =>
					answer_get_backable_candidate(
						&mut ctx,
						&view,
						relay_parent,
						para,
						required_path,
						tx,
					)
					.await?,
			},
		}
	}
}

async fn handle_active_leaves_update<Context>(
	ctx: &mut Context,
	mut view: View,
	update: ActiveLeavesUpdate,
) -> JfyiErrorResult<View>
where
	Context: SubsystemContext<Message = ProspectiveParachainsMessage>,
	Context: overseer::SubsystemContext<Message = ProspectiveParachainsMessage>,
{
	// 1. clean up inactive leaves
	// 2. determine all scheduled para at new block
	// 3. construct new fragment tree for each para for each new leaf
	// 4. prune candidate storage.

	for deactivated in &update.deactivated {
		view.active_leaves.remove(deactivated);
	}

	if let Some(activated) = update.activated {
		let hash = activated.hash;
		let scheduled_paras = fetch_upcoming_paras(ctx, &hash).await?;

		let block_info: RelayChainBlockInfo = unimplemented!();

		let ancestry = fetch_ancestry(&mut ctx, hash, MAX_ANCESTRY).await?;

		// Find constraints.
		let mut fragment_trees = HashMap::new();
		for para in scheduled_paras {
			let candidate_storage =
				view.candidate_storage.entry(para).or_insert_with(CandidateStorage::new);

			let constraints = fetch_constraints(&mut ctx, &hash, para).await?;

			let constraints = match constraints {
				Some(c) => c,
				None => {
					// This indicates a runtime conflict of some kind.

					tracing::debug!(
						target: LOG_TARGET,
						para_id = ?para,
						relay_parent = ?hash,
						"Failed to get inclusion constraints."
					);

					continue
				},
			};

			let scope = TreeScope::with_ancestors(
				para,
				block_info.clone(),
				constraints,
				MAX_DEPTH,
				ancestry.iter().cloned(),
			)
			.expect("ancestors are provided in reverse order and correctly; qed");

			let tree = FragmentTree::populate(scope, &*candidate_storage);
			fragment_trees.insert(para, tree);
		}

		// TODO [now]: notify subsystems of new trees.

		view.active_leaves
			.insert(hash, RelayBlockViewData { block_info, fragment_trees });
	}

	if !update.deactivated.is_empty() {
		// This has potential to be a hotspot.
		prune_view_candidate_storage(&mut view);
	}

	Ok(view)
}

fn prune_view_candidate_storage(view: &mut View) {
	let active_leaves = &view.active_leaves;
	view.candidate_storage.retain(|para_id, storage| {
		let mut coverage = HashSet::new();
		let mut contained = false;
		for head in active_leaves.values() {
			if let Some(tree) = head.fragment_trees.get(&para_id) {
				coverage.extend(tree.candidates());
			}
		}

		if !contained {
			return false
		}

		storage.retain(|h| coverage.contains(&h));

		// Even if `storage` is now empty, we retain.
		// This maintains a convenient invariant that para-id storage exists
		// as long as there's an active head which schedules the para.
		true
	})
}

async fn handle_candidate_seconded<Context>(
	ctx: &mut Context,
	view: &mut View,
	para: ParaId,
	candidate: CommittedCandidateReceipt,
	pvd: PersistedValidationData,
) -> JfyiErrorResult<()>
where
	Context: SubsystemContext<Message = ProspectiveParachainsMessage>,
	Context: overseer::SubsystemContext<Message = ProspectiveParachainsMessage>,
{
	// Add the candidate to storage.
	// Then attempt to add it to all trees.
	let storage = match view.candidate_storage.get_mut(&para) {
		None => {
			tracing::warn!(
				target: LOG_TARGET,
				para_id = ?para,
				candidate_hash = ?candidate.hash(),
				"Received seconded candidate for inactive para",
			);

			return Ok(())
		},
		Some(storage) => storage,
	};

	let candidate_hash = match storage.add_candidate(candidate, pvd) {
		Ok(c) => c,
		Err(crate::fragment_tree::PersistedValidationDataMismatch) => {
			// We can't log the candidate hash without either doing more ~expensive
			// hashing but this branch indicates something is seriously wrong elsewhere
			// so it's doubtful that it would affect debugging.

			tracing::warn!(
				target: LOG_TARGET,
				para = ?para,
				"Received seconded candidate had mismatching validation data",
			);

			return Ok(())
		},
	};

	for (_, leaf_data) in &mut view.active_leaves {
		if let Some(tree) = leaf_data.fragment_trees.get_mut(&para) {
			tree.add_and_populate(candidate_hash, &*storage);
			// TODO [now]: notify other subsystems of changes.
		}
	}

	Ok(())
}

async fn handle_candidate_backed<Context>(
	ctx: &mut Context,
	view: &mut View,
	para: ParaId,
	candidate_hash: CandidateHash,
) -> JfyiErrorResult<()>
where
	Context: SubsystemContext<Message = ProspectiveParachainsMessage>,
	Context: overseer::SubsystemContext<Message = ProspectiveParachainsMessage>,
{
	unimplemented!()
}

async fn answer_get_backable_candidate<Context>(
	ctx: &mut Context,
	view: &View,
	relay_parent: Hash,
	para: ParaId,
	required_path: Vec<CandidateHash>,
	tx: oneshot::Sender<Option<CandidateHash>>,
) -> JfyiErrorResult<()>
where
	Context: SubsystemContext<Message = ProspectiveParachainsMessage>,
	Context: overseer::SubsystemContext<Message = ProspectiveParachainsMessage>,
{
	unimplemented!()
}

async fn fetch_constraints<Context>(
	ctx: &mut Context,
	relay_parent: &Hash,
	para_id: ParaId,
) -> JfyiErrorResult<Option<Constraints>> {
	unimplemented!()
}

async fn fetch_upcoming_paras<Context>(
	ctx: &mut Context,
	relay_parent: &Hash,
) -> JfyiErrorResult<Vec<ParaId>> {
	unimplemented!()
}

// Fetch ancestors in descending order, up to the amount requested.
async fn fetch_ancestry<Context>(
	ctx: &mut Context,
	relay_parent: Hash,
	ancestors: usize,
) -> JfyiErrorResult<Vec<RelayChainBlockInfo>> {
	unimplemented!()
}

#[derive(Clone)]
struct MetricsInner;

/// Prospective parachain metrics.
#[derive(Default, Clone)]
pub struct Metrics(Option<MetricsInner>);

// TODO [now]: impl metrics