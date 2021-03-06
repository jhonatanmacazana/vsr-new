#!/usr/bin/env node

const isInstalledGlobally = require("is-installed-globally");

const { install, run, uninstall } = require("./binary");

(async () => {
  if (isInstalledGlobally) {
    run();
  } else {
    await install();
    setTimeout(() => {
      run();
      uninstall();
    }, 3000);
  }
})();
