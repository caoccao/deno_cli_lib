/*
* Copyright (c) 2024. caoccao.com Sam Cao
* All rights reserved.

* Licensed under the Apache License, Version 2.0 (the "License")
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at

* http://www.apache.org/licenses/LICENSE-2.0

* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

// Usage: deno run --allow-all patch_deno.ts

import * as flags from "https://deno.land/std/flags/mod.ts"
import * as fs from "https://deno.land/std/fs/mod.ts"
import * as path from "https://deno.land/std/path/mod.ts"

const CARGO_TOML = 'Cargo.toml'
const CLI = 'cli'
const LIB_RS = 'lib.rs'
const VERSION = '0.1.0'

function patchDeno(denoRepoDirPath: string): Number {
  const scriptDirPath = path.dirname(path.fromFileUrl(import.meta.url))
  const sourceDirPath = path.join(scriptDirPath, '../../', CLI)
  const targetDirPath = fs.existsSync(denoRepoDirPath)
    ? denoRepoDirPath
    : path.join(scriptDirPath, '../../../', denoRepoDirPath, CLI)
  if (fs.existsSync(targetDirPath)) {
    console.info(`%cFound ${targetDirPath}.`, 'color: green')
  } else {
    console.error(`%cCould not find ${targetDirPath}.`, 'color: red')
    return 1
  }
  // Copy lib.rs
  const sourceLibRsFilePath = path.join(sourceDirPath, LIB_RS)
  if (fs.existsSync(sourceLibRsFilePath)) {
    console.info(`%cFound ${sourceLibRsFilePath}.`, 'color: green')
  } else {
    console.error(`%sCould not find ${sourceLibRsFilePath}.`, 'color: red')
    return 1
  }
  const targetLibRsFilePath = path.join(targetDirPath, LIB_RS)
  console.info(`Copy from ${sourceLibRsFilePath} to ${targetLibRsFilePath}.`)
  fs.copySync(sourceLibRsFilePath, targetLibRsFilePath, { overwrite: true })
  // Copy Cargo.toml
  const sourceCargoTomlFilePath = path.join(sourceDirPath, CARGO_TOML)
  if (fs.existsSync(sourceCargoTomlFilePath)) {
    console.info(`%cFound ${sourceCargoTomlFilePath}.`, 'color: green')
  } else {
    console.error(`%sCould not find ${sourceCargoTomlFilePath}.`, 'color: red')
    return 1
  }
  const targetCargoTomlFilePath = path.join(targetDirPath, CARGO_TOML)
  if (fs.existsSync(targetCargoTomlFilePath)) {
    const targetText: string = Deno.readTextFileSync(targetCargoTomlFilePath)
    if (targetText.includes('[lib]')) {
      console.info(`%cSkip ${targetCargoTomlFilePath}.`, 'color: yellow')
    } else {
      const sourceText: string = Deno.readTextFileSync(sourceCargoTomlFilePath)
      console.info(`Copy from ${sourceCargoTomlFilePath} to ${targetCargoTomlFilePath}.`)
      Deno.writeTextFileSync(targetCargoTomlFilePath, `${targetText}\n${sourceText}`);
    }
  } else {
    console.error(`%sCould not find ${targetCargoTomlFilePath}.`, 'color: red')
    return 1
  }
  return 0
}

console.info('Patch Deno')

const args = flags.parse(Deno.args, {
  alias: {
    "help": "h",
    "version": "v",
    "path": "p",
  },
  boolean: [
    "help",
    "version",
  ],
  string: [
    "path",
  ],
  default: {
    help: false,
    path: "../deno",
    version: false,
  },
})

if (args.help) {
  console.info(`Usage: patch_deno.ts -p <path-to-deno>
    -p, --path      Path to the Deno repository
    -h, --help      Print this help page
    -v, --version   Print version
  `)
} else if (args.version) {
  console.info(`Version: ${VERSION}`)
} else {
  Deno.exit(patchDeno(args.path))
}
