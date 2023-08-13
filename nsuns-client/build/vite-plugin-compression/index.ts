import * as path from "path";
import * as fs from "fs";
import * as util from "util";
import * as zlib from "zlib";
import chalk from "chalk";
import { Plugin, ResolvedConfig, normalizePath } from "vite";
import { files } from "./fs";

export type PluginOptions = {
  filter: RegExp | ((filename: string) => boolean);
};

type CompressionResult = {
  path: string;
  prettyPath: string;
  uncompressedBytes: number;
  compressedBytes: number;
  prettySize: string;
};

const gzip = util.promisify(zlib.gzip);

const displaySize = (bytes: number): string => {
  return `${(bytes / 1000).toFixed(2)} kB`;
};

const highlightFileName = (name: string): string => {
  const parsed = path.parse(name);

  const ext = parsed.ext == ".gz" ? path.parse(parsed.name).ext : parsed.ext;

  switch (ext.toLowerCase()) {
    case ".css":
      return chalk.magenta(name);
    case ".js":
      return chalk.cyan(name);
    default:
      return chalk.green(name);
  }
};

export default function compression(
  options: PluginOptions = {
    filter: () => true,
  }
): Plugin {
  let outputPath: string;
  let config: ResolvedConfig;

  const name = "vite:compression";

  const test =
    typeof options.filter === "function" ? options.filter : (path: string) => (options.filter as RegExp).test(path);

  return {
    name,
    apply: "build",
    enforce: "post",
    configResolved: (resolved) => {
      config = resolved;
      outputPath = path.isAbsolute(resolved.build.outDir)
        ? resolved.build.outDir
        : path.resolve(resolved.root, resolved.build.outDir);
    },
    closeBundle: async () => {
      const results: CompressionResult[] = [];

      for await (const info of files(outputPath, test)) {
        const buffer = await fs.promises.readFile(info.path);
        const compressed = await gzip(buffer);

        const p = info.path + ".gz";

        const assetPath = normalizePath(path.relative(outputPath, p));

        const prettyPath = chalk.dim(config.build.outDir + "/") + highlightFileName(assetPath);
        const prettySize = chalk.dim.bold(`${displaySize(compressed.byteLength)}`);

        results.push({
          path: p,
          prettyPath,
          compressedBytes: compressed.byteLength,
          uncompressedBytes: info.size,
          prettySize,
        });

        await fs.promises.writeFile(p, compressed);
      }

      if (results.length == 0) return;

      const longestPath = results.reduce((max, result) => Math.max(max, result.prettyPath.length), 0);
      const longestSize = results.reduce((max, result) => Math.max(max, result.prettySize.length), 0);

      config.logger.info(chalk.cyan(name));

      for (const result of results) {
        const spacing = " ".repeat(longestPath + longestSize - result.prettyPath.length - result.prettySize.length);
        config.logger.info(`${result.prettyPath}${spacing}  ${result.prettySize}`);
      }

      const check = chalk.green("✓");

      config.logger.info(`${check} compressed assets.`);
    },
  };
}
