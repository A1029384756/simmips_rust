	# test of the basic control instructions
	.data
x:	.word 1
y:	.word 2

	.text
main:
	lw $t0, x
	lw $t1, y
	
	beq $t0, $t0, next1
next1:

	bne $t0, $t1, next2
next2:

	blt $t0, $t1, next3
next3:

	ble $t0, $t0, next4
next4:

	bgt $t1, $t0, next5
next5:

	bge $t0, $t0, next6
next6:
	
end:
	j	end
