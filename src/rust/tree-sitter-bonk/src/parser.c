#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 11
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 10
#define ALIAS_COUNT 0
#define TOKEN_COUNT 5
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 6
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 4

enum {
  sym_date = 1,
  sym_description = 2,
  sym_account = 3,
  sym_amount = 4,
  sym_ledger = 5,
  sym_transaction = 6,
  sym_posting = 7,
  aux_sym_ledger_repeat1 = 8,
  aux_sym_transaction_repeat1 = 9,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_date] = "date",
  [sym_description] = "description",
  [sym_account] = "account",
  [sym_amount] = "amount",
  [sym_ledger] = "ledger",
  [sym_transaction] = "transaction",
  [sym_posting] = "posting",
  [aux_sym_ledger_repeat1] = "ledger_repeat1",
  [aux_sym_transaction_repeat1] = "transaction_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_date] = sym_date,
  [sym_description] = sym_description,
  [sym_account] = sym_account,
  [sym_amount] = sym_amount,
  [sym_ledger] = sym_ledger,
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
  [sym_date] = {
    .visible = true,
    .named = true,
  },
  [sym_description] = {
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
  field_description = 4,
  field_posting = 5,
  field_transaction = 6,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_account] = "account",
  [field_amount] = "amount",
  [field_date] = "date",
  [field_description] = "description",
  [field_posting] = "posting",
  [field_transaction] = "transaction",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 3},
  [3] = {.index = 4, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_transaction, 0},
  [1] =
    {field_date, 0},
    {field_description, 1},
    {field_posting, 2},
  [4] =
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
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(17);
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == '-') ADVANCE(11);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(24);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(19);
      if (lookahead == '\\') ADVANCE(5);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      if (lookahead == '-') ADVANCE(11);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(2)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(25);
      END_STATE();
    case 3:
      if (lookahead == '-') ADVANCE(9);
      END_STATE();
    case 4:
      if (lookahead == '-') ADVANCE(10);
      END_STATE();
    case 5:
      if (lookahead == '"' ||
          lookahead == '\\' ||
          lookahead == 'b' ||
          lookahead == 'f' ||
          lookahead == 'n' ||
          lookahead == 'r' ||
          lookahead == 't') ADVANCE(1);
      END_STATE();
    case 6:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(26);
      END_STATE();
    case 7:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(4);
      END_STATE();
    case 8:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(18);
      END_STATE();
    case 9:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(7);
      END_STATE();
    case 10:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(8);
      END_STATE();
    case 11:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(25);
      END_STATE();
    case 12:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(3);
      END_STATE();
    case 13:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(12);
      END_STATE();
    case 14:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(13);
      END_STATE();
    case 15:
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 16:
      if (eof) ADVANCE(17);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(16)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(14);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(sym_date);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(sym_description);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == ':') ADVANCE(15);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '-') ADVANCE(9);
      if (lookahead == '.') ADVANCE(6);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(25);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '.') ADVANCE(6);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(21);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '.') ADVANCE(6);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(22);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '.') ADVANCE(6);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(23);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '.') ADVANCE(6);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(25);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_amount);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(26);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 16},
  [2] = {.lex_state = 16},
  [3] = {.lex_state = 16},
  [4] = {.lex_state = 16},
  [5] = {.lex_state = 16},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 16},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 2},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_date] = ACTIONS(1),
    [sym_description] = ACTIONS(1),
    [sym_account] = ACTIONS(1),
    [sym_amount] = ACTIONS(1),
  },
  [1] = {
    [sym_ledger] = STATE(9),
    [sym_transaction] = STATE(4),
    [aux_sym_ledger_repeat1] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym_date] = ACTIONS(5),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 3,
    ACTIONS(9), 1,
      sym_account,
    ACTIONS(7), 2,
      ts_builtin_sym_end,
      sym_date,
    STATE(3), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [12] = 3,
    ACTIONS(13), 1,
      sym_account,
    ACTIONS(11), 2,
      ts_builtin_sym_end,
      sym_date,
    STATE(3), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [24] = 3,
    ACTIONS(5), 1,
      sym_date,
    ACTIONS(16), 1,
      ts_builtin_sym_end,
    STATE(5), 2,
      sym_transaction,
      aux_sym_ledger_repeat1,
  [35] = 3,
    ACTIONS(18), 1,
      ts_builtin_sym_end,
    ACTIONS(20), 1,
      sym_date,
    STATE(5), 2,
      sym_transaction,
      aux_sym_ledger_repeat1,
  [46] = 2,
    ACTIONS(9), 1,
      sym_account,
    STATE(2), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [54] = 1,
    ACTIONS(23), 3,
      ts_builtin_sym_end,
      sym_date,
      sym_account,
  [60] = 1,
    ACTIONS(25), 1,
      sym_description,
  [64] = 1,
    ACTIONS(27), 1,
      ts_builtin_sym_end,
  [68] = 1,
    ACTIONS(29), 1,
      sym_amount,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 12,
  [SMALL_STATE(4)] = 24,
  [SMALL_STATE(5)] = 35,
  [SMALL_STATE(6)] = 46,
  [SMALL_STATE(7)] = 54,
  [SMALL_STATE(8)] = 60,
  [SMALL_STATE(9)] = 64,
  [SMALL_STATE(10)] = 68,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ledger, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [7] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_transaction, 3, .production_id = 2),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_transaction_repeat1, 2),
  [13] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_transaction_repeat1, 2), SHIFT_REPEAT(10),
  [16] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ledger, 1, .production_id = 1),
  [18] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2),
  [20] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2), SHIFT_REPEAT(8),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_posting, 2, .production_id = 3),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [27] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
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
