window.SIDEBAR_ITEMS = {"constant":[["CHUNK_REQUEST_TIMEOUT","Timeout for requesting availability chunks."],["DISPUTE_REQUEST_TIMEOUT","We can have relative large timeouts here, there is no value of hitting a timeout as we want to get statements through to each node in any case."],["MAX_PARALLEL_STATEMENT_REQUESTS","We don’t want a slow peer to slow down all the others, at the same time we want to get out the data quickly in full to at least some peers (as this will reduce load on us as they then can start serving the data). So this value is a trade-off. 3 seems to be sensible. So we would need to have 3 slow nodes connected, to delay transfer for others by `STATEMENTS_TIMEOUT`."]],"enum":[["Protocol","A protocol per subsystem seems to make the most sense, this way we don’t need any dispatching within protocols."],["ProtocolName","The protocol name transmitted on the wire."]],"mod":[["incoming","Everything related to handling of incoming requests."],["network","Configuration of the networking layer."],["outgoing","Everything related to handling of outgoing requests."],["v1","Actual versioned requests and responses, that are sent over the wire. Requests and responses as sent over the wire for the individual protocols."]],"struct":[["ProtocolIter","An iterator over the variants of [Self]"],["ReqProtocolNames","Type for getting on the wire [`Protocol`] names using genesis hash & fork id."],["RequestResponseConfig","Configuration for a single request-response protocol."]],"trait":[["IsRequest","Common properties of any `Request`."]]};