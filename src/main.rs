use core::ascii;

enum KindToken {
    PLUS,
    MINUS,
    DOT,
    LEFTARROW,
    RIGHTARROW
}

#[warn(dead_code)]
fn kind_token_to_string(kind: KindToken) -> String {
    match kind {
        KindToken::PLUS =>       "+".to_string(),
        KindToken::MINUS =>      "-".to_string(),
        KindToken::DOT =>        ".".to_string(),
        KindToken::LEFTARROW =>  "<".to_string(),
        KindToken::RIGHTARROW => ">".to_string(),
        _ => panic!("( kind_token_to_string ) KindToken invalid")
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
            '+' => tokens.push(Token { kind: KindToken::PLUS }),
            '-' => tokens.push(Token { kind: KindToken::MINUS }),
            '.' => tokens.push(Token { kind: KindToken::DOT }),
            '<' => tokens.push(Token { kind: KindToken::LEFTARROW }),
            '>' => tokens.push(Token { kind: KindToken::RIGHTARROW }),
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
    if cursor > 0 {
        cells[cursor] -= 1;
    } else {
        panic!("( cell_decrement ) cursor < size cells ");
    }
}	

fn cell_write(cells: &Vec<u8>, cursor: usize) {
    print!("{}", cells[cursor] as char);
}

fn fuck_interpreter(tokens: Vec<Token>, cells: &mut Vec<u8>, cursor: &mut usize) {
    for token in tokens {
        match token.kind {
            KindToken::PLUS => cell_increment(cells, *cursor),
            KindToken::MINUS => cell_decrement(cells, *cursor),
            KindToken::DOT => cell_write(cells, *cursor),
            KindToken::LEFTARROW => cursor_decrement(cursor),
            KindToken::RIGHTARROW => cursor_increment(cursor),
        }
    }
}

fn main() {
    let source: String = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++ ++++ ++.".to_string();
    let mut cursor: usize = 0; 
    let mut data: Vec<u8> = vec![0; 30000]; // Initialize cells with a size of 30,000
    
    let tokens = fuck_tokenizer(&source);
    fuck_interpreter(tokens, &mut data, &mut cursor);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let source = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.".to_string();
        let tokens = fuck_tokenizer(&source);
        
        // Verify the number of tokens is correct
        assert_eq!(tokens.len(), 43);

        // Verify all tokens are of type PLUS
        for token in tokens {
            match token.kind {
                KindToken::PLUS => println!("PLUS kind: {}", kind_token_to_string(token.kind)),
                KindToken::MINUS => println!("MINUS kind: {}", kind_token_to_string(token.kind)),
                KindToken::DOT=> println!("DOT kind: {}", kind_token_to_string(token.kind)),
                KindToken::LEFTARROW => println!("LEFTARROW kind: {}", kind_token_to_string(token.kind)),
                KindToken::RIGHTARROW => println!("RIGHTARROW kind: {}", kind_token_to_string(token.kind)),
            }
        }
    }

    #[test]
    fn test_interpreter() {
        let source = "+++.+++.".to_string();
        let tokens = fuck_tokenizer(&source);
        let mut cells = vec![0; 30000]; // Initialize cells
        let mut cursor: usize = 0;

        fuck_interpreter(tokens, &mut cells, &mut cursor);

        // Check the values in the cells after interpretation
        assert_eq!(cells[cursor], 3); // After "+++" the cell should be 3
        // Additional assertions can be added based on the expected behavior
    }
}
