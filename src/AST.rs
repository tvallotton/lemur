


struct Asignment {
    name: String, 
    args: Vec[String],
    value: Expr
}



 
type Id = String;

struct MonadicBiding {
    id: Id,
    monad: Box<Expr>
}

struct TypeLiteral {
    name: String,
    args: Vec<String>
}

struct Type {
    constraints: Vec<TypeLiteral>,
    signature:  Vec<TypeLiteral>
}



enum Expr {
    FuncCall {
        name: String
        arg0: Box<Expr>
    },

    Let {
        asignments: Vec<Asignment>,
        expression: Box<Expr>
    }

    Identifier(Id),

    Literal {
        value: String,
    },
 
    ListComp {
        value: Box<Expr>, 
        binds: Vec<MonadicBiding>,
        filters: Vec<Expr>
    },

    IfThenElse {
        cond:  Box<Expr>,
        then:  Box<Expr>,
        else_: Box<Expr>
    },
    WhenGuard {
        cond:  Box<Expr>,
        then:  Box<Expr>,
        else_: Box<Expr>
    },
    Caseof {
        value: Box<Expr>, 
        matches: Vec<Expr, Expr>
    }
}