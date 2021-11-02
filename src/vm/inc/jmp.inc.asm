#ruledef
{
    ; unconditional jump to addr
    jmp {j} => asm { sub.neg Z, Z+1, T, j }

    ; unconditional jump to j stored at addr
    jmp.reg {addr} => asm
    {
        mov addr, $+7 ; replace 0xdead with addr
        jmp 0xdead 
    }

    jmp.lt {r}, {j} => asm 
    {
        sub.lt r, Z, T, j 
    }

    jmp.le {r}, {j} => asm
    {
        sub.le r, Z, T, j 
    }
 
    jmp.eq {r}, {j} => asm 
    {
        sub.eq r, Z, T, j 
    }

    jmp.ge {r}, {j} => asm
    {
        sub.ge r, Z, T, j 
    }

    jmp.gt {r}, {j} => asm 
    {
        sub.gt r, Z, T, j 
    }

    jmp.ne {r}, {j} => asm
    {
        sub.ne r, Z, T, j 
    }
}