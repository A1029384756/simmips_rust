	# test of basic logical instructions
	.data
	TRUE = 1
	FALSE = 0

test1:	.word TRUE
test2:	.word FALSE
	
	.text
main:
	lw	$t0, test1
	lw	$t1, test2
	
	and	$t2, $t0, $t1
	and	$t2, $t0, TRUE
	nor	$t2, $t0, $t1
	nor	$t2, $t0, TRUE
	not	$t2, $t0
	not	$t2, $t0
	or	$t2, $t0, $t1
	or	$t2, $t0, TRUE
	xor	$t2, $t0, $t1
	xor	$t2, $t0, TRUE
	
