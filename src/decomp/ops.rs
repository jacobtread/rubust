use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::class::class::ClassPath;
use crate::class::constant::{Constant, ConstantPool, NameAndType};
use crate::class::descriptor::{Descriptor, MethodDescriptor};
use crate::class::member::Member;
use crate::class::op::{Instr, InstrSet};
use crate::decomp::ops::AST::ArrayLength;
use crate::error::{ConstantError, DecompileError};

#[derive(Debug, Clone)]
pub enum VarType {
    Byte,
    Int,
    Float,
    Double,
    Long,
    Reference,
}

impl Display for VarType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   VarType::Reference => panic!("not enough context to print reference"),
                   VarType::Int => "int",
                   VarType::Float => "float",
                   VarType::Double => "double",
                   VarType::Long => "long",
                   VarType::Byte => "byte",
               }
        )
    }
}


#[derive(Debug, Clone)]
pub struct Block {
    instructions: InstrSet,
    branches: Vec<u64>,
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

#[derive(Debug, Clone)]
pub enum AST {
    BasicCast {
        value: Box<AST>,
        cast_type: VarType,
    },
    ClassCast {
        value: Box<AST>,
        cast_type: ClassPath,
    },
    Static {
        field_data: Descriptor,
    },
    Variable {
        index: u16,
        var_type: VarType,
    },
    Call {
        method_data: NameAndType,
        reference: Box<AST>,
        args: Vec<AST>,
    },
    ArrayLength {
        reference: Box<AST>,
    },
    ArrayReference {
        array_type: ClassPath,
        dimensions: u8,
    },
    ConstInt(i64),
    ConstFloat(f64),
    ConstString(String),
    VoidReturn,
    Set {
        index: u16,
        value: Box<AST>,
    },
    Mul {
        lhs: Box<AST>,
        rhs: Box<AST>,
    },
}

pub fn decompile_block(
    block: &Block,
    constant_pool: &ConstantPool,
) -> Result<Vec<AST>, DecompileError> {
    let mut statements = Vec::new();
    let mut stack: Vec<AST> = Vec::new();
    for (pos, code) in &block.instructions {
        match code {
            Instr::ILoad(index) => {
                stack.push(AST::Variable {
                    index: *index,
                    var_type: VarType::Int,
                });
            }
            Instr::LLoad(index) => {
                stack.push(AST::Variable {
                    index: *index,
                    var_type: VarType::Long,
                });
            }
            Instr::FLoad(index) => {
                stack.push(AST::Variable {
                    index: *index,
                    var_type: VarType::Float,
                });
            }
            Instr::DLoad(index) => {
                stack.push(AST::Variable {
                    index: *index,
                    var_type: VarType::Double,
                });
            }
            Instr::ALoad(index) => {
                stack.push(AST::Variable {
                    index: *index,
                    var_type: VarType::Reference,
                });
            }
            Instr::InvokeSpecial(index) | Instr::InvokeVirtual(index) => {
                let method = constant_pool.get_member_ref(*index)?;
                let nat = constant_pool.get_name_and_type(method.name_and_type_info)?;
                let descriptor = &nat.descriptor;
                if let Descriptor::Method(method) = descriptor {
                    let mut args = Vec::new();
                    for _ in 0..method.parameters.len() {
                        args.push(stack.pop().ok_or(DecompileError::EmptyStack)?);
                    }
                    args.reverse();
                    let reference = Box::new(stack.pop().ok_or(DecompileError::EmptyStack)?);
                    if let Descriptor::Void = *method.return_type {
                        statements.push(AST::Call {
                            method_data: nat,
                            reference,
                            args,
                        });
                    } else {
                        stack.push(AST::Call {
                            method_data: nat,
                            reference,
                            args,
                        });
                    }
                } else {
                    Err(DecompileError::ExpectedMethodDescriptor)?;
                }
            }
            Instr::Return => {
                statements.push(AST::VoidReturn);
            }
            Instr::IStore(index)
            | Instr::LStore(index)
            | Instr::FStore(index)
            | Instr::DStore(index)
            | Instr::AStore(index) => {
                statements.push(AST::Set {
                    index: *index,
                    value: Box::new(stack.pop().ok_or(DecompileError::EmptyStack)?),
                });
            }
            Instr::GetStatic(index) => {
                let field = constant_pool.get_member_ref(*index)?;
                let nat = constant_pool.get_name_and_type(field.name_and_type_info)?;
                let descriptor = nat.descriptor;
                stack.push(AST::Static { field_data: descriptor });
            }
            Instr::ArrayLength => {
                let reference = Box::new(stack.pop().ok_or(DecompileError::EmptyStack)?);
                stack.push(AST::ArrayLength { reference });
            }
            Instr::LoadConst(index) => {
                let value = match constant_pool.inner.get(index)
                    .ok_or(ConstantError::NotFound(*index))? {
                    Constant::String(index) => AST::ConstString(constant_pool.get_utf8(*index)?.clone()),
                    Constant::Long(value) => AST::ConstInt(*value),
                    Constant::Integer(value) => AST::ConstInt(*value as i64),
                    Constant::Double(value) => AST::ConstFloat(*value),
                    Constant::Float(value) => AST::ConstFloat(*value as f64),
                    _ => unimplemented!(),
                };
                stack.push(value);
            }
            Instr::IConst(value) => stack.push(AST::ConstInt(*value as i64)),
            Instr::IMul => {
                let rhs = Box::new(stack.pop().ok_or(DecompileError::EmptyStack)?);
                let lhs = Box::new(stack.pop().ok_or(DecompileError::EmptyStack)?);
                stack.push(AST::Mul { lhs, rhs });
            }
            Instr::I2b => {
                let cast_type = VarType::Byte;
                let value = Box::new(stack.pop().ok_or(DecompileError::EmptyStack)?);
                stack.push(AST::BasicCast { cast_type, value })
            }
            Instr::CheckCast(index) => {
                let cast_type = constant_pool.get_class_path_required(*index)?;
                let value = Box::new(stack.pop().ok_or(DecompileError::EmptyStack)?);
                stack.push(AST::ClassCast { cast_type, value })
            }
            Instr::PutField(index) => {
                stack.pop().ok_or(DecompileError::EmptyStack)?;
                stack.pop().ok_or(DecompileError::EmptyStack)?;
            }
            Instr::MultiANewArray { index, dimensions } => {
                let i = *index;
                let d = *dimensions;
                for _ in 0..d {
                    stack.pop().ok_or(DecompileError::EmptyStack)?;
                }
                let clazz = constant_pool.get_class_path(i)?
                    .ok_or(ConstantError::NotFound(i))?;
                stack.push(AST::ArrayReference {
                    dimensions: d,
                    array_type: clazz,
                })
            }
            v => println!("{:?}", v),
        }
    }
    if stack.len() != 0 {
        return Err(DecompileError::StackSize(stack.len()));
    }
    Ok(statements)
}