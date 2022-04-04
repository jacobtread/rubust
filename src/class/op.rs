use std::fmt::{Display, Formatter, write};
use std::io::Read;

use regex::internal::Inst;

use crate::io::{read_byte_vec, Readable};
use num_enum::{IntoPrimitive, TryFromPrimitive};

macro_rules! define_op_codes {
    (
         $(
          $enum_value:ident ($op_code:literal, $name:literal $(, $argc:literal)?)
         ),* $(,)?
    ) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
        #[repr(u8)]
        pub enum OpCodes {
            $($enum_value = $op_code,)*
        }

        impl OpCodes {
            pub fn get_name(&self) -> &'static str {
                match self {
                    $(OpCodes::$enum_value => $name,)*
                    _ => "unknown_opcode"
                }
            }

            pub fn get_argc(&self) -> usize {
                match self {
                    $($(OpCodes::$enum_value => $argc,)?)*
                    _ => 0
                }
            }
        }
    };
}

define_op_codes! {
    AALOAD (0x32, "aaload"),
    AASTORE (0x53, "aastore"),
    ACONST_NULL (0x01, "aconst_null"),
    ALOAD (0x19, "aload", 1),
    ALOAD_0 (0x2A, "aload_0"),
    ALOAD_1 (0x2B, "aload_1"),
    ALOAD_2 (0x2C, "aload_2"),
    ALOAD_3 (0x2D, "aload_3"),
    ANEWARRAY (0xBD, "anewarray", 2),
    ARETURN (0xB0, "areturn"),
    ARRAYLENGTH (0xBE, "arraylength"),
    ASTORE (0x3A, "astore", 1),
    ASTORE_0 (0x4B, "astore_0"),
    ASTORE_1 (0x4C, "astore_1"),
    ASTORE_2 (0x4D, "astore_2"),
    ASTORE_3 (0x4E, "astore_3"),
    ATHROW (0xBF, "athrow"),
    BALOAD (0x33, "baload"),
    BASTORE (0x54, "bastore"),
    BIPUSH (0x10, "bipush", 1),
    BREAKPOINT (0xCA, "breakpoint"),
    CALOAD (0x34, "caload"),
    CASTORE (0x55, "castore"),
    CHECKCAST (0xC0, "checkcast", 2),
    D2F (0x90, "d2f"),
    D2I (0x8E, "d2i"),
    D2L (0x8F, "d2l"),
    DADD (0x63, "dadd"),
    DALOAD (0x31, "daload"),
    DASTORE (0x052, "dastore"),
    DCMPG (0x98, "dcmpg"),
    DCMPL (0x97, "dcmpl"),
    DCONST_0 (0x0E, "dconst_0"),
    DCONST_1 (0x0F, "dconst_1"),
    DDIV (0x6F, "ddiv"),
    DLOAD (0x18, "dload", 1),
    DLOAD_0 (0x26, "dload_0"),
    DLOAD_1 (0x27, "dload_1"),
    DLOAD_2 (0x28, "dload_2"),
    DLOAD_3 (0x29, "dload_3"),
    DMUL (0x6B, "dmul"),
    DNEG (0x77, "dneg"),
    DREM (0x73, "drem"),
    DRETURN (0xAF, "dreturn"),
    DSTORE (0x39, "dstore", 1),
    DSTORE_0 (0x47, "dstore_0"),
    DSTORE_1 (0x48, "dstore_1"),
    DSTORE_2 (0x49, "dstore_2"),
    DSTORE_3 (0x4A, "dstore_3"),
    DSUB (0x67, "dsub"),
    DUP (0x59, "dup"),
    DUP_X1 (0x5A, "dup_x1"),
    DUP_X2 (0x5B, "dup_x2"),
    DUP2 (0x5C, "dup2"),
    DUP2_x1 (0x5D, "dup2_x1"),
    DUP2_x2 (0x5E, "dup2_x2"),
    F2D (0x8D, "f2d"),
    F2I (0x8B, "f2i"),
    F2L (0x8C, "f2l"),
    FADD (0x62, "fadd"),
    FALOAD (0x30, "faload"),
    FASTORE (0x51, "fastore"),
    FCMPG (0x96, "fcmpg"),
    FCMPL (0x95, "fcmpl"),
    FCONST_0(0x0B, "fconst_0"),
    FCONST_1(0x0C, "fconst_1"),
    FCONST_2(0x0D, "fconst_2"),
    FDIV (0x6E, "fdiv"),
    FLOAD (0x17, "fload", 1),
    FLOAT_0 (0x22, "float_0"),
    FLOAT_1 (0x23, "float_1"),
    FLOAT_2 (0x24, "float_2"),
    FLOAT_3 (0x25, "float_3"),
    FMUL (0x6A, "fmul"),
    FNEG (0x76, "fneg"),
    FREM (0x72, "frem"),
    FRETURN (0xAE, "freturn"),
    FSTORE(0x38, "fstore", 1),
    FSTORE_0 (0x43, "fstore_0"),
    FSTORE_1 (0x44, "fstore_1"),
    FSTORE_2 (0x45, "fstore_2"),
    FSTORE_3 (0x46, "fstore_3"),
    FSUB (0x66, "fsub"),
    GETFIELD (0xB4, "getfield", 2),
    GETSTATIC (0xB2, "getstatic", 2),
    GOTO (0xA7, "goto", 2),
    GOTO_W (0xC8, "goto_w", 4),
    I2B (0x91, "i2b"),
    I2C (0x92, "i2c"),
    I2D (0x87, "i2d"),
    I2F (0x86, "i2f"),
    I2L (0x85, "i2l"),
    I2S (0x93, "i2s"),
    IADD (0x60, "iadd"),
    IALOAD (0x2E, "iaload"),
    IAND (0x7E, "iand"),
    IASTORE (0x4F, "iastore"),
    ICONST_M1 (0x02, "iconst_m1"),
    ICONST_0 (0x03, "iconst_0"),
    ICONST_1 (0x04, "iconst_1"),
    ICONST_2 (0x05, "iconst_2"),
    ICONST_3 (0x06, "iconst_3"),
    ICONST_4 (0x07, "iconst_4"),
    ICONST_5 (0x08, "iconst_5"),
    IDIV (0x6C, "idiv"),
    IF_ACMPEQ (0xA5, "ifacmpeq", 2),
    IF_ACMPNE (0xA6, "if_acmpne", 2),
    IF_ICMPEQ (0x9F, "if_icmeq", 2),
    IF_ICMPGE (0xA2, "if_icmpge", 2),
    IF_ICMPGT (0xA3, "if_icmpgt", 2),
    IF_ICMPLE (0xA4, "if_icmple", 2),
    IF_ICMPLT (0xA1, "if_icmplt", 2),
    IF_ICMPNE (0xA0, "if_icmpne", 2),
    IFEQ (0x99, "ifeq", 2),
    IFGE (0x9C, "ifge", 2),
    IFGT (0x9D, "ifgt", 2),
    IFFE (0x9E, "iffe", 2),
    IFLT (0x9B, "iflt", 2),
    IFNE (0x9A, "ifne", 2),
    IFNONNLL (0xC7, "ifnonnull", 2),
    IFNLL (0xC6, "ifnull", 2),
    IINC (0x84, "iinc", 2),
    ILOAD (0x15, "iload", 1),
    ILOAD_0 (0x1A, "iload_0"),
    ILOAD_1 (0x1B, "iload_1"),
    ILOAD_2 (0x1C, "iload_2"),
    ILOAD_3 (0x1D, "iload_3"),
    IMPDEP1 (0xFE, "impdep1"),
    IMPDEP2 (0xFF, "impdep2"),
    IMUL (0x68, "imul"),
    INEG (0x74, "ineg"),
    INSTANCEOF (0xC1, "instanceof", 2),
    INVOKEDYNAMIC (0xBA, "invokedynamic", 4),
    INVOKEINTERFACE (0xB9, "invokeinterface", 4),
    INVOKESPECIAL (0xB7, "invokespecial", 2),
    INVOKESTATIC (0xB8, "invokestatic", 2),
    INVOKEVIRTUAL (0xB6, "invokevirtual", 2),
    IOR (0x80, "ior"),
    IREM (0x70, "irem"),
    IRETURN (0xAC, "ireturn"),
    ISHL (0x78, "ishl"),
    ISHR (0x7A, "ishr"),
    ISTORE (0x36, "istore", 1),
    ISTORE_0 (0x3B, "istore_0"),
    ISTORE_1 (0x3C, "istore_1"),
    ISTORE_2 (0x3D, "istore_2"),
    ISTORE_3 (0x3E, "istore_3"),
    ISUB (0x64, "isub"),
    IUSHR (0x7C, "iushr"),
    IXOR (0x82, "ixor"),
    JSR (0xA8, "jsr", 2),
    JSR_W (0xC9, "jsr_w", 4),
    L2D (0x8A, "l2d"),
    L2F (0x89, "l2f"),
    L2I (0x88, "l2i"),
    LADD (0x61, "ladd"),
    LALOAD (0x2F, "laload"),
    LAND (0x7F, "land"),
    LASTORE (0x50, "lastore"),
    LCMP (0x94, "lcmp"),
    LCONST_0 (0x09, "lconst_0"),
    LCONST_1 (0x0A, "lconst_1"),
    LDC (0x12, "ldc", 1),
    LDC_W (0x13, "ldc_w", 2),
    LDC2_W (0x14, "ldc2_w", 2),
    LDIV (0x6D, "ldiv"),
    LLOAD (0x16, "lload", 1),
    LLOAD_0 (0x1E, "lload_0"),
    LLOAD_1 (0x1F, "lload_1"),
    LLOAD_2 (0x20, "lload_2"),
    LLOAD_3 (0x21, "lload_3"),
    LMUL (0x69, "lmul"),
    LNEG (0x75, "lneg"),
    LOOKUPSWITCH (0xAB, "lookupswitch", 4),
    LOR (0x81, "lor"),
    LREM (0x71, "lrem"),
    LRETURN (0xAD, "lreturn"),
    LSHL (0x79, "shl"),
    LSHR (0x7B, "lshr"),
    LSTORE (0x37, "lstore", 1),
    LSTORE_0 (0x3F, "lstore_0"),
    LSTORE_1 (0x40, "lstore_1"),
    LSTORE_2 (0x41, "lstore_2"),
    LSTORE_3 (0x42, "lstore_3"),
    LSUB (0x65, "lsub"),
    LUSHR (0x7D, "lushr"),
    LXOR (0x83, "lxor"),
    MONITORENTER (0xC2, "monitorenter"),
    MONITOREXIT (0xC3, "monitorexit"),
    MULTIANEWARRAY (0xC5, "multianewarray", 3),
    NEW (0xBB, "new", 2),
    NEWARRAY (0xBC, "newarray", 1),
    NOP (0x00, "nop"),
    POP (0x57, "pop"),
    POP2 (0x58, "pop2"),
    PUTFIELD (0xB5, "putfield", 2),
    PUTSTATIC (0xB3, "putstatic", 2),
    RET (0xA9, "ret", 1),
    RETURN (0xB1, "return"),
    SALOAD (0x35, "saload"),
    SASTORE (0x56, "sastore"),
    SIPUSH (0x11, "sipush", 2),
    SWAP (0x5F, "swap"),
    TABLESWITCH (0xAA, "tableswitch", 4),
    WIDE (0xC4, "wide", 3)
}

impl Display for OpCodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Instruction {
    pub op: OpCodes,
    pub args: Vec<u8>,
}

impl Readable for Instruction {
    fn read<B: Read>(i: &mut B) -> anyhow::Result<Self> where Self: Sized {
        let op = OpCodes::try_from(u8::read(i)?)
            .map_err(anyhow::Error::from)?;
        let args = read_byte_vec(i, op.get_argc())?;
        Ok(Instruction {
            op,
            args,
        })
    }
}


