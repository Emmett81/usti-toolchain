#bits 32

#ruledef
{
    sub.neg {a:u32}, {b:u32}, {r:u32}, {j:u32} => a`32 @ b`32 @ r`32 @ j`32

    sub {a}, {b}, {r} => asm { sub.neg a, b, r, $+4 }

    jmp {j} => asm { sub.neg Z, Z+1, T, j }
    
    sub.lt {a}, {b}, {r}, {j} => asm { sub.neg a, b, r, j }

    sub.le {a}, {b}, {r}, {j} => asm
    {
        sub.neg a, b, r, j 
        sub.neg r, Z+1, T, j 
    }

    sub.eq {a}, {b}, {r}, {j} => asm
    {
        sub.ne a, b, r, end 
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

; <- insert object code here

reset:
mov deadcode, r0 
hlt  

r0:
#d32 0

; <- include registers here

T:
#d32 0

Z:
#d32 0
#d32 1

; <- include numbers here

deadcode:
#d32 0xDEADC0DE

main:
hlt