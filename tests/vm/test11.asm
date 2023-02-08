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
        div $t0, $t1 # 2/4 = 0 rem 2
        mflo $t8
        mfhi $t9

        lw $t0, var2
        lw $t1, var1
        div $t0, $t1 # -2/2 = -1 rem 0 
        mflo $t8
        mfhi $t9

        lw $t0, var3
        li $t1, VALUE
        div $t0, $t1 # 1073741824/4 = 268435456 rem 0
        mflo $t8
        mfhi $t9

        lw $t0, var3
        li $t1, VALUE2
        div $t0, $t1 # 1073741824/-4 = -268435456 rem 0 
        mflo $t8
        mfhi $t9

        # divide by 0
        div $t0, $0
	
