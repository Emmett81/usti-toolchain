#ruledef
{
    push {a}, {sp}, {se} => asm
    {
        sub.eq sp, se, T, end 
        sub sp, 1, sp
        mov a, sp
        end: 
    }



    pop {y}, {sp}, {ss} => asm
    {
        mov sp, a
        sub.eq sp, ss, T, end
        add sp, 1, sp
        end: 
    }
}