window.SIDEBAR_ITEMS = {"constant":[["MAX_OVERWEIGHT_MESSAGES","Maximum amount of overweight messages that can exist in the queue at any given time."],["MAX_UPWARD_MESSAGE_SIZE_BOUND","Maximum value that `config.max_upward_message_size` can be set to"]],"enum":[["AcceptanceCheckErr","An error returned by [`check_upward_messages`] that indicates a violation of one of acceptance criteria rules."],["Call","Contains one variant per dispatchable that can be called by an extrinsic."],["Error","Custom dispatch errors of this pallet."],["Event","The event emitted by this pallet."]],"mod":[["migration",""],["pallet","The module that hosts all the FRAME types needed to add this pallet to a runtime."]],"struct":[["Pallet","The pallet implementing the on-chain logic."],["TestWeightInfo","fallback implementation"],["XcmSink","A specific implementation of a `UmpSink` where messages are in the XCM format and will be forwarded to the XCM Executor."]],"trait":[["Config","Configuration trait of this pallet."],["UmpSink","All upward messages coming from parachains will be funneled into an implementation of this trait."],["WeightInfo","Weight information of this pallet."]],"type":[["MessageId","Simple type used to identify messages for the purpose of reporting events. Secure if and only if the message content is unique."],["Module","Type alias to `Pallet`, to be used by `construct_runtime`."],["NeedsDispatch","The ordered list of `ParaId`s that have a `RelayDispatchQueue` entry."],["NextDispatchRoundStartWith","This is the para that gets will get dispatched first during the next upward dispatchable queue execution round."],["Overweight","The messages that exceeded max individual message weight budget."],["OverweightCount","The number of overweight messages ever recorded in `Overweight` (and thus the lowest free index)."],["OverweightIndex","Index used to identify overweight messages."],["RelayDispatchQueueSize","Size of the dispatch queues. Caches sizes of the queues in `RelayDispatchQueue`."],["RelayDispatchQueues","The messages waiting to be handled by the relay-chain originating from a certain parachain."]]};