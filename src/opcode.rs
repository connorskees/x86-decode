use std::fmt;

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types, dead_code)]
pub enum OpCode {
    /// PUSH r64
    Push_r64,

    /// SUB r/m32, imm8
    Sub_rm32_imm8,

    /// SUB r/m64, imm8
    Sub_rm64_imm8,

    /// SUB r/m32, r32
    Sub_rm32_r32,

    /// SUB r/m64, r64
    Sub_rm64_r64,

    /// CMP r/m32, imm8
    Cmp_rm32_imm8,

    /// CMP r/m64, imm8
    Cmp_rm64_imm8,

    /// CMP r/m32, r32
    Cmp_rm32_r32,

    /// CMP r/m64, r64
    Cmp_rm64_r64,

    /// JB rel16
    Jb_rel16,

    /// JB rel32
    Jb_rel32,

    /// JAE rel32
    Jae_rel32,

    /// LEA r64,m
    Lea_r64_m,

    /// BSR r64, r/m64
    Bsr_r64_rm64,

    /// XOR r/m32, imm8
    Xor_rm32_imm8,

    /// XOR r/m64, imm8
    Xor_rm64_imm8,

    /// XOR r/m32, r32
    Xor_rm32_r32,

    /// XOR r/m64, r64
    Xor_rm64_r64,

    /// MOV r/m32, imm32
    Mov_rm32_imm32,

    /// MOV r/m64, imm32
    Mov_rm64_imm32,

    /// MOV r32,r/m32
    Mov_r32_rm32,

    /// MOV r64,r/m64
    Mov_r64_rm64,

    /// MOV r/m32,r32
    Mov_rm32_r32,

    /// MOV r/m64,r64
    Mov_rm64_r64,

    /// SHR r/m32, CL
    Shr_rm32_cl,

    /// SHR r/m64, CL
    Shr_rm64_cl,

    /// SHR r/m32, 1
    Shr_rm32_1,

    /// SHR r/m64, 1
    Shr_rm64_1,

    /// SHR r/m32, imm8
    Shr_rm32_imm8,

    /// SHR r/m64, imm8
    Shr_rm64_imm8,

    /// AND r/m32, imm8
    And_rm32_imm8,

    /// AND r/m64, imm8
    And_rm64_imm8,

    /// AND r/m32, r32
    And_rm32_r32,

    /// AND r/m64, r64
    And_rm64_r64,

    /// SHL r/m32, imm8
    ///
    /// This is equivalent to SAL
    Shl_rm32_imm8,

    /// SHL r/m64, imm8
    Shl_rm64_imm8,

    /// ADD r/m32, r32
    Add_rm32_r32,

    /// ADD r/m64, r64
    Add_rm64_r64,

    /// CMOVAE r32, r/m32
    Cmovae_r32_rm32,

    /// CMOVAE r64, r/m64
    Cmovae_r64_rm64,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Push_r64 => "push",
            Self::Sub_rm32_imm8 | Self::Sub_rm64_imm8 | Self::Sub_rm32_r32 | Self::Sub_rm64_r64 => {
                "sub"
            }
            Self::Cmp_rm32_imm8 | Self::Cmp_rm64_imm8 | Self::Cmp_rm32_r32 | Self::Cmp_rm64_r64 => {
                "cmp"
            }
            Self::Lea_r64_m => "lea",
            Self::Bsr_r64_rm64 => "bsr",
            Self::Jb_rel16 | Self::Jb_rel32 => "jb",
            Self::Xor_rm32_imm8 | Self::Xor_rm64_imm8 | Self::Xor_rm32_r32 | Self::Xor_rm64_r64 => {
                "xor"
            }
            Self::Mov_rm32_imm32
            | Self::Mov_rm64_imm32
            | Self::Mov_rm32_r32
            | Self::Mov_rm64_r64
            | Self::Mov_r32_rm32
            | Self::Mov_r64_rm64 => "mov",
            Self::Shr_rm32_cl
            | Self::Shr_rm64_cl
            | Self::Shr_rm32_1
            | Self::Shr_rm64_1
            | Self::Shr_rm32_imm8
            | Self::Shr_rm64_imm8 => "shr",
            Self::And_rm32_imm8 | Self::And_rm64_imm8 | Self::And_rm32_r32 | Self::And_rm64_r64 => {
                "and"
            }
            Self::Shl_rm32_imm8 | Self::Shl_rm64_imm8 => "shl",
            Self::Add_rm32_r32 | Self::Add_rm64_r64 => "add",
            Self::Cmovae_r32_rm32 | Self::Cmovae_r64_rm64 => "cmovae",
            Self::Jae_rel32 => "jae",
        })
    }
}
