#once

#include "vm.asm"

#ruledef 
{
    add {a}, {b}, {r} => asm
    {
        inv b, r4 
        sub a, r4, r 
    }

    add.lt {a}, {b}, {r}, {j} => asm
    {
        inv b, r4 
        sub.lt a, r4, r, j 
    }

    add.le {a}, {b}, {r}, {j} => asm
    {
        inv b, r4 
        sub.le a, r4, r, j 
    }

    add.eq {a}, {b}, {r}, {j} => asm
    {
        inv b, r4 
        sub.eq a, r4, r, j 
    }

    add.ge {a}, {b}, {r}, {j} => asm
    {
        inv b, r4 
        sub.ge a, r4, r, j 
    }

    add.gt {a}, {b}, {r}, {j} => asm
    {
        inv b, r4 
        sub.gt a, r4, r, j 
    }

    add.ne {a}, {b}, {r}, {j} => asm
    {
        inv b, r4 
        sub.ne a, r4, r, j 
    }
}
