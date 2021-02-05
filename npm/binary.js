const { Binary } = require("binary-install");
const os = require("os");

function getPlatform() {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT" && arch === "x64") return "win64";
  if (type === "Linux" && arch === "x64") return "linux";
  if (type === "Darwin" && arch === "x64") return "macos";

  throw new Error(
    `Unsupported platform: ${type} ${arch}. Please create an issue at https://github.com/jhonatanmacazana/vsr-new/issues`
  );
}

function getBinary() {
  const platform = getPlatform();
  const version = require("../package.json").version;
  const url = `https://github.com/jhonatanmacazana/vsr-new/releases/download/v${version}/vsr-new-${platform}.tar.gz`;
  const binary = new Binary("vsr-new", url);

  return binary;
}

function install() {
  const binary = getBinary();
  binary.install();
}

function run() {
  const binary = getBinary();
  binary.run();
}

module.exports = {
  install,
  run,
};
