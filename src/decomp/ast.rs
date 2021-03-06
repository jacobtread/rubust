use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Write;

use crate::class::access::AccessFlag;
use crate::class::attribute::CodeAttr;
use crate::class::class::ClassPath;
use crate::class::constant::{Constant, ConstantPool, MemberReference};
use crate::class::descriptor::Descriptor;
use crate::class::member::Member;
use crate::class::op::{ArrayType, BranchIndex, Instr, InstrSet};
use crate::decomp::writer::WriteResult;
use crate::error::{ConstantError, DecompileError, StackError};

#[derive(Debug, Clone)]
pub enum VarType {
    Byte,
    Int,
    Float,
    Double,
    Long,
    Short,
    Boolean,
    Char,
    Reference,
}

impl Display for VarType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   VarType::Reference => "ref",
                   VarType::Int => "int",
                   VarType::Float => "float",
                   VarType::Double => "double",
                   VarType::Long => "long",
                   VarType::Byte => "byte",
                   VarType::Short => "short",
                   VarType::Boolean => "boolean",
                   VarType::Char => "char",
               }
        )
    }
}

type DecompileResult<T> = Result<T, DecompileError>;
type ASTSet = Vec<AST>;

struct Stack {
    values: Vec<AST>,
}

impl Stack {
    fn new() -> Stack {
        Stack { values: Vec::new() }
    }

    fn pop(&mut self) -> Result<AST, StackError> {
        match self.values.pop() {
            None => Err(StackError::Empty),
            Some(value) => Ok(value)
        }
    }

    fn pop_boxed(&mut self) -> Result<Box<AST>, StackError> {
        match self.values.pop() {
            None => Err(StackError::Empty),
            Some(value) => Ok(Box::new(value))
        }
    }

    fn push(&mut self, value: AST) {
        self.values.push(value);
    }

    fn empty(&self) -> Result<(), StackError> {
        if !self.values.is_empty() {
            Err(StackError::Remaining(self.values.len() as usize))
        } else {
            Ok(())
        }
    }

    fn prim_cast(&mut self, primitive: VarType) -> Result<(), StackError> {
        let value = self.pop_boxed()?;
        self.push(AST::PrimitiveCast {
            primitive,
            value,
        });
        Ok(())
    }

    fn swap(&mut self) -> Result<(), StackError> {
        let len = self.values.len();
        if len >= 2 {
            self.values.swap(len - 1, len - 2);
            Ok(())
        } else {
            Err(StackError::NotEnough(2, len))
        }
    }

    fn pop2(&mut self) -> Result<(), StackError> {
        let top = self.pop()?;
        match top {
            AST::DoubleConstant(_) | AST::LongConstant(_) => Ok(()),
            _ => {
                self.pop()?;
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    instructions: InstrSet,
    branches: Vec<u64>,
}

impl Block {
    pub fn decompile(&self, constant_pool: &ConstantPool) -> DecompileResult<ASTSet> {
        let mut statements = ASTSet::new();
        let mut stack = Stack::new();

        for (_, code) in &self.instructions {
            match code {
                Instr::Swap => { stack.swap()?; }
                Instr::ILoad(index) => { stack.push(AST::Variable(*index, VarType::Int)); }
                Instr::LLoad(index) => { stack.push(AST::Variable(*index, VarType::Long)); }
                Instr::FLoad(index) => { stack.push(AST::Variable(*index, VarType::Float)); }
                Instr::DLoad(index) => { stack.push(AST::Variable(*index, VarType::Double)); }
                Instr::ALoad(index) => { stack.push(AST::Variable(*index, VarType::Reference)); }
                Instr::InvokeSpecial(index) |
                Instr::InvokeInterface(index) |
                Instr::InvokeVirtual(index) => {
                    let member = constant_pool.get_member_ref(index)?;
                    let descriptor = &member.name_and_type.descriptor;
                    if let Descriptor::Method(method) = descriptor {
                        let mut args = Vec::new();
                        for _ in 0..method.parameters.len() {
                            args.push(stack.pop()?);
                        }
                        args.reverse();
                        let reference = stack.pop_boxed()?;
                        if let Descriptor::Void = *method.return_type {
                            statements.push(AST::MethodCall { member, reference, args });
                        } else {
                            stack.push(AST::MethodCall { member, reference, args });
                        }
                    } else {
                        Err(DecompileError::ExpectedMethodDescriptor)?;
                    }
                }
                Instr::InvokeStatic(index) => {
                    let member = constant_pool.get_member_ref(index)?;
                    let descriptor = &member.name_and_type.descriptor;
                    if let Descriptor::Method(method) = descriptor {
                        let mut args = Vec::new();
                        for _ in 0..method.parameters.len() {
                            args.push(stack.pop()?);
                        }
                        args.reverse();
                        if let Descriptor::Void = *method.return_type {
                            statements.push(AST::StaticCall { member, args });
                        } else {
                            stack.push(AST::StaticCall { member, args });
                        }
                    } else {
                        Err(DecompileError::ExpectedMethodDescriptor)?;
                    }
                }
                Instr::InvokeDynamic(index) => {
                    match constant_pool.inner.get(index) {
                        None => Err(ConstantError::NotFound(*index))?,
                        Some(value) => {
                            match value {
                                Constant::InvokeDynamic(_) => {}
                                _ => Err(ConstantError::ExpectedInvokeDynamic(*index))?
                            }
                        }
                    }
                    unimplemented!("invoke dynamic not implemented yet")
                }
                Instr::Return => { statements.push(AST::VoidReturn); }
                Instr::IStore(index) |
                Instr::LStore(index) |
                Instr::FStore(index) |
                Instr::DStore(index) |
                Instr::AStore(index) => {
                    statements.push(AST::Set(*index, stack.pop_boxed()?));
                }
                Instr::IAStore |
                Instr::LAStore |
                Instr::DAStore |
                Instr::CAStore |
                Instr::BAStore |
                Instr::AAStore |
                Instr::SAStore |
                Instr::FAStore => {
                    let reference = stack.pop_boxed()?;
                    let index = stack.pop_boxed()?;
                    let value = stack.pop_boxed()?;
                    statements.push(AST::ArrayStore { reference, index, value })
                }
                Instr::IALoad |
                Instr::SALoad |
                Instr::DALoad |
                Instr::LALoad |
                Instr::CALoad |
                Instr::BALoad |
                Instr::AALoad |
                Instr::FALoad => {
                    let index = stack.pop_boxed()?;
                    let reference = stack.pop_boxed()?;
                    stack.push(AST::ArrayLoad { reference, index })
                }
                Instr::PutField(index) => {
                    let member = constant_pool.get_member_ref(index)?;
                    let value = stack.pop_boxed()?;
                    let reference = stack.pop_boxed()?;
                    statements.push(AST::FieldSet(member, reference, value));
                }
                Instr::GetField(index) => {
                    let member = constant_pool.get_member_ref(index)?;
                    let reference = stack.pop_boxed()?;
                    stack.push(AST::FieldGet(member, reference));
                }
                Instr::PutStatic(index) => {
                    let member = constant_pool.get_member_ref(index)?;
                    let value = stack.pop_boxed()?;
                    statements.push(AST::StaticSet(member, value))
                }
                Instr::GetStatic(index) => {
                    let member = constant_pool.get_member_ref(index)?;
                    stack.push(AST::StaticGet(member));
                }
                Instr::ArrayLength => {
                    let reference = stack.pop_boxed()?;
                    stack.push(AST::ArrayLength(reference));
                }
                Instr::LoadConst(index) => {
                    stack.push(match constant_pool.inner.get(index)
                        .ok_or(ConstantError::NotFound(*index))? {
                        Constant::Integer(value) => AST::IntegerConstant(*value),
                        Constant::Float(value) => AST::FloatConstant(*value),
                        Constant::Long(value) => AST::LongConstant(*value),
                        Constant::Double(value) => AST::DoubleConstant(*value),
                        Constant::String(index) => AST::StringConst(constant_pool.get_utf8(index)?.clone()),
                        _ => unimplemented!(),
                    });
                }
                Instr::CheckCast(index) => {
                    let class = constant_pool.get_class_path_required(index)?;
                    let value = stack.pop_boxed()?;
                    stack.push(AST::ClassCast { value, class })
                }
                Instr::IConst(value) => { stack.push(AST::IntegerConstant(*value)); }
                Instr::DConst(value) => { stack.push(AST::DoubleConstant(*value)); }
                Instr::FConst(value) => { stack.push(AST::FloatConstant(*value)); }
                Instr::LConst(value) => { stack.push(AST::LongConstant(*value)); }
                Instr::IMul | Instr::FMul | Instr::DMul | Instr::LMul => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    stack.push(AST::Mul(left, right));
                }
                Instr::IDiv | Instr::FDiv | Instr::DDiv | Instr::LDiv => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    stack.push(AST::Div(left, right));
                }
                Instr::IAdd | Instr::FAdd | Instr::DAdd | Instr::LAdd => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    stack.push(AST::Add(left, right));
                }
                Instr::ISub | Instr::FSub | Instr::DSub | Instr::LSub => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    stack.push(AST::Sub(left, right));
                }
                Instr::IRem | Instr::FRem | Instr::DRem | Instr::LRem => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    stack.push(AST::Sub(left, right));
                }
                Instr::F2l | Instr::D2l | Instr::I2l => { stack.prim_cast(VarType::Long)?; }
                Instr::F2d | Instr::I2d | Instr::L2d => { stack.prim_cast(VarType::Double)?; }
                Instr::F2i | Instr::D2i | Instr::L2i => { stack.prim_cast(VarType::Int)?; }
                Instr::I2f | Instr::D2f | Instr::L2f => { stack.prim_cast(VarType::Float)?; }
                Instr::I2b => { stack.prim_cast(VarType::Float)?; }
                Instr::I2s => { stack.prim_cast(VarType::Short)?; }
                Instr::I2c => { stack.prim_cast(VarType::Char)?; }
                Instr::AConstNull => { stack.push(AST::Null); }
                Instr::BIPush(value) => { stack.push(AST::Int(*value as i32)); }
                Instr::SIPush(value) => { stack.push(AST::Short(*value)); }
                Instr::Pop => { stack.pop()?; }
                Instr::Pop2 => { stack.pop2()?; }
                Instr::AReturn |
                Instr::IReturn |
                Instr::FReturn |
                Instr::DReturn |
                Instr::LReturn => {
                    let value = stack.pop_boxed()?;
                    statements.push(AST::Return(value))
                }
                Instr::MonitorEnter | Instr::MonitorExit => { stack.pop()?; }
                Instr::Nop => {}
                Instr::New(index) => {
                    let class = constant_pool.get_class_path_required(index)?;
                    stack.push(AST::New(class))
                }
                Instr::FCmpL | Instr::DCmpL => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    stack.push(AST::Comparison(ComparisonMode::Less, left, right));
                }
                Instr::FCmpG | Instr::DCmpG => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    stack.push(AST::Comparison(ComparisonMode::Greater, left, right));
                }
                Instr::LCmp => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    stack.push(AST::SignedComparison(left, right))
                }
                Instr::IAnd | Instr::LAnd => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    stack.push(AST::BitwiseAnd(left, right))
                }
                Instr::IOr | Instr::LOr => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    stack.push(AST::BitwiseOr(left, right))
                }
                Instr::IXOr | Instr::LXOr => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    stack.push(AST::Xor(left, right))
                }
                Instr::IShL | Instr::LShL => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    stack.push(AST::BitwiseShl(left, right))
                }
                Instr::IShR | Instr::LShR => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    stack.push(AST::BitwiseShr(left, right))
                }
                Instr::IUShR | Instr::LUShR => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    stack.push(AST::LogicalShr(left, right))
                }
                Instr::INeg | Instr::FNeg | Instr::DNeg | Instr::LNeg => {
                    let value = stack.pop_boxed()?;
                    stack.push(AST::Negate(value))
                }
                Instr::IInc { index, value } => { statements.push(AST::Increment { index: *index, value: *value }); }
                Instr::NewArray(array_type) => {
                    let count = stack.pop_boxed()?;
                    stack.push(AST::PrimitiveArray { array_type: *array_type, count })
                }
                Instr::MultiANewArray { index, dimensions } => {
                    let d = *dimensions;
                    for _ in 0..d {
                        stack.pop()?;
                    }
                    let class = constant_pool.get_class_path_required(index)?;
                    stack.push(AST::NewArrayMulti {
                        dimensions: d,
                        array_type: class,
                    })
                }
                Instr::Dup => {
                    let last = stack.values.last()
                        .ok_or(StackError::NotEnough(1, 0))?.clone();
                    stack.push(last)
                }
                Instr::DupX1 => {
                    let last = stack.values.last()
                        .ok_or(StackError::NotEnough(1, 0))?.clone();
                    stack.values.insert(stack.values.len() - 2, last)
                }
                Instr::DupX2 => {
                    let last = stack.values.last()
                        .ok_or(StackError::NotEnough(1, 0))?.clone();
                    stack.values.insert(stack.values.len() - 3, last)
                }
                Instr::Dup2 => {
                    let length = stack.values.len();
                    let last = stack.values.get(length - 1)
                        .ok_or(StackError::NotEnough(1, 0))?.clone();
                    let second_last = stack.values.get(length - 2)
                        .ok_or(StackError::NotEnough(1, 0))?.clone();
                    stack.push(second_last);
                    stack.push(last);
                }
                Instr::Dup2X1 => {
                    let length = stack.values.len();
                    let last = stack.values.get(length - 1)
                        .ok_or(StackError::NotEnough(1, 0))?.clone();
                    let second_last = stack.values.get(length - 2)
                        .ok_or(StackError::NotEnough(1, 0))?.clone();
                    stack.values.insert(stack.values.len() - 2, second_last);
                    stack.values.insert(stack.values.len() - 3, last);
                }
                Instr::Dup2X2 => {
                    let length = stack.values.len();
                    let last = stack.values.get(length - 1)
                        .ok_or(StackError::NotEnough(1, 0))?.clone();
                    let second_last = stack.values.get(length - 2)
                        .ok_or(StackError::NotEnough(1, 0))?.clone();
                    stack.values.insert(stack.values.len() - 3, second_last);
                    stack.values.insert(stack.values.len() - 4, last);
                }
                Instr::InstanceOf(index) => {
                    let class = constant_pool.get_class_path_required(index)?;
                    let reference = stack.pop_boxed()?;
                    stack.push(AST::InstanceOf(reference, class))
                }
                Instr::LookupSwitch { default, pairs } => {
                    let key = stack.pop_boxed()?;
                    statements.push(AST::SwitchLookup { key, default: *default, pairs: pairs.clone() })
                }
                Instr::TableSwitch { default, low, high, offsets } => {
                    let key = stack.pop_boxed()?;
                    statements.push(AST::SwitchTable { key, default: *default, low: *low, high: *high, offsets: offsets.clone() })
                }
                Instr::IfICmpEq(index) | Instr::IfACmpEq(index) => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    statements.push(AST::IfEqual(left, right, *index))
                }
                Instr::IfICmpNe(index) | Instr::IfACmpNe(index) => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    statements.push(AST::IfNotEqual(left, right, *index))
                }
                Instr::IfICmpGt(index) => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    statements.push(AST::IfGreaterThan(left, right, *index))
                }
                Instr::IfICmpGe(index) => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    statements.push(AST::IfGreaterThanOrEqual(left, right, *index))
                }
                Instr::IfICmpLt(index) => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    statements.push(AST::IfLessThan(left, right, *index))
                }
                Instr::IfICmpLe(index) => {
                    let left = stack.pop_boxed()?;
                    let right = stack.pop_boxed()?;
                    statements.push(AST::IfLessThanOrEqual(left, right, *index))
                }
                Instr::IfEq(index) => {
                    let value = stack.pop_boxed()?;
                    statements.push(AST::IfEq(value, *index))
                }
                Instr::IfGe(index) => {
                    let value = stack.pop_boxed()?;
                    statements.push(AST::IfGe(value, *index))
                }
                Instr::IfGt(index) => {
                    let value = stack.pop_boxed()?;
                    statements.push(AST::IfGt(value, *index))
                }
                Instr::IfLe(index) => {
                    let value = stack.pop_boxed()?;
                    statements.push(AST::IfLe(value, *index))
                }
                Instr::IfLt(index) => {
                    let value = stack.pop_boxed()?;
                    statements.push(AST::IfLt(value, *index))
                }
                Instr::IfNonNull(index) => {
                    let value = stack.pop_boxed()?;
                    statements.push(AST::IfNonnull(value, *index))
                }
                Instr::IfNull(index) => {
                    let value = stack.pop_boxed()?;
                    statements.push(AST::IfNull(value, *index))
                }
                Instr::JSr(index) => {
                    stack.push(AST::JSR(*index));
                }
                _ => {}
            };
        }
        stack.empty()?;
        Ok(statements)
    }
}

