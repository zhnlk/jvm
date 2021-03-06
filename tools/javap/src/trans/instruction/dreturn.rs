use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Dreturn;

impl Instruction for Dreturn {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::dreturn,
            icp: 0,
            wide: false,
        };

        (info, pc + 1)
    }
}
