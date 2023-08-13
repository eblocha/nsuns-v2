import * as fs from "fs";
import * as path from "path";

async function exists(path: string) {
  return fs.promises
    .access(path, fs.constants.F_OK)
    .then(() => true)
    .catch(() => false);
}

export type FileInfo = {
  path: string;
  stats: fs.Stats;
};

/**
 * recursively iterate over the files in the provided path.
 */
export async function* files(p: string): AsyncGenerator<FileInfo, void, void> {
  if (!(await exists(p))) return;

  const stats = await fs.promises.stat(p);

  if (stats.isDirectory()) {
    for (const dirent of await fs.promises.readdir(p, { withFileTypes: true })) {
      for await (const result of files(path.join(dirent.path, dirent.name))) {
        yield result;
      }
    }
  }

  const info: FileInfo = {
    path: p,
    stats,
  };

  yield info;
}
