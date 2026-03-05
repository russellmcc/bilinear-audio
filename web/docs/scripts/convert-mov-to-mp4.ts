import { $ } from "bun";
import { basename, dirname, extname, join, resolve } from "node:path";

const usage = `Usage: bun run convert-video -- <input.mov> [output.mp4]

Examples:
  bun run convert-video -- src/assets/images/fluffyverb-ui.mov
  bun run convert-video -- src/assets/images/input.mov src/assets/images/output.mp4`;

const fail = (message: string, exitCode = 1): never => {
  console.error(message);
  process.exit(exitCode);
};

const inputArg = Bun.argv[2] ?? fail(usage);
const outputArg = Bun.argv[3];

const inputPath = resolve(inputArg);

if (!(await Bun.file(inputPath).exists())) {
  fail(`Input file does not exist: ${inputPath}`);
}

if (extname(inputPath).toLowerCase() !== ".mov") {
  fail(`Expected a .mov input file, received: ${inputPath}`);
}

const defaultOutputPath = join(
  dirname(inputPath),
  `${basename(inputPath, extname(inputPath))}.mp4`,
);
const outputPath = resolve(outputArg ?? defaultOutputPath);

if (extname(outputPath).toLowerCase() !== ".mp4") {
  fail(`Expected a .mp4 output file, received: ${outputPath}`);
}

const result =
  await $`ffmpeg -y -i ${inputPath} -an -vf fps=24 -c:v libx264 -crf 23 -pix_fmt yuv420p -movflags +faststart ${outputPath}`.nothrow();

if (result.exitCode !== 0) {
  process.exit(result.exitCode);
}

console.log(`Created ${outputPath}`);
