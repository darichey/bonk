#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 18
#define LARGE_STATE_COUNT 4
#define SYMBOL_COUNT 15
#define ALIAS_COUNT 0
#define TOKEN_COUNT 8
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 9
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 10

enum {
  anon_sym_import = 1,
  anon_sym_account = 2,
  sym_date = 3,
  sym_description = 4,
  sym_account = 5,
  sym_amount = 6,
  sym_path = 7,
  sym_ledger = 8,
  sym_import = 9,
  sym_declare_account = 10,
  sym_transaction = 11,
  sym_posting = 12,
  aux_sym_ledger_repeat1 = 13,
  aux_sym_transaction_repeat1 = 14,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_import] = "import",
  [anon_sym_account] = "account",
  [sym_date] = "date",
  [sym_description] = "description",
  [sym_account] = "account",
  [sym_amount] = "amount",
  [sym_path] = "path",
  [sym_ledger] = "ledger",
  [sym_import] = "import",
  [sym_declare_account] = "declare_account",
  [sym_transaction] = "transaction",
  [sym_posting] = "posting",
  [aux_sym_ledger_repeat1] = "ledger_repeat1",
  [aux_sym_transaction_repeat1] = "transaction_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_import] = anon_sym_import,
  [anon_sym_account] = anon_sym_account,
  [sym_date] = sym_date,
  [sym_description] = sym_description,
  [sym_account] = sym_account,
  [sym_amount] = sym_amount,
  [sym_path] = sym_path,
  [sym_ledger] = sym_ledger,
  [sym_import] = sym_import,
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
  [anon_sym_import] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_account] = {
    .visible = true,
    .named = false,
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
  [sym_path] = {
    .visible = true,
    .named = true,
  },
  [sym_ledger] = {
    .visible = true,
    .named = true,
  },
  [sym_import] = {
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
  field_import = 6,
  field_path = 7,
  field_posting = 8,
  field_transaction = 9,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_account] = "account",
  [field_amount] = "amount",
  [field_date] = "date",
  [field_declare_account] = "declare_account",
  [field_description] = "description",
  [field_import] = "import",
  [field_path] = "path",
  [field_posting] = "posting",
  [field_transaction] = "transaction",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 1},
  [4] = {.index = 3, .length = 3},
  [5] = {.index = 6, .length = 1},
  [6] = {.index = 7, .length = 1},
  [7] = {.index = 8, .length = 6},
  [8] = {.index = 14, .length = 3},
  [9] = {.index = 17, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_import, 0},
  [1] =
    {field_declare_account, 0},
  [2] =
    {field_transaction, 0},
  [3] =
    {field_declare_account, 0, .inherited = true},
    {field_import, 0, .inherited = true},
    {field_transaction, 0, .inherited = true},
  [6] =
    {field_path, 1},
  [7] =
    {field_account, 1},
  [8] =
    {field_declare_account, 0, .inherited = true},
    {field_declare_account, 1, .inherited = true},
    {field_import, 0, .inherited = true},
    {field_import, 1, .inherited = true},
    {field_transaction, 0, .inherited = true},
    {field_transaction, 1, .inherited = true},
  [14] =
    {field_date, 0},
    {field_description, 1},
    {field_posting, 2},
  [17] =
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
  [15] = 15,
  [16] = 16,
  [17] = 17,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(33);
      if (lookahead == '"') ADVANCE(3);
      if (lookahead == '-') ADVANCE(26);
      if (lookahead == 'a') ADVANCE(41);
      if (lookahead == 'i') ADVANCE(42);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(55);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 1:
      if (lookahead == '\n') SKIP(2)
      if (lookahead == '/') ADVANCE(7);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(1);
      if (lookahead != 0) ADVANCE(8);
      END_STATE();
    case 2:
      if (lookahead == '\n') SKIP(2)
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(1);
      if (lookahead != 0) ADVANCE(8);
      END_STATE();
    case 3:
      if (lookahead == '"') ADVANCE(39);
      if (lookahead == '\\') ADVANCE(20);
      if (lookahead != 0) ADVANCE(3);
      END_STATE();
    case 4:
      if (lookahead == '-') ADVANCE(26);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(4)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(56);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 5:
      if (lookahead == '-') ADVANCE(24);
      END_STATE();
    case 6:
      if (lookahead == '-') ADVANCE(25);
      END_STATE();
    case 7:
      if (lookahead == '/') ADVANCE(7);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(8);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(58);
      END_STATE();
    case 8:
      if (lookahead == '/') ADVANCE(7);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(8);
      END_STATE();
    case 9:
      if (lookahead == 'c') ADVANCE(13);
      END_STATE();
    case 10:
      if (lookahead == 'c') ADVANCE(9);
      END_STATE();
    case 11:
      if (lookahead == 'm') ADVANCE(15);
      END_STATE();
    case 12:
      if (lookahead == 'n') ADVANCE(18);
      END_STATE();
    case 13:
      if (lookahead == 'o') ADVANCE(19);
      END_STATE();
    case 14:
      if (lookahead == 'o') ADVANCE(16);
      END_STATE();
    case 15:
      if (lookahead == 'p') ADVANCE(14);
      END_STATE();
    case 16:
      if (lookahead == 'r') ADVANCE(17);
      END_STATE();
    case 17:
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 18:
      if (lookahead == 't') ADVANCE(36);
      END_STATE();
    case 19:
      if (lookahead == 'u') ADVANCE(12);
      END_STATE();
    case 20:
      if (lookahead == '"' ||
          lookahead == '\\' ||
          lookahead == 'b' ||
          lookahead == 'f' ||
          lookahead == 'n' ||
          lookahead == 'r' ||
          lookahead == 't') ADVANCE(3);
      END_STATE();
    case 21:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(57);
      END_STATE();
    case 22:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(6);
      END_STATE();
    case 23:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(38);
      END_STATE();
    case 24:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(22);
      END_STATE();
    case 25:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(23);
      END_STATE();
    case 26:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(56);
      END_STATE();
    case 27:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(5);
      END_STATE();
    case 28:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(27);
      END_STATE();
    case 29:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(28);
      END_STATE();
    case 30:
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 31:
      if (eof) ADVANCE(33);
      if (lookahead == 'a') ADVANCE(41);
      if (lookahead == 'i') ADVANCE(42);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(31)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(29);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 32:
      if (eof) ADVANCE(33);
      if (lookahead == 'a') ADVANCE(10);
      if (lookahead == 'i') ADVANCE(11);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(32)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(29);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_import);
      if (lookahead == ':') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_account);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_account);
      if (lookahead == ':') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(sym_date);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(sym_description);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == ':') ADVANCE(30);
      if (lookahead == 'c') ADVANCE(44);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == ':') ADVANCE(30);
      if (lookahead == 'c') ADVANCE(40);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == ':') ADVANCE(30);
      if (lookahead == 'm') ADVANCE(46);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == ':') ADVANCE(30);
      if (lookahead == 'n') ADVANCE(49);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == ':') ADVANCE(30);
      if (lookahead == 'o') ADVANCE(50);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == ':') ADVANCE(30);
      if (lookahead == 'o') ADVANCE(47);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == ':') ADVANCE(30);
      if (lookahead == 'p') ADVANCE(45);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == ':') ADVANCE(30);
      if (lookahead == 'r') ADVANCE(48);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == ':') ADVANCE(30);
      if (lookahead == 't') ADVANCE(35);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == ':') ADVANCE(30);
      if (lookahead == 't') ADVANCE(37);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == ':') ADVANCE(30);
      if (lookahead == 'u') ADVANCE(43);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(sym_account);
      if (lookahead == ':') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(51);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '-') ADVANCE(24);
      if (lookahead == '.') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(56);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '.') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(52);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '.') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(53);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '.') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(54);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(sym_amount);
      if (lookahead == '.') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(56);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(sym_amount);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(57);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(sym_path);
      if (lookahead == '/') ADVANCE(7);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(8);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(58);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 32},
  [2] = {.lex_state = 32},
  [3] = {.lex_state = 32},
  [4] = {.lex_state = 31},
  [5] = {.lex_state = 31},
  [6] = {.lex_state = 31},
  [7] = {.lex_state = 32},
  [8] = {.lex_state = 32},
  [9] = {.lex_state = 32},
  [10] = {.lex_state = 32},
  [11] = {.lex_state = 32},
  [12] = {.lex_state = 4},
  [13] = {.lex_state = 2},
  [14] = {.lex_state = 4},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 4},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_import] = ACTIONS(1),
    [anon_sym_account] = ACTIONS(1),
    [sym_date] = ACTIONS(1),
    [sym_description] = ACTIONS(1),
    [sym_account] = ACTIONS(1),
    [sym_amount] = ACTIONS(1),
  },
  [1] = {
    [sym_ledger] = STATE(16),
    [sym_import] = STATE(7),
    [sym_declare_account] = STATE(8),
    [sym_transaction] = STATE(9),
    [aux_sym_ledger_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_import] = ACTIONS(5),
    [anon_sym_account] = ACTIONS(7),
    [sym_date] = ACTIONS(9),
  },
  [2] = {
    [sym_import] = STATE(7),
    [sym_declare_account] = STATE(8),
    [sym_transaction] = STATE(9),
    [aux_sym_ledger_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(11),
    [anon_sym_import] = ACTIONS(5),
    [anon_sym_account] = ACTIONS(7),
    [sym_date] = ACTIONS(9),
  },
  [3] = {
    [sym_import] = STATE(7),
    [sym_declare_account] = STATE(8),
    [sym_transaction] = STATE(9),
    [aux_sym_ledger_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(13),
    [anon_sym_import] = ACTIONS(15),
    [anon_sym_account] = ACTIONS(18),
    [sym_date] = ACTIONS(21),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(28), 1,
      sym_account,
    ACTIONS(24), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(26), 2,
      anon_sym_import,
      anon_sym_account,
    STATE(5), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [16] = 4,
    ACTIONS(34), 1,
      sym_account,
    ACTIONS(30), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(32), 2,
      anon_sym_import,
      anon_sym_account,
    STATE(5), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [32] = 2,
    ACTIONS(37), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(39), 3,
      anon_sym_import,
      anon_sym_account,
      sym_account,
  [42] = 1,
    ACTIONS(41), 4,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_account,
      sym_date,
  [49] = 1,
    ACTIONS(43), 4,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_account,
      sym_date,
  [56] = 1,
    ACTIONS(45), 4,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_account,
      sym_date,
  [63] = 1,
    ACTIONS(47), 4,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_account,
      sym_date,
  [70] = 1,
    ACTIONS(49), 4,
      ts_builtin_sym_end,
      anon_sym_import,
      anon_sym_account,
      sym_date,
  [77] = 2,
    ACTIONS(51), 1,
      sym_account,
    STATE(4), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [85] = 1,
    ACTIONS(53), 1,
      sym_path,
  [89] = 1,
    ACTIONS(55), 1,
      sym_account,
  [93] = 1,
    ACTIONS(57), 1,
      sym_description,
  [97] = 1,
    ACTIONS(59), 1,
      ts_builtin_sym_end,
  [101] = 1,
    ACTIONS(61), 1,
      sym_amount,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(4)] = 0,
  [SMALL_STATE(5)] = 16,
  [SMALL_STATE(6)] = 32,
  [SMALL_STATE(7)] = 42,
  [SMALL_STATE(8)] = 49,
  [SMALL_STATE(9)] = 56,
  [SMALL_STATE(10)] = 63,
  [SMALL_STATE(11)] = 70,
  [SMALL_STATE(12)] = 77,
  [SMALL_STATE(13)] = 85,
  [SMALL_STATE(14)] = 89,
  [SMALL_STATE(15)] = 93,
  [SMALL_STATE(16)] = 97,
  [SMALL_STATE(17)] = 101,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ledger, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ledger, 1, .production_id = 4),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2, .production_id = 7),
  [15] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2, .production_id = 7), SHIFT_REPEAT(13),
  [18] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2, .production_id = 7), SHIFT_REPEAT(14),
  [21] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2, .production_id = 7), SHIFT_REPEAT(15),
  [24] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_transaction, 3, .production_id = 8),
  [26] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_transaction, 3, .production_id = 8),
  [28] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [30] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_transaction_repeat1, 2),
  [32] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_transaction_repeat1, 2),
  [34] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_transaction_repeat1, 2), SHIFT_REPEAT(17),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_posting, 2, .production_id = 9),
  [39] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_posting, 2, .production_id = 9),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 1, .production_id = 1),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 1, .production_id = 2),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 1, .production_id = 3),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import, 2, .production_id = 5),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declare_account, 2, .production_id = 6),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [59] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
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
