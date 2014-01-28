use std::hashmap::HashMap;

use Environment;
use eval;
use Atom = tree::Leaf;
use List = tree::Branch;


#[test]
fn test_eval_symbol() {
    let mut env = Environment{parent: None, bindings: HashMap::new()};

    // symbol not found in env should be eval err
    let foo = Atom(~"foo");
    let bar = Atom(~"bar");

    assert!( eval(foo.clone(), &env).is_err() );

    env.bindings.insert(~"foo", bar.clone());
    let foo_eval = eval(foo.clone(), &env);
    assert!( foo_eval.is_ok() && foo_eval.unwrap().eq(&bar) );

    let env2 = Environment{parent: Some(&env), bindings: HashMap::new()};
    let foo2_eval = eval(foo.clone(), &env2);
    assert!( foo2_eval.is_ok() && foo2_eval.unwrap().eq(&bar) );
}

#[test]
fn test_eval_empty_list() {
    let env = Environment{parent: None, bindings: HashMap::new()};
    assert!( eval(List(~[]), &env).is_err() );
}

#[test]
fn test_eval_quote() {
    let mut env = Environment{parent: None, bindings: HashMap::new()};

    let nil = List(~[]);
    let foo = Atom(~"foo");
    let bar = Atom(~"bar");
    let quote = Atom(~"quote");

    let qnil = List(~[quote.clone(), nil.clone()]);
    let qnil_eval = eval(qnil, &env);
    assert!( qnil_eval.is_ok() && qnil_eval.unwrap().eq(&nil) );

    let qfoo = List(~[quote.clone(), foo.clone()]);
    let qfoo2 = qfoo.clone();
    let qfoo_eval = eval(qfoo, &env);
    assert!( qfoo_eval.is_ok() && qfoo_eval.unwrap().eq(&foo) );

    // "(quote foo)" should evaluate to "foo" regardless of what the symbol foo is
    // bound to in the environment
    env.bindings.insert(~"foo", bar.clone());
    let qfoo2_eval = eval(qfoo2, &env);
    assert!( qfoo2_eval.is_ok() && qfoo2_eval.unwrap().eq(&foo) );

    let list = List(~[foo.clone(), bar.clone(), Atom(~"baz")]);
    let qlist = List(~[quote.clone(), list.clone()]);
    let qlist_eval = eval(qlist, &env);
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().eq(&list) );
}

#[test]
fn test_eval_atom() {
    let env = Environment{parent: None, bindings: HashMap::new()};

    let foo = Atom(~"foo");
    let bar = Atom(~"bar");
    let nil = List(~[]);
    let quote = Atom(~"quote");
    let atom = Atom(~"atom");
    let t = Atom(~"t");

    let qfoo = List(~[quote.clone(), foo.clone()]);
    let qnil = List(~[quote.clone(), nil.clone()]);

    let qfoo_eval = eval(List(~[atom.clone(), qfoo]), &env);
    assert!( qfoo_eval.is_ok() && qfoo_eval.unwrap().eq(&t) );

    let qnil_eval = eval(List(~[atom.clone(), qnil]), &env);
    assert!( qnil_eval.is_ok() && qnil_eval.unwrap().eq(&t) );

    let list = List(~[foo.clone(), bar.clone()]);
    let qlist = List(~[quote.clone(), list.clone()]);
    let qlist_eval = eval(List(~[atom.clone(), qlist]), &env);
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().eq(&nil) );

}
