macro_rules! define_op_codes {
    (
         $(
          $enum_value:ident ($op_code:literal, $name:literal $(, $argc:literal)?)
         ),* $(,)?
    ) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
        #[repr(u8)]
        enum OpCodes {
            $($enum_value = $op_code,)*
        }

        impl OpCodes {
            fn get_name(&self) -> &'static str {
                match self {
                    $(OpCodes::$enum_value => $name,)*
                    _ => "unknown_opcode"
                }
            }

            fn get_argc(&self) -> usize {
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
    GOTO_W (0xCA, "goto_w", 4),
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
    IF_ACMPEQ (0xA5, "ifacmpeq"),
    
}