#[derive(Debug, Clone)]
pub enum ComparisonMode {
    Greater,
    Less,
}

#[derive(Debug, Clone)]
pub enum AST {
    Variable(u16, VarType),
    Set(u16, Box<AST>),
    FieldSet(MemberReference, Box<AST>, Box<AST>),
    FieldGet(MemberReference, Box<AST>),
    Mul(Box<AST>, Box<AST>),
    Div(Box<AST>, Box<AST>),
    Sub(Box<AST>, Box<AST>),
    Add(Box<AST>, Box<AST>),
    New(ClassPath),
    StaticSet(MemberReference, Box<AST>),
    StaticGet(MemberReference),
    MethodCall {
        member: MemberReference,
        reference: Box<AST>,
        args: Vec<AST>,
    },
    StaticCall {
        member: MemberReference,
        args: Vec<AST>,
    },
    InstanceOf(Box<AST>, ClassPath),
    Comparison(ComparisonMode, Box<AST>, Box<AST>),
    SignedComparison(Box<AST>, Box<AST>),
    PrimitiveCast {
        value: Box<AST>,
        primitive: VarType,
    },
    ClassCast {
        value: Box<AST>,
        class: ClassPath,
    },
    NewArrayMulti {
        array_type: ClassPath,
        dimensions: u8,
    },
    PrimitiveArray {
        array_type: ArrayType,
        count: Box<AST>,
    },
    ArrayLength(Box<AST>),
    ArrayLoad {
        reference: Box<AST>,
        index: Box<AST>,
    },
    ArrayStore {
        reference: Box<AST>,
        index: Box<AST>,
        value: Box<AST>,
    },
    SwitchLookup {
        key: Box<AST>,
        default: u32,
        pairs: Vec<(i32, u32)>,
    },
    SwitchTable {
        key: Box<AST>,
        default: u32,
        low: u32,
        high: u32,
        offsets: Vec<u32>,
    },
    StringConst(String),
    IntegerConstant(i32),
    FloatConstant(f32),
    LongConstant(i64),
    DoubleConstant(f64),
    Short(i16),
    Int(i32),
    Null,
    VoidReturn,
    Return(Box<AST>),
    Negate(Box<AST>),
    Xor(Box<AST>, Box<AST>),
    BitwiseAnd(Box<AST>, Box<AST>),
    BitwiseOr(Box<AST>, Box<AST>),
    BitwiseShl(Box<AST>, Box<AST>),
    BitwiseShr(Box<AST>, Box<AST>),
    LogicalShr(Box<AST>, Box<AST>),
    Remainder(Box<AST>, Box<AST>),
    Increment { index: u16, value: i16 },
    // Conditionals
    IfEqual(Box<AST>, Box<AST>, BranchIndex),
    IfNotEqual(Box<AST>, Box<AST>, BranchIndex),
    IfGreaterThanOrEqual(Box<AST>, Box<AST>, BranchIndex),
    IfGreaterThan(Box<AST>, Box<AST>, BranchIndex),
    IfLessThan(Box<AST>, Box<AST>, BranchIndex),
    IfLessThanOrEqual(Box<AST>, Box<AST>, BranchIndex),
    IfEq(Box<AST>, BranchIndex),
    IfGe(Box<AST>, BranchIndex),
    IfGt(Box<AST>, BranchIndex),
    IfLe(Box<AST>, BranchIndex),
    IfLt(Box<AST>, BranchIndex),
    IfNe(Box<AST>, BranchIndex),
    IfNonnull(Box<AST>, BranchIndex),
    IfNull(Box<AST>, BranchIndex),
    JSR(BranchIndex),
}

impl AST {
    pub fn write_java<W: Write>(&self, o: &mut W, member: &Member, code_attr: &CodeAttr) -> WriteResult {
        let access = member.access_flags;
        match self {
            AST::Variable(index, _) => {
                if *index == 0 && !access.is_set(AccessFlag::Static) {
                    write!(o, "this")?;
                } else {
                    write!(o, "var{}", index)?;
                }
            }
            AST::Set(index, value) => {
                if *index == 0 && !access.is_set(AccessFlag::Static) {
                    write!(o, "this = ")?;
                } else {
                    write!(o, "var{} = ", index)?;
                }
                value.write_java(o, member, code_attr)?;
                write!(o, ";")?;
            }
            AST::FieldGet(field, reference) => {
                reference.write_java(o, member, code_attr)?;
                let name = &field.name_and_type.name;
                write!(o, ".{}", name)?;
            }
            AST::FieldSet(field, reference, value) => {
                reference.write_java(o, member, code_attr)?;
                let name = &field.name_and_type.name;
                write!(o, ".{} = ", name)?;
                value.write_java(o, member, code_attr)?;
                write!(o, ";")?;
            }
            AST::Mul(left, right) => {
                left.write_java(o, member, code_attr)?;
                write!(o, " * ")?;
                right.write_java(o, member, code_attr)?;
            }
            AST::Div(left, right) => {
                left.write_java(o, member, code_attr)?;
                write!(o, " / ")?;
                right.write_java(o, member, code_attr)?;
            }
            AST::Sub(left, right) => {
                left.write_java(o, member, code_attr)?;
                write!(o, " - ")?;
                right.write_java(o, member, code_attr)?;
            }
            AST::Add(left, right) => {
                left.write_java(o, member, code_attr)?;
                write!(o, " + ")?;
                right.write_java(o, member, code_attr)?;
            }
            AST::Xor(left, right) => {
                left.write_java(o, member, code_attr)?;
                write!(o, " ^ ")?;
                right.write_java(o, member, code_attr)?;
            }
            AST::BitwiseAnd(left, right) => {
                left.write_java(o, member, code_attr)?;
                write!(o, " & ")?;
                right.write_java(o, member, code_attr)?;
            }
            AST::BitwiseOr(left, right) => {
                left.write_java(o, member, code_attr)?;
                write!(o, " | ")?;
                right.write_java(o, member, code_attr)?;
            }
            AST::BitwiseShl(left, right) => {
                left.write_java(o, member, code_attr)?;
                write!(o, " << ")?;
                right.write_java(o, member, code_attr)?;
            }
            AST::BitwiseShr(left, right) => {
                left.write_java(o, member, code_attr)?;
                write!(o, " >> ")?;
                right.write_java(o, member, code_attr)?;
            }
            AST::LogicalShr(left, right) => {
                left.write_java(o, member, code_attr)?;
                write!(o, " >>> ")?;
                right.write_java(o, member, code_attr)?;
            }
            AST::Remainder(left, right) => {
                left.write_java(o, member, code_attr)?;
                write!(o, " % ")?;
                right.write_java(o, member, code_attr)?;
            }
            AST::Increment { index, value } => {

                if *value == 1 {
                    write!(o, "var{}++", index)?;
                } else {
                    write!(o, "var{} += {}", index, value)?;
                }
            }
            AST::Null => { write!(o, "null")?; }
            AST::Negate(value) => {
                write!(o, "-")?;
                value.write_java(o, member, code_attr)?;
            }
            AST::New(class) => {
                write!(o, "new {}()", class.name)?;
            }
            AST::StaticSet(field, value) => {
                write!(o, "{}.{} = ", field.class.name, field.name_and_type.name)?;
                value.write_java(o, member, code_attr)?;
            }
            AST::StaticGet(field) => {
                write!(o, "{}.{}", field.class.name, field.name_and_type.name)?;
            }
            AST::MethodCall { member: method, reference, args } => {
                reference.write_java(o, member, code_attr)?;
                let name = &method.name_and_type.name;
                if name == "<init>" {
                    write!(o, "(")?;
                } else {
                    write!(o, ".{}(", name)?;
                }
                if !args.is_empty() {
                    let max = args.len() - 1;
                    for (i, value) in args.iter().enumerate() {
                        value.write_java(o, member, code_attr)?;
                        if i != max {
                            write!(o, ", ")?;
                        }
                    }
                }
                write!(o, ")")?;
                if let Descriptor::Method(met) = &method.name_and_type.descriptor {
                    if let Descriptor::Void = *met.return_type {
                        write!(o, ";")?;
                    }
                }
            }
            AST::StaticCall { member: method, args } => {
                write!(o, "{}.{}(", method.class.name, method.name_and_type.name)?;
                if !args.is_empty() {
                    let max = args.len() - 1;
                    for (i, value) in args.iter().enumerate() {
                        value.write_java(o, member, code_attr)?;
                        if i != max {
                            write!(o, ", ")?;
                        }
                    }
                }
                write!(o, ")")?;
                if let Descriptor::Method(met) = &method.name_and_type.descriptor {
                    if let Descriptor::Void = *met.return_type {
                        write!(o, ";")?;
                    }
                }
            }
            AST::InstanceOf(value, class) => {
                value.write_java(o, member, code_attr)?;
                write!(o, " instanceof {}", class.name)?;
            }
            AST::Comparison(mode, left, right) => {
                left.write_java(o, member, code_attr)?;
                write!(o, " {} ", match mode {
                    ComparisonMode::Greater => ">",
                    ComparisonMode::Less => "<"
                })?;
                right.write_java(o, member, code_attr)?;
            }
            AST::SignedComparison(_, _) => unimplemented!(),
            AST::PrimitiveCast { value, primitive } => {
                write!(o, "(({}) (", primitive)?;
                value.write_java(o, member, code_attr)?;
                write!(o, "))")?;
            }
            AST::ClassCast { value, class } => {
                write!(o, "(({}) (", class.name)?;
                value.write_java(o, member, code_attr)?;
                write!(o, "))")?;
            }
            AST::StringConst(value) => { write!(o, "\"{}\"", value)?; }
            AST::IntegerConstant(value) |
            AST::Int(value) => { write!(o, "{}", value)?; }
            AST::Short(value) => { write!(o, "{}", value)?; }
            AST::FloatConstant(value) => { write!(o, "{}F", value)?; }
            AST::LongConstant(value) => { write!(o, "{}L", value)?; }
            AST::DoubleConstant(value) => { write!(o, "{}D", value)?; }
            AST::VoidReturn => { write!(o, "return;")?; }
            AST::Return(value) => {
                write!(o, "return ")?;
                value.write_java(o, member, code_attr)?;
                write!(o, ";")?;
            }
            AST::NewArrayMulti { array_type, dimensions } => {
                let descriptor = Descriptor::parse(array_type.name.as_str());
                if let Descriptor::Array(array_desc) = descriptor {
                    let mut dim = *dimensions;
                    if array_desc.dimensions != dim {
                        dim = array_desc.dimensions;
                    }
                    write!(o, "new {}{}", array_desc.descriptor.to_java(), "[]".repeat(dim as usize))?;
                } else {
                    write!(o, "/* bad array descriptor */")?;
                }
            }
            AST::ArrayLoad { index, reference } => {
                reference.write_java(o, member, code_attr)?;
                write!(o, "[")?;
                index.write_java(o, member, code_attr)?;
                write!(o, "]")?;
            }
            AST::ArrayLength (reference)=> {
                reference.write_java(o, member, code_attr)?;
                write!(o, ".length")?;
            }
            AST::IfEq(value, branch) => {
                write!(o, "ifeq ")?;
                value.write_java(o, member, code_attr)?;
                write!(o, " goto: {}", branch)?;
            }
            AST::IfGreaterThanOrEqual(left, right, branch) => {
                left.write_java(o, member, code_attr)?;
                write!(o, " >= ")?;
                right.write_java(o, member, code_attr)?;
                write!(o, " goto: {}", branch)?;
            }
            v => { write!(o, "{:?}", v)?; }
        }
        Ok(())
    }
}

