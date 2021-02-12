const os = require("os");
const Binary = require("ts-binary-install");

const getPlatform = () => {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT" && arch === "x64") return "win64";
  if (type === "Linux" && arch === "x64") return "linux";
  if (type === "Darwin" && arch === "x64") return "macos";

  throw new Error(
    `Unsupported platform: ${type} ${arch}. Please create an issue at https://github.com/jhonatanmacazana/vsr-new/issues`
  );
};

const getBinary = () => {
  const platform = getPlatform();
  const version = require("../package.json").version;
  const url = `https://github.com/jhonatanmacazana/vsr-new/releases/download/v${version}/vsr-new-${platform}.tar.gz`;

  const binary = new Binary(url, { name: "vsr-new" });

  return binary;
};

const install = () => {
  const binary = getBinary();
  return binary.install();
};

const uninstall = () => {
  const binary = getBinary();
  binary.uninstall();
};

const run = () => {
  const binary = getBinary();
  binary.run();
};

module.exports = {
  getBinary,
  install,
  run,
  uninstall,
};
