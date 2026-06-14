import { Command } from "commander";
import kleur from "kleur";
import { Connection, PublicKey } from "@solana/web3.js";

const LIEN_EXECUTOR_ID = "5yNMqcyZsGQJk4xvw4jjvoRBSnGs8mgramEa3HQe5faD";

const EVENT_KIND_LABEL: Record<number, string> = {
  0: "beforeDeposit",
  1: "afterDeposit",
  2: "beforeBorrow",
  3: "afterBorrow",
  4: "beforeRepay",
  5: "afterRepay",
  6: "beforeLiquidate",
  7: "afterLiquidate",
};

interface ParsedReceipt {
  signature: string;
  slot: number;
  blockTime: number | null;
  event_kind?: number;
  hook_count_eligible?: number;
  hook_count_skipped?: number;
  composition?: string;
  pool?: string;
}

function parseExecutorLogs(logs: string[]): Partial<ParsedReceipt> {
  // The executor emits a CompositionExecuted event via Anchor's `emit!`. Anchor
  // prints "Program data: <base64>" lines for these. We don't decode the full
  // borsh payload here — that's what @liens/sdk does. The CLI surface is a
  // human-readable summary, so we look for the program log markers and pull
  // the obvious bytes (event kind + counts) from the borsh prefix.
  const out: Partial<ParsedReceipt> = {};
  for (const line of logs) {
    if (!line.includes("Program data:")) continue;
    const b64 = line.split("Program data:")[1]?.trim();
    if (!b64) continue;
    let buf: Buffer;
    try {
      buf = Buffer.from(b64, "base64");
    } catch {
      continue;
    }
    // Anchor event layout: 8-byte discriminator + struct fields.
    // CompositionExecuted: composition(32) + pool(32) + event_kind(u8) +
    // position_owner(32) + adapter(u8) + hook_count_eligible(u8) +
    // hook_count_skipped(u8) + timestamp(i64)
    if (buf.length < 8 + 32 + 32 + 1 + 32 + 1 + 1 + 1) continue;
    out.composition = new PublicKey(buf.subarray(8, 40)).toBase58();
    out.pool = new PublicKey(buf.subarray(40, 72)).toBase58();
    out.event_kind = buf[72];
    // position_owner = bytes 73..105
    // adapter = byte 105
    out.hook_count_eligible = buf[106];
    out.hook_count_skipped = buf[107];
    break;
  }
  return out;
}

export function receiptsCommand(): Command {
  return new Command("receipts")
    .description(
      "Pull a HookRan receipt off the chain and pretty-print it. Single signature for now; per-pool window coming next.",
    )
    .option("--signature <sig>", "Transaction signature to inspect")
    .option(
      "--rpc <url>",
      "RPC endpoint",
      "https://api.mainnet-beta.solana.com",
    )
    .action(async (opts: { signature?: string; rpc: string }) => {
      if (!opts.signature) {
        console.error(
          kleur.red("error:") +
            " --signature is required. example: lien receipts --signature 3AWddY...",
        );
        process.exitCode = 1;
        return;
      }
      const conn = new Connection(opts.rpc, "confirmed");
      const tx = await conn.getTransaction(opts.signature, {
        maxSupportedTransactionVersion: 0,
      });
      if (!tx) {
        console.error(
          kleur.red("error:") +
            ` no transaction found at ${opts.signature} on ${opts.rpc}`,
        );
        process.exitCode = 1;
        return;
      }
      const logs = tx.meta?.logMessages ?? [];
      const parsed = parseExecutorLogs(logs);
      const receipt: ParsedReceipt = {
        signature: opts.signature,
        slot: tx.slot,
        blockTime: tx.blockTime ?? null,
        ...parsed,
      };
      console.log(kleur.bold().yellow("lien receipts"));
      console.log(`  signature       ${receipt.signature}`);
      console.log(`  slot            ${receipt.slot}`);
      console.log(
        `  block time      ${
          receipt.blockTime
            ? new Date(receipt.blockTime * 1000).toISOString()
            : "(unknown)"
        }`,
      );
      console.log(`  executor        ${LIEN_EXECUTOR_ID}`);
      if (receipt.event_kind !== undefined) {
        console.log(
          `  event           ${EVENT_KIND_LABEL[receipt.event_kind] ?? "?"} (kind=${receipt.event_kind})`,
        );
        console.log(`  pool            ${receipt.pool ?? "?"}`);
        console.log(`  composition     ${receipt.composition ?? "?"}`);
        console.log(`  hooks eligible  ${receipt.hook_count_eligible ?? "?"}`);
        console.log(`  hooks skipped   ${receipt.hook_count_skipped ?? "?"}`);
      } else {
        console.log(
          kleur.dim(
            "  (no CompositionExecuted event in this transaction — was the signature an executor call?)",
          ),
        );
      }
      // Raw logs for forensics
      console.log("");
      console.log(kleur.dim("  raw program logs:"));
      for (const line of logs) {
        if (line.startsWith("Program log:") || line.includes("Program data:")) {
          console.log(kleur.dim(`    ${line}`));
        }
      }
    });
}
