use crate::program::REGISTER_NUM;
use crate::trace::instruction::{Instruction, Opcode};
use serde::{Serialize, Deserialize};
use plonky2::field::goldilocks_field::GoldilocksField;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub clk: u32,
    pub pc: u64,
    // todo delete , use r15
    pub fp: u64,
    //todo for debug
    pub instruction: Instruction,
    pub regs: [GoldilocksField; REGISTER_NUM],
    pub flag: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Trace {
    pub raw_instructions: Vec<Instruction>,
    // todo need the compiler raw input
    pub raw_binary_instructions: Vec<(String, Option<String>)>,
    // todo need limit the trace size
    pub exec: Vec<Step>,
    pub memory:  Vec<String>,
}

impl Trace {
    pub fn insert_step(& mut self, clk:u32, pc: u64, fp: u64, instruction: Instruction, regs: [GoldilocksField; REGISTER_NUM], flag: bool, v_addr: Option<u32>) {

        let step = Step {
            clk,
            pc,
            fp,
            instruction,
            regs,
            flag,
        };
        self.exec.push(step);
    }
}