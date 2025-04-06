fn main() {
    use std::fs;
    use std::io::{self, Read};
    use std::path::Path;

    // 常量定义
    const OPCODE_BLT: u32 = 0b001000;   // blt 如果 rs1 <s rs2，则 pc += sext(offset)

    // C类型指令编码（bne/sw/blt）
    // 格式: imm_high[31:21] rs2[20:16] rs1[15:11] imm_low[10:6] opcode[5:0]
    fn encode_c(opcode: u32, rs1: u8, rs2: u8, offset: i16) -> u32 {
        // 处理有符号扩展
        let offset_u32 = offset as u32;
        // 提取高11位和低5位
        let imm_high = (offset_u32 >> 5) & 0x7FF;
        let imm_low = offset_u32 & 0x1F;
        
        (imm_high << 21) |
        ((rs1 as u32) << 16) |  // rs1放在[20:16]
        ((rs2 as u32) << 11) |  // rs2放在[15:11]
        (imm_low << 6) |
        (opcode & 0x3F)
    }

    fn encode_blt(rs1: u8, rs2: u8, offset: i16) -> u32 {
        encode_c(OPCODE_BLT, rs1, rs2, offset)
    }

    // 测试 blt x5, x6, +12
    let expected = 0b00000000000_00110_00101_01100_001000;
    let result = encode_blt(5, 6, 12);
    
    println!("Expected: {:032b}", expected);
    println!("Result:   {:032b}", result);
    println!("Expected formatted: 0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (expected >> 21) & 0x7FF,
        (expected >> 16) & 0x1F,
        (expected >> 11) & 0x1F,
        (expected >> 6) & 0x1F,
        expected & 0x3F);
    println!("Result formatted:   0b{:011b}_{:05b}_{:05b}_{:05b}_{:06b}", 
        (result >> 21) & 0x7FF,
        (result >> 16) & 0x1F,
        (result >> 11) & 0x1F,
        (result >> 6) & 0x1F,
        result & 0x3F);
    
    if result == expected {
        println!("Test passed!");
    } else {
        println!("Test failed!");
    }
}
