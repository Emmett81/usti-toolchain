#ruledef
{
    mov.lt {a}, {y}, {j} => asm
    {
        sub.lt a, Z, y, j
    }

    mov.le {a}, {y}, {j} => asm
    {
        sub.le a, Z, y, j 
    }

    mov.eq {a}, {y}, {j} => asm
    {
        sub.eq a, Z, y, j
    }

    mov.ge {a}, {y}, {j} => asm
    {
       sub.ge a, Z, y, j 
    }

    mov.gt {a}, {y}, {j} => asm
    {
        sub.gt a, Z, y, j
    }

    mov.ne {a}, {y}, {j} => asm
    {
        sub.ne a, Z, y, j      
    }

    mov.val {value}, {y} => asm
    {
        mov value, T ; output addr to have {value}=$-4
        mov $-4, y   ; mov {value}, y
    }

}