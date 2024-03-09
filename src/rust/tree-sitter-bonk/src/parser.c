#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 15
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 12
#define ALIAS_COUNT 0
#define TOKEN_COUNT 6
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 7
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 9

enum {
  anon_sym_account = 1,
  sym_date = 2,
  sym_string = 3,
  sym_account = 4,
  sym_amount = 5,
  sym_ledger = 6,
  sym_declare_account = 7,
  sym_transaction = 8,
  sym_posting = 9,
  aux_sym_ledger_repeat1 = 10,
  aux_sym_transaction_repeat1 = 11,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_account] = "account",
  [sym_date] = "date",
  [sym_string] = "string",
  [sym_account] = "account",
  [sym_amount] = "amount",
  [sym_ledger] = "ledger",
  [sym_declare_account] = "declare_account",
  [sym_transaction] = "transaction",
  [sym_posting] = "posting",
  [aux_sym_ledger_repeat1] = "ledger_repeat1",
  [aux_sym_transaction_repeat1] = "transaction_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_account] = anon_sym_account,
  [sym_date] = sym_date,
  [sym_string] = sym_string,
  [sym_account] = sym_account,
  [sym_amount] = sym_amount,
  [sym_ledger] = sym_ledger,
  [sym_declare_account] = sym_declare_account,
  [sym_transaction] = sym_transaction,
  [sym_posting] = sym_posting,
  [aux_sym_ledger_repeat1] = aux_sym_ledger_repeat1,
  [aux_sym_transaction_repeat1] = aux_sym_transaction_repeat1,
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
  [sym_date] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_account] = {
    .visible = true,
    .named = true,
  },
  [sym_amount] = {
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
  [aux_sym_ledger_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_transaction_repeat1] = {
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
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(23);
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == '-') ADVANCE(17);
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
      if (lookahead == 't') ADVANCE(24);
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
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 22:
      if (eof) ADVANCE(23);
      if (lookahead == 'a') ADVANCE(5);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(22)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(20);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_account);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_account);
      if (lookahead == '/') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_date);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == '/') ADVANCE(21);
      if (lookahead == 'c') ADVANCE(31);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == '/') ADVANCE(21);
      if (lookahead == 'c') ADVANCE(28);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == '/') ADVANCE(21);
      if (lookahead == 'n') ADVANCE(32);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == '/') ADVANCE(21);
      if (lookahead == 'o') ADVANCE(33);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == '/') ADVANCE(21);
      if (lookahead == 't') ADVANCE(25);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == '/') ADVANCE(21);
      if (lookahead == 'u') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == '/') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '-') ADVANCE(15);
      if (lookahead == '.') ADVANCE(12);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(39);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '.') ADVANCE(12);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(35);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '.') ADVANCE(12);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(36);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '.') ADVANCE(12);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(37);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '.') ADVANCE(12);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(39);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym_amount);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(40);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 22},
  [2] = {.lex_state = 22},
  [3] = {.lex_state = 22},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 22},
  [9] = {.lex_state = 22},
  [10] = {.lex_state = 22},
  [11] = {.lex_state = 10},
  [12] = {.lex_state = 10},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_account] = ACTIONS(1),
    [sym_date] = ACTIONS(1),
    [sym_string] = ACTIONS(1),
    [sym_account] = ACTIONS(1),
    [sym_amount] = ACTIONS(1),
  },
  [1] = {
    [sym_ledger] = STATE(14),
    [sym_declare_account] = STATE(8),
    [sym_transaction] = STATE(9),
    [aux_sym_ledger_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_account] = ACTIONS(5),
    [sym_date] = ACTIONS(7),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 6,
    ACTIONS(5), 1,
      anon_sym_account,
    ACTIONS(7), 1,
      sym_date,
    ACTIONS(9), 1,
      ts_builtin_sym_end,
    STATE(3), 1,
      aux_sym_ledger_repeat1,
    STATE(8), 1,
      sym_declare_account,
    STATE(9), 1,
      sym_transaction,
  [19] = 6,
    ACTIONS(11), 1,
      ts_builtin_sym_end,
    ACTIONS(13), 1,
      anon_sym_account,
    ACTIONS(16), 1,
      sym_date,
    STATE(3), 1,
      aux_sym_ledger_repeat1,
    STATE(8), 1,
      sym_declare_account,
    STATE(9), 1,
      sym_transaction,
  [38] = 4,
    ACTIONS(21), 1,
      anon_sym_account,
    ACTIONS(23), 1,
      sym_account,
    ACTIONS(19), 2,
      ts_builtin_sym_end,
      sym_date,
    STATE(5), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [53] = 4,
    ACTIONS(27), 1,
      anon_sym_account,
    ACTIONS(29), 1,
      sym_account,
    ACTIONS(25), 2,
      ts_builtin_sym_end,
      sym_date,
    STATE(5), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [68] = 3,
    ACTIONS(36), 1,
      sym_amount,
    ACTIONS(32), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(34), 2,
      anon_sym_account,
      sym_account,
  [80] = 2,
    ACTIONS(38), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(40), 2,
      anon_sym_account,
      sym_account,
  [89] = 1,
    ACTIONS(42), 3,
      ts_builtin_sym_end,
      anon_sym_account,
      sym_date,
  [95] = 1,
    ACTIONS(44), 3,
      ts_builtin_sym_end,
      anon_sym_account,
      sym_date,
  [101] = 1,
    ACTIONS(46), 3,
      ts_builtin_sym_end,
      anon_sym_account,
      sym_date,
  [107] = 2,
    ACTIONS(48), 1,
      sym_account,
    STATE(4), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [115] = 1,
    ACTIONS(50), 1,
      sym_account,
  [119] = 1,
    ACTIONS(52), 1,
      sym_string,
  [123] = 1,
    ACTIONS(54), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 19,
  [SMALL_STATE(4)] = 38,
  [SMALL_STATE(5)] = 53,
  [SMALL_STATE(6)] = 68,
  [SMALL_STATE(7)] = 80,
  [SMALL_STATE(8)] = 89,
  [SMALL_STATE(9)] = 95,
  [SMALL_STATE(10)] = 101,
  [SMALL_STATE(11)] = 107,
  [SMALL_STATE(12)] = 115,
  [SMALL_STATE(13)] = 119,
  [SMALL_STATE(14)] = 123,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ledger, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ledger, 1, .production_id = 3),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2, .production_id = 5),
  [13] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2, .production_id = 5), SHIFT_REPEAT(12),
  [16] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2, .production_id = 5), SHIFT_REPEAT(13),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_transaction, 3, .production_id = 7),
  [21] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_transaction, 3, .production_id = 7),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_transaction_repeat1, 2),
  [27] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_transaction_repeat1, 2),
  [29] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_transaction_repeat1, 2), SHIFT_REPEAT(6),
  [32] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_posting, 1, .production_id = 6),
  [34] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_posting, 1, .production_id = 6),
  [36] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [38] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_posting, 2, .production_id = 8),
  [40] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_posting, 2, .production_id = 8),
  [42] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 1, .production_id = 1),
  [44] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 1, .production_id = 2),
  [46] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declare_account, 2, .production_id = 4),
  [48] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [50] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [52] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [54] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
