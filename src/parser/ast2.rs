use rug;


type Identifier = String;
use std::collections::HashMap;



struct Module {
    imports: Vex<Import>,
    data_declarations: Vec<DataDecl>
    assignments: Vec<Asignment>
}


enum DataDecl {
    
    Enum {
        name: Identifier,
        args: Vec<>
        
        
    }

    Data {
        name: Identifier,
        args: Vec<Identifier>,
        body: HashMap<Identifier, TypeSignature>
    }
    // Instance*

}



//        context           value
//    ______________     __________
// f: (a: SomeClass) =>  a b -> c d
//                       ___
//                      type  
//     ____________________________
//         type_signature
// ________________________________
//      type_declaration



struct Type {
    generic: Bool,
    name: Identifier,
    params: Vec<Type>,
}
struct TypeSignature {
    context: Vec<TypeDeclaration>,
    value: Vec<Type>
}

struct TypeDeclaration {
    name: Identifier,
    value: TypeSignature,
}




    



struct Asignment {
    name: Identifier,
    args: Vec<String>,
    value: Box<Expr>,
}


enum Primitive {
    Float(f64),
    Complex, 
    Integer(i64),
    String, 
    Char(String),
    List(Vec<Expr>),
    NamedTuple(HashMap<Identifier, Box<Expr>>),
    AnonymusTuple(Vec<Expr>),
}


enum Expr {
    FuncCall {
        name: Identifier,
        arg0: Box<Expr>,
    },

    Let {
        asignments: Vec<Asignment>,
        expression: Box<Expr>,
    },

    Identifier(Identifier),
    

    Literal {
        type_: Primitive,
        value: String,
    },
    ListComp {
        value: Box<Expr>,
        binds: Vec<MonadicBiding>,
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
