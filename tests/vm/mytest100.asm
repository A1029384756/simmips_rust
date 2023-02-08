	.data

	.text
main:
	li $t0, 0
	li $t1, 1	
loop:	add $t2, $t0, $t1
	move $t0, $t2
	j loop
