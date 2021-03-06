use super::super::lexer::tokens::*;
use rug;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Module {
    
    pub imports: Vec<Import>,
    pub submodules: HashMap<Variable, Module>,
    pub types: Vec<Type>,
    pub data: Vec<DataDecl>,
    pub assignments: Vec<Assignment>,
}


pub type MonadicBinding = Assignment;
pub type Variable = String;
pub use import::*;
pub use data::*;
pub use types::*;

mod data {
    use super::*;

    #[derive(PartialEq, Debug)]
    pub struct Struct {
        name: Variable,
        body: Vec<TypeDeclaration>,
    }

    #[derive(PartialEq, Debug)]
    pub enum DataDecl {
        Synm {
            name: Type,
            def: Type,
        },
        Enum {
            name: Variable,
            args: Vec<Variable>,
            structs: Struct,
        },
        Data {
            name: Variable,
            args: Vec<Variable>,
            body: Vec<TypeDeclaration>,
        },
    }
}

mod import {
    use super::*;
    #[derive(PartialEq, Debug)]
    pub struct Import {
        name: Identifier,
        pseudonym: Option<Variable>,
    }
}

// TYPE DECLARATIONS

mod types {
    use super::*;
    #[derive(PartialEq, Debug)]
    pub struct Type {
        generic: bool,
        name: Identifier,
        params: Vec<Type>,
    }
    #[derive(PartialEq, Debug)]
    pub struct TypeSignature {
        context: Vec<TypeDeclaration>,
        value: Vec<Type>,
    }
    #[derive(PartialEq, Debug)]
    pub struct TypeDeclaration {
        name: Variable,
        value: TypeSignature,
    }
}

#[derive(PartialEq, Debug)]
pub enum Pattern {
    Primitive(Primitive),
    Tuple(Vec<Pattern>),
    NamedTuple(HashMap<Variable, Pattern>),
    List(Vec<Pattern>),
    Variable(Variable),
    Constructor(Identifier, HashMap<Variable, Pattern>),
}

#[derive(PartialEq, Debug, )]
pub struct Assignment {
    name: Option<Identifier>,
    args: Vec<Pattern>,
    value: Box<Expr>,
}
#[derive(PartialEq, Debug)]
pub enum Primitive {
    Bool(bool),
    Float(f64),
    Complex(f64),
    Integer(rug::Integer),
    String(String),
    Char(char),
    List(Vec<Expr>),
    NamedTuple(HashMap<Variable, Expr>),
    AnonymusTuple(Vec<Expr>),
}
#[derive(PartialEq, Debug)]

// function call
// let
// index access array[i]
// if then else
// literals
// list comprehensions
// when cuards
// case of expression
// binary operator
// do notation
// standalone variable
// Instance call
// slices 1..10
pub enum Expr {
    Variable(Identifier),
    Literal(Primitive),
    FuncCall {
        name: Identifier,
        arg0: Box<Expr>,
    },

    // let
    //     a = val
    //     a
    Let {
        asignments: Vec<Assignment>,
        expression: Box<Expr>,
    },
    //  [ value | bind <- Expr, assign = 3, filter % 2 == 0 ]
    ListComp {
        value: Box<Expr>,
        binds: Vec<Assignment>,
        assignments: Vec<Assignment>,
        filters: Vec<Expr>,
    },

    // if condition then val0 else val1
    IfThenElse {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },

    // when
    //     | cond0 => expr0
    //     | cond1 => expr1
    WhenGuard {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },

    // case value of
    //     val0 => expr0
    //     val1 => expr1
    Caseof {
        value: Box<Expr>,
        matches: (Box<Expr>, Box<Expr>),
    },

    // do
    //    x <- monad
    //    y = 4
    //    expr0
    Do {
        steps: Vec<Result<MonadicBinding, Assignment>>,
        expr: Box<Expr>,
    },
}


#[derive(PartialEq, Debug)]
pub struct Identifier {
    parent: Variable,
    children: Option<Box<Identifier>>,
}


impl Module {
    pub fn new() -> Module {
        Module {
            assignments: vec![],
            data: vec![],
            types: vec![],
            imports: vec![],
            submodules: HashMap::new(),
        }
    }
}