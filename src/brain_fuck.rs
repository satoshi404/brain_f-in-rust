#[derive(Debug, PartialEq, Eq)]
pub enum KindToken {
    Plus,
    Minus,
    Dot,
    LeftArrow,
    LeftBracket,
    RightBracket,
    RightArrow,
}

#[allow(dead_code)]
pub fn kind_token_to_string(kind: KindToken) -> String {
    match kind {
        KindToken::Plus => "+".to_string(),
        KindToken::Minus => "-".to_string(),
        KindToken::Dot => ".".to_string(),
        KindToken::LeftBracket => "[".to_string(),
        KindToken::RightBracket => "]".to_string(),
        KindToken::LeftArrow => "<".to_string(),
        KindToken::RightArrow => ">".to_string(),
    }
}

pub struct Token {
    pub kind: KindToken,
}

pub fn tokenizer(path: &str) -> Vec<Token> {
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
            _ => panic!("( tokenizer ) Token invalid"),
        }
    }
    tokens
}

pub fn cursor_inc(cursor: &mut usize) {
    if *cursor < 30000 {
        *cursor += 1;
    } else {
        panic!("( cursor_increment ) cursor out of bounds");
    }
}

pub fn cursor_dec(cursor: &mut usize) {
    if *cursor > 0 {
        *cursor -= 1;
    } else {
        panic!("( cursor_decrement ) cursor < 0");
    }
}

pub fn cell_inc(cells: &mut Vec<u8>, cursor: usize) {
    cells[cursor] += 1;
}

pub fn cell_dec(cells: &mut Vec<u8>, cursor: usize) {
    cells[cursor] -= 1;
}

pub fn cell_write(cells: &Vec<u8>, cursor: usize) {
    print!("{}", cells[cursor] as char); 
}

pub fn interpreter(tokens: Vec<Token>, cells: &mut Vec<u8>, cursor: &mut usize) {
    let mut pc = 0; // Program counter
    let mut loop_stack = Vec::new();

    while pc < tokens.len() {
        match &tokens[pc].kind {
            KindToken::Plus => cell_inc(cells, *cursor),
            KindToken::Minus => cell_dec(cells, *cursor),
            KindToken::Dot => cell_write(cells, *cursor), 
            KindToken::LeftBracket => {
                if cells[*cursor] == 0 {
                    let mut depth = 1;
                    while depth > 0 && pc < tokens.len() - 1 {
                        pc += 1;
                        if tokens[pc].kind == KindToken::LeftBracket {
                            depth += 1;
                        } else if tokens[pc].kind == KindToken::RightBracket {
                            depth -= 1;
                        }
                    }
                } else {
                    loop_stack.push(pc);
                }
            },
            KindToken::RightBracket => {
                if cells[*cursor] != 0 {
                    if let Some(last) = loop_stack.last() {
                        pc = *last; 
                    }
                } else {
                    loop_stack.pop(); 
                }
            },
            KindToken::LeftArrow => cursor_dec(cursor),
            KindToken::RightArrow => cursor_inc(cursor),
        }
        pc += 1; 
    }
}
