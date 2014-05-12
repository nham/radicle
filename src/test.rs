#![allow(unused_imports)]
use eval::eval;
use super::{HashMap, Env, Nil, Atom, List, Tree};

fn make_atom(s: &str) -> Tree<~str> {
    Atom(s.to_owned())
}

#[test]
fn test_eval_symbol() {
    let mut env = Env::new();

    // symbol not found in env should be eval err
    let foo = make_atom("foo");
    let bar = make_atom("bar");

    assert!( eval(env.clone(), foo.clone()).is_err() );

    env.bindings.insert("foo".to_owned(), bar.clone());
    let foo_eval = eval(env.clone(), foo.clone());
    assert!( foo_eval.is_ok() && foo_eval.unwrap().val1().eq(&bar) );
}

#[test]
fn test_eval_empty_list() {
    assert!( eval(Env::new(), List(~[])).is_err() );
}

#[test]
fn test_eval_quote() {
    let mut env = Env::new();

    let nil = List(~[]);
    let foo = make_atom("foo");
    let bar = make_atom("bar");
    let quote = make_atom("quote");

    let qnil = List(~[quote.clone(), nil.clone()]);
    let qnil_eval = eval(env.clone(), qnil);
    assert!( qnil_eval.is_ok() && qnil_eval.unwrap().val1().eq(&nil) );

    let qfoo = List(~[quote.clone(), foo.clone()]);
    let qfoo2 = qfoo.clone();
    let qfoo_eval = eval(env.clone(), qfoo);
    assert!( qfoo_eval.is_ok() && qfoo_eval.unwrap().val1().eq(&foo) );

    // "(quote foo)" should evaluate to "foo" regardless of what the symbol foo is
    // bound to in the environment
    env.bindings.insert("foo".to_owned(), bar.clone());
    let qfoo2_eval = eval(env.clone(), qfoo2);
    assert!( qfoo2_eval.is_ok() && qfoo2_eval.unwrap().val1().eq(&foo) );

    let list = List(~[foo.clone(), bar.clone(), make_atom("baz")]);
    let qlist = List(~[quote.clone(), list.clone()]);
    let qlist_eval = eval(env, qlist);
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().val1().eq(&list) );
}

#[test]
fn test_eval_atom() {
    let mut env = Env::new();

    let foo = make_atom("foo");
    let bar = make_atom("bar");
    let nil = List(~[]);
    let quote = make_atom("quote");
    let atom = make_atom("atom");
    let t = make_atom("t");

    let qfoo = List(~[quote.clone(), foo.clone()]);
    let qnil = List(~[quote.clone(), nil.clone()]);

    let qfoo_eval = eval(env.clone(), List(~[atom.clone(), qfoo]));
    assert!( qfoo_eval.is_ok() && qfoo_eval.unwrap().val1().eq(&t) );

    let qnil_eval = eval(env.clone(), List(~[atom.clone(), qnil]));
    assert!( qnil_eval.is_ok() && qnil_eval.unwrap().val1().eq(&t) );

    let list = List(~[foo.clone(), bar.clone()]);
    let qlist = List(~[quote.clone(), list.clone()]);
    let qlist_eval = eval(env, List(~[atom.clone(), qlist]));
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().val1().eq(&nil) );

}

#[test]
fn test_eval_eq() {
    let mut env = Env::new();

    let foo = make_atom("foo");
    let bar = make_atom("bar");
    let nil = List(~[]);
    let quote = make_atom("quote");
    let t = make_atom("t");
    let eq = make_atom("eq");

    let qnil = List(~[quote.clone(), nil.clone()]);
    let qfoo = List(~[quote.clone(), foo.clone()]);

    let eq_raw_sym_eval = eval(env.clone(), List(~[eq.clone(), foo.clone(), foo.clone()]));
    assert!( eq_raw_sym_eval.is_err() );

    let eq_diff_eval = eval(env.clone(), List(~[eq.clone(), foo.clone(), bar.clone()]));
    assert!( eq_raw_sym_eval.is_err() );

    let eq_qnil_eval = eval(env.clone(), List(~[eq.clone(), qnil.clone(), qnil.clone()]));
    assert!( eq_qnil_eval.is_ok() && eq_qnil_eval.unwrap().val1().eq(&t) );

    let eq_qfoo_eval = eval(env, List(~[eq.clone(), qfoo.clone(), qfoo.clone()]));
    assert!( eq_qfoo_eval.is_ok() && eq_qfoo_eval.unwrap().val1().eq(&t) );

}

