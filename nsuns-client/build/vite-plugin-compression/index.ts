import * as path from "path";
import * as fs from "fs";
import * as zlib from "zlib";
import chalk from "chalk";
import { Plugin, ResolvedConfig, normalizePath } from "vite";
import { FileInfo, files } from "./fs";
import { Transform } from "stream";

export type FileTestFn = (info: FileInfo) => boolean;

export type Algorithm = "gzip" | "deflate" | "brotli";

export type PluginOptions = {
  filter: RegExp | FileTestFn;
  minSize: number;
  verbose: boolean;
  algorithm: Algorithm;
  ext?: string;
};

const defaultOptions: PluginOptions = {
  filter: /\.(html|css|js|json|mjs)$/i,
  minSize: 1024,
  verbose: true,
  algorithm: "gzip",
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

const getExt = (algorithm: Algorithm): string => {
  switch (algorithm) {
    case "gzip":
      return ".gz";
    case "brotli":
      return ".br";
    case "deflate":
      return ".zz";
  }
};

const getTransform = (algorithm: Algorithm): Transform => {
  switch (algorithm) {
    case "gzip":
      return zlib.createGzip();
    case "brotli":
      return zlib.createBrotliCompress();
    case "deflate":
      return zlib.createDeflate();
  }
};

const displaySize = (bytes: number): string => {
  return `${(bytes / 1000).toFixed(2)} kB`;
};

const highlightFileName = (name: string, ignoreExt?: string): string => {
  const parsed = path.parse(name);

  const ext = parsed.ext == ignoreExt ? path.parse(parsed.name).ext : parsed.ext;

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

  const ext = opts.ext || getExt(opts.algorithm);

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
      if (opts.verbose) {
        config.logger.info(chalk.cyan(name) + " " + chalk.green(opts.algorithm));
      }

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

              const outputPath = info.path + ext;

              fs.createReadStream(info.path)
                .pipe(getTransform(opts.algorithm))
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

      if (results.length == 0 || !opts.verbose) return;

      const displayResults: CompressionResultDisplay[] = results.map((result) => {
        const asset = path.join(config.build.outDir, path.relative(outputPath, result.path));
        const assetDir = path.dirname(asset);
        const assetName = path.basename(asset);

        const prettifiedPath = chalk.dim(normalizePath(assetDir) + "/") + highlightFileName(assetName, ext);
        const size = chalk.dim.bold(displaySize(result.compressedBytes));

        return {
          path: prettifiedPath,
          size,
        };
      });

      const longestPath = displayResults.reduce((max, result) => Math.max(max, result.path.length), 0);
      const longestSize = displayResults.reduce((max, result) => Math.max(max, result.size.length), 0);

      for (const result of displayResults) {
        const spacing = " ".repeat(longestPath + longestSize - result.path.length - result.size.length);
        config.logger.info(`${result.path}${spacing}  ${result.size}`);
      }

      const check = chalk.green("✓");

      config.logger.info(`${check} compressed assets.`);
    },
  };
}
