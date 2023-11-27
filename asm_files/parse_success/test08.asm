  .data
  
  .text
main:
  addi $t0, $0, 0
  addi $t1, $0, 1	
loop:	
  add $t2, $t0, $t1
  j loop
