#[macro_use]
extern crate nom;

use nom::types::CompleteStr;
use nom::{alpha, digit, space};
use std::collections::HashMap;
use std::str::FromStr;
// use std::cell::RefCell;

struct Parser {
    symbol_table: HashMap<String, i64>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            symbol_table: HashMap::<String, i64>::new(),
        }
    }
    
    method!(
        parens<Parser, CompleteStr, i64>,
        mut self,
        ws!(delimited!(tag!("("), call_m!(self.expr), tag!(")")))
    );

    // Figure out where this goes
    method!(
        assignment<Parser, CompleteStr, i64>,
        mut self,
        do_parse!(
            id: alpha
                >> opt!(space)
                >> tag!("=")
                >> opt!(space)
                >> expr: call_m!(self.expr)
                >> ({
                    println!("trying to assign");
                    let ident_entry = self.symbol_table.entry(id.to_string()).or_insert(0);
                    *ident_entry = expr;
                    expr
                })
        )
    );

    method!(
        identifier<Parser, CompleteStr, i64>,
        mut self,
        do_parse!(
            tag!("$")
                >> init: alpha
                >> ({
                    println!("Trying to find ident");
                    println!("{:?}", self.symbol_table);
                    let ident_val = self.symbol_table.get(&init.to_string());
                    match ident_val {
                        Some(a) => *a,
                        None => 0,
                    }
                })
        )
    );

    method!(
        factor<Parser, CompleteStr, i64>,
        mut self,
        alt!(
            call_m!(self.assignment)
                | map_res!(ws!(digit), |s: CompleteStr| {
                    println!("{:?}", s);
                    FromStr::from_str(s.0)
                })
                | call_m!(self.parens)
        )
    );

    method!(
        term<Parser, CompleteStr, i64>,
        mut self,
        do_parse!(
            init: call_m!(self.factor)
                >> res: fold_many0!(
                    pair!(alt!(char!('*') | char!('/')), call_m!(self.factor)),
                    init,
                    |acc, (op, val): (char, i64)| if op == '*' { acc * val } else { acc / val }
                )
                >> (res)
        )
    );

    method!(
        expr<Parser, CompleteStr, i64>,
        mut self,
        do_parse!(
            init: call_m!(self.term)
                >> res: fold_many0!(
                    pair!(alt!(char!('+') | char!('-')), call_m!(self.term)),
                    init,
                    |acc, (op, val): (char, i64)| if op == '+' { acc + val } else { acc - val }
                )
                >> (res)
        )
    );
}

// God this makes shit easy
