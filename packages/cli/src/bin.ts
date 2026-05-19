#!/usr/bin/env node
import { Command } from "commander";

import { createCommand } from "./commands/create.js";
import { simulateCommand } from "./commands/simulate.js";
import { deployCommand } from "./commands/deploy.js";
import { listCommand } from "./commands/list.js";
import { actionCommand } from "./commands/action.js";

const program = new Command();
program
  .name("lien")
  .description(
    "Lien — tie your loans. Create, simulate, and deploy Solana lending hooks.",
  )
  .version("0.1.0");

program.addCommand(createCommand());
program.addCommand(simulateCommand());
program.addCommand(deployCommand());
program.addCommand(listCommand());
program.addCommand(actionCommand());

program.parseAsync(process.argv).catch((err) => {
  console.error(err instanceof Error ? err.message : err);
  process.exitCode = 1;
});
