#[macro_use]
extern crate nom;

use nom::*;

// Non-parsed form integer, to support bigint and stuffs
pub enum Number {
    // -1234.5678 => Dec(true, [1, 2, 3, 4, 5, 6, 7, 8], 4)
    Dec(bool, Vec<u8>, u8),

    // 0354 => Oct([3, 5, 4])
    Oct(Vec<u8>),

    // 0xAB12 => Hex([10, 11, 1, 2])
    Hex(Vec<u8>),
}

pub enum Token {
    TLeftBracket,
    RTightBracket,
    TLeftBrace,
    TRightBrace,
    TComma,
    TColon,
    TMinus,
    TString(String),
    TNumber(Number),
    TIdent(String),
}

// json

named!(ws<Token>, many0!(one_of!("\t\n\r ")));

named!(left_bracket<Token>, map!(terminated!(char!('['), ws), |_| TLeftBracket));
named!(right_bracket<Token>, map!(terminated!(char!(']'), ws), |_| TRightBracket));
named!(left_brace<Token>, map!(terminated!(char!('{'), ws), |_| TLeftBrace));
named!(right_brace<Token>, map!(terminated!(char!('}'), ws), |_| TRightBrace));
named!(comma<Token>, map!(terminated!(char!(','), ws), |_| TComma));
named!(colon<Token>, map!(terminated!(char!(':'), ws), |_| TColon));
named!(minus<Token>, map!(terminated!(char!('-'), ws), |_| TMinus));

// TODO: it is more complex 
// change it later https://datatracker.ietf.org/doc/html/rfc7159#section-7
named!(string<Token>, map!(
    tuple!(char!('\"'), alphanumeric0, char!('\"'), ws),
    |(_, s, _, _)| TString(s.into_iter().collect())))

named!(int<Token>, do_parse!(
    int: alt!(int_hex | int_oct | int_dec) >>
    (TNumber(int)))

named!(int_dec<Number>, do_parse!(
    minus: opt!(char!('-')) >>
    numbers: many1!(number) >>
    frac: opt!(do_parse!(
        char!('.') >>
        decimals: many1!(number)),
        (decimals.chars().map(|c| c.to_digit(10).unwrap()))),
    (match frac {
        Some(decimal) => Number(minus.is_some(), [numbers, decimal].concat(), decimal.len()),
        None => Number(minus.is_some(), numbers, 0)
    }))


named!(lower<Token>, one_of!("abcdefghijklmnopqrstuvwxyz"));
named!(upper<Token>, one_of!("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));

named!(number<Token>, one_of!("0123456789"));
named!(hex_number<Token>, one_of!("0123456789ABCDEFabcdef"));
named!(oct_number<Token>, one_of!("01234567"));

named!(newline<Token>, one_of!("\r\n"));
named!(binop_Token<Token>, one_of!(":~!@#$%^&*-+=<>/?._|"));


