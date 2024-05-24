const { execSync } = require("child_process");
let cmd = "--best --brute ./target/release/accelerator-rs";
if (process.platform === "win32") {
  cmd += ".exe";
  cmd = "\"./tools/upx/win.exe\" " + cmd;
} else {
  cmd = "tools/upx/linux " + cmd;
}

execSync("cargo build --release", { stdio: "inherit", shell: true });
execSync(cmd, { stdio: "inherit", shell: true });