use bytelox::OpCode;

fn main() {
    let instructions = vec![OpCode::from(1.2), OpCode::Return];

    for (index, instruction) in instructions.into_iter().enumerate() {
        println!("index: {}, {}", index, instruction);
    }
}
