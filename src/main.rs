#![warn(missing_debug_implementations)]
use std::fmt;

use elf::{Elf, TextSection};

use crate::{
    opcode::OpCode,
    operand::{Displacement, Immediate, MemoryOperand, Operand, Operands, Rel, SibMemoryOperand},
    register::Register,
};

mod opcode;
mod operand;
mod register;

fn main() {
    let mut elf = Elf::new("../elf/hw-elf").unwrap();

    let t = elf.text().unwrap().unwrap();

    let mut decoder = Decoder::new(t);

    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    println!("{}", decoder.next_inst().unwrap());
    // println!("{}", decoder.next_inst().unwrap());

    dbg!(decoder.next_inst());
    // dbg!(decoder.next_inst());
    // dbg!(decoder.next_inst());
    // dbg!(decoder.next_inst());
    // dbg!(decoder.next_inst());
    // dbg!(decoder.next_inst());
    // dbg!(decoder.next_inst());

    // dbg!(t.inner[0]);
    // dbg!(t.inner[1]);
    // dbg!(t.inner[2]);
    // dbg!(t.inner[3]);
    // dbg!(t.inner[4]);
    // dbg!(t.inner[5]);
}

struct Decoder<'a> {
    text: TextSection<'a>,
    cursor: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(text: TextSection<'a>) -> Self {
        Self { text, cursor: 0 }
    }

    fn next_byte(&mut self) -> Option<u8> {
        self.text.inner.get(self.cursor).map(|b| {
            self.cursor += 1;
            *b
        })
    }

    fn calculate_len(&self, address: usize) -> u8 {
        (self.cursor - (address - self.text.offset)) as u8
    }

    fn imm8(&mut self) -> Option<i8> {
        self.next_byte().map(|b| b as i8)
    }

    fn imm32(&mut self) -> Option<i32> {
        Some(i32::from_le_bytes([
            self.next_byte()?,
            self.next_byte()?,
            self.next_byte()?,
            self.next_byte()?,
        ]))
    }

    /// Construct an instruction with an MR operand encoding
    ///
    /// The instruction takes two operands, the first being r/m and the second being
    /// r. The operands are constructed from the mod_rm rm bits and mod_rm reg bits
    /// respectively
    fn mr(
        &mut self,
        rex: Rex,
        get_register: fn(u8) -> Register,
        address: usize,
        thirty_two: OpCode,
        sixty_four: OpCode,
    ) -> Option<Instruction> {
        let mod_rm = ModRM::new(self.next_byte()?);

        let operand_one = self.memory_operand(
            mod_rm,
            rex,
            get_register,
            get_register(mod_rm.register_bits_from_rm(rex)),
        )?;
        let operand_two = get_register(mod_rm.register_bits_from_reg(rex));

        let operands = Operands::two(
            Operand::MemoryOperand(operand_one),
            Operand::Register(operand_two),
        );

        let op_code = if rex.w_set() { sixty_four } else { thirty_two };

        Some(Instruction {
            address,
            op_code,
            operands,
            len: self.calculate_len(address),
        })
    }

    /// Construct an instruction with an RM operand encoding
    ///
    /// The instruction takes two operands, the first being r and the second being
    /// r/m. The operands are constructed from the mod_rm reg bits and mod_rm rm bits
    /// respectively
    fn rm(
        &mut self,
        rex: Rex,
        get_register: fn(u8) -> Register,
        address: usize,
        thirty_two: OpCode,
        sixty_four: OpCode,
    ) -> Option<Instruction> {
        let mod_rm = ModRM::new(self.next_byte()?);

        let operand_one = get_register(mod_rm.register_bits_from_reg(rex));
        let operand_two = self.memory_operand(
            mod_rm,
            rex,
            get_register,
            get_register(mod_rm.register_bits_from_rm(rex)),
        )?;

        let operands = Operands::two(
            Operand::Register(operand_one),
            Operand::MemoryOperand(operand_two),
        );

        let op_code = if rex.w_set() { sixty_four } else { thirty_two };

        Some(Instruction {
            address,
            op_code,
            operands,
            len: self.calculate_len(address),
        })
    }

    fn memory_operand(
        &mut self,
        mod_rm: ModRM,
        rex: Rex,
        get_register: fn(u8) -> Register,
        memory_register: Register,
    ) -> Option<MemoryOperand> {
        // let memory_register = get_register(mod_rm.register_bits_from_rm(rex));

        if mod_rm.rm() == 0b100 {
            let sib = Sib::new(self.next_byte()?);

            let index = ((rex.x_set() as u8) << 3) | sib.index();
            let base = ((rex.b_set() as u8) << 3) | sib.base();

            let sib_operand = match mod_rm.mode() {
                0b00 => match index {
                    0b0000..=0b0011 => match base {
                        0b0000..=0b0100 => SibMemoryOperand::BaseIndexS {
                            base: get_register(base),
                            index: get_register(index),
                            s: 2_u8.pow(sib.scale() as u32),
                        },
                        _ => todo!(),
                    },
                    0b0100 => todo!(),
                    0b0101..=0b1111 => match base {
                        0b0000..=0b0100 | 0b0110..=0b1100 => SibMemoryOperand::BaseIndexS {
                            base: get_register(base),
                            index: get_register(index),
                            s: 2_u8.pow(sib.scale() as u32),
                        },
                        0b0101 => todo!(),
                        _ => todo!(),
                    },
                    _ => unreachable!(),
                },
                0b01 => match index {
                    0b0000..=0b0011 | 0b0101..=0b1111 => SibMemoryOperand::BaseIndexSDisp {
                        base: get_register(base),
                        index: get_register(index),
                        s: 2_u8.pow(sib.scale() as u32),
                        displacement: Displacement::One(self.imm8()?),
                    },
                    0b0100 => todo!(),
                    _ => unreachable!(),
                },
                0b10 => todo!(),
                _ => unreachable!(),
            };

            return Some(MemoryOperand::Sib(sib_operand));
        }

        let memory_operand = match mod_rm.mode() {
            0b00 => MemoryOperand::RegisterIndirect {
                register: memory_register,
                displacement: Displacement::None,
            },
            0b01 => MemoryOperand::RegisterIndirect {
                register: memory_register,
                displacement: Displacement::One(self.imm8()?),
            },
            0b10 => MemoryOperand::RegisterIndirect {
                register: memory_register,
                displacement: Displacement::Four(self.imm32()?),
            },
            0b11 => MemoryOperand::RegisterDirect(memory_register),
            _ => unreachable!(),
        };

        Some(memory_operand)
    }

    pub fn next_inst(&mut self) -> Option<Instruction> {
        let address = self.cursor + self.text.offset;
        let mut next_byte = self.next_byte()?;

        if next_byte == 0x66 {
            todo!("toggle between 16 and 32bit")
        }

        let rex = if let Some(rex) = Rex::try_new(next_byte) {
            next_byte = self.next_byte()?;
            rex
        } else {
            Rex::empty()
        };

        let get_register = if rex.w_set() {
            Register::r64
        } else {
            Register::r32
        };

        Some(match next_byte {
            // push
            op @ 0x50..=0x5f => {
                let register = Register::r64(op & 0b1111);

                let operands = Operands::one(Operand::Register(register));

                Instruction {
                    op_code: OpCode::Push_r64,
                    operands,
                    address,
                    len: self.calculate_len(address),
                }
            }
            0x83 => {
                let mod_rm = ModRM::new(self.next_byte()?);

                let register = get_register(mod_rm.register_bits_from_rm(rex));

                let imm = self.imm8()?;

                let op_code = if rex.w_set() {
                    match mod_rm.register() {
                        4 => OpCode::And_rm64_imm8,
                        5 => OpCode::Sub_rm64_imm8,
                        6 => OpCode::Xor_rm64_imm8,
                        7 => OpCode::Cmp_rm64_imm8,
                        b @ 0..=7 => todo!("{}", b),
                        _ => unreachable!(),
                    }
                } else {
                    match mod_rm.register() {
                        4 => OpCode::And_rm32_imm8,
                        5 => OpCode::Sub_rm32_imm8,
                        6 => OpCode::Xor_rm32_imm8,
                        7 => OpCode::Cmp_rm32_imm8,
                        0..=7 => todo!(),
                        _ => unreachable!(),
                    }
                };

                let operands = Operands::two(
                    Operand::Register(register),
                    Operand::Immediate(Immediate::Eight(imm)),
                );

                Instruction {
                    op_code,
                    operands,
                    address,
                    len: self.calculate_len(address),
                }
            }
            0x0f => match self.next_byte()? {
                // jb
                0x82 => {
                    let offset = self.imm32()?;
                    let len = self.calculate_len(address);
                    let operand = Rel::ThirtyTwo(offset + len as i32);

                    let operands = Operands::one(Operand::Rel(operand));

                    Instruction {
                        op_code: OpCode::Jb_rel32,
                        operands,
                        address,
                        len,
                    }
                }
                // bsr
                0xbd => {
                    let mod_rm = ModRM::new(self.next_byte()?);
                    let operand_one = get_register(mod_rm.register_bits_from_reg(rex));
                    let operand_two = get_register(mod_rm.register_bits_from_rm(rex));

                    let operands = Operands::two(
                        Operand::Register(operand_one),
                        Operand::Register(operand_two),
                    );

                    Instruction {
                        address,
                        operands,
                        op_code: OpCode::Bsr_r64_rm64,
                        len: self.calculate_len(address),
                    }
                }
                // cmovae
                0x43 => {
                    let mod_rm = ModRM::new(self.next_byte()?);

                    let operand_one = get_register(mod_rm.register_bits_from_reg(rex));
                    let operand_two = get_register(mod_rm.register_bits_from_rm(rex));

                    let operands = Operands::two(
                        Operand::Register(operand_one),
                        Operand::Register(operand_two),
                    );

                    let op_code = if rex.w_set() {
                        OpCode::Cmovae_r64_rm64
                    } else {
                        OpCode::Cmovae_r32_rm32
                    };

                    Instruction {
                        address,
                        op_code,
                        operands,
                        len: self.calculate_len(address),
                    }
                }
                // jae
                0x83 => {
                    let offset = self.imm32()?;
                    let len = self.calculate_len(address);
                    let operands = Operands::one(Operand::Rel(Rel::ThirtyTwo(offset + len as i32)));

                    Instruction {
                        address,
                        op_code: OpCode::Jae_rel32,
                        operands,
                        len,
                    }
                }
                b => todo!("{:#x}", b),
            },
            // lea
            0x8d => {
                let mod_rm = ModRM::new(self.next_byte()?);

                let register = get_register(mod_rm.register_bits_from_reg(rex));

                let memory_register = get_register(mod_rm.register_bits_from_rm(rex));

                let memory_operand =
                    self.memory_operand(mod_rm, rex, get_register, memory_register)?;

                let operands = Operands::two(
                    Operand::Register(register),
                    Operand::MemoryOperand(memory_operand),
                );

                Instruction {
                    address,
                    operands,
                    op_code: OpCode::Lea_r64_m,
                    len: self.calculate_len(address),
                }
            }
            // mov
            0xc7 => {
                let mod_rm = ModRM::new(self.next_byte()?);

                let register = get_register(mod_rm.register_bits_from_rm(rex));

                let op_code = if rex.w_set() {
                    match mod_rm.register() {
                        0 => OpCode::Mov_rm64_imm32,
                        _ => unimplemented!(),
                    }
                } else {
                    match mod_rm.register() {
                        0 => OpCode::Mov_rm32_imm32,
                        _ => unimplemented!(),
                    }
                };

                let imm = Immediate::ThirtyTwo(self.imm32()?);

                let operands = Operands::two(Operand::Register(register), Operand::Immediate(imm));

                Instruction {
                    address,
                    op_code,
                    operands,
                    len: self.calculate_len(address),
                }
            }
            // shr
            0xd3 => {
                let mod_rm = ModRM::new(self.next_byte()?);

                let register = get_register(mod_rm.register_bits_from_rm(rex));

                let op_code = if rex.w_set() {
                    match mod_rm.register() {
                        5 => OpCode::Shr_rm64_cl,
                        _ => unimplemented!(),
                    }
                } else {
                    match mod_rm.register() {
                        5 => OpCode::Shr_rm32_cl,
                        _ => unimplemented!(),
                    }
                };

                let operands =
                    Operands::two(Operand::Register(register), Operand::Register(Register::Cl));

                Instruction {
                    address,
                    op_code,
                    operands,
                    len: self.calculate_len(address),
                }
            }
            // mov
            0x89 => self.mr(
                rex,
                get_register,
                address,
                OpCode::Mov_rm32_r32,
                OpCode::Mov_rm64_r64,
            )?,
            0xd1 => {
                let mod_rm = ModRM::new(self.next_byte()?);

                let op_code = if rex.w_set() {
                    match mod_rm.register() {
                        5 => OpCode::Shr_rm64_1,
                        0..=7 => todo!(),
                        _ => unreachable!(),
                    }
                } else {
                    match mod_rm.register() {
                        5 => OpCode::Shr_rm32_1,
                        0..=7 => todo!(),
                        _ => unreachable!(),
                    }
                };

                let operand_one = get_register(mod_rm.register_bits_from_rm(rex));

                let operands = Operands::two(
                    Operand::Register(operand_one),
                    Operand::Immediate(Immediate::Eight(1)),
                );

                Instruction {
                    address,
                    op_code,
                    operands,
                    len: self.calculate_len(address),
                }
            }
            // shl/shr
            0xc1 => {
                let mod_rm = ModRM::new(self.next_byte()?);

                let op_code = if rex.w_set() {
                    match mod_rm.register() {
                        4 => OpCode::Shl_rm64_imm8,
                        5 => OpCode::Shr_rm64_imm8,
                        b @ 0..=7 => todo!("{}", b),
                        _ => unreachable!(),
                    }
                } else {
                    match mod_rm.register() {
                        4 => OpCode::Shl_rm32_imm8,
                        5 => OpCode::Shr_rm32_imm8,
                        b @ 0..=7 => todo!("{}", b),
                        _ => unreachable!(),
                    }
                };

                let operand_one = get_register(mod_rm.register_bits_from_rm(rex));
                let operand_two = Immediate::Eight(self.imm8()?);

                let operands = Operands::two(
                    Operand::Register(operand_one),
                    Operand::Immediate(operand_two),
                );

                Instruction {
                    address,
                    op_code,
                    operands,
                    len: self.calculate_len(address),
                }
            }
            // xor
            0x31 => {
                let mod_rm = ModRM::new(self.next_byte()?);

                let operand_one = get_register(mod_rm.register_bits_from_rm(rex));
                let operand_two = get_register(mod_rm.register_bits_from_reg(rex));

                let op_code = if rex.w_set() {
                    OpCode::Xor_rm64_r64
                } else {
                    OpCode::Xor_rm32_r32
                };

                let operands = Operands::two(
                    Operand::Register(operand_one),
                    Operand::Register(operand_two),
                );

                Instruction {
                    address,
                    op_code,
                    operands,
                    len: self.calculate_len(address),
                }
            }
            // add
            0x01 => {
                let mod_rm = ModRM::new(self.next_byte()?);

                let operand_one = get_register(mod_rm.register_bits_from_rm(rex));
                let operand_two = get_register(mod_rm.register_bits_from_reg(rex));

                let operands = Operands::two(
                    Operand::Register(operand_one),
                    Operand::Register(operand_two),
                );

                let op_code = if rex.w_set() {
                    OpCode::Add_rm64_r64
                } else {
                    OpCode::Add_rm32_r32
                };

                Instruction {
                    address,
                    op_code,
                    operands,
                    len: self.calculate_len(address),
                }
            }
            // and
            0x21 => {
                let mod_rm = ModRM::new(self.next_byte()?);

                let operand_one = get_register(mod_rm.register_bits_from_rm(rex));
                let operand_two = get_register(mod_rm.register_bits_from_reg(rex));

                let operands = Operands::two(
                    Operand::Register(operand_one),
                    Operand::Register(operand_two),
                );

                let op_code = if rex.w_set() {
                    OpCode::And_rm64_r64
                } else {
                    OpCode::And_rm32_r32
                };

                Instruction {
                    address,
                    op_code,
                    operands,
                    len: self.calculate_len(address),
                }
            }
            // cmp
            0x39 => self.mr(
                rex,
                get_register,
                address,
                OpCode::Cmp_rm32_r32,
                OpCode::Cmp_rm64_r64,
            )?,
            // sub
            0x29 => self.mr(
                rex,
                get_register,
                address,
                OpCode::Sub_rm32_r32,
                OpCode::Sub_rm64_r64,
            )?,
            // mov
            0x8b => self.rm(
                rex,
                get_register,
                address,
                OpCode::Mov_r32_rm32,
                OpCode::Mov_r64_rm64,
            )?,
            // 0x24 => {}
            b => todo!("{:#x}", b),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Rex {
    inner: u8,
}

impl Rex {
    const W: u8 = 0b1000;
    const R: u8 = 0b0100;
    const X: u8 = 0b0010;
    const B: u8 = 0b0001;

    pub fn new(b: u8) -> Self {
        Self { inner: b }
    }

    pub fn empty() -> Self {
        Self { inner: 0 }
    }

    pub fn try_new(b: u8) -> Option<Self> {
        if b >> 4 == 0b0100 {
            return Some(Self::new(b));
        }

        None
    }

    /// When true, a 64-bit operand size is used. Otherwise, when 0, the default
    /// operand size is used
    pub fn w_set(&self) -> bool {
        self.inner & Rex::W > 0
    }

    pub fn r_set(&self) -> bool {
        self.inner & Rex::R > 0
    }

    pub fn b_set(&self) -> bool {
        self.inner & Rex::B > 0
    }

    pub fn x_set(&self) -> bool {
        self.inner & Rex::X > 0
    }
}

impl Default for Rex {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Clone, Copy)]
struct ModRM {
    inner: u8,
}

impl ModRM {
    pub fn new(b: u8) -> Self {
        Self { inner: b }
    }

    pub fn mode(&self) -> u8 {
        self.inner >> 6
    }

    pub fn register(&self) -> u8 {
        (self.inner >> 3) & 0b111
    }

    pub fn rm(&self) -> u8 {
        self.inner & 0b111
    }

    pub fn register_bits_from_reg(&self, rex: Rex) -> u8 {
        ((rex.r_set() as u8) << 3) | self.register()
    }

    pub fn register_bits_from_rm(&self, rex: Rex) -> u8 {
        ((rex.b_set() as u8) << 3) | self.rm()
    }
}

impl fmt::Debug for ModRM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModRM")
            .field("mode", &format!("0b{:02b}", self.mode()))
            .field("reg", &format!("0b{:03b}", self.register()))
            .field("rm", &format!("0b{:03b}", self.rm()))
            .finish()
    }
}

struct Sib {
    inner: u8,
}

impl Sib {
    pub fn new(b: u8) -> Self {
        Self { inner: b }
    }

    pub fn empty() -> Self {
        Self { inner: 0 }
    }

    pub fn scale(&self) -> u8 {
        self.inner >> 6
    }

    pub fn index(&self) -> u8 {
        (self.inner >> 3) & 0b111
    }

    pub fn base(&self) -> u8 {
        self.inner & 0b111
    }
}

impl fmt::Debug for Sib {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sib")
            .field("scale", &format!("{:#b}", self.scale()))
            .field("index", &format!("{:#b}", self.index()))
            .field("base", &format!("{:#b}", self.base()))
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    op_code: OpCode,
    operands: Operands,
    address: usize,
    len: u8,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.op_code, self.operands)
    }
}
