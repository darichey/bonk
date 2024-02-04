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
    transaction: ($) =>
      seq(
        field("date", $.date),
        field("description", $.description),
        field("posting", repeat1($.posting))
      ),
    posting: ($) => seq(field("account", $.account), field("amount", $.amount)),

    declare_account: ($) => seq("account", field("account", $.account)),

    date: ($) => /\d{4}-\d{2}-\d{2}/,
    description: ($) => /"([^"\\]|\\["\\bnfrt])*"/,
    account: ($) => /[A-Za-z_][A-Za-z0-9_]*(:[A-Za-z_][A-Za-z0-9_]*)*/,
    amount: ($) => /-?\d+(\.\d+)?/,
  },
});
