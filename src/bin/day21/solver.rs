use std::collections::{HashMap, HashSet};

use crate::dt::*;

/// Given a collection of Exprs, evaluates those exprs until it can make no
/// further progress. Returns a HashSet of unevaluated NamedExprs, and a map
/// of fully-evaluated Exprs to their final value.
/// This function solves part one, and is used to pre-process part two.
pub fn solve_exprs(
    exprs: &[NamedResolvableExpr],
) -> (HashSet<NamedExpr>, HashMap<Ident, i64>) {
    // A HashMap of all exprs we've not been able to resolve yet.
    let mut exprs_pending: HashSet<NamedExpr> = HashSet::new();
    // A HashMap of exprs whose value is now known.
    let mut exprs_resolved: HashMap<Ident, i64> = HashMap::new();

    // For each NamedExpr, put resolved values into exprs_resolved, and
    // unresolved values innto exprs_pending.
    for named_expr in exprs {
        match named_expr.expr {
            ResolvableExpr::Expr(expr) => {
                assert!(exprs_pending.insert(NamedExpr {
                    name: named_expr.name,
                    expr
                }))
            },
            ResolvableExpr::Val(v) => {
                assert!(exprs_resolved.insert(named_expr.name, v).is_none())
            },
            ResolvableExpr::Unknown => (),
        }
    }

    // Spin until exprs_resolved contains root (or we stop resolving exprs).
    loop {
        let prev_len = exprs_pending.len();

        let mut newly_resolved_exprs: Vec<NamedExpr> = Vec::new();
        for &expr in exprs_pending.iter() {
            if let Some(&lhs) = exprs_resolved.get(&expr.expr.lhs) {
                if let Some(&rhs) = exprs_resolved.get(&expr.expr.rhs) {
                    let value = expr.expr.op.apply(lhs, rhs);
                    exprs_resolved.insert(expr.name, value);
                    newly_resolved_exprs.push(expr);
                }
            }
        }

        newly_resolved_exprs
            .into_iter()
            .for_each(|expr| assert!(exprs_pending.remove(&expr)));

        if exprs_pending.len() == prev_len {
            // Unable to evaluate anything else.
            break;
        }
    }

    (exprs_pending, exprs_resolved)
}

/// Given an output from solve_exprs, attempt to determine the unknown value.
/// Returns the final values resolved from the attempt.
/// This function solves part two.
///
/// * `pending` is a set of named monkeys with expressions that can't be fully
///   resolved yet.
/// * `resolved` is a map of names to their values that have been fully
///   determined.
pub fn solve_unknown(
    pending: &HashSet<NamedExpr>,
    resolved: &HashMap<Ident, i64>,
) -> i64 {
    // Mash both the pending and resolved values into a single map of indent
    // to ResolvableExpr.
    let mut state: HashMap<Ident, ResolvableExpr> = HashMap::new();

    for ne in pending {
        assert!(state
            .insert(ne.name, ResolvableExpr::Expr(ne.expr))
            .is_none(),);
    }

    for (&ident, &v) in resolved {
        assert!(state.insert(ident, ResolvableExpr::Val(v)).is_none());
    }

    assert!(state.insert(IDENT_HUMN, ResolvableExpr::Unknown).is_none());

    // println!("State is: {state:#?}");

    // Now work downwards from the root. At this point, all the child nodes of
    // the root should have a single unknown value. Find this value at each
    // stage to determine the value the next expr must resolve to, and repeat.

    let root = match state.get(&IDENT_ROOT) {
        Some(ResolvableExpr::Expr(e)) => e,
        _ => panic!(),
    };

    let mut target_val;
    let mut cur_ident;

    if let ResolvableExpr::Val(lhs) = state[&root.lhs] {
        target_val = lhs;
        cur_ident = root.rhs;
    } else if let ResolvableExpr::Val(rhs) = state[&root.rhs] {
        target_val = rhs;
        cur_ident = root.lhs;
    } else {
        panic!("could not identify a resolved value in the root");
    }

    // println!("want {cur_ident} = {target_val}");

    while cur_ident != IDENT_HUMN {
        let this_node = match state.get(&cur_ident) {
            Some(ResolvableExpr::Expr(e)) => e,
            _ => panic!(),
        };

        let lhs = state[&this_node.lhs];
        let rhs = state[&this_node.rhs];

        if let ResolvableExpr::Val(lhs) = lhs {
            assert!(!matches!(rhs, ResolvableExpr::Val(_)));
            target_val = this_node.op.find_rhs(lhs, target_val);
            cur_ident = this_node.rhs;
        } else if let ResolvableExpr::Val(rhs) = rhs {
            assert!(!matches!(lhs, ResolvableExpr::Val(_)));
            target_val = this_node.op.find_lhs(rhs, target_val);
            cur_ident = this_node.lhs;
        } else {
            panic!("no candidate val for ident {cur_ident}");
        }
    }

    target_val
}
