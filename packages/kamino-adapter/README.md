# @liens/kamino-adapter

Wraps `@kamino-finance/klend-sdk`'s `KaminoMarket` + `KaminoObligation` and produces a normalised `LifecycleEvent` for the Lien runtime.

```ts
import { KaminoAdapter } from "@liens/kamino-adapter";

const adapter = new KaminoAdapter({
  rpcEndpoint: "https://api.mainnet-beta.solana.com",
  marketAddress: kaminoMarket,
});
const snapshot = await adapter.snapshotPool(kaminoMarket);
```
