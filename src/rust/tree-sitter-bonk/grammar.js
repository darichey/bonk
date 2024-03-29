// TODO: typecheck this
module.exports = grammar({
  name: "bonk",

  extras: ($) => [$.comment, /[\s\f\uFEFF\u2060\u200B]|\r?\n/],

  rules: {
    ledger: ($) =>
      repeat(
        choice(
          field("declare_account", $.declare_account),
          field("transaction", $.transaction)
        )
      ),

    metadata: ($) =>
      seq(
        field("key", $.ident),
        ":",
        field("value", choice($.date, $.string, $.ident, $.number))
      ),

    declare_account: ($) =>
      seq(
        "account",
        field("account", $.account),
        repeat(field("metadata", $.metadata))
      ),

    transaction: ($) =>
      seq(
        field("date", $.date),
        field("description", $.string),
        field("posting", repeat1($.posting))
      ),

    posting: ($) =>
      seq(field("account", $.account), field("amount", optional($.number))),

    account: ($) => seq($.ident, repeat(seq("/", $.ident))),

    date: ($) => /\d{4}-\d{2}-\d{2}/,
    string: ($) => /"([^"\\]|\\["\\bnfrt])*"/,
    ident: ($) => /[A-Za-z_][A-Za-z0-9_]*/,
    number: ($) => /-?\d+(\.\d+)?/,
    comment: ($) => /#.*/,
  },
});
