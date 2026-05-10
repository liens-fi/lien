# @liens-fi/solend-adapter

Wraps `@solendprotocol/solend-sdk`'s `SolendMarket` and produces a normalised `LifecycleEvent` for the Lien runtime.

```ts
import { SolendAdapter } from "@liens-fi/solend-adapter";

const adapter = new SolendAdapter({ rpcEndpoint: "https://api.mainnet-beta.solana.com" });
const snapshot = await adapter.snapshotPool(solendMarket);
```
