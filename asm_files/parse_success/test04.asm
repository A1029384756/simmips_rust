	# test of basic logical instructions
	.data
	TRUE = 1
	FALSE = 0

test1:	.word 1
test2:	.word 0
	
	.text
main:
	lw	$t0, test1
	lw	$t1, test2
	
	and	$t2, $t0, $t1
	andi	$t2, $t0, TRUE
	nor	$t2, $t0, $t1
	or	$t2, $t0, $t1
	ori	$t2, $t0, TRUE
	
