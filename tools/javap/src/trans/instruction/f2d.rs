use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct F2D;

impl Instruction for F2D {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::f2d,
            icp: 0,
            wide: false,
        };

        (info, pc + 1)
    }
}
