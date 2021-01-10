use rug;
use std::collections::HashMap;
use super::super::lexer::tokens::*;


#[derive(PartialEq)]
pub struct Module {
    pub name: Variable,
    pub imports: Vec<Import>,        
    pub sub_modules: Vec<Module>,    
    pub type_declarations: Vec<Type>, 
    pub data_declarations: Vec<DataDecl>,
    pub assignments: Vec<Assignment>,
}

// DATA DECLARATIONS
#[derive(PartialEq)]
pub struct Struct {
    name: Variable,
    body: Vec<TypeDeclaration>,
}

#[derive(PartialEq)]
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

// IMPORTS
// import some.library
// import some.library as shorthand
// import super.some_modules
#[derive(PartialEq)]
pub struct Import {
    name: Identifier,
    pseudonym: Option<Variable>,
}




// TYPE DECLARATIONS
#[derive(PartialEq)]
pub struct Type {
    generic: bool,
    name: Identifier,
    params: Vec<Type>,
}
#[derive(PartialEq)]
pub struct TypeSignature {
    context: Vec<TypeDeclaration>,
    value: Vec<Type>,
}
#[derive(PartialEq)]
pub struct TypeDeclaration {
    name: Variable,
    value: TypeSignature,
}

#[derive(PartialEq)]
pub enum Pattern {
    Primitive(Primitive),
    Tuple(Vec<Pattern>),
    NamedTuple(HashMap<Variable, Pattern>),
    List(Vec<Pattern>),
    Variable(Variable),
    Constructor(Identifier, HashMap<Variable, Pattern>),
}

#[derive(PartialEq)]
pub struct Assignment {
    name: Option<Identifier>,
    args: Vec<Pattern>,
    value: Box<Expr>,
}
#[derive(PartialEq)]
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
#[derive(PartialEq)]

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

    IfThenElse {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
    WhenGuard {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
    Caseof {
        value: Box<Expr>,
        matches: (Box<Expr>, Box<Expr>),
    },

    Do {
        bind: Option<Assignment>,
        assignment: Option<Assignment>,
        expr: Box<Expr>,
    },
}

pub type Variable = String;

#[derive(PartialEq)]
pub struct Identifier {
    parent: Variable,
    children: Result<Variable, Identifier>,
}


// MODULE

// Asignments

// with pattern matching
// plain variables

// (n1, n2) = expr
