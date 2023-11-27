	# a test of basic arithmetic operations
	.data
x:	.word	1
y:	.word	1
z:	.space 	4

	.text
main:
  lw 	  $t0, x
  lw 	  $t1, y
  add 	$t2, $t0, $t1
  addi 	$t2, $t0, 2
  addu  $t2, $t0, $t1
  addiu $t2, $t0, 2
  sub 	$t2, $t0, $t1
  subu 	$t2, $t0, $t1
	
