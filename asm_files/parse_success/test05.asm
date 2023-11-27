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

end:
	j	end
