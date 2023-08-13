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
  size: number;
};

/**
 * recursively iterate over the files in the provided path.
 * @param test A test function to filter out unwanted files.
 */
export async function* files(p: string, test?: (path: string) => boolean): AsyncGenerator<FileInfo, void, void> {
  if (!(await exists(p))) return;

  const stat = await fs.promises.stat(p);

  if (stat.isDirectory()) {
    for (const dirent of await fs.promises.readdir(p, { withFileTypes: true })) {
      for await (const result of files(path.join(dirent.path, dirent.name), test)) {
        yield result;
      }
    }
  }

  if (!test || test(p)) {
    yield { path: p, size: stat.size };
  }
}
