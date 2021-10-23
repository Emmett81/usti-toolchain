#ruledef
{
    emit {a:u8} => a`8

    op {a} => asm
    {
        emit a 
    }

    op {a}, {b} => asm
    {
        emit a
        emit b
    }
}

op 10