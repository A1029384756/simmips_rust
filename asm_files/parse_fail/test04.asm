	# a test of basic arithmetic operations
	.data
x:	.word	1
y:	.word	1
z:	.space 	4

	.text
main:
	lw 	$t0, x
	lw 	$t1, y
	add 	$t2, $t0, $t1
	add 	$t2, $t0, 2
	addu 	$t2, $t0, $t1
	addu 	$t2, $t0, 2
	sub 	$t2, $t0, $t1
	sub 	$t2, $t0, 2
	subu 	$t2, $t0, $t1
	subu 	$t2, $t0, 2
	mul 	$t2, $t0, $t1
	mul 	$t2, $t0, 2
	mulo 	$t2, $t0, $t1
	mulo 	$t2, $t0, 2
	mulou 	$t2, $t0, $t1
	mulou 	$t2, $t0, 2
	mult	$t0, $t1
	multu	$t0, $t1
	div 	$t2, $t0, $t1
	div 	$t2, $t0, 2
	divu 	$t2, $t0, $t1
	divu 	$t2, $t0, 2
	div	$t0, $t1
	divu	$t0, $t1
	rem 	$t2, $t0, $t1
	rem 	$t2, $t0	# parse error
	remu 	$t2, $t0, $t1
	remu 	$t2, $t0, 2
	abs	$t0, $t1
	neg	$t0, $t1
	negu	$t0, $t1
	
