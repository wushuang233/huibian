# 反汇编结果
# 格式: [地址] [十六进制表示] [汇编指令]

0000:  800001C5  lui x7, -32768
0004:  004C39C2  addi x7, x7, 76
0008:  FFFC3846  lw x1, -4(x7)
000C:  00020849  slli x1, x1, 2
0010:  00040082  addi x2, x0, 4
0014:  000000C1  add x3, x0, x0
0018:  00020A0A  sub x8, x1, x2
001C:  00071901  add x4, x3, x7
0020:  00002146  lw x5, 0(x4)
0024:  00042186  lw x6, 4(x4)
0028:  00062B08  blt x6, x5, 12
002C:  00062007  sw x4, 0(x6)
0030:  00052107  sw x4, 4(x5)
0034:  000418C2  addi x3, x3, 4
0038:  FFE34103  bne x3, x8, -28
003C:  00041082  addi x2, x2, 4
0040:  FFC20D03  bne x2, x1, -44
0044:  00000000  halt
