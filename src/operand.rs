use std::fmt;

use crate::register::Register;

#[derive(Debug, Clone, Copy)]
pub enum Immediate {
    Eight(i8),
    Sixteen(i16),
    ThirtyTwo(i32),
    SixtyFour(i64),
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryOperand {
    RegisterIndirect {
        register: Register,
        displacement: Displacement,
    },
    Sib(SibMemoryOperand),
    RegisterDirect(Register),
}

impl fmt::Display for MemoryOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegisterIndirect {
                register,
                displacement,
            } => {
                write!(f, "[{}{}]", register, displacement)
            }
            Self::Sib(sib) => write!(f, "{}", sib),
            Self::RegisterDirect(reg) => write!(f, "{}", reg),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Rel {
    Eight(i8),
    Sixteen(i16),
    ThirtyTwo(i32),
    SixtyFour(i64),
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Register(Register),
    Immediate(Immediate),
    Rel(Rel),
    MemoryOperand(MemoryOperand),
    None,
}

impl Operand {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Register(r) => write!(f, "{}", r),
            Self::Immediate(imm) => write!(f, "{:x}", imm),
            Self::Rel(rel) => write!(f, "{:x}", rel),
            Self::MemoryOperand(m) => write!(f, "{}", m),
            Self::None => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SibMemoryOperand {
    BaseIndexS {
        base: Register,
        index: Register,
        s: u8,
    },
    BaseIndexSDisp {
        base: Register,
        index: Register,
        s: u8,
        displacement: Displacement,
    },
    Base(Register),
    Disp(Displacement),
    IndexSDisp {
        index: Register,
        s: u8,
        displacement: Displacement,
    },
}

impl fmt::Display for SibMemoryOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BaseIndexS { base, index, s } => write!(f, "[{}+{}*{:#x}]", base, index, s),
            Self::BaseIndexSDisp {
                base,
                index,
                s,
                displacement,
            } => write!(f, "[{}+{}*{}{}]", base, index, s, displacement),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Displacement {
    None,
    One(i8),
    Four(i32),
}

impl Displacement {
    pub fn is_positive(&self) -> bool {
        match self {
            Self::None => false,
            Self::One(n) => n.is_positive(),
            Self::Four(n) => n.is_positive(),
        }
    }
}

impl fmt::Display for Displacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = match *self {
            Self::None => 0,
            Self::One(n) => n as i32,
            Self::Four(n) => n,
        };

        if n == 0 {
            return Ok(());
        }

        if n.is_positive() {
            write!(f, "+0x{:x}", n)
        } else {
            write!(f, "-0x{:x}", -n)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Operands {
    operands: [Operand; 4],
}

impl Operands {
    pub fn one(operand: Operand) -> Self {
        Self {
            operands: [operand, Operand::None, Operand::None, Operand::None],
        }
    }

    pub fn two(operand_one: Operand, operand_two: Operand) -> Self {
        Self {
            operands: [operand_one, operand_two, Operand::None, Operand::None],
        }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;

        for _ in 0..4 {
            if self.operands[len].is_none() {
                return len;
            }

            len += 1
        }

        len
    }
}

impl fmt::Display for Operands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &self.operands[..self.len()]
                .into_iter()
                .map(|operand| operand.to_string())
                .collect::<Vec<String>>()
                .join(","),
        )
    }
}

macro_rules! impl_lower_hex {
    ($name:ident) => {
        impl fmt::LowerHex for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match *self {
                    Self::Eight(n) => write!(f, "{:#x}", n),
                    Self::Sixteen(n) => write!(f, "{:#x}", n),
                    Self::ThirtyTwo(n) => write!(f, "{:#x}", n),
                    Self::SixtyFour(n) => write!(f, "{:#x}", n),
                }
            }
        }
    };
}

impl_lower_hex!(Immediate);
impl_lower_hex!(Rel);
