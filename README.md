# RISC-V简易汇编器和反汇编器

这是一个简单的RISC-V汇编工具集，包含汇编器和反汇编器功能。

## 功能

### 汇编器功能
- 支持基本的RISC-V指令集（add, mul, addi, bne, lui, lw, sw, blt, halt）
- 将汇编代码转换为二进制机器码
- 输出二进制文件(.o)和可读的文本表示(.txt)

### 反汇编器功能
- 将二进制指令文件转换回汇编代码
- 支持所有指令类型的反汇编（A型、B型、C型）
- 输出文件包含每条指令的十六进制表示和反汇编后的汇编代码

## 用法

```bash
# 汇编功能
./riscv-tools asm <汇编文件名>

# 例如：编译asm/sum.asm，输出到out/sum.o和out/sum.txt
./riscv-tools asm sum

# 反汇编功能
./riscv-tools disasm <输入二进制文件> <输出汇编文件>

# 例如：反汇编out/sum.o为out/sum_disasm.asm
./riscv-tools disasm out/sum.o out/sum_disasm.asm
```

## 指令格式

本工具支持以下三种指令格式：

### A类型指令（add/mul）
- 格式: 前11位0 + rs2[5位] + rs1[5位] + rd[5位] + opcode[6位]
- 示例: `add x1, x2, x3` → `0b00000000000_00011_00010_00001_000001`

### B类型指令（addi/lui/lw）
- 格式: imm[16位] + rs1[5位] + rd[5位] + opcode[6位]
- 示例: `addi x1, x0, 10` → `0b00000000000_01010_00000_00001_000010`

### C类型指令（bne/sw/blt）
- 格式: imm_high[11位] + rs2[5位] + rs1[5位] + imm_low[5位] + opcode[6位]
- 示例: `bne x3, x2, -8` → `0b11111111111_00010_00011_11000_000011`

### 特殊指令
- halt: 全0指令 `0b00000000000_00000_00000_00000_000000`

## 目录结构

```
.
├── asm/           # 汇编源文件目录
│   ├── sum.asm            # 计算1到10的和的汇编程序
│   └── ...
├── out/           # 输出文件目录（程序运行时自动创建）
│   ├── *.o        # 二进制机器码文件（小端序）
│   └── *.txt      # 可读的二进制表示文件
├── src/           # 源代码目录
│   └── main.rs    # 主程序代码
└── Cargo.toml     # 项目配置文件
``` 