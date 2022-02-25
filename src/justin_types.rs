mod json_types


// A.2 Expressions

pub enum UseVar {
    Ident(String),
}

pub enum ExprImpure {
    JsonExpr(json_types::Expr<json_types::Impure>),
    QuasiExpr(QuasiExpr),
    AssignExpr(AssignExpr),
    Var(UseVar),
}

pub enum ExprPure {
    JsonExpr(json_types::Expr<json_types::Pure>),
    Var(UseVar),
}

// TODO: quasiquotes
pub enum QuasiExpr {
    
}

pub enum CallPostOp {
    Index(IndexExpr),
    Get(IdentName),
    Quasi(QuasiExpr),
    Args(Vec<(bool, AssignExpr)>),
}

pub enum CallExpr {
    CallExpr(ExprImpure, Vec<CallPostOp>),
}

pub enum IndexExpr {
    Number(Number),
    Positive(UnaryExpr),
}

// Arithmetics

pub enum UnaryOp {
    VoidOp, // Void
    TypeOfOp, // typeof
    PositiveOp, // +
    NegativeOp, // -
    TildeOp, // ~
    NotOp, // !
}

pub enum UnaryExpr {
    UnaryExpr(Vec<UnaryOp>, CallExpr),
}

impl UnaryExpr {
    fn parse(tokens: Vec<Token>) -> Self {

    }
}

pub enum PowExpr {
    PowExpr(Vec<UpdateExpr>, UnaryExpr),
}

impl PowExpr {
    fn parse(tokens: Vec<Token>) -> Self {

    }
}

pub enum MulLevelOp {
    MulOp, // *
    DivOp, // /
    ModOp, // %
}

impl BinaryOp for MulLevelOp {
    type Child = PowExpr
}

pub enum AddLevelOp {
    AddOp, // +
    SubOp, // -
}

impl BinaryOp for AddLevelOp {
    type ChildExpr = BinaryExpr<MulLevelOp>
}

pub enum ShiftLevelOp {
    LeftShiftOp, // <<
    RightShiftOp, // >>
    UnsignedRightShiftOp, // >>>
}

impl BinaryOp for ShiftLevelOp {
    type ChildExpr = BinaryExpr<AddLevelOp>
}

pub enum RelLevelOp {
    LessThanEqualOp, // <=
    LessThanOp, // <
    GreaterThanOp, // >
    GreaterThanEqualOp, // >=
}

pub enum EqLevelOp {
    EqualOp, // ===
    NotEqualOp, // !==
}

pub enum BitLevelOp {
    BitAndOp, // &
    BitXorOp, // ^
    BitOrOp, // |
}

pub enum EagerLevelOp {
    RelLevel(RelLevelOp),
    EqLevel(EqLevelOp),
    BitLevel(BitLevelOp),
}

impl BinaryOp for EagerLevelOp {
    type ChildExpr = BinaryExpr<ShiftLevelOp>
}

pub enum AndThenLevelOp {
    AndThenOp, // &&
}

impl BinaryOp for AndThenLevelOp {
    type ChildExpr = BinaryExpr<EagerLevelOp>
}

pub enum OrElseLevelOp {
    OrElseOp, // ||
}

impl BinaryOp for OrElseLevelOp {
    type ChildExpr = BinaryExpr<AndThenLevelOp>
}

trait BinaryOp: Token {
    type ChildExpr;
}

// OrElseOp > AndThenOp > EagerOp > ShiftOp > AddOp > MulOp > PowOp (> Unary)
pub struct BinaryExpr<Op: BinaryOp> {
    First: Op::ChildExpr,
    Rest: Vec<(Op, Op::ChildExpr)>,
}

impl BinaryExpr<Op> {
    fn parse(tokens: Vec<Tokens>) -> Self where <Op as BinaryOp>::ChildExpr: Parsable {
        // parse 
    }
}

pub enum CondExpr {
    Ternary(BinaryExpr<OrElseLevelOp>, CondExpr, CondExpr),
    OrElse(BinaryExpr<OrElseLevelOp>),
}

pub enum AssignExpr {
    CondExpr(CondExpr),
}

