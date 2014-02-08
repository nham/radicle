use eval::eval;
use super::{HashMap, Env, Nil, Atom, List};

#[test]
fn test_eval_symbol() {
    let mut env = Env::new();

    // symbol not found in env should be eval err
    let foo = Atom(~"foo");
    let bar = Atom(~"bar");

    assert!( eval(&env, foo.clone()).is_err() );

    env.bindings.insert(~"foo", bar.clone());
    let foo_eval = eval(&env, foo.clone());
    assert!( foo_eval.is_ok() && foo_eval.unwrap().eq(&bar) );

    let env2 = Env{parent: Some(&env), bindings: HashMap::new()};
    let foo2_eval = eval(&env2, foo.clone());
    assert!( foo2_eval.is_ok() && foo2_eval.unwrap().eq(&bar) );
}

#[test]
fn test_eval_empty_list() {
    let env = Env::new();
    assert!( eval(&env, List(~[])).is_err() );
}

#[test]
fn test_eval_quote() {
    let mut env = Env::new();

    let nil = List(~[]);
    let foo = Atom(~"foo");
    let bar = Atom(~"bar");
    let quote = Atom(~"quote");

    let qnil = List(~[quote.clone(), nil.clone()]);
    let qnil_eval = eval(&env, qnil);
    assert!( qnil_eval.is_ok() && qnil_eval.unwrap().eq(&nil) );

    let qfoo = List(~[quote.clone(), foo.clone()]);
    let qfoo2 = qfoo.clone();
    let qfoo_eval = eval(&env, qfoo);
    assert!( qfoo_eval.is_ok() && qfoo_eval.unwrap().eq(&foo) );

    // "(quote foo)" should evaluate to "foo" regardless of what the symbol foo is
    // bound to in the environment
    env.bindings.insert(~"foo", bar.clone());
    let qfoo2_eval = eval(&env, qfoo2);
    assert!( qfoo2_eval.is_ok() && qfoo2_eval.unwrap().eq(&foo) );

    let list = List(~[foo.clone(), bar.clone(), Atom(~"baz")]);
    let qlist = List(~[quote.clone(), list.clone()]);
    let qlist_eval = eval(&env, qlist);
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().eq(&list) );
}

#[test]
fn test_eval_atom() {
    let env = Env::new();

    let foo = Atom(~"foo");
    let bar = Atom(~"bar");
    let nil = List(~[]);
    let quote = Atom(~"quote");
    let atom = Atom(~"atom");
    let t = Atom(~"t");

    let qfoo = List(~[quote.clone(), foo.clone()]);
    let qnil = List(~[quote.clone(), nil.clone()]);

    let qfoo_eval = eval(&env, List(~[atom.clone(), qfoo]));
    assert!( qfoo_eval.is_ok() && qfoo_eval.unwrap().eq(&t) );

    let qnil_eval = eval(&env, List(~[atom.clone(), qnil]));
    assert!( qnil_eval.is_ok() && qnil_eval.unwrap().eq(&t) );

    let list = List(~[foo.clone(), bar.clone()]);
    let qlist = List(~[quote.clone(), list.clone()]);
    let qlist_eval = eval(&env, List(~[atom.clone(), qlist]));
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().eq(&nil) );

}

#[test]
fn test_eval_eq() {
    let env = Env::new();

    let foo = Atom(~"foo");
    let bar = Atom(~"bar");
    let nil = List(~[]);
    let quote = Atom(~"quote");
    let t = Atom(~"t");
    let eq = Atom(~"eq");

    let qnil = List(~[quote.clone(), nil.clone()]);
    let qfoo = List(~[quote.clone(), foo.clone()]);

    let eq_raw_sym_eval = eval(&env, List(~[eq.clone(), foo.clone(), foo.clone()]));
    assert!( eq_raw_sym_eval.is_err() );

    let eq_diff_eval = eval(&env, List(~[eq.clone(), foo.clone(), bar.clone()]));
    assert!( eq_raw_sym_eval.is_err() );

    let eq_qnil_eval = eval(&env, List(~[eq.clone(), qnil.clone(), qnil.clone()]));
    assert!( eq_qnil_eval.is_ok() && eq_qnil_eval.unwrap().eq(&t) );

    let eq_qfoo_eval = eval(&env, List(~[eq.clone(), qfoo.clone(), qfoo.clone()]));
    assert!( eq_qfoo_eval.is_ok() && eq_qfoo_eval.unwrap().eq(&t) );

}

#[test]
fn test_eval_car() {
    let env = Env::new();

    let foo = Atom(~"foo");
    let bar = Atom(~"bar");
    let nil = List(~[]);
    let quote = Atom(~"quote");
    let car = Atom(~"car");

    let qfoo = List(~[quote.clone(), foo.clone()]);
    let qnil = List(~[quote.clone(), nil.clone()]);

    let qfoo_eval = eval(&env, List(~[car.clone(), qfoo]));
    assert!( qfoo_eval.is_err() );

    let qnil_eval = eval(&env, List(~[car.clone(), qnil]));
    assert!( qnil_eval.is_err() );

    let list = List(~[foo.clone(), bar.clone()]);
    let qlist = List(~[quote.clone(), list.clone()]);
    let qlist_eval = eval(&env, List(~[car.clone(), qlist]));
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().eq(&foo) );

}

#[test]
fn test_eval_cdr() {
    let env = Env::new();

    let foo = Atom(~"foo");
    let bar = Atom(~"bar");
    let nil = List(~[]);
    let quote = Atom(~"quote");
    let cdr = Atom(~"cdr");

    let qfoo = List(~[quote.clone(), foo.clone()]);
    let qnil = List(~[quote.clone(), nil.clone()]);

    let qfoo_eval = eval(&env, List(~[cdr.clone(), qfoo]));
    assert!( qfoo_eval.is_err() );

    let qnil_eval = eval(&env, List(~[cdr.clone(), qnil]));
    assert!( qnil_eval.is_err() );

    let list = List(~[foo.clone(), bar.clone()]);
    let qlist = List(~[quote.clone(), list.clone()]);
    let qlist_eval = eval(&env, List(~[cdr.clone(), qlist]));

    let list_foo = List(~[bar.clone()]);
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().eq(&list_foo) );
}

#[test]
fn test_eval_cons() {
    let env = Env::new();

    let foo = Atom(~"foo");
    let bar = Atom(~"bar");
    let quote = Atom(~"quote");
    let cons = Atom(~"cons");

    let bar_list = List(~[bar.clone()]);
    let qbar_list = List(~[quote.clone(), bar_list.clone()]);

    let bar_eval = eval(&env, List(~[cons.clone(), foo.clone(), bar.clone()]));
    assert!( bar_eval.is_err() );

    let qfoo = List(~[quote.clone(), foo.clone()]);
    let barlist_eval = eval(&env, List(~[cons.clone(), qfoo.clone(), qbar_list.clone()]));
    let foobar_list = List(~[foo.clone(), bar.clone()]);
    assert!( barlist_eval.is_ok() && barlist_eval.unwrap().eq(&foobar_list) );
}

#[test]
fn test_eval_cond() {
    let env = Env::new();

    let foo = Atom(~"foo");
    let bar = Atom(~"bar");
    let baz = Atom(~"baz");
    let quote = Atom(~"quote");
    let cond = Atom(~"cond");
    let qfoo = List(~[quote.clone(), foo.clone()]);
    let qbar = List(~[quote.clone(), bar.clone()]);
    let qbaz = List(~[quote.clone(), baz.clone()]);
    let qt = List(~[quote.clone(), Atom(~"t")]);

    let list = List(~[cond.clone(), 
                      List(~[qfoo.clone(), qbar.clone()]), 
                      List(~[qt.clone(), qbaz.clone()])]);

    let eval_list = eval(&env, list);

    assert!( eval_list.is_ok() && eval_list.unwrap().eq(&baz) );


    let no_t_list = List(~[cond.clone(), 
                      List(~[qfoo.clone(), qbar.clone()])]);
    let eval_no_t_list = eval(&env, no_t_list);
    assert!( eval_no_t_list.is_ok() && eval_no_t_list.unwrap().is_nil() );
}