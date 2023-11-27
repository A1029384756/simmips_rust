	# a test for basic data movement
	.data
avar:	.word 	0
bvar:	.half	1
cvar:	.byte	3

	.text
main:	
	lui $t0, 45
  srl $t0, $t0, 16
	lw $t1, avar
	lhu $t2, bvar
	lbu $t2, cvar
	sw $t1, avar
	sh $t2, bvar
	sb $t2, cvar

	addi $t0, $0, 0
	
