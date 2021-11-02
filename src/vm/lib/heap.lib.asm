#ruledef
{
    ; start = alloc(size)
    alloc {size}, {start} => asm 
    {
        sub HEAP, size, HEAP 
        mov HEAP, start 
    }
}

HEAP:
#d32 MEM_SIZE