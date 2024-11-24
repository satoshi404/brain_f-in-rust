#[derive(Debug)]
#[derive(PartialEq, Eq)]
enum KindToken {
    Plus,
    Minus,
    Dot,
    LeftArrow,
    LeftBracket,
    RightBracket,
    RightArrow
}

#[allow(dead_code)]
fn kind_token_to_string(kind: KindToken) -> String {
    match kind {
        KindToken::Plus => "+".to_string(),
        KindToken::Minus => "-".to_string(),
        KindToken::Dot => ".".to_string(),
        KindToken::LeftBracket => "[".to_string(),
        KindToken::RightBracket => "]".to_string(),
        KindToken::LeftArrow => "<".to_string(),
        KindToken::RightArrow => ">".to_string()
    }
}

struct Token {
    kind: KindToken,
}

fn fuck_tokenizer(path: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for ch in path.chars() {
        match ch {
            ' ' => continue,
            '+' => tokens.push(Token { kind: KindToken::Plus }),
            '-' => tokens.push(Token { kind: KindToken::Minus }),
            '.' => tokens.push(Token { kind: KindToken::Dot }),
            '[' => tokens.push(Token { kind: KindToken::LeftBracket }),
            ']' => tokens.push(Token { kind: KindToken::RightBracket }),
            '<' => tokens.push(Token { kind: KindToken::LeftArrow }),
            '>' => tokens.push(Token { kind: KindToken::RightArrow }),
            _ => { panic!("( fuck_tokenizer ) Token invalid")} 
        }
    }
    tokens
}

fn cursor_increment(cursor: &mut usize) {
    if *cursor < 30000 {
        *cursor += 1;
    } else {
        panic!("( cursor_increment ) cursor out of bounds");
    }
}

fn cursor_decrement(cursor: &mut usize) {
    if *cursor > 0 {
        *cursor -= 1;
    } else {
        panic!("( cursor_decrement ) cursor < 0");
    }
}   

fn cell_increment(cells: &mut Vec<u8>, cursor: usize) {
    cells[cursor] += 1;
}

fn cell_decrement(cells: &mut Vec<u8>, cursor: usize) {
    cells[cursor] -= 1;
}   

fn cell_write(cells: &Vec<u8>, cursor: usize) {
    print!("{}", cells[cursor] as char);
}

fn fuck_interpreter(tokens: Vec<Token>, cells: &mut Vec<u8>, cursor: &mut usize) {
    for token in tokens {
        match token.kind {
            KindToken::Plus => cell_increment(cells, *cursor),
            KindToken::Minus => cell_decrement(cells, *cursor),
            KindToken::Dot => cell_write(cells, *cursor),
            KindToken::LeftBracket => println!("The token '[' not implemented yet"),
            KindToken::RightBracket => println!("The token ']' not implemented yet"),
            KindToken::LeftArrow => cursor_decrement(cursor),
            KindToken::RightArrow => cursor_increment(cursor),
        }
    }
}

fn main() {
    let source: String = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++ ++++ ++.[]".to_string();
    let mut cursor: usize = 0; 
    let mut data: Vec<u8> = vec![0; 30000]; // Initialize cells with a size of 30,000
    
    let tokens = fuck_tokenizer(&source);
    fuck_interpreter(tokens, &mut data, &mut cursor);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind};

    #[test]
    fn test_tokenizer() {
        let source = "+-.[++]<>".to_string();
        let tokens = fuck_tokenizer(&source);
        
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

        cursor_increment(&mut cursor);
        assert_eq!(cursor, 1);

        cursor_decrement(&mut cursor);
        assert_eq!(cursor, 0);

        // Test out-of-bounds increment
        for _ in 0..30000 {
            cursor_increment(&mut cursor);
        }
        assert_eq!(cursor, 30000);

        let cursor2 = 30000;
        let result = catch_unwind(|| {
            let mut cursor2 = cursor2;
            cursor_increment(&mut cursor2);
        });
        assert!(result.is_err());

        // Test out-of-bounds decrement
        for _ in 0..30000 {
            cursor_decrement(&mut cursor);
        }
        assert_eq!(cursor, 0);

        let cursor3 = 0;
        let result = catch_unwind(|| {
            let mut cursor3 = cursor3;
            cursor_decrement(&mut cursor3);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_cell_operations() {
        let mut cells = vec![0; 30000];
        let cursor: usize = 0;

        cell_increment(&mut cells, cursor);
        assert_eq!(cells[cursor], 1);

        cell_decrement(&mut cells, cursor);
        assert_eq!(cells[cursor], 0);
    }
}
