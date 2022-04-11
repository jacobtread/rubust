#![allow(dead_code)]

pub const BYTECODE_JAVA_LE_4: u8 = 48;
pub const BYTECODE_JAVA_5: u8 = 49;
pub const BYTECODE_JAVA_6: u8 = 50;
pub const BYTECODE_JAVA_7: u8 = 51;
pub const BYTECODE_JAVA_8: u8 = 52;
pub const BYTECODE_JAVA_9: u8 = 53;
pub const BYTECODE_JAVA_10: u8 = 54;
pub const BYTECODE_JAVA_11: u8 = 55;
pub const BYTECODE_JAVA_12: u8 = 56;
pub const BYTECODE_JAVA_13: u8 = 57;
pub const BYTECODE_JAVA_14: u8 = 58;
pub const BYTECODE_JAVA_15: u8 = 59;
pub const BYTECODE_JAVA_16: u8 = 60;
pub const BYTECODE_JAVA_17: u8 = 61;

pub const TYPE_BYTE: u8 = 0;
pub const TYPE_CHAR: u8 = 1;
pub const TYPE_DOUBLE: u8 = 2;
pub const TYPE_FLOAT: u8 = 3;
pub const TYPE_INT: u8 = 4;
pub const TYPE_LONG: u8 = 5;
pub const TYPE_SHORT: u8 = 6;
pub const TYPE_BOOLEAN: u8 = 7;
pub const TYPE_OBJECT: u8 = 8;
pub const TYPE_ADDRESS: u8 = 9;
pub const TYPE_VOID: u8 = 10;
pub const TYPE_ANY: u8 = 11;
pub const TYPE_GROUP2EMPTY: u8 = 12;
pub const TYPE_NULL: u8 = 13;
pub const TYPE_NOTINITIALIZED: u8 = 14;
pub const TYPE_BYTECHAR: u8 = 15;
pub const TYPE_SHORTCHAR: u8 = 16;
pub const TYPE_UNKNOWN: u8 = 17;
pub const TYPE_GENVAR: u8 = 18;
pub const TYPE_FAMILY_UNKNOWN: u8 = 0;
pub const TYPE_FAMILY_BOOLEAN: u8 = 1;
pub const TYPE_FAMILY_INTEGER: u8 = 2;
pub const TYPE_FAMILY_FLOAT: u8 = 3;
pub const TYPE_FAMILY_LONG: u8 = 4;
pub const TYPE_FAMILY_DOUBLE: u8 = 5;
pub const TYPE_FAMILY_OBJECT: u8 = 6;


pub const ACC_PUBLIC: u16 = 0x0001;
pub const ACC_PRIVATE: u16 = 0x0002;
pub const ACC_PROTECTED: u16 = 0x0004;
pub const ACC_STATIC: u16 = 0x0008;
pub const ACC_FINAL: u16 = 0x0010;
pub const ACC_SYNCHRONIZED: u8 = 0x0020;
pub const ACC_OPEN: u16 = 0x0020;
pub const ACC_NATIVE: u16 = 0x0100;
pub const ACC_ABSTRACT: u16 = 0x0400;
pub const ACC_STRICT: u16 = 0x0800;
pub const ACC_VOLATILE: u16 = 0x0040;
pub const ACC_BRIDGE: u16 = 0x0040;
pub const ACC_TRANSIENT: u16 = 0x0080;
pub const ACC_VARARGS: u16 = 0x0080;
pub const ACC_SYNTHETIC: u16 = 0x1000;
pub const ACC_ANNOTATION: u16 = 0x2000;
pub const ACC_ENUM: u16 = 0x4000;
pub const ACC_MANDATED: u16 = 0x8000;
pub const ACC_MODULE: u16 = 0x8000;
pub const ACC_SUPER: u16 = 0x0020;
pub const ACC_INTERFACE: u16 = 0x0200;

pub const GROUP_GENERAL: u8 = 1;
pub const GROUP_JUMP: u8 = 2;
pub const GROUP_SWITCH: u8 = 3;
pub const GROUP_INVOCATION: u8 = 4;
pub const GROUP_FIELDACCESS: u8 = 5;
pub const GROUP_RETURN: u8 = 6;


pub const CONSTANT_UTF8: u8 = 1;
pub const CONSTANT_INTEGER: u8 = 3;
pub const CONSTANT_FLOAT: u8 = 4;
pub const CONSTANT_LONG: u8 = 5;
pub const CONSTANT_DOUBLE: u8 = 6;
pub const CONSTANT_CLASS: u8 = 7;
pub const CONSTANT_STRING: u8 = 8;
pub const CONSTANT_FIELDREF: u8 = 9;
pub const CONSTANT_METHODREF: u8 = 10;
pub const CONSTANT_INTERFACE_METHODREF: u8 = 11;
pub const CONSTANT_NAME_AND_TYPE: u8 = 12;
pub const CONSTANT_METHOD_HANDLE: u8 = 15;
pub const CONSTANT_METHOD_TYPE: u8 = 16;
pub const CONSTANT_DYNAMIC: u8 = 17;
pub const CONSTANT_INVOKE_DYNAMIC: u8 = 18;
pub const CONSTANT_MODULE: u8 = 19;
pub const CONSTANT_PACKAGE: u8 = 20;

pub const CONSTANT_METHOD_HANDLE_REF_GET_FIELD: u8 = 1;
pub const CONSTANT_METHOD_HANDLE_REF_GET_STATIC: u8 = 2;
pub const CONSTANT_METHOD_HANDLE_REF_PUT_FIELD: u8 = 3;
pub const CONSTANT_METHOD_HANDLE_REF_PUT_STATIC: u8 = 4;
pub const CONSTANT_METHOD_HANDLE_REF_INVOKE_VIRTUAL: u8 = 5;
pub const CONSTANT_METHOD_HANDLE_REF_INVOKE_STATIC: u8 = 6;
pub const CONSTANT_METHOD_HANDLE_REF_INVOKE_SPECIAL: u8 = 7;
pub const CONSTANT_METHOD_HANDLE_REF_NEW_INVOKE_SPECIAL: u8 = 8;
pub const CONSTANT_METHOD_HANDLE_REF_INVOKE_INTERFACE: u8 = 9;

