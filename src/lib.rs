/// Gregory Ganley

#[macro_use]
extern crate nom;

use nom::types::CompleteStr;
use nom::{digit};
use std::str::FromStr;

named!(
    pub factor<CompleteStr, i64>,
    alt!(map_res!(ws!(digit), |s: CompleteStr| {
        FromStr::from_str(s.0)
    })
         | parens
    )
);

named!(
    pub term<CompleteStr, i64>,
    do_parse!(
        init: factor
            >> res: fold_many0!(
                pair!(alt!(char!('*') | char!('/')), factor),
                init,
                |acc, (op, val): (char, i64)| {
                    println!("Factor");
                    if op == '*' { acc * val } else { acc / val }
                }
            )
            >> (res)
    )
);

named!(
    pub expr<CompleteStr, i64>,
    do_parse!(
        init: term
            >> res: fold_many0!(
                pair!(alt!(char!('+') | char!('-')), term),
                init,
                |acc, (op, val): (char, i64)| {
                    println!("Expr");
                    if op == '+' { acc + val } else { acc - val }
                }
            )
            >> (res)
    )
);

named!(
    pub binopexpr<CompleteStr, i64>,
    do_parse!(
        init: expr
            >> res: fold_many0!(
                pair!(alt!(tag!("<=") | tag!("<")), expr),
                init,
                |acc, (op, val): (CompleteStr, i64)| {
                    println!("Binopexpr");
                    if op == CompleteStr("<=") { if acc <= val {1} else {0} } else { if acc < val {1} else {0}}
                }
            )
            >> (res)
    )
);

named!(
    pub parens<CompleteStr, i64>,
    ws!(delimited!(tag!("("), binopexpr, tag!(")")))
);
// struct Parser {
//     symbol_table: HashMap<String, i64>,
// }

// impl Parser {
//     pub fn new() -> Parser {
//         Parser {
//             symbol_table: HashMap::<String, i64>::new(),
//         }
//     }



//     // // Figure out where this goes
//     // method!(
//     //     assignment<Parser, CompleteStr, i64>,
//     //     mut self,
//     //     do_parse!(
//     //         id: alpha
//     //             >> opt!(space)
//     //             >> tag!("=")
//     //             >> opt!(space)
//     //             >> expr: call_m!(self.expr)
//     //             >> ({
//     //                 println!("trying to assign");
//     //                 let ident_entry = self.symbol_table.entry(id.to_string()).or_insert(0);
//     //                 *ident_entry = expr;
//     //                 expr
//     //             })
//     //     )
//     // );

//     // method!(
//     //     identifier<Parser, CompleteStr, i64>,
//     //     mut self,
//     //     do_parse!(
//     //         tag!("$")
//     //             >> init: alpha
//     //             >> ({
//     //                 println!("Trying to find ident");
//     //                 println!("{:?}", self.symbol_table);
//     //                 let ident_val = self.symbol_table.get(&init.to_string());
//     //                 match ident_val {
//     //                     Some(a) => *a,
//     //                     None => 0,
//     //                 }
//     //             })
//     //     )
//     // );


// }

// God this makes shit easy
