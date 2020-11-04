// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
import {
  dirname,
  fromFileUrl,
  join,
} from "https://deno.land/std@0.76.0/path/mod.ts";
export { dirname, join };
export { existsSync } from "https://deno.land/std@0.76.0/fs/mod.ts";

export const ROOT_PATH = dirname(dirname(fromFileUrl(import.meta.url)));
const THIRD_PARTY_PATH = join(ROOT_PATH, "third_party");

/**
 *  Recursively list all files in (a subdirectory of) a git worktree.
 *    * Optionally, glob patterns may be specified to e.g. only list files with a
 *      certain extension.
 *    * Untracked files are included, unless they're listed in .gitignore.
 *    * Directory names themselves are not listed (but the files inside are).
 *    * Submodules and their contents are ignored entirely.
 *    * This function fails if the query matches no files.
 */
export async function gitLsFiles(baseDir, patterns) {
  baseDir = Deno.realPathSync(baseDir);
  const cmd = [
    "git",
    "-C",
    baseDir,
    "ls-files",
    "-z",
    "--exclude-standard",
    "--cached",
    "--modified",
    "--others",
    "--",
    ...patterns,
  ];
  const p = Deno.run({
    cmd,
    stdout: "piped",
  });
  const { success } = await p.status();

  if (!success) {
    throw new Error("gitLsFiles failed");
  }

  const output = new TextDecoder().decode(await p.output());
  p.close();

  const files = output.split("\0").filter((line) => line.length > 0).map(
    (filePath) => {
      return Deno.realPathSync(join(baseDir, filePath));
    },
  );

  return files;
}

/** List all files staged for commit */
export async function gitStaged(baseDir, patterns) {
  baseDir = Deno.realPathSync(baseDir);
  const cmd = [
    "git",
    "-C",
    baseDir,
    "diff",
    "--staged",
    "--diff-filter=ACMR",
    "--name-only",
    "-z",
    "--",
    ...patterns,
  ];

  const p = Deno.run({
    cmd,
    stdout: "piped",
  });
  const { success } = await p.status();

  if (!success) {
    throw new Error("gitLsFiles failed");
  }

  const output = new TextDecoder().decode(await p.output());
  p.close();

  const files = output.split("\0").filter((line) => line.length > 0).map(
    (filePath) => {
      return Deno.realPathSync(join(baseDir, filePath));
    },
  );

  return files;
}

function buildMode() {
  if (Deno.args.contains("--release")) {
    return "release";
  }

  return "debug";
}

export function buildPath() {
  join(ROOT_PATH, "target", buildMode());
}

export function getPrebuiltToolPath(toolName) {
  const PREBUILT_PATH = join(ROOT_PATH, "third_party", "prebuilt");

  const platformDirName = {
    "windows": "win",
    "darwin": "mac",
    "linux": "linux64",
  }[Deno.build.os];
  const executableSuffix = Deno.build.os === "windows" ? ".exe" : "";
  return join(PREBUILT_PATH, platformDirName, toolName + executableSuffix);
}
