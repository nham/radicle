use std::hashmap::HashMap;

use Environment;
use eval;
use tree::Tree;
use Atom = tree::Leaf;
use List = tree::Branch;

#[test]
fn test_eval_symbol() {
    let mut env = Environment{parent: None, bindings: HashMap::new()};

    // symbol not found in env should be eval err
    assert!( eval(Atom(~"foo"), &env).is_err() );

    let bar = Atom(~"bar");
    let bar2 = bar.clone();
    env.bindings.insert(~"foo", bar);
    let foo_eval = eval(Atom(~"foo"), &env);
    assert!( foo_eval.is_ok() && foo_eval.unwrap().eq(&bar2) );

    let env2 = Environment{parent: Some(&env), bindings: HashMap::new()};
    let foo2_eval = eval(Atom(~"foo"), &env2);
    assert!( foo2_eval.is_ok() && foo2_eval.unwrap().eq(&bar2) );
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
    let qnil = List(~[Atom(~"quote"), nil.clone()]);
    let qnil_eval = eval(qnil, &env);
    assert!( qnil_eval.is_ok() && qnil_eval.unwrap().eq(&nil) );

    let foo = Atom(~"foo");
    let qfoo = List(~[Atom(~"quote"), foo.clone()]);
    let qfoo2 = qfoo.clone();
    let qfoo_eval = eval(qfoo, &env);
    assert!( qfoo_eval.is_ok() && qfoo_eval.unwrap().eq(&foo) );

    // "(quote foo)" should evaluate to "foo" regardless of what the symbol foo is
    // bound to in the environment
    env.bindings.insert(~"foo", Atom(~"bar"));
    let qfoo2_eval = eval(qfoo2, &env);
    assert!( qfoo2_eval.is_ok() && qfoo2_eval.unwrap().eq(&foo) );

    let list = List(~[Atom(~"foo"), Atom(~"bar"), Atom(~"baz")]);
    let qlist = List(~[Atom(~"quote"), list.clone()]);
    let qlist_eval = eval(qlist, &env);
    assert!( qlist_eval.is_ok() && qlist_eval.unwrap().eq(&list) );
}
