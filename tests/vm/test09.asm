        .data
VALUE = 4
VALUE2 = -4
var1:   .word 2
var2:   .word -2
var3:   .word 1073741824 # = 2^30
        .text
main:
        lw $t0, var1
        li $t1, VALUE
        mult $t0, $t1 # 2*4 = 8
        mflo $t8
        mfhi $t9

        lw $t0, var2
        lw $t1, var1
        mult $t0, $t1 # -2*2 = -4 
        mflo $t8
        mfhi $t9

        lw $t0, var3
        li $t1, VALUE
        mult $t0, $t1 # 1073741824*4 = 4294967296 (overflow)
        mflo $t8
        mfhi $t9

        lw $t0, var3
        li $t1, VALUE2
        mult $t0, $t1 # 1073741824*-4 = -4294967296 (overflow)
        mflo $t8
        mfhi $t9
	
