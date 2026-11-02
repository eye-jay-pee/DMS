const SIGN_MODULUS: u8 = 2;
    const DEGREES_MODULUS: u16 = 360;
    const MINUTES_MODULUS: u8 = 60;
    const SECONDS_MODULUS: u8 = 60;
    
    
    const SIGN_BIT_WIDTH: 1;
    const DEGREES_BIT_WIDTH: usize = 10;
    const MINUTES_BIT_WIDTH: usize = 7;
    const SECONDS_BIT_WIDTH: usize = 15;
    
    const SIGN_BIT_OFFSET: usize = 
        SECONDS_BIT_WIDTH + MINUTES_BIT_WIDTH + DEGREES_BIT_WIDTH;
    const DEGREES_BIT_OFFSET: usize = SECONDS_BIT_WIDTH + MINUTES_BIT_WIDTH;
    const MINUTES_BIT_OFFSET: usize = SECONDS_BIT_WIDTH;
    const SECONDS_BIT_OFFSET: usize = 0;
    
    const DEGREES_BIT_MASK: u32 = (1 << DEGREES_BIT_WIDTH) - 1;
    const MINUTES_BIT_MASK: u32 = (1 << MINUTES_BIT_WIDTH) - 1;
        const SECONDS_BIT_MASK: u32 = (1 << SECONDS_BIT_WIDTH) - 1;
    

