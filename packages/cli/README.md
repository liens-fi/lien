# lien-cli

```
npm i -g lien-cli
```

## Commands

```
lien list                            print the standard hook library
lien create hook --name MyHook       scaffold a hook source file
lien create composition              prompt-driven composition.ts
lien simulate --pool SOL-USDC        run the deterministic simulator
lien action                          write .github/workflows/lien-hook-ci.yml
lien deploy --cluster mainnet        print an Anchor deploy plan (does not broadcast)
```

`lien deploy` is plan-only by design — actually running `anchor deploy` requires the operator to confirm the keypair pubkey and balance.
