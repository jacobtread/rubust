use std::collections::HashMap;
use std::detect::__is_feature_detected::popcnt;
use std::fmt::{Display, Formatter};
use crate::class::class::ClassPath;

use crate::class::constant::{ConstantPool, MemberReference};
use crate::class::descriptor::Descriptor;
use crate::class::op::{Instr, InstrSet};
use crate::error::{DecompileError, StackError};

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

    fn pop_count(&mut self, amount: usize) -> Result<(), StackError> {
        for _ in 0..amount {
            self.pop()?
        }
        Ok(())
    }

    fn push(&mut self, value: AST) {
        self.values.push(value)
    }
}

impl Block {
    fn decompile(&self, constant_pool: &ConstantPool) -> DecompileResult<ASTSet> {
        let mut statements = Vec::new();
        let mut stack = Stack::new();

        for (pos, code) in &self.instructions {
            match code {
                Instr::ILoad(index) => stack.push(AST::Variable(*index, VarType::Int)),
                Instr::LLoad(index) => stack.push(AST::Variable(*index, VarType::Long)),
                Instr::FLoad(index) => stack.push(AST::Variable(*index, VarType::Float)),
                Instr::DLoad(index) => stack.push(AST::Variable(*index, VarType::Double)),
                Instr::ALoad(index) => stack.push(AST::Variable(*index, VarType::Reference)),
                Instr::InvokeSpecial(index) |
                Instr::InvokeVirtual(index) => {
                    let member = constant_pool.get_member_ref(index)?;
                    let descriptor = &member.name_and_type.descriptor;
                    if let Descriptor::Method(method) = descriptor {
                        let mut args = Vec::new();
                        for _ in 0..method.parameters.len() {
                            args.push(stack.pop()?);
                        }
                        args.reverse();
                        let reference = Box::new(stack.pop()?);
                        if let &Descriptor::Void = method.return_type {
                            statements.push(AST::)
                        }
                    } else {
                        Err(DecompileError::ExpectedMethodDescriptor)?;
                    }
                }
                _ => {}
            }
        }

        if stack.len() != 0 {}

        Ok(statements)
    }
}

#[derive(Debug, Clone)]
pub enum AST {
    Variable(u16, VarType),
    Set(u16, Box<AST>),
    Mul(Box<AST>, Box<AST>),
    Static(MemberReference),
    PrimitiveCast {
        value: Box<AST>,
        primitive: VarType
    },
    ClassCast {
        value: Box<AST>,
        class: ClassPath,
    },
    StringConst(String),
    FloatConst(f64),
    IntConst(i64),

    VoidReturn
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