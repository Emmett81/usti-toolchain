#once

#include "vm.asm"

#ruledef
{
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