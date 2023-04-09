use egg::*;
use parse_display::{Display, FromStr};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Display, FromStr)]
#[display("(x:{x},y:{y})")]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

define_language! {
    pub enum DummyLang {
        Num(i32),
        Point2D(Point2D),
        "add" = Add([Id; 2]),
    }
}

// QUESTION 1 - How do I get parsing working?
#[test]
fn testParsePoint2D() {
    let p : Point2D = "(x:24,y:36)".parse().unwrap();
    assert_eq!(p, Point2D{x: 24, y: 36});

    let mut egraph: EGraph<DummyLang, ()> = 
        EGraph::<DummyLang,()>::default().with_explanations_enabled();
    let node = egraph.add(DummyLang::Point2D(p));
    let expr = egraph.id_to_expr(node);
    let exprViaString: RecExpr<DummyLang> = "(x:24,y:36)".parse().unwrap(); // How do I get this working?
    assert_eq!(exprViaString, expr);
}

#[test]
fn testParseInt32() {
    let i : i32 = "24".parse().unwrap();
    assert_eq!(i, 24);

    let mut egraph: EGraph<DummyLang, ()> = 
        EGraph::<DummyLang,()>::default().with_explanations_enabled();
    let node = egraph.add(DummyLang::Num(i));
    let expr = egraph.id_to_expr(node);
    let exprViaString: RecExpr<DummyLang> = "24".parse().unwrap();
    assert_eq!(exprViaString, expr);
}

// QUESTION 2 - How do I walk a RecExpr without ID?
#[test]
fn walkRecExpr() {
    let mut expr: RecExpr<DummyLang> = RecExpr::default();
    let a = expr.add(DummyLang::Num(24));
    let b = expr.add(DummyLang::Num(36));
    let add = expr.add(DummyLang::Add([a, b]));

    // Here I know 'add' is the root id of expr. So I can match on it.
    match expr[add] {
        DummyLang::Add(ids) => {
            assert_eq!(ids[0], a);
            assert_eq!(ids[1], b);
        },
        _ => panic!("This node is a add expression"),
    }

    let exprFromString:RecExpr<DummyLang> = "(add 24 36)".parse().unwrap();
    // Question: What's the root ID of exprFromString?
    // (Let's say I want to walk the ast of the expression, how do I do that?)
}

fn main() {
    println!("Hello, world!");
}
