#!/usr/bin/env node

const isInstalledGlobally = require("is-installed-globally");

const { install, run } = require("./binary");

(async () => {
  if (isInstalledGlobally) {
    run();
  } else {
    await install();
    setTimeout(run, 3000);
  }
})();
