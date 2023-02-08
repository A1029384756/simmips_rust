        .data
VALUE = 2
var1:   .word 1
var2:   .word 12
var3:   .word -1
        .text
main:
        lw $t0, var1
        lw $t1, var2
        lw $t2, var3
        subu $t3, $t0, VALUE # 1-2 = -1 = 4294967295
        subu $t4, $t1, $t0 # 12-1 = 11
        subu $t5, $t2, VALUE # -1-2 = -3 = 4294967293
	
