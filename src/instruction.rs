pub enum InstructionEnum {
    End,
    Print,
    Add,
    Subtract,
    Multiply,
    Divide,
    Input,
    Push,
    Pop,
    LoopBegin,
    LoopEnd,
    Set,
}

#[derive(Clone, Copy)]
pub struct DuckInstruction {
    pub op_code: usize,
    pub n: usize,
    pub y: usize,
    pub arg_c: usize,
}