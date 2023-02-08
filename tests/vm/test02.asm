        .data
r1:     .space 4
r2:     .space 12
r3:     .space 4
var:    .word 7

        .text
main:
        la $t0, r2
     	lw $t1, var

	sw $t1, 0
	sw $t1, $t0
	sw $t1, 4($t0)
	sw $t1, 8(r2)
	sw $t1, r3
	
	
