import path from "node:path";
import { $ } from "bun";

const outDir = "_site";

const workspacePath = path.join(import.meta.path, "..", "..", "..", "..");

// Build the documentation
await $`bun run web-build docs`.cwd(workspacePath);

// Clear the output directory
await $`rm -rf ${outDir}`.cwd(workspacePath);

// Copy the documentation into the temporary directory
await $`mkdir -p ${outDir}`.cwd(workspacePath);
await $`cp -r web/docs/out/* ${outDir}/`.cwd(workspacePath);
