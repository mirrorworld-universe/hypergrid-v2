# HyperGrid Grid V2

The second generation of the HyperGrid Grid implementation, for Sonic v2.

See [Reference Doc](https://docs.google.com/document/d/1G9ZxCjbIDAw0pImqVdXrJ4-tqjyMGFPWFifFaQgfDX8/edit?usp=sharing)

## Workspace

```
"cache",
"core",
"node",
"p2p",
"rpc",
"storage",
"svm"
```

### `grid-node`
Combining all layers into a single CLI executable node
- CLI Runner
- Configuration Builder
- Central Hub 

### `grid-cache`
Contains caching mechanisms for grid storage

### `grid-core`
Contains Hypergrid Framework specific constructs and interactions
- HSSN Touchpoint
- Hyperfuse Touchpoint

### `grid-p2p`
Contains peering mechanism for network state reconciliation
- LibP2P
- DHTs
- Routing Tables
- State Reconciliation
- State Streaming

### `grid-rpc`
Contains main ingress gateway for grid node
- HTTP JSON-RPC endpoints
- Websockets JSON-RPC endpoints

### `grid-storage`
Contains grid specific storage implementation
- Ledger Storage
- Accounts Storage

### `grid-svm`
Contains Solana specific runtime implementations
- L1 Account Loading
- Transaction Processor
- Program Caching
- VM Interactions
