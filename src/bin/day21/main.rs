use std::error::Error;

use dt::*;
use solver::*;

mod dt;
mod solver;

fn main() -> Result<(), Box<dyn Error>> {
    let mut exprs = parse_lines("day21")?;

    let (_, resolved) = solve_exprs(&exprs);
    println!("part one: {}", resolved[&IDENT_ROOT]);

    // In preparation for part two, re-use the initial part one state, but
    // change the values of the "root" and "humn" nodes accordingly,
    for expr in exprs.iter_mut() {
        if expr.name == IDENT_ROOT {
            if let ResolvableExpr::Expr(ref mut e) = expr.expr {
                e.op = Op::Eq;
            } else {
                panic!("root not an expr as expected");
            }
        } else if expr.name == IDENT_HUMN {
            expr.expr = ResolvableExpr::Unknown;
        }
    }

    let (pending, resolved) = solve_exprs(&exprs);

    println!("part two: {}", solve_unknown(&pending, &resolved));

    Ok(())
}
