use rug;

pub struct Module {
    pub imports: Vex<Import>,             // ready to test
    pub data_declarations: Vec<DataDecl>, //
    pub assignments: Vec<Asignment>,
    pub type_declarations: Vec<Type>,
}

// IMPORTS
pub struct Import {
    name: InstanceCall,
    pseudonym: Identifier,
}



// // TYPE DECLARATIONS
// struct Type {
//     generic: Bool,
//     name: Identifier,
//     params: Vec<Type>,
// }
// struct TypeSignature {
//     context: Vec<TypeDeclaration>,
//     value: Vec<Type>,
// }

// struct TypeDeclaration {
//     name: Identifier,
//     value: TypeSignature,
// }


// // ASIGNMENTS

// struct Asignment {
//     name: Identifier,
//     args: Vec<String>,
//     value: Box<Expr>,
// }


// enum Primitive {
//     Float(f64),
//     Complex, 
//     Integer(i64),
//     String,
//     Char(String),
//     List(Vec<Expr>),
//     NamedTuple(HashMap<Identifier, Box<Expr>>),
//     AnonymusTuple(Vec<Expr>),
// }





// enum Expr {
//     // function call            #
//     // let                      #
//     // index access array[i]    
//     // if then else             #
//     // literals 
//     // list comprehensions      
//     // when cuards              
//     // case of expression
//     // binary operator
//     // do notation
//     // standalone variable
//     // Instance call
//     Variable(Identifier),
    
//     Literal(Primitive),
    
//     FuncCall {
//         name: Identifier,
//         arg0: Box<Expr>,
//     },

//     // ready
//     Let {
//         asignments: Vec<Asignment>,
//         expression: Box<Expr>,
//     },

    
    

    

//     ListComp {
//         value: Box<Expr>,
//         binds: Vec<MonadicBiding>,
//         filters: Vec<Expr>,
//     },
//     IfThenElse {
//         cond: Box<Expr>,
//         then: Box<Expr>,
//         else_: Box<Expr>,
//     },
//     WhenGuard {
//         cond: Box<Expr>,
//         then: Box<Expr>,
//         else_: Box<Expr>,
//     },
//     Caseof {
//         value: Box<Expr>,
//         matches: (Box<Expr>, Box<Expr>),
//     },
// }


// pub Variable = String;

// pub Identifier {
//     parent: String,
//     children: Vec<String>
// }
// impl Identifier {
//     fn base(&self) -> bool {
//         self.children.len() == 0
//     }
// }




#[cfg(test)]
mod tests {
    use super::*;
    use rug;
}
