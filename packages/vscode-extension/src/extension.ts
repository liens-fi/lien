import * as vscode from "vscode";

import { renderKnotDiagram } from "./diagram.js";
import { runSimulation } from "./simulate.js";

export function activate(context: vscode.ExtensionContext): void {
  context.subscriptions.push(
    vscode.commands.registerCommand("lien.openDesigner", openDesigner),
    vscode.commands.registerCommand("lien.simulate", () => simulate(context)),
    vscode.commands.registerCommand("lien.deployPlan", showDeployPlan),
  );
}

export function deactivate(): void {
  /* no-op */
}

function openDesigner(): void {
  const panel = vscode.window.createWebviewPanel(
    "lien.designer",
    "Lien Hook Designer",
    vscode.ViewColumn.Beside,
    { enableScripts: true, retainContextWhenHidden: true },
  );
  panel.webview.html = renderKnotDiagram();
}

async function simulate(context: vscode.ExtensionContext): Promise<void> {
  const pool = vscode.workspace.getConfiguration("lien").get<string>("defaultPool") ?? "SOL-USDC";
  const report = runSimulation({ pool, steps: 60 });
  const channel = vscode.window.createOutputChannel("Lien");
  context.subscriptions.push(channel);
  channel.show(true);
  channel.appendLine(`Lien simulation — pool=${pool}, events=${report.totalEvents}`);
  channel.appendLine(`  ltv overrides       ${report.ltvOverrides}`);
  channel.appendLine(`  rate overrides      ${report.rateOverrides}`);
  channel.appendLine(`  liquidations delay  ${report.liquidationsDelayed}`);
  channel.appendLine(`  liquidations exec   ${report.liquidationsExecuted}`);
  channel.appendLine(`  borrows rejected    ${report.borrowsRejected}`);
}

function showDeployPlan(): void {
  const cluster = vscode.workspace.getConfiguration("lien").get<string>("cluster") ?? "localnet";
  const lines = [
    `Lien deploy plan — cluster: ${cluster}`,
    "",
    "1) anchor build",
    "2) solana balance --keypair <YOUR_KEYPAIR>",
    `3) anchor deploy --provider.cluster ${cluster} --provider.wallet <YOUR_KEYPAIR>`,
    "4) solana program show <PROGRAM_ID>",
    "",
    cluster === "mainnet"
      ? "Mainnet deploy requires explicit operator approval. Confirm the keypair pubkey before broadcasting."
      : "",
  ];
  vscode.window.showInformationMessage(lines.filter(Boolean).join("\n"), { modal: false });
}
