// TODO: typecheck this
module.exports = grammar({
  name: "bonk",

  rules: {
    ledger: ($) =>
      repeat(
        choice(
          field("declare_account", $.declare_account),
          field("transaction", $.transaction)
        )
      ),

    declare_account: ($) => seq("account", field("account", $.account)),

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
  },
});
