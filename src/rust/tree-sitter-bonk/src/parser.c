#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 26
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 18
#define ALIAS_COUNT 0
#define TOKEN_COUNT 8
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 10
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 13

enum {
  anon_sym_COLON = 1,
  anon_sym_account = 2,
  anon_sym_SLASH = 3,
  sym_date = 4,
  sym_string = 5,
  sym_ident = 6,
  sym_number = 7,
  sym_ledger = 8,
  sym_metadata = 9,
  sym_declare_account = 10,
  sym_transaction = 11,
  sym_posting = 12,
  sym_account = 13,
  aux_sym_ledger_repeat1 = 14,
  aux_sym_declare_account_repeat1 = 15,
  aux_sym_transaction_repeat1 = 16,
  aux_sym_account_repeat1 = 17,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_COLON] = ":",
  [anon_sym_account] = "account",
  [anon_sym_SLASH] = "/",
  [sym_date] = "date",
  [sym_string] = "string",
  [sym_ident] = "ident",
  [sym_number] = "number",
  [sym_ledger] = "ledger",
  [sym_metadata] = "metadata",
  [sym_declare_account] = "declare_account",
  [sym_transaction] = "transaction",
  [sym_posting] = "posting",
  [sym_account] = "account",
  [aux_sym_ledger_repeat1] = "ledger_repeat1",
  [aux_sym_declare_account_repeat1] = "declare_account_repeat1",
  [aux_sym_transaction_repeat1] = "transaction_repeat1",
  [aux_sym_account_repeat1] = "account_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_account] = anon_sym_account,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [sym_date] = sym_date,
  [sym_string] = sym_string,
  [sym_ident] = sym_ident,
  [sym_number] = sym_number,
  [sym_ledger] = sym_ledger,
  [sym_metadata] = sym_metadata,
  [sym_declare_account] = sym_declare_account,
  [sym_transaction] = sym_transaction,
  [sym_posting] = sym_posting,
  [sym_account] = sym_account,
  [aux_sym_ledger_repeat1] = aux_sym_ledger_repeat1,
  [aux_sym_declare_account_repeat1] = aux_sym_declare_account_repeat1,
  [aux_sym_transaction_repeat1] = aux_sym_transaction_repeat1,
  [aux_sym_account_repeat1] = aux_sym_account_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
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
  [sym_metadata] = {
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
  [aux_sym_declare_account_repeat1] = {
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
  field_key = 6,
  field_metadata = 7,
  field_posting = 8,
  field_transaction = 9,
  field_value = 10,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_account] = "account",
  [field_amount] = "amount",
  [field_date] = "date",
  [field_declare_account] = "declare_account",
  [field_description] = "description",
  [field_key] = "key",
  [field_metadata] = "metadata",
  [field_posting] = "posting",
  [field_transaction] = "transaction",
  [field_value] = "value",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 2},
  [4] = {.index = 4, .length = 1},
  [5] = {.index = 5, .length = 4},
  [6] = {.index = 9, .length = 1},
  [7] = {.index = 10, .length = 2},
  [8] = {.index = 12, .length = 1},
  [9] = {.index = 13, .length = 3},
  [10] = {.index = 16, .length = 2},
  [11] = {.index = 18, .length = 2},
  [12] = {.index = 20, .length = 2},
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
    {field_metadata, 0},
  [10] =
    {field_account, 1},
    {field_metadata, 2, .inherited = true},
  [12] =
    {field_account, 0},
  [13] =
    {field_date, 0},
    {field_description, 1},
    {field_posting, 2},
  [16] =
    {field_metadata, 0, .inherited = true},
    {field_metadata, 1, .inherited = true},
  [18] =
    {field_account, 0},
    {field_amount, 1},
  [20] =
    {field_key, 0},
    {field_value, 2},
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
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(22);
      if (lookahead == '"') ADVANCE(2);
      if (lookahead == '-') ADVANCE(17);
      if (lookahead == '/') ADVANCE(26);
      if (lookahead == ':') ADVANCE(23);
      if (lookahead == 'a') ADVANCE(30);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(39);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(2);
      if (lookahead == '-') ADVANCE(17);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(39);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 2:
      if (lookahead == '"') ADVANCE(28);
      if (lookahead == '\\') ADVANCE(11);
      if (lookahead != 0) ADVANCE(2);
      END_STATE();
    case 3:
      if (lookahead == '-') ADVANCE(15);
      END_STATE();
    case 4:
      if (lookahead == '-') ADVANCE(16);
      END_STATE();
    case 5:
      if (lookahead == 'c') ADVANCE(8);
      END_STATE();
    case 6:
      if (lookahead == 'c') ADVANCE(5);
      END_STATE();
    case 7:
      if (lookahead == 'n') ADVANCE(9);
      END_STATE();
    case 8:
      if (lookahead == 'o') ADVANCE(10);
      END_STATE();
    case 9:
      if (lookahead == 't') ADVANCE(24);
      END_STATE();
    case 10:
      if (lookahead == 'u') ADVANCE(7);
      END_STATE();
    case 11:
      if (lookahead == '"' ||
          lookahead == '\\' ||
          lookahead == 'b' ||
          lookahead == 'f' ||
          lookahead == 'n' ||
          lookahead == 'r' ||
          lookahead == 't') ADVANCE(2);
      END_STATE();
    case 12:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(41);
      END_STATE();
    case 13:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(4);
      END_STATE();
    case 14:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(27);
      END_STATE();
    case 15:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(13);
      END_STATE();
    case 16:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(14);
      END_STATE();
    case 17:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(40);
      END_STATE();
    case 18:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(3);
      END_STATE();
    case 19:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(18);
      END_STATE();
    case 20:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(19);
      END_STATE();
    case 21:
      if (eof) ADVANCE(22);
      if (lookahead == 'a') ADVANCE(6);
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
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_account);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_account);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_SLASH);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_date);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_ident);
      if (lookahead == 'c') ADVANCE(32);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_ident);
      if (lookahead == 'c') ADVANCE(29);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_ident);
      if (lookahead == 'n') ADVANCE(33);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym_ident);
      if (lookahead == 'o') ADVANCE(34);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(sym_ident);
      if (lookahead == 't') ADVANCE(25);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(sym_ident);
      if (lookahead == 'u') ADVANCE(31);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(sym_ident);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(sym_number);
      if (lookahead == '-') ADVANCE(15);
      if (lookahead == '.') ADVANCE(12);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(40);
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
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(38);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym_number);
      if (lookahead == '.') ADVANCE(12);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(40);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(41);
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
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 21},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 1},
  [17] = {.lex_state = 1},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 21},
  [20] = {.lex_state = 21},
  [21] = {.lex_state = 1},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 1},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_account] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [sym_date] = ACTIONS(1),
    [sym_string] = ACTIONS(1),
    [sym_ident] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
  },
  [1] = {
    [sym_ledger] = STATE(24),
    [sym_declare_account] = STATE(20),
    [sym_transaction] = STATE(19),
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
  [16] = 5,
    ACTIONS(18), 1,
      anon_sym_account,
    ACTIONS(20), 1,
      sym_ident,
    STATE(13), 1,
      sym_account,
    ACTIONS(16), 2,
      ts_builtin_sym_end,
      sym_date,
    STATE(3), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [34] = 4,
    ACTIONS(27), 1,
      anon_sym_SLASH,
    STATE(5), 1,
      aux_sym_account_repeat1,
    ACTIONS(23), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(25), 3,
      anon_sym_account,
      sym_ident,
      sym_number,
  [50] = 4,
    ACTIONS(27), 1,
      anon_sym_SLASH,
    STATE(2), 1,
      aux_sym_account_repeat1,
    ACTIONS(29), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(31), 3,
      anon_sym_account,
      sym_ident,
      sym_number,
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
    STATE(3), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [84] = 5,
    ACTIONS(41), 1,
      anon_sym_account,
    ACTIONS(43), 1,
      sym_ident,
    STATE(7), 1,
      aux_sym_declare_account_repeat1,
    STATE(18), 1,
      sym_metadata,
    ACTIONS(39), 2,
      ts_builtin_sym_end,
      sym_date,
  [101] = 6,
    ACTIONS(5), 1,
      anon_sym_account,
    ACTIONS(7), 1,
      sym_date,
    ACTIONS(46), 1,
      ts_builtin_sym_end,
    STATE(10), 1,
      aux_sym_ledger_repeat1,
    STATE(19), 1,
      sym_transaction,
    STATE(20), 1,
      sym_declare_account,
  [120] = 5,
    ACTIONS(50), 1,
      anon_sym_account,
    ACTIONS(52), 1,
      sym_ident,
    STATE(11), 1,
      aux_sym_declare_account_repeat1,
    STATE(18), 1,
      sym_metadata,
    ACTIONS(48), 2,
      ts_builtin_sym_end,
      sym_date,
  [137] = 6,
    ACTIONS(54), 1,
      ts_builtin_sym_end,
    ACTIONS(56), 1,
      anon_sym_account,
    ACTIONS(59), 1,
      sym_date,
    STATE(10), 1,
      aux_sym_ledger_repeat1,
    STATE(19), 1,
      sym_transaction,
    STATE(20), 1,
      sym_declare_account,
  [156] = 5,
    ACTIONS(52), 1,
      sym_ident,
    ACTIONS(64), 1,
      anon_sym_account,
    STATE(7), 1,
      aux_sym_declare_account_repeat1,
    STATE(18), 1,
      sym_metadata,
    ACTIONS(62), 2,
      ts_builtin_sym_end,
      sym_date,
  [173] = 2,
    ACTIONS(9), 3,
      ts_builtin_sym_end,
      anon_sym_SLASH,
      sym_date,
    ACTIONS(11), 3,
      anon_sym_account,
      sym_ident,
      sym_number,
  [184] = 3,
    ACTIONS(70), 1,
      sym_number,
    ACTIONS(66), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(68), 2,
      anon_sym_account,
      sym_ident,
  [196] = 2,
    ACTIONS(72), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(74), 2,
      anon_sym_account,
      sym_ident,
  [205] = 2,
    ACTIONS(76), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(78), 2,
      anon_sym_account,
      sym_ident,
  [214] = 3,
    ACTIONS(80), 1,
      sym_ident,
    STATE(13), 1,
      sym_account,
    STATE(6), 2,
      sym_posting,
      aux_sym_transaction_repeat1,
  [225] = 2,
    ACTIONS(84), 1,
      sym_number,
    ACTIONS(82), 3,
      sym_date,
      sym_string,
      sym_ident,
  [234] = 2,
    ACTIONS(86), 2,
      ts_builtin_sym_end,
      sym_date,
    ACTIONS(88), 2,
      anon_sym_account,
      sym_ident,
  [243] = 1,
    ACTIONS(90), 3,
      ts_builtin_sym_end,
      anon_sym_account,
      sym_date,
  [249] = 1,
    ACTIONS(92), 3,
      ts_builtin_sym_end,
      anon_sym_account,
      sym_date,
  [255] = 2,
    ACTIONS(80), 1,
      sym_ident,
    STATE(9), 1,
      sym_account,
  [262] = 1,
    ACTIONS(94), 1,
      anon_sym_COLON,
  [266] = 1,
    ACTIONS(96), 1,
      sym_ident,
  [270] = 1,
    ACTIONS(98), 1,
      ts_builtin_sym_end,
  [274] = 1,
    ACTIONS(100), 1,
      sym_string,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 16,
  [SMALL_STATE(4)] = 34,
  [SMALL_STATE(5)] = 50,
  [SMALL_STATE(6)] = 66,
  [SMALL_STATE(7)] = 84,
  [SMALL_STATE(8)] = 101,
  [SMALL_STATE(9)] = 120,
  [SMALL_STATE(10)] = 137,
  [SMALL_STATE(11)] = 156,
  [SMALL_STATE(12)] = 173,
  [SMALL_STATE(13)] = 184,
  [SMALL_STATE(14)] = 196,
  [SMALL_STATE(15)] = 205,
  [SMALL_STATE(16)] = 214,
  [SMALL_STATE(17)] = 225,
  [SMALL_STATE(18)] = 234,
  [SMALL_STATE(19)] = 243,
  [SMALL_STATE(20)] = 249,
  [SMALL_STATE(21)] = 255,
  [SMALL_STATE(22)] = 262,
  [SMALL_STATE(23)] = 266,
  [SMALL_STATE(24)] = 270,
  [SMALL_STATE(25)] = 274,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ledger, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_account_repeat1, 2),
  [11] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_account_repeat1, 2),
  [13] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_account_repeat1, 2), SHIFT_REPEAT(23),
  [16] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_transaction_repeat1, 2),
  [18] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_transaction_repeat1, 2),
  [20] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_transaction_repeat1, 2), SHIFT_REPEAT(4),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_account, 1),
  [25] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_account, 1),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_account, 2),
  [31] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_account, 2),
  [33] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_transaction, 3, .production_id = 9),
  [35] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_transaction, 3, .production_id = 9),
  [37] = {.entry = {.count = 1, .reusable = false}}, SHIFT(4),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declare_account_repeat1, 2, .production_id = 10),
  [41] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_declare_account_repeat1, 2, .production_id = 10),
  [43] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_declare_account_repeat1, 2, .production_id = 10), SHIFT_REPEAT(22),
  [46] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ledger, 1, .production_id = 3),
  [48] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declare_account, 2, .production_id = 4),
  [50] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_declare_account, 2, .production_id = 4),
  [52] = {.entry = {.count = 1, .reusable = false}}, SHIFT(22),
  [54] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2, .production_id = 5),
  [56] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2, .production_id = 5), SHIFT_REPEAT(21),
  [59] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 2, .production_id = 5), SHIFT_REPEAT(25),
  [62] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declare_account, 3, .production_id = 7),
  [64] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_declare_account, 3, .production_id = 7),
  [66] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_posting, 1, .production_id = 8),
  [68] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_posting, 1, .production_id = 8),
  [70] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [72] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_metadata, 3, .production_id = 12),
  [74] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_metadata, 3, .production_id = 12),
  [76] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_posting, 2, .production_id = 11),
  [78] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_posting, 2, .production_id = 11),
  [80] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [82] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [84] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [86] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declare_account_repeat1, 1, .production_id = 6),
  [88] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_declare_account_repeat1, 1, .production_id = 6),
  [90] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 1, .production_id = 2),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ledger_repeat1, 1, .production_id = 1),
  [94] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [96] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [98] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [100] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
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
