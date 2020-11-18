module.exports = {
  istanbulReporter: ["html", "text"],
  onCompileComplete: async function (_config) {
    await run("typechain");
  },
  skipFiles: ["test"],
};
