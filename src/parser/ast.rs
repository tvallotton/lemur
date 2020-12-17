type Identifier = String;

struct Asignment {
    name: Identifier,
    args: Vec<String>,
    value: Box<Expr>,
}

struct MonadicBiding {
    name: Identifier,
    monad: Box<Expr>,
}

struct TypeLiteral {
    name: Identifier,
    args: Vec<String>,
}

struct Type {
    constraints: Vec<TypeLiteral>,
    signature: Vec<TypeLiteral>,
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
