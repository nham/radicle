#![allow(unused_imports)]
use eval::eval;
use super::{HashMap, Env, Nil, Atom, List, Tree};

fn make_atom(s: &str) -> Tree<String> {
    Atom(s.to_string())
}

fn make_nil() -> Tree<String> {
    List(vec!())
}

fn make_2list(i1: Tree<String>, i2: Tree<String>) -> Tree<String> {
    List(vec!(i1, i2))
}

fn quote_expr(e: Tree<String>) -> Tree<String> {
    make_2list(make_atom("quote"), e)
}

#[test]
fn test_eval_symbol() {
    let mut env = Env::new();

    // symbol not found in env should be eval err
    let foo = make_atom("foo");
    let bar = make_atom("bar");

    assert!( eval(&mut env, foo.clone()).is_err() );

    env.bindings.insert("foo".to_string(), bar.clone());
    let foo_eval = eval(&mut env, foo.clone());
    assert!( foo_eval.is_ok() && foo_eval.unwrap().eq(&bar) );
}

#[test]
fn test_eval_empty_list() {
    assert!( eval(&mut Env::new(), List(vec!())).is_err() );
}

#[test]
fn test_eval_quote() {
    let mut env = Env::new();

    let nil = make_nil();
    let foo = make_atom("foo");
    let bar = make_atom("bar");

    let qnil = quote_expr(nil.clone());
    let qnil_eval = eval(&mut env, qnil);
    assert!( qnil_eval.is_ok() && qnil_eval.unwrap().eq(&nil) );

    let qfoo = quote_expr(foo.clone());
    let qfoo2 = qfoo.clone();
    let qfoo_eval = eval(&mut env, qfoo);
    assert!( qfoo_eval.is_ok() && qfoo_eval.unwrap().eq(&foo) );

    // "(quote foo)" should evaluate to "foo" regardless of what the symbol foo is
    // bound to in the environment
    env.bindings.insert("foo".to_string(), bar.clone());
    let qfoo2_eval = eval(&mut env, qfoo2);
    assert!( qfoo2_eval.is_ok() && qfoo2_eval.unwrap().eq(&foo) );

    let list = List(vec!(foo.clone(), bar.clone(), make_atom("baz")));
    let qlist = quote_expr(list.clone());
    let qlist_eval = eval(&mut env, qlist);
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().eq(&list) );
}

#[test]
fn test_eval_atom() {
    let mut env = Env::new();

    let foo = make_atom("foo");
    let bar = make_atom("bar");
    let nil = make_nil();
    let atom = make_atom("atom");
    let t = make_atom("t");

    let qfoo = quote_expr(foo.clone());
    let qnil = quote_expr(nil.clone());

    let qfoo_eval = eval(&mut env, make_2list(atom.clone(), qfoo));
    assert!( qfoo_eval.is_ok() && qfoo_eval.unwrap().eq(&t) );

    let qnil_eval = eval(&mut env, make_2list(atom.clone(), qnil));
    assert!( qnil_eval.is_ok() && qnil_eval.unwrap().eq(&t) );

    let list = make_2list(foo.clone(), bar.clone());
    let qlist = quote_expr(list.clone());
    let qlist_eval = eval(&mut env, make_2list(atom.clone(), qlist));
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().eq(&nil) );

}

#[test]
fn test_eval_eq() {
    let mut env = Env::new();

    let foo = make_atom("foo");
    let bar = make_atom("bar");
    let nil = make_nil();
    let t = make_atom("t");
    let eq = make_atom("eq");

    let qnil = quote_expr(nil.clone());
    let qfoo = quote_expr(foo.clone());

    let eq_raw_sym_eval = eval(&mut env, List(vec!(eq.clone(), foo.clone(), foo.clone())));
    assert!( eq_raw_sym_eval.is_err() );

    let eq_diff_eval = eval(&mut env, List(vec!(eq.clone(), foo.clone(), bar.clone())));
    assert!( eq_raw_sym_eval.is_err() );

    let eq_qnil_eval = eval(&mut env, List(vec!(eq.clone(), qnil.clone(), qnil.clone())));
    assert!( eq_qnil_eval.is_ok() && eq_qnil_eval.unwrap().eq(&t) );

    let eq_qfoo_eval = eval(&mut env, List(vec!(eq.clone(), qfoo.clone(), qfoo.clone())));
    assert!( eq_qfoo_eval.is_ok() && eq_qfoo_eval.unwrap().eq(&t) );

}

