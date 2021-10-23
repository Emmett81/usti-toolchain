#once

#bits 32

#ruledef
{
    sub.neg {a:u32}, {b:u32}, {r:u32}, {j:u32} => a`32 @ b`32 @ r`32 @ j`32

    sub {a}, {b}, {r} => asm { sub.neg a, b, r, $+4 }

    inv {a}, {y} => asm { sub.neg Z, a, y, $+4 }

    jmp {j} => asm { sub.neg Z, Z+1, T, j }

    call {a}, {b}, {r}, {j}, {addr} => asm
    {
        mov a, ra
        mov b, rb 
        mov r, rr 
        mov j, rj
        mov $+8, rv
        jmp addr 
    }

    sub.lt {a}, {b}, {r}, {j} => asm 
    { 
        sub.neg a, b, r, j 
    }

    sub.le {a}, {b}, {r}, {j} => asm
    {
        sub.lt a, b, r, j 
        sub.lt r, Z+1, T, j 
    }

    sub.eq {a}, {b}, {r}, {j} => asm
    {
        sub.lt a, b, r, end 
        sub.gt r, Z, T, end 
        jmp j  
        end:
    }
    
    sub.ge {a}, {b}, {r}, {j} => asm
    {
        sub.gt a, b, r, j 
        sub.lt r, Z+1, T, j                                                   
    }
    
    sub.gt {a}, {b}, {r}, {j} => asm
    {
        sub.le a, b, r, end
        jmp j 
        end:
    }
    
    sub.ne {a}, {b}, {r}, {j} => asm
    {
        sub.eq a, b, r, end 
        jmp j 
        end:
    }

    nop => asm { sub.neg Z, Z, Z, $+4 }
    mov {a}, {y} => asm { sub a, Z, y }
    hlt => asm { sub Z, Z, 0x80000000 }
    rst => asm { sub Z, Z, 0x80000002 }
}

; default handlers
rstAddr:
nop
jmp main

vecAddr: 
nop
jmp main

; <- insert object code

reset:
mov deadcode, r0 
hlt  

;* Stack *;

SP:
#d32 Stack_Start

Stack_End: ; <- real Stack here
#d32 0  ; <- real Stack here
#d32 0  ; <- real Stack here
#d32 0  ; <- real Stack here
#d32 0  ; <- real Stack here
#d32 0  ; <- real Stack here
Stack_Start:  ; <- real Stack here


;* Registers *;

T:
#d32 0

ra:
#d32 0
rb:
#d32 0
rr:
#d32 0
rj:
#d32 0
rv:
#d32 0

r0:
#d32 0
r1:
#d32 0
r2:
#d32 0
r3:
#d32 0
r4:
#d32 0

;* VALUES *;

Z:
#d32 0
#d32 1

deadcode:
#d32 0xDEADC0DE

main:
rst