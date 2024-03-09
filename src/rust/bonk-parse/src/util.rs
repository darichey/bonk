use crate::ast;
use tree_sitter::{InputEdit, Point};

pub fn position_to_byte_offset(text: &str, line: usize, col: usize) -> usize {
    let mut cur_line = 0;
    let mut cur_col = 0;

    for (offset, c) in text.char_indices() {
        if cur_line == line && cur_col == col {
            return offset;
        }

        if c == '\n' {
            cur_line += 1;
            cur_col = 0;
        } else {
            cur_col += 1;
        }
    }

    text.len()
}

pub fn byte_offset_to_position(text: &str, offset: usize) -> (usize, usize) {
    let mut cur_line = 0;
    let mut cur_col = 0;

    for (o, c) in text.char_indices() {
        if o == offset {
            break;
        }

        if c == '\n' {
            cur_line += 1;
            cur_col = 0;
        } else {
            cur_col += 1;
        }
    }

    (cur_line, cur_col)
}

#[allow(clippy::too_many_arguments)] // I'm fine with this for now
pub fn edit_ledger(
    ledger: &mut ast::Ledger,
    old_src: &str,
    new_src: &str,
    start_line: usize,
    start_col: usize,
    end_line: usize,
    end_col: usize,
    change_length: usize,
) {
    let start_byte = position_to_byte_offset(old_src, start_line, start_col);
    let new_end_byte = start_byte + change_length;
    let new_end_position = byte_offset_to_position(new_src, new_end_byte);

    ledger.0.edit(&InputEdit {
        start_byte,
        old_end_byte: position_to_byte_offset(old_src, end_line, end_col),
        new_end_byte,
        start_position: Point {
            row: start_line,
            column: start_col,
        },
        old_end_position: Point {
            row: end_line,
            column: end_col,
        },
        new_end_position: Point {
            row: new_end_position.0,
            column: new_end_position.1,
        },
    });
}

#[cfg(test)]
mod tests {
    use crate::{
        util::{edit_ledger, position_to_byte_offset},
        Parser,
    };

    #[test]
    fn test_edit() {
        let src = r#"2023-01-01 "Mcdonald's"
  expenses/fast_food          10.91
  liabilities/my_credit_card -10.91"#;

        let mut ledger = Parser::new().parse(src, None);

        let old_src = src;
        let new_src = r#"foo "Mcdonald's"
  expenses/fast_food          10.91
  liabilities/my_credit_card -10.91"#;

        edit_ledger(&mut ledger, old_src, new_src, 0, 0, 0, 10, 3);

        insta::assert_debug_snapshot!(
            ledger,
            @r###"
        (ledger [0, 0] - [2, 35]
          transaction: (transaction [0, 0] - [2, 35]
            date: (date [0, 0] - [0, 3])
            description: (string [0, 4] - [0, 16])
            posting: (posting [1, 2] - [1, 35]
              account: (account [1, 2] - [1, 20])
              amount: (number [1, 30] - [1, 35]))
            posting: (posting [2, 2] - [2, 35]
              account: (account [2, 2] - [2, 28])
              amount: (number [2, 29] - [2, 35]))))
        "###
        );
    }

    #[test]
    fn test_position_to_byte_offset() {
        let s = "foo\nbars\nbazzz";

        assert_eq!(position_to_byte_offset(s, 0, 0), 0);

        assert_eq!(position_to_byte_offset(s, 0, 2), 2);

        assert_eq!(position_to_byte_offset(s, 1, 0), 4);

        assert_eq!(position_to_byte_offset(s, 2, 3), 12);

        assert_eq!(position_to_byte_offset(s, 2, 5), 14)
    }
}
