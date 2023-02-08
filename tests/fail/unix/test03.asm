	# a test for basic data movement
	.data
avar:	.word 	0
bvar:	.half	1
cvar:	.byte	3

	.text
main:	
	li $t0, $t1 # Parse Error
	lw $t1, avar
	lh $t2, bvar
	lb $t2, cvar
	sw $t1, avar
	sh $t2, bvar
	sb $t2, cvar

	move $t0, $0
	
