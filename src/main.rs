use brain::brain_fuck;

fn main() {
    let source: String = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.".to_string();
    let mut cursor: usize = 0;
    let mut data: Vec<u8> = vec![0; 30000]; 
    let tokens = brain_fuck::tokenizer(&source);
    
    brain_fuck::interpreter(tokens, &mut data, &mut cursor);
    
    println!(); 
}
