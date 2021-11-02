#bits 32

#ruledef
{
    sub.neg {a:u32}, {b:u32}, {r:u32}, {j:u32} => a`32 @ b`32 @ r`32 @ j`32

    inv {a}, {y} => asm { sub.neg Z, a, y, $+4 }
    nop => asm { sub.neg Z, Z, Z, $+4 }
    mov {a}, {y} => asm { sub a, Z, y }
    hlt => asm { sub Z, Z, 0x80000000 }
    rst => asm { sub Z, Z, 0x80000002 }
}

MEM_SIZE = 4096;

;* Instructions  *;

#include "inc/inc.asm"

;* Default handlers *;

rstAddr:
nop
jmp main

vecAddr: 
nop
jmp main

;* Libraries *;

#include "lib/lib.asm"

;* Dead man's switch *;

dead:
mov DEADCODE, r0 
hlt  

;* Start of program *;

main:

mov v24, r0
mov v12, r1

sub r0, r1, r2

end:
rst
sig:
hlt