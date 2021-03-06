#![allow(non_camel_case_types)]
use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Dconst_0;

impl Instruction for Dconst_0 {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::dconst_0,
            icp: 0,
            wide: false,
        };

        (info, pc + 1)
    }
}
