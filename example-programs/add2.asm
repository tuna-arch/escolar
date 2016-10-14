bits 16

mov 0x2, r0
addp out, r0

mov 0x0, r1 # we want to print to OUT
outp r1, r0 # print value of r0 to OUT.
