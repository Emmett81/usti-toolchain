#ruledef 
{
    emit {x:u8} => x`8

    op => asm {
        emit $+4
    }
}

op