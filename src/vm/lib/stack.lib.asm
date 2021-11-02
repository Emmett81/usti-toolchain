STACK_SIZE = b13

#ruledef
{    
    stack.new {size}, {sp} => asm
    {
        
        mov HEAP, sp 
        alloc size, T 
    }

    stack.push {a}, {sp} => asm
    {
        dec sp 
        mov sp, $+6 ; set R in instr below
        mov a, T 
    } 

    stack.pop {y}, {sp} => asm 
    {
        mov sp, $+4 ; set A in instr below
        mov T, y 
        inc sp
    }

    push {a} => asm
    {
        stack.push a, SP 
    }

    pop {y} => asm 
    {
        stack.pop y, SP
    }
}

SP:
#d32 0
