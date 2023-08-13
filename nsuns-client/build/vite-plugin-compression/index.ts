import * as path from "path";
import * as fs from "fs";
import * as zlib from "zlib";
import chalk from "chalk";
import { Plugin, ResolvedConfig, normalizePath } from "vite";
import { FileInfo, files } from "./fs";

export type FileTestFn = (info: FileInfo) => boolean;

export type PluginOptions = {
  filter: RegExp | FileTestFn;
  minSize: number;
  summary: boolean;
};

const defaultOptions: PluginOptions = {
  filter: /\.(html|css|js|json|mjs)$/i,
  minSize: 1024,
  summary: true,
};

type CompressionResult = {
  path: string;
  uncompressedBytes: number;
  compressedBytes: number;
};

type CompressionResultDisplay = {
  path: string;
  size: string;
};

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

export default function compression(options: Partial<PluginOptions>): Plugin {
  let outputPath: string;
  let config: ResolvedConfig;

  const name = "vite:compression";

  const opts = {
    ...defaultOptions,
    ...options,
  };

  const test: FileTestFn =
    opts.filter instanceof RegExp
      ? (info) => {
          return info.stats.size >= opts.minSize && (opts.filter as RegExp).test(info.path);
        }
      : (info) => {
          return info.stats.size >= opts.minSize && (opts.filter as FileTestFn)(info);
        };

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
      const toCompress: FileInfo[] = [];

      for await (const info of files(outputPath)) {
        if (test(info)) {
          toCompress.push(info);
        }
      }

      const results: CompressionResult[] = await Promise.all(
        toCompress.map(
          (info) =>
            new Promise<CompressionResult>((resolve, reject) => {
              let total = 0;

              const outputPath = info.path + ".gz";

              return fs
                .createReadStream(info.path)
                .pipe(zlib.createGzip())
                .on("data", (chunk: { length: number }) => {
                  total += chunk.length;
                })
                .pipe(fs.createWriteStream(outputPath))
                .on("finish", () =>
                  resolve({
                    path: outputPath,
                    compressedBytes: total,
                    uncompressedBytes: info.stats.size,
                  })
                )
                .on("error", reject);
            })
        )
      );

      if (results.length == 0 || !opts.summary) return;

      const displayResults: CompressionResultDisplay[] = results.map((result) => {
        const assetPath = normalizePath(path.relative(outputPath, result.path));
        const prettifiedPath = chalk.dim(config.build.outDir + "/") + highlightFileName(assetPath);
        const size = chalk.dim.bold(`${displaySize(result.compressedBytes)}`);

        return {
          path: prettifiedPath,
          size,
        };
      });

      const longestPath = displayResults.reduce((max, result) => Math.max(max, result.path.length), 0);
      const longestSize = displayResults.reduce((max, result) => Math.max(max, result.size.length), 0);

      config.logger.info(chalk.cyan(name));

      for (const result of displayResults) {
        const spacing = " ".repeat(longestPath + longestSize - result.path.length - result.size.length);
        config.logger.info(`${result.path}${spacing}  ${result.size}`);
      }

      const check = chalk.green("✓");

      config.logger.info(`${check} compressed assets.`);
    },
  };
}