#[test]
fn test_eval_first() {
    let mut env = Env::new();

    let foo = make_atom("foo");
    let bar = make_atom("bar");
    let nil = make_nil();
    let first = make_atom("first");

    let qfoo = quote_expr(foo.clone());
    let qnil = quote_expr(nil.clone());

    let qfoo_eval = eval(&mut env, make_2list(first.clone(), qfoo));
    assert!( qfoo_eval.is_err() );

    let qnil_eval = eval(&mut env, make_2list(first.clone(), qnil));
    assert!( qnil_eval.is_err() );

    let list = make_2list(foo.clone(), bar.clone());
    let qlist = quote_expr(list.clone());
    let qlist_eval = eval(&mut env, make_2list(first.clone(), qlist));
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().eq(&foo) );

}

#[test]
fn test_eval_rest() {
    let mut env = Env::new();

    let foo = make_atom("foo");
    let bar = make_atom("bar");
    let nil = make_nil();
    let rest = make_atom("rest");

    let qfoo = quote_expr(foo.clone());
    let qnil = quote_expr(nil.clone());

    let qfoo_eval = eval(&mut env, make_2list(rest.clone(), qfoo));
    assert!( qfoo_eval.is_err() );

    let qnil_eval = eval(&mut env, make_2list(rest.clone(), qnil));
    assert!( qnil_eval.is_err() );

    let list = make_2list(foo.clone(), bar.clone());
    let qlist = quote_expr(list.clone());
    let qlist_eval = eval(&mut env, make_2list(rest.clone(), qlist));

    let list_foo = List(vec!(bar.clone()));
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().eq(&list_foo) );
}

#[test]
fn test_eval_cons() {
    let mut env = Env::new();

    let foo = make_atom("foo");
    let bar = make_atom("bar");
    let cons = make_atom("cons");

    let bar_list = List(vec!(bar.clone()));
    let qbar_list = quote_expr(bar_list.clone());

    let bar_eval = eval(&mut env, List(vec!(cons.clone(), foo.clone(), bar.clone())));
    assert!( bar_eval.is_err() );

    let qfoo = quote_expr(foo.clone());
    let barlist_eval = eval(&mut env, List(vec!(cons.clone(), qfoo.clone(), qbar_list.clone())));
    let foobar_list = make_2list(foo.clone(), bar.clone());
    assert!( barlist_eval.is_ok() && barlist_eval.unwrap().eq(&foobar_list) );
}

#[test]
fn test_eval_cond() {
    let mut env = Env::new();

    let foo = make_atom("foo");
    let bar = make_atom("bar");
    let baz = make_atom("baz");
    let cond = make_atom("cond");

    let qfoo = quote_expr(foo.clone());
    let qbar = quote_expr(bar.clone());
    let qbaz = quote_expr(baz.clone());
    let qt = quote_expr(make_atom("t"));

    let list = List(vec!(cond.clone(), 
                      make_2list(qfoo.clone(), qbar.clone()), 
                      make_2list(qt.clone(), qbaz.clone())));

    let eval_list = eval(&mut env, list);

    assert!( eval_list.is_ok() && eval_list.unwrap().eq(&baz) );


    let no_t_list = make_2list(cond.clone(), 
                               make_2list(qfoo.clone(), qbar.clone()));
    let eval_no_t_list = eval(&mut env, no_t_list);
    assert!( eval_no_t_list.is_ok() && eval_no_t_list.unwrap().is_nil() );
}
