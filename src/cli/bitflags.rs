use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        const RECURSIVE         = 0b00001;
        const NO_DEREFERENCE    = 0b00010;
        const FORCE             = 0b00100;
        const UPDATE            = 0b01000;
        const VERBOSE           = 0b10000;
    }
}