window.SIDEBAR_ITEMS = {"constant":[["ASSIGNMENT_KEY_TYPE_ID","The key type ID for parachain assignment key."],["COLLATOR_KEY_TYPE_ID","The key type ID for a collator key."],["LOWEST_PUBLIC_ID","The ID of the first publicly registerable parachain."],["LOWEST_USER_ID","The ID of the first user (non-system) parachain."],["MAX_CODE_SIZE","Maximum compressed code size we support right now. At the moment we have runtime upgrade on chain, which restricts scalability severely. If we want to have bigger values, we should fix that first."],["MAX_HEAD_DATA_SIZE","Maximum head data size we support right now."],["MAX_POV_SIZE","Maximum PoV size we support right now."],["PARACHAINS_INHERENT_IDENTIFIER","Unique identifier for the Parachains Inherent"],["PARACHAIN_KEY_TYPE_ID","The key type ID for a parachain validator key."],["POLKADOT_ENGINE_ID","Consensus engine id for polkadot v1 consensus engine."]],"enum":[["CandidateEvent","An even concerning a candidate."],["CompactStatement","Statements that can be made about parachain candidates. These are the actual values that are signed."],["ConsensusLog","A consensus log item for polkadot validation. To be used with [`POLKADOT_ENGINE_ID`]."],["CoreOccupied","What is occupying a specific availability core."],["CoreState","The state of a particular availability core."],["DisputeStatement","A statement about a candidate, to be used within the dispute resolution process."],["InvalidDisputeStatementKind","Different kinds of statements of invalidity on a candidate."],["OccupiedCoreAssumption","An assumption being made about the state of an occupied core."],["PvfExecTimeoutKind","Type discriminator for PVF execution timeouts"],["PvfPrepTimeoutKind","Type discriminator for PVF preparation timeouts"],["RuntimeMetricOp","Runtime metric operations."],["UpgradeGoAhead","A struct that the relay-chain communicates to a parachain indicating what course of action the parachain should take in the coordinated parachain validation code upgrade process."],["UpgradeRestriction","A possible upgrade restriction that prevents a parachain from performing an upgrade."],["ValidDisputeStatementKind","Different kinds of statements of validity on  a candidate."],["ValidityAttestation","An either implicit or explicit attestation to the validity of a parachain candidate."],["ValidityError","Custom validity errors used in Polkadot while validating transactions."]],"fn":[["byzantine_threshold","The maximum number of validators `f` which may safely be faulty."],["check_candidate_backing","Verify the backing of the given candidate."],["collator_signature_payload","Get a collator signature payload on a relay-parent, block-data combo."],["supermajority_threshold","The supermajority threshold of validators which represents a subset guaranteed to have at least f+1 honest validators."]],"mod":[["executor_params","Abstract execution environment parameter set."],["metric_definitions","Contains all runtime metrics defined as constants."],["well_known_keys","A declarations of storage keys where an external observer can find some interesting data."]],"struct":[["AbridgedHostConfiguration","Abridged version of `HostConfiguration` (from the `Configuration` parachains host runtime module) meant to be used by a parachain or PDK such as cumulus."],["AbridgedHrmpChannel","Abridged version of `HrmpChannel` (from the `Hrmp` parachains host runtime module) meant to be used by a parachain or PDK such as cumulus."],["ApprovalVote","A vote of approval on a candidate."],["AvailabilityBitfield","A bitfield concerning availability of backed candidates."],["BackedCandidate","A backed (or backable, depending on context) candidate."],["BlakeTwo256","Blake2-256 Hash implementation."],["CandidateCommitments","Commitments made in a `CandidateReceipt`. Many of these are outputs of validation."],["CandidateDescriptor","A unique descriptor of the candidate receipt."],["CandidateHash","Unit type wrapper around [`type@Hash`] that represents a candidate hash."],["CandidateReceipt","A candidate-receipt."],["CheckedDisputeStatementSet","A checked set of dispute statements."],["CommittedCandidateReceipt","A candidate-receipt with commitments directly included."],["CoreIndex","The unique (during session) index of a core."],["DisputeState","The entire state of a dispute."],["DisputeStatementSet","A set of statements about a specific candidate."],["ExplicitDisputeStatement","An explicit statement on a candidate issued as part of a dispute."],["FullCandidateReceipt","All data pertaining to the execution of a para candidate."],["GroupIndex","The unique (during session) index of a validator group."],["GroupRotationInfo","A helper data-type for tracking validator-group rotations."],["HeadData","Parachain head data included in the chain."],["HrmpChannelId","A type that uniquely identifies an HRMP channel. An HRMP channel is established between two paras. In text, we use the notation `(A, B)` to specify a channel between A and B. The channels are unidirectional, meaning that `(A, B)` and `(B, A)` refer to different channels. The convention is that we use the first item tuple for the sender and the second for the recipient. Only one channel is allowed between two participants in one direction, i.e. there cannot be 2 different channels identified by `(A, B)`. A channel with the same para id in sender and recipient is invalid. That is, however, not enforced."],["Id","Unique identifier of a parachain."],["InboundDownwardMessage","A wrapped version of `DownwardMessage`. The difference is that it has attached the block number when the message was sent."],["InboundHrmpMessage","An HRMP message seen from the perspective of a recipient."],["IndexedVec","`IndexedVec` struct indexed by type specific indices."],["InherentData","Parachains inherent-data passed into the runtime by a block author"],["OccupiedCore","Information about a core which is currently occupied."],["OutboundHrmpMessage","An HRMP message seen from the perspective of a sender."],["ParathreadClaim","A claim on authoring the next block for a given parathread."],["ParathreadEntry","An entry tracking a claim to ensure it does not pass the maximum number of retries."],["PersistedValidationData","The validation data provides information about how to create the inputs for validation of a candidate. This information is derived from the chain state and will vary from para to para, although some fields may be the same for every para."],["PvfCheckStatement","A statement from the specified validator whether the given validation code passes PVF pre-checking or not anchored to the given session index."],["RuntimeMetricLabel","A metric label."],["RuntimeMetricLabels","A set of metric labels."],["RuntimeMetricUpdate","Runtime metric update event."],["ScheduledCore","Information about a core which is currently occupied."],["ScrapedOnChainVotes","Scraped runtime backing votes and resolved disputes."],["SessionInfo","Information about validator sets of a session."],["Signed","Signed data with signature already verified."],["SigningContext","A type returned by runtime with current session index and a parent hash."],["Slot","Unit type wrapper that represents a slot."],["UncheckedExtrinsic","Simple blob to hold an extrinsic without committing to its format and ensure it is serialized correctly."],["UncheckedSigned","Unchecked signed data, can be converted to `Signed` by checking the signature."],["ValidationCode","Parachain validation code."],["ValidationCodeHash","Unit type wrapper around [`type@Hash`] that represents a validation code hash."],["ValidatorIndex","Index of the validator is used as a lightweight replacement of the `ValidatorId` when appropriate."]],"trait":[["EncodeAs","This helper trait ensures that we can encode `Statement` as `CompactStatement`, and anything as itself."],["HashT","Abstraction around hashing"],["TypeIndex","Trait required for type specific indices e.g. `ValidatorIndex` and `GroupIndex`"]],"type":[["AccountId","Alias to the opaque account ID type for this chain, actually a `AccountId32`. This is always 32 bytes."],["AccountIndex","The type for looking up accounts. We don’t expect more than 4 billion of them."],["AccountPublic","Alias to the public key used for this chain, actually a `MultiSigner`. Like the signature, this also isn’t a fixed size when encoded, as different cryptos have different size public keys."],["AssignmentId","The public key of a keypair used by a validator for determining assignments to approve included parachain candidates."],["AssignmentPair","The full keypair used by a validator for determining assignments to approve included parachain candidates."],["AuthorityDiscoveryId","An authority discovery authority identifier."],["Balance","The balance of an account. 128-bits (or 38 significant decimal figures) will allow for 10 m currency (`10^7`) at a resolution to all for one second’s worth of an annualised 50% reward be paid to a unit holder (`10^11` unit denomination), or `10^18` total atomic units, to grow at 50%/year for 51 years (`10^9` multiplier) for an eventual total of `10^27` units (27 significant decimal figures). We round denomination to `10^12` (12 SDF), and leave the other redundancy at the upper end so that 32 bits may be multiplied with a balance in 128 bits without worrying about overflow."],["Block","Block type."],["BlockId","Block ID."],["BlockNumber","The block number type used by Polkadot. 32-bits will allow for 136 years of blocks assuming 1 block per second."],["CandidateIndex","The index of the candidate in the list of candidates fully included as-of the block."],["ChainId","Identifier for a chain. 32-bit should be plenty."],["CheckedMultiDisputeStatementSet","A set of checked dispute statements."],["CollatorId","Identity that collators use."],["CollatorPair","A Parachain collator keypair."],["CollatorSignature","Signature on candidate’s block data by a collator."],["DownwardMessage","A message sent from the relay-chain down to a parachain."],["Hash","A hash of some data used by the relay chain."],["Header","Header type."],["HorizontalMessages",""],["Moment","An instant or duration in time."],["MultiDisputeStatementSet","A set of dispute statements."],["Nonce","Index of a transaction in the relay chain. 32-bit should be plenty."],["Remark","The information that goes alongside a `transfer_into_parachain` operation. Entirely opaque, it will generally be used for identifying the reason for the transfer. Typically it will hold the destination account to which the transfer should be credited. If still more information is needed, then this should be a hash with the pre-image presented via an off-chain mechanism on the parachain."],["RuntimeMetricLabelValue","A metric label value."],["RuntimeMetricLabelValues","A set of metric label values."],["SessionIndex","Simple index type with which we can count sessions."],["Signature","Alias to type for a signature for a transaction on the relay chain. This allows one of several kinds of underlying crypto to be used, so isn’t a fixed size when encoded."],["SignedAvailabilityBitfield","A bitfield signed by a particular validator about the availability of pending candidates."],["SignedAvailabilityBitfields","A set of signed availability bitfields. Should be sorted by validator index, ascending."],["SignedStatement","A signed compact statement, suitable to be sent to the chain."],["UncheckedSignedAvailabilityBitfield","A signed bitfield with signature not yet checked."],["UncheckedSignedAvailabilityBitfields","A set of unchecked signed availability bitfields. Should be sorted by validator index, ascending."],["UncheckedSignedStatement","A signed compact statement, with signature not yet checked."],["UpwardMessage","A message from a parachain to its Relay Chain."],["UpwardMessages",""],["ValidatorId","Identity that parachain validators use when signing validation messages."],["ValidatorPair","A Parachain validator keypair."],["ValidatorSignature","Signature with which parachain validators sign blocks."]]};