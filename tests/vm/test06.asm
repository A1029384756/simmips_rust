        .data
VALUE = 12
var:    .word 31
        .text
main:
        lw $t0, var
        addu $t1, $t0, VALUE # 31+12=43
        addu $t2, $t1, $t0 # 43+31=74
	
