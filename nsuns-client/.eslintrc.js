const path = require("path");

module.exports = {
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:@typescript-eslint/recommended-requiring-type-checking",
  ],
  plugins: ["@typescript-eslint"],
  env: {
    browser: true,
    node: false,
  },
  parser: "@typescript-eslint/parser",
  parserOptions: {
    project: [path.join(__dirname, "tsconfig.json")],
  },
  ignorePatterns: ["dist", "node_modules", "coverage", "*.js"],
  rules: {
    "@typescript-eslint/no-explicit-any": ["off"],
    "@typescript-eslint/restrict-template-expressions": ["off"],
  },
};
