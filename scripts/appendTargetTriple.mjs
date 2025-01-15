import { execSync } from 'child_process';
import fs from 'fs';

const extension = process.platform === 'win32' ? '.exe' : '';
const rustInfo = execSync('rustc -vV');
const targetTriple = /host: (\S+)/g.exec(rustInfo)[1];

if (!targetTriple) {
  console.error('Failed to determine platform target triple');
}

if (process.argv.length == 2) {
  throw "Please provide executable name which needs to be renamed"
}

// get the executable name from command line arguments
const executableName = process.argv[2]
fs.renameSync(
  `src-tauri/binaries/${executableName}`,
  `src-tauri/binaries/${executableName}-${targetTriple}${extension}`
);
