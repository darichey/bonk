module.exports = grammar({
  name: "bonk",

  rules: {
    ledger: ($) => repeat($.transaction),
    transaction: ($) => seq($.date, $.description, repeat1($.posting)),
    posting: ($) => seq($.account, optional($.amount)),

    date: ($) => /\d{4}-\d{2}-\d{2}/,
    description: ($) => /"([^"\\]|\\["\\bnfrt])*"/,
    account: ($) => /[A-Za-z_][A-Za-z0-9_]*(:[A-Za-z_][A-Za-z0-9_]*)*/,
    amount: ($) => /-?\d+(\.\d+)?/,
  },
});
