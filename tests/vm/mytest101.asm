	.data
count:	.word 0
	.text
main:
	lw $t0, count
	li $t1, 1	
loop:	add $t2, $t0, $t1
	move $t0, $t2
	sw $t0, count
	j loop
