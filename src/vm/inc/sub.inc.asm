 #ruledef
 {
    sub {a}, {b}, {r} => asm { sub.neg a, b, r, $+4 }

    dec {x} => asm
    {
        sub x, 1, x
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
        sub.lt r, Z+1, r1, j
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
 }
