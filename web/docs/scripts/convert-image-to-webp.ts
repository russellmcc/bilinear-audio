import { $ } from "bun";
import { basename, dirname, extname, join, resolve } from "node:path";

const usage = `Usage: bun run convert-image -- <input.png|input.jpg|input.jpeg> [output.webp]

Examples:
  bun run convert-image -- src/assets/images/fluffyverb_hero.png
  bun run convert-image -- src/assets/images/keybed.jpg src/assets/images/keybed.webp`;

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

const inputExtension = extname(inputPath).toLowerCase();
const supportedInputExtensions = new Set([".png", ".jpg", ".jpeg"]);

if (!supportedInputExtensions.has(inputExtension)) {
  fail(`Expected a .png, .jpg, or .jpeg input file, received: ${inputPath}`);
}

const defaultOutputPath = join(
  dirname(inputPath),
  `${basename(inputPath, inputExtension)}.webp`,
);
const outputPath = resolve(outputArg ?? defaultOutputPath);

if (extname(outputPath).toLowerCase() !== ".webp") {
  fail(`Expected a .webp output file, received: ${outputPath}`);
}

const { exitCode } =
  await $`cwebp -q 85 -alpha_q 100 -m 6 -mt ${inputPath} -o ${outputPath}`.nothrow();

if (exitCode !== 0) {
  process.exit(exitCode);
}

console.log(`Created ${outputPath}`);
