use rug;
use std::collections::HashMap;

pub struct Module {
    pub imports: Vec<Import>,         // ready
    pub type_declarations: Vec<Type>, // ready
    pub data_declarations: Vec<DataDecl>,
    pub assignments: Vec<Asignment>,
}

// DATA DECLARATIONS

pub struct Struct {
    name: Variable,
    body: Vec<TypeDeclaration>,
}

pub enum DataDecl {
    Synm {
        name: Type, 
        def: Type
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
pub struct Import {
    name: Identifier,
    pseudonym: Variable,
}

// TYPE DECLARATIONS
pub struct Type {
    generic: bool,
    name: Identifier,
    params: Vec<Type>,
}
pub struct TypeSignature {
    context: Vec<TypeDeclaration>,
    value: Vec<Type>,
}

pub struct TypeDeclaration {
    name: Variable,
    value: TypeSignature,
}

// ASIGNMENTS

pub struct Asignment {
    name: Identifier,
    args: Vec<String>,
    value: Box<Expr>,
}

pub enum Primitive {
    Float(f64),
    Complex(f64),
    Integer(rug::Integer),
    String(String),
    Char(char),
    List(Vec<Expr>),
    NamedTuple(HashMap<Variable, Box<Expr>>),
    AnonymusTuple(Vec<Expr>),
}

pub enum Expr {

    // function call            #
    // let                      #
    // index access array[i]
    // if then else             #
    // literals
    // list comprehensions
    // when cuards
    // case of expression
    // binary operator
    // do notation
    // standalone variable
    // Instance call
    // slices 1..10 
    Variable(Identifier),
    //
    Literal(Primitive),
    //
    FuncCall {
        name: Identifier,
        arg0: Box<Expr>,
    },

    // ready
    Let {
        asignments: Vec<Asignment>,
        expression: Box<Expr>,
    },

    //
    //

    //  [ value | bind <- Expr, assign = 3, filter % 2 == 0 ]
    ListComp {
        value: Box<Expr>,   
        binds: HashMap<Variable, Expr>,
        assignments: Vec<Asignment>,
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
}

pub type Variable = String;
pub struct Identifier {
    parent: Variable,
    children: Vec<Variable>,
}




