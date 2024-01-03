module.exports = grammar({
  name: "bonk",

  rules: {
    source_file: ($) => choice($.date, $.description, $.account, $.amount),

    date: ($) => /\d{4}-\d{2}-\d{2}/,
    description: ($) => /"([^"\\]|\\["\\bnfrt])*"/,
    account: ($) => /[A-Za-z_][A-Za-z0-9_]*(:[A-Za-z_][A-Za-z0-9_]*)*/,
    amount: ($) => /-?\d+(\.\d+)?/,
  },
});
