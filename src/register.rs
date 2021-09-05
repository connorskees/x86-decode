use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Register {
    Rax,
    Rbx,
    Rcx,
    Rdx,

    /// Register base pointer
    ///
    /// Start of stack
    Rbp,

    /// Register stack pointer
    ///
    /// Current location in stack
    ///
    /// This grows backwards
    Rsp,

    /// Register source index
    Rsi,

    /// Register destination index
    Rdi,

    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,

    Eax,
    Ebx,
    Ecx,
    Edx,
    Ebp,
    Esp,
    Esi,

    Cl,
}

impl Register {
    pub fn r64(b: u8) -> Self {
        match b & 0b1111 {
            0b0000 => Self::Rax,
            0b0001 => Self::Rcx,
            0b0010 => Self::Rdx,
            0b0011 => Self::Rbx,
            0b0100 => Self::Rsp,
            0b0101 => Self::Rbp,
            0b0110 => Self::Rsi,
            0b0111 => Self::Rdi,

            0b1000 => Self::R8,
            0b1001 => Self::R9,
            0b1010 => Self::R10,
            0b1011 => Self::R11,
            0b1100 => Self::R12,
            0b1101 => Self::R13,
            0b1110 => Self::R14,
            0b1111 => Self::R15,
            _ => unreachable!(),
        }
    }

    pub fn r32(b: u8) -> Self {
        match b & 0b1111 {
            0b0000 => Self::Eax,
            0b0001 => Self::Ecx,
            0b0010 => Self::Edx,
            0b0011 => Self::Ebx,
            0b0100 => Self::Esp,
            0b0101 => Self::Ebp,
            0b0110 => Self::Esi,
            b => todo!("{:b}", b),
        }
    }

    pub fn as_string(&self) -> &'static str {
        match self {
            Self::Rax => "rax",
            Self::Rcx => "rcx",
            Self::Rdx => "rdx",
            Self::Rbx => "rbx",
            Self::Rsp => "rsp",
            Self::Rbp => "rbp",
            Self::Rsi => "rsi",
            Self::Rdi => "rdi",
            Self::R8 => "r8",
            Self::R9 => "r9",
            Self::R10 => "r10",
            Self::R11 => "r11",
            Self::R12 => "r12",
            Self::R13 => "r13",
            Self::R14 => "r14",
            Self::R15 => "r15",
            Self::Eax => "eax",
            Self::Ecx => "ecx",
            Self::Edx => "edx",
            Self::Ebx => "ebx",
            Self::Esp => "esp",
            Self::Ebp => "ebp",
            Self::Esi => "esi",
            Self::Cl => "cl",
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_string())
    }
}
