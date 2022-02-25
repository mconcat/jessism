use nom::*;

struct Pure;
struct Impure;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Expr<P> {
    DataLiteral(DataLiteral),
    Array(Array<P>),
    Record(Record<P>),
}

impl Expr<P> {
    fn parse(tokens: Vec<Token>) -> Self {
        // try either dataliteral, array<P>, record<P>, or hole
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum DataLiteral {
    NullLiteral,
    FalseLiteral,
    TrueLiteral,
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
}

impl DataLiteral {
    fn parse(tokens: Vec<Token>) -> Self {
        // try either literals, and parse number and string accordingly, with whitespace 
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Array<P> {
    Array(Vec<Expr<P>>),
}

impl Array {
    fn parse(tokens: Vec<Token>) -> Self {
        // try [ Expr<P> , Expr<P> ,? ]
    }
}

pub enum Record<P> {
    Record(Vec<(PropName, Expr<P>)>),
}

impl Record<P> {
    fn parse(tokens: Vec<Token>) -> Self {
        // try { PropName : Expr<P> , PropName : Expr ,? }
    }
}

pub enum PropName {
    PropName(String),
}

impl PropName {
    fn parse(input: Input) -> Result<(Self, Input), ParseError> {
        // just a string for now
        
    }
}
