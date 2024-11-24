
use brain_fuck_in_rust::tokenizer;
use brain_fuck_in_rust::cursor_inc;
use brain_fuck_in_rust::cursor_dec;
use brain_fuck_in_rust::cell_inc;
use brain_fuck_in_rust::cell_dec;




#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind};

    #[test]
    fn test_tokenizer() {
        let source = "+-.[++]<>".to_string();
        let tokens = tokenizer(&source);

        // Verify the number of tokens is correct
        assert_eq!(tokens.len(), 9);

        // Verify all tokens are of the correct type
        let kinds = vec![
            KindToken::Plus,
            KindToken::Minus,
            KindToken::Dot,
            KindToken::LeftBracket,
            KindToken::Plus,
            KindToken::Plus,
            KindToken::RightBracket,
            KindToken::LeftArrow,
            KindToken::RightArrow,
        ];

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind, kinds[i]);
        }
    }

    #[test]
    fn test_cursor_operations() {
        let mut cursor = 0;

        cursor_inc(&mut cursor);
        assert_eq!(cursor, 1);

        cursor_dec(&mut cursor);
        assert_eq!(cursor, 0);

        // Test out-of-bounds increment
        for _ in 0..30000 {
            cursor_inc(&mut cursor);
        }
        assert_eq!(cursor, 30000);

        let cursor2 = 30000;
        let result = catch_unwind(|| {
            let mut cursor2 = cursor2;
            cursor_inc(&mut cursor2);
        });
        assert!(result.is_err());

        // Test out-of-bounds decrement
        for _ in 0..30000 {
            cursor_dec(&mut cursor);
        }
        assert_eq!(cursor, 0);

        let cursor3 = 0;
        let result = catch_unwind(|| {
            let mut cursor3 = cursor3;
            cursor_dec(&mut cursor3);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_cell_operations() {
        let mut cells = vec![0; 30000];
        let cursor: usize = 0;

        cell_inc(&mut cells, cursor);
        assert_eq!(cells[cursor], 1);
                                                                                                cell_dec(&mut cells, cursor);
        assert_eq!(cells[cursor], 0);
    }
}