#[test]
fn test_eval_first() {
    let mut env = Env::new();

    let foo = make_atom("foo");
    let bar = make_atom("bar");
    let nil = List(~[]);
    let quote = make_atom("quote");
    let first = make_atom("first");

    let qfoo = List(~[quote.clone(), foo.clone()]);
    let qnil = List(~[quote.clone(), nil.clone()]);

    let qfoo_eval = eval(env.clone(), List(~[first.clone(), qfoo]));
    assert!( qfoo_eval.is_err() );

    let qnil_eval = eval(env.clone(), List(~[first.clone(), qnil]));
    assert!( qnil_eval.is_err() );

    let list = List(~[foo.clone(), bar.clone()]);
    let qlist = List(~[quote.clone(), list.clone()]);
    let qlist_eval = eval(env, List(~[first.clone(), qlist]));
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().val1().eq(&foo) );

}

#[test]
fn test_eval_rest() {
    let mut env = Env::new();

    let foo = make_atom("foo");
    let bar = make_atom("bar");
    let nil = List(~[]);
    let quote = make_atom("quote");
    let rest = make_atom("rest");

    let qfoo = List(~[quote.clone(), foo.clone()]);
    let qnil = List(~[quote.clone(), nil.clone()]);

    let qfoo_eval = eval(env.clone(), List(~[rest.clone(), qfoo]));
    assert!( qfoo_eval.is_err() );

    let qnil_eval = eval(env.clone(), List(~[rest.clone(), qnil]));
    assert!( qnil_eval.is_err() );

    let list = List(~[foo.clone(), bar.clone()]);
    let qlist = List(~[quote.clone(), list.clone()]);
    let qlist_eval = eval(env, List(~[rest.clone(), qlist]));

    let list_foo = List(~[bar.clone()]);
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().val1().eq(&list_foo) );
}

#[test]
fn test_eval_cons() {
    let mut env = Env::new();

    let foo = make_atom("foo");
    let bar = make_atom("bar");
    let quote = make_atom("quote");
    let cons = make_atom("cons");

    let bar_list = List(~[bar.clone()]);
    let qbar_list = List(~[quote.clone(), bar_list.clone()]);

    let bar_eval = eval(env.clone(), List(~[cons.clone(), foo.clone(), bar.clone()]));
    assert!( bar_eval.is_err() );

    let qfoo = List(~[quote.clone(), foo.clone()]);
    let barlist_eval = eval(env, List(~[cons.clone(), qfoo.clone(), qbar_list.clone()]));
    let foobar_list = List(~[foo.clone(), bar.clone()]);
    assert!( barlist_eval.is_ok() && barlist_eval.unwrap().val1().eq(&foobar_list) );
}

#[test]
fn test_eval_cond() {
    let mut env = Env::new();

    let foo = make_atom("foo");
    let bar = make_atom("bar");
    let baz = make_atom("baz");
    let quote = make_atom("quote");
    let cond = make_atom("cond");
    let qfoo = List(~[quote.clone(), foo.clone()]);
    let qbar = List(~[quote.clone(), bar.clone()]);
    let qbaz = List(~[quote.clone(), baz.clone()]);
    let qt = List(~[quote.clone(), make_atom("t")]);

    let list = List(~[cond.clone(), 
                      List(~[qfoo.clone(), qbar.clone()]), 
                      List(~[qt.clone(), qbaz.clone()])]);

    let eval_list = eval(env.clone(), list);

    assert!( eval_list.is_ok() && eval_list.unwrap().val1().eq(&baz) );


    let no_t_list = List(~[cond.clone(), 
                      List(~[qfoo.clone(), qbar.clone()])]);
    let eval_no_t_list = eval(env, no_t_list);
    assert!( eval_no_t_list.is_ok() && eval_no_t_list.unwrap().val1().is_nil() );
}