pub fn get_index_for_pos(instructions: &InstrSet, pos: u16) -> Option<usize> {
    for (i, (i_pos, _)) in instructions.iter().enumerate() {
        if i_pos == &(pos as u64) {
            return Some(i);
        }
    }
    None
}

pub fn split_at_multiple<T: Clone>(vec: Vec<T>, split_indices: Vec<usize>) -> Vec<Vec<T>> {
    let mut split_indices = split_indices;
    split_indices.sort();
    split_indices.dedup();
    let mut output = Vec::with_capacity(split_indices.len() + 1);

    if split_indices.len() == 0 {
        return vec![vec];
    } else {
        if split_indices[0] == 0 {
            split_indices.remove(0);
        }
        if *split_indices.last().unwrap() == vec.len() {
            split_indices.remove(vec.len() - 1);
        }
        let mut split_vector = vec;
        for i in 0..split_indices.len() {
            let index = split_indices[i] - if i == 0 { 0 } else { split_indices[i - 1] };
            let (first, second) = split_vector.split_at(index);

            output.push(first.to_vec());
            if i + 1 == split_indices.len() {
                output.push(second.to_vec());
            }
            split_vector = second.to_vec();
        }
    }
    output
}

pub fn gen_control_flow_graph(instructions: &InstrSet) -> HashMap<u64, Block> {
    let mut jump_indices = Vec::new();
    for (i, (_, instr)) in instructions.iter().enumerate() {
        match &instr {
            Instr::IfNe(branch)
            | Instr::IfEq(branch)
            | Instr::IfLe(branch)
            | Instr::IfGe(branch)
            | Instr::IfGt(branch)
            | Instr::IfLt(branch)
            | Instr::IfICmpEq(branch)
            | Instr::IfICmpNe(branch)
            | Instr::IfICmpGt(branch)
            | Instr::IfICmpGe(branch)
            | Instr::IfICmpLt(branch)
            | Instr::IfICmpLe(branch) => {
                let true_pos = get_index_for_pos(&instructions, *branch).unwrap();
                jump_indices.push(true_pos);
                let false_pos = i + 1;
                jump_indices.push(false_pos);
            }
            Instr::Goto(branch) => {
                let jump_pos = get_index_for_pos(&instructions, *branch).unwrap();
                jump_indices.push(jump_pos);
            }
            _ => {}
        }
    }

    let raw_blocks = split_at_multiple(instructions.clone(), jump_indices);
    let mut blocks: HashMap<u64, Block> = raw_blocks
        .iter()
        .map(|el| {
            (
                el[0].0,
                Block {
                    instructions: el.clone(),
                    branches: Vec::new(),
                },
            )
        })
        .collect();

    for (_, block) in &mut blocks {
        let (last_pos, last_instr) = block.instructions.last().unwrap();
        let next = instructions
            .iter()
            .skip_while(|el| el.0 <= *last_pos)
            .next();

        match last_instr {
            Instr::IfNe(branch)
            | Instr::IfEq(branch)
            | Instr::IfLe(branch)
            | Instr::IfGe(branch)
            | Instr::IfGt(branch)
            | Instr::IfLt(branch)
            | Instr::IfICmpEq(branch)
            | Instr::IfICmpNe(branch)
            | Instr::IfICmpGt(branch)
            | Instr::IfICmpGe(branch)
            | Instr::IfICmpLt(branch)
            | Instr::IfICmpLe(branch) => {
                let next_pos = next.unwrap().0;
                block.branches.push(*branch as u64);
                block.branches.push(next_pos);
            }
            Instr::Goto(branch) => {
                block.branches.push(*branch as u64);
            }
            Instr::Return
            | Instr::AReturn
            | Instr::IReturn
            | Instr::LReturn
            | Instr::DReturn
            | Instr::FReturn => {}
            _ => {
                let next_pos = next.unwrap().0;
                block.branches.push(next_pos);
            }
        }
    }
    blocks
}

pub fn find_paths(blocks: &HashMap<u64, Block>, node: u64, path_in: Vec<u64>) -> Vec<Vec<u64>> {
    let block: &Block = blocks.get(&node).unwrap();
    let start_vector = vec![node];
    let mut path = path_in;
    path.push(node);
    let mut paths = Vec::new();
    if block.branches.len() == 0 {
        paths.push(start_vector);
    } else {
        for b in &block.branches {
            if !path.contains(b) {
                for p in find_paths(blocks, *b, path.clone()) {
                    let mut v = start_vector.clone();
                    v.extend(p.iter());
                    paths.push(v);
                }
            } else {
                let mut v = start_vector.clone();
                v.push(*b);
                paths.push(v);
            }
        }
    }
    paths
}