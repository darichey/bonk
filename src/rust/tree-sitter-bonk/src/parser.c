#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 25
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 15
#define ALIAS_COUNT 0
#define TOKEN_COUNT 7
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 7
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 9

enum {
  anon_sym_account = 1,
  anon_sym_SLASH = 2,
  sym_date = 3,
  sym_string = 4,
  sym_ident = 5,
  sym_number = 6,
  sym_ledger = 7,
  sym_declare_account = 8,
  sym_transaction = 9,
  sym_posting = 10,
  sym_account = 11,
  aux_sym_ledger_repeat1 = 12,
  aux_sym_transaction_repeat1 = 13,
  aux_sym_account_repeat1 = 14,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_account] = "account",
  [anon_sym_SLASH] = "/",
  [sym_date] = "date",
  [sym_string] = "string",
  [sym_ident] = "ident",
  [sym_number] = "number",
  [sym_ledger] = "ledger",
  [sym_declare_account] = "declare_account",
  [sym_transaction] = "transaction",
  [sym_posting] = "posting",
  [sym_account] = "account",
  [aux_sym_ledger_repeat1] = "ledger_repeat1",
  [aux_sym_transaction_repeat1] = "transaction_repeat1",
  [aux_sym_account_repeat1] = "account_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_account] = anon_sym_account,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [sym_date] = sym_date,
  [sym_string] = sym_string,
  [sym_ident] = sym_ident,
  [sym_number] = sym_number,
  [sym_ledger] = sym_ledger,
  [sym_declare_account] = sym_declare_account,
  [sym_transaction] = sym_transaction,
  [sym_posting] = sym_posting,
  [sym_account] = sym_account,
  [aux_sym_ledger_repeat1] = aux_sym_ledger_repeat1,
  [aux_sym_transaction_repeat1] = aux_sym_transaction_repeat1,
  [aux_sym_account_repeat1] = aux_sym_account_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_account] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [sym_date] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_ident] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_ledger] = {
    .visible = true,
    .named = true,
  },
  [sym_declare_account] = {
    .visible = true,
    .named = true,
  },
  [sym_transaction] = {
    .visible = true,
    .named = true,
  },
  [sym_posting] = {
    .visible = true,
    .named = true,
  },
  [sym_account] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_ledger_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_transaction_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_account_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_account = 1,
  field_amount = 2,
  field_date = 3,
  field_declare_account = 4,
  field_description = 5,
  field_posting = 6,
  field_transaction = 7,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_account] = "account",
  [field_amount] = "amount",
  [field_date] = "date",
  [field_declare_account] = "declare_account",
  [field_description] = "description",
  [field_posting] = "posting",
  [field_transaction] = "transaction",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 2},
  [4] = {.index = 4, .length = 1},
  [5] = {.index = 5, .length = 4},
  [6] = {.index = 9, .length = 1},
  [7] = {.index = 10, .length = 3},
  [8] = {.index = 13, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_declare_account, 0},
  [1] =
    {field_transaction, 0},
  [2] =
    {field_declare_account, 0, .inherited = true},
    {field_transaction, 0, .inherited = true},
  [4] =
    {field_account, 1},
  [5] =
    {field_declare_account, 0, .inherited = true},
    {field_declare_account, 1, .inherited = true},
    {field_transaction, 0, .inherited = true},
    {field_transaction, 1, .inherited = true},
  [9] =
    {field_account, 0},
  [10] =
    {field_date, 0},
    {field_description, 1},
    {field_posting, 2},
  [13] =
    {field_account, 0},
    {field_amount, 1},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 2,
  [11] = 4,
  [12] = 3,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 7,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 21,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(22);
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == '-') ADVANCE(17);
      if (lookahead == '/') ADVANCE(25);
      if (lookahead == 'a') ADVANCE(29);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(38);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(27);
      if (lookahead == '\\') ADVANCE(11);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      if (lookahead == '-') ADVANCE(15);
      END_STATE();
    case 3:
      if (lookahead == '-') ADVANCE(16);
      END_STATE();
    case 4:
      if (lookahead == 'c') ADVANCE(7);
      END_STATE();
    case 5:
      if (lookahead == 'c') ADVANCE(4);
      END_STATE();
    case 6:
      if (lookahead == 'n') ADVANCE(8);
      END_STATE();
    case 7:
      if (lookahead == 'o') ADVANCE(9);
      END_STATE();
    case 8:
      if (lookahead == 't') ADVANCE(23);
      END_STATE();
    case 9:
      if (lookahead == 'u') ADVANCE(6);
      END_STATE();
    case 10:
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(10)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 11:
      if (lookahead == '"' ||
          lookahead == '\\' ||
          lookahead == 'b' ||
          lookahead == 'f' ||
          lookahead == 'n' ||
          lookahead == 'r' ||
          lookahead == 't') ADVANCE(1);
      END_STATE();
    case 12:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(40);
      END_STATE();
    case 13:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(3);
      END_STATE();
    case 14:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(26);
      END_STATE();
    case 15:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(13);
      END_STATE();
    case 16:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(14);
      END_STATE();
    case 17:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(39);
      END_STATE();
    case 18:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(2);
      END_STATE();
    case 19:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(18);
      END_STATE();
    case 20:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(19);
      END_STATE();
    case 21:
      if (eof) ADVANCE(22);
      if (lookahead == '/') ADVANCE(25);
      if (lookahead == 'a') ADVANCE(5);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(21)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(20);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_account);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_account);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_SLASH);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_date);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_ident);
      if (lookahead == 'c') ADVANCE(31);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_ident);
      if (lookahead == 'c') ADVANCE(28);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_ident);
      if (lookahead == 'n') ADVANCE(32);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_ident);
      if (lookahead == 'o') ADVANCE(33);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym_ident);
      if (lookahead == 't') ADVANCE(24);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(sym_ident);
      if (lookahead == 'u') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(sym_ident);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(sym_number);
      if (lookahead == '-') ADVANCE(15);
      if (lookahead == '.') ADVANCE(12);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(39);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(sym_number);
      if (lookahead == '.') ADVANCE(12);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(35);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(sym_number);
      if (lookahead == '.') ADVANCE(12);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(36);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(sym_number);
      if (lookahead == '.') ADVANCE(12);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(37);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(sym_number);
      if (lookahead == '.') ADVANCE(12);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(39);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(40);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 21},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 21},
  [9] = {.lex_state = 21},
  [10] = {.lex_state = 21},
  [11] = {.lex_state = 21},
  [12] = {.lex_state = 21},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 10},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 21},
  [17] = {.lex_state = 21},
  [18] = {.lex_state = 21},
  [19] = {.lex_state = 21},
  [20] = {.lex_state = 10},
  [21] = {.lex_state = 10},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 10},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_account] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [sym_date] = ACTIONS(1),
    [sym_string] = ACTIONS(1),
    [sym_ident] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
  },
  [1] = {
    [sym_ledger] = STATE(22),
    [sym_declare_account] = STATE(19),
    [sym_transaction] = STATE(17),
    [aux_sym_ledger_repeat1] = STATE(8),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_account] = ACTIONS(5),
    [sym_date] = ACTIONS(7),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(13), 1,
      anon_sym_SLASH,
    STATE(2), 1,
      aux_sym_account_repeat1,
    ACTIONS(9), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(11), 3,
      anon_sym_account,
      sym_ident,
      sym_number,
  [16] = 4,
    ACTIONS(20), 1,
      anon_sym_SLASH,
    STATE(2), 1,
      aux_sym_account_repeat1,
    ACTIONS(16), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(18), 3,
      anon_sym_account,
      sym_ident,
      sym_number,
  [32] = 4,
    ACTIONS(20), 1,
      anon_sym_SLASH,
    STATE(3), 1,
      aux_sym_account_repeat1,
    ACTIONS(22), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(24), 3,
      anon_sym_account,
      sym_ident,
      sym_number,
  [48] = 5,
    ACTIONS(28), 1,
      anon_sym_account,
    ACTIONS(30), 1,
      sym_ident,
    STATE(13), 1,
      sym_account,
    ACTIONS(26), 2,
      ts_builtin_sym_end,
      sym_date,
    STATE(5), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [66] = 5,
    ACTIONS(35), 1,
      anon_sym_account,
    ACTIONS(37), 1,
      sym_ident,
    STATE(13), 1,
      sym_account,
    ACTIONS(33), 2,
      ts_builtin_sym_end,
      sym_date,
    STATE(5), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [84] = 2,
    ACTIONS(9), 3,
      ts_builtin_sym_end,
      anon_sym_SLASH,
      sym_date,
    ACTIONS(11), 3,
      anon_sym_account,
      sym_ident,
      sym_number,
  [95] = 6,
    ACTIONS(5), 1,
      anon_sym_account,
    ACTIONS(7), 1,
      sym_date,
    ACTIONS(39), 1,
      ts_builtin_sym_end,
    STATE(9), 1,
      aux_sym_ledger_repeat1,
    STATE(17), 1,
      sym_transaction,
    STATE(19), 1,
      sym_declare_account,
  [114] = 6,
    ACTIONS(41), 1,
      ts_builtin_sym_end,
    ACTIONS(43), 1,
      anon_sym_account,
    ACTIONS(46), 1,
      sym_date,
    STATE(9), 1,
      aux_sym_ledger_repeat1,
    STATE(17), 1,
      sym_transaction,
    STATE(19), 1,
      sym_declare_account,
  [133] = 3,
    ACTIONS(49), 1,
      anon_sym_SLASH,
    STATE(10), 1,
      aux_sym_account_repeat1,
    ACTIONS(9), 3,
      ts_builtin_sym_end,
      anon_sym_account,
      sym_date,
  [145] = 3,
    ACTIONS(52), 1,
      anon_sym_SLASH,
    STATE(12), 1,
      aux_sym_account_repeat1,
    ACTIONS(22), 3,
      ts_builtin_sym_end,
      anon_sym_account,
      sym_date,
  [157] = 3,
    ACTIONS(52), 1,
      anon_sym_SLASH,
    STATE(10), 1,
      aux_sym_account_repeat1,
    ACTIONS(16), 3,
      ts_builtin_sym_end,
      anon_sym_account,
      sym_date,
  [169] = 3,
    ACTIONS(58), 1,
      sym_number,
    ACTIONS(54), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(56), 2,
      anon_sym_account,
      sym_ident,
  [181] = 3,
    ACTIONS(60), 1,
      sym_ident,
    STATE(13), 1,
      sym_account,
    STATE(6), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [192] = 2,
    ACTIONS(62), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(64), 2,
      anon_sym_account,
      sym_ident,
  [201] = 1,
    ACTIONS(9), 4,
      ts_builtin_sym_end,
      anon_sym_account,
      anon_sym_SLASH,
      sym_date,
  [208] = 1,
    ACTIONS(66), 3,
      ts_builtin_sym_end,
      anon_sym_account,
      sym_date,
  [214] = 1,
    ACTIONS(68), 3,
      ts_builtin_sym_end,
      anon_sym_account,
      sym_date,
  [220] = 1,
    ACTIONS(70), 3,
      ts_builtin_sym_end,
      anon_sym_account,
      sym_date,
  [226] = 2,
    ACTIONS(72), 1,
      sym_ident,
    STATE(18), 1,
      sym_account,
  [233] = 1,
    ACTIONS(74), 1,
      sym_ident,
  [237] = 1,
    ACTIONS(76), 1,
      ts_builtin_sym_end,
  [241] = 1,
    ACTIONS(78), 1,
      sym_string,
  [245] = 1,
    ACTIONS(80), 1,
      sym_ident,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 16,
  [SMALL_STATE(4)] = 32,
  [SMALL_STATE(5)] = 48,
  [SMALL_STATE(6)] = 66,
  [SMALL_STATE(7)] = 84,
  [SMALL_STATE(8)] = 95,
  [SMALL_STATE(9)] = 114,
  [SMALL_STATE(10)] = 133,
  [SMALL_STATE(11)] = 145,
  [SMALL_STATE(12)] = 157,
  [SMALL_STATE(13)] = 169,
  [SMALL_STATE(14)] = 181,
  [SMALL_STATE(15)] = 192,
  [SMALL_STATE(16)] = 201,
  [SMALL_STATE(17)] = 208,
  [SMALL_STATE(18)] = 214,
  [SMALL_STATE(19)] = 220,
  [SMALL_STATE(20)] = 226,
  [SMALL_STATE(21)] = 233,
  [SMALL_STATE(22)] = 237,
  [SMALL_STATE(23)] = 241,
  [SMALL_STATE(24)] = 245,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ledger, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_account_repeat1, 2),
  [11] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_account_repeat1, 2),
  [13] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_account_repeat1, 2), SHIFT_REPEAT(24),
  [16] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_account, 2),
  [18] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_account, 2),
  [20] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [22] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_account, 1),
  [24] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_account, 1),
  [26] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_transaction_repeat1, 2),
  [28] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_transaction_repeat1, 2),
  [30] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_transaction_repeat1, 2), SHIFT_REPEAT(4),
  [33] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_transaction, 3, .production_id = 7),
  [35] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_transaction, 3, .production_id = 7),
  [37] = {.entry = {.count = 1, .reusable = false}}, SHIFT(4),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ledger, 1, .production_id = 3),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2, .production_id = 5),
  [43] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2, .production_id = 5), SHIFT_REPEAT(20),
  [46] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2, .production_id = 5), SHIFT_REPEAT(23),
  [49] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_account_repeat1, 2), SHIFT_REPEAT(21),
  [52] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [54] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_posting, 1, .production_id = 6),
  [56] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_posting, 1, .production_id = 6),
  [58] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [60] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [62] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_posting, 2, .production_id = 8),
  [64] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_posting, 2, .production_id = 8),
  [66] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 1, .production_id = 2),
  [68] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declare_account, 2, .production_id = 4),
  [70] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 1, .production_id = 1),
  [72] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [74] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [76] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [78] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [80] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_bonk(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
