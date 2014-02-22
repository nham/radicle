use super::{Expr, Env, HashMap, MoveItems, Nil, Atom, List};

type EnvExpr = (Env, Expr);
type EvalResult = Result<EnvExpr, ~str>;

/// The heart and soul of Radicle.
pub fn eval(env: Env, expr: Expr) -> EvalResult {
    debug!(" :: Entered eval, expr = {:?}\n", expr);
    match expr {
        Nil => Ok( (env, Nil) ),
        Atom(ref s) => {
            let res = env.find_copy(s);
            if res.is_none() {
                Err(format!("Symbol `{}` not found.", *s))
            } else {
                Ok( (env, res.unwrap()) )
            }
        },
        List(vec) => {
            if vec.len() == 0 {
                return Err(~"No procedure to call. TODO: a better error message?");
            }

            if is_symbol("quote", &vec[0]) {
                if vec.len() != 2 {
                    Err(~"`quote` expects exactly one argument.")
                } else {
                    Ok( (env, vec[1]) )
                }
            } else if is_symbol("atom", &vec[0]) {
                eval_atom(env, vec)
            } else if is_symbol("eq", &vec[0]) {
                eval_eq(env, vec)
            } else if is_symbol("first", &vec[0]) {
                eval_first(env, vec)
            } else if is_symbol("rest", &vec[0]) {
                eval_rest(env, vec)
            } else if is_symbol("cons", &vec[0]) {
                eval_cons(env, vec)
            } else if is_symbol("cond", &vec[0]) {
                eval_cond(env, vec)
            } else if is_symbol("defun", &vec[0]) {
                eval_defun(env, vec)
            } else {
                eval_func_call(env, vec)
            }
        }
    }
}

fn eval_atom(env: Env, vec: ~[Expr]) -> EvalResult {
    if vec.len() != 2 {
        Err(~"`atom` expects exactly one argument.")
    } else {
        let val = try!( eval(env.clone(), vec[1]) ).val1();
        if val.is_atom() || val.is_empty_list() {
            Ok( (env, Atom(~"t")) )
        } else {
            Ok( (env, List(~[])) )
        }
    }
}


fn eval_eq(env: Env, vec: ~[Expr]) -> EvalResult {

    if vec.len() != 3 {
        Err(~"`eq` expects exactly two arguments.")
    } else {
        let val1 = try!( eval(env.clone(), vec[1].clone()) ).val1();
        let val2 = try!( eval(env.clone(), vec[2]) ).val1();
        if (val1.is_empty_list() && val2.is_empty_list())
           || (val1.is_atom() && val2.is_atom() && val1.eq(&val2)) {
            Ok( (env, Atom(~"t")) )
        } else {
            Ok( (env, List(~[])) )
        }
    }
}


fn eval_first(env: Env, vec: ~[Expr]) -> EvalResult {

    if vec.len() != 2 {
        Err(~"`first` expects exactly one argument.")
    } else {
        let val = try!( eval(env.clone(), vec[1]) ).val1();
        if val.is_list() && !val.is_empty_list() {
            let list = val.unwrap_branch();
            Ok( (env, list[0]) )
        } else {
            debug!("argument is {:?}\n", val);
            Err(~"`first`'s argument must be a non-empty list")
        }
    }
}

fn eval_rest(env: Env, vec: ~[Expr]) -> EvalResult {

    if vec.len() != 2 {
        Err(~"`rest` expects exactly one argument.")
    } else {
        let val = try!( eval(env.clone(), vec[1]) ).val1();
        if val.is_list() && !val.is_empty_list() {
            let mut list = val.unwrap_branch();
            list.shift();
            Ok( (env.clone(), List(list)) )
        } else {
            Err(~"`rest`'s argument must be a non-empty list")
        }
    }
}

fn eval_cons(env: Env, vec: ~[Expr]) -> EvalResult {

    if vec.len() != 3 {
        Err(~"`cons` expects exactly two arguments.")
    } else {
        let val1 = try!( eval(env.clone(), vec[1].clone()) ).val1();
        let val2 = try!( eval(env.clone(), vec[2]) ).val1();

        if val2.is_list() {
            let mut list = val2.unwrap_branch();
            list.unshift(val1);
            Ok( (env, List(list)) )
        } else {
            Err(~"`cons`'s second argument must be a list")
        }
    }
}

fn eval_cond(env: Env, vec: ~[Expr]) -> EvalResult {
    let mut i = 1;
    while i < vec.len() {
        if !vec[i].is_list() {
            return Err(~"Invalid argument to `cond`");
        }

        let arg = vec[i].clone();
        let list = arg.unwrap_branch();

        if list.len() != 2 {
            return Err(~"Invalid argument to `cond`");
        } else {
            let res = eval(env.clone(), list[0].clone());
            let val = try!(res).val1();

            if val.eq( &Atom(~"t") ) {
                return eval(env, list[1]);
            }
        }

        i += 1;
    }

    Ok( (env, Nil) )
}


fn eval_defun(env: Env, vec: ~[Expr]) -> EvalResult {
    if vec.len() != 4 {
        Err(~"`defun` expects exactly three arguments.")
    } else {

        if !vec[1].is_atom() {
            return Err(~"First argument to `defun` must be a symbol");
        }

        {
            let params = vec[2].get_ref_branch();
            for p in params.iter() {
                if !p.is_atom() {
                    return Err(~"Second argument to `defun` must be a list of params");
                } 
            }
        }

        let label_expr = List(~[Atom(~"label"), 
                                vec[1].clone(),
                                List(~[Atom(~"lambda"), 
                                       vec[2].clone(),
                                       vec[3].clone()])]);
        let mut new_env = env.clone();
        new_env.bindings.insert(vec[1].unwrap_leaf(), label_expr);
        Ok( (new_env, Nil) )
    }
}


struct FuncLiteral {
    params: ~[~str],
    body: Expr,
    sym: Option<~str>, // lambdas will have None, labels will have Some
}


fn parse_func_literal(expr: &Expr) -> Option<FuncLiteral> {
    let lambda = parse_lambda_literal(expr);
    if lambda.is_none() {
        parse_label_literal(expr)
    } else {
        lambda
    }
}

fn parse_lambda_literal(expr: &Expr) -> Option<FuncLiteral> {
    if !expr.is_list() {
        return None;
    }

    let vec = expr.get_ref_branch();

    if vec.len() != 3 
       || !vec[1].is_list() 
       || !is_symbol("lambda", &vec[0]) {
        return None;
    }

    let params = vec[1].get_ref_branch();
    let mut plist = ~[];

    for p in params.iter() {
        if !p.is_atom() {
            return None;
        } else {
            plist.push ( p.clone().unwrap_leaf() );
        }
    }

    Some( FuncLiteral{ params: plist, body: vec[2].clone(), sym: None } )
}

fn parse_label_literal(expr: &Expr) -> Option<FuncLiteral> {
    if !expr.is_list() {
        return None;
    }

    let vec = expr.get_ref_branch();

    if vec.len() != 3 
       || !vec[1].is_atom() 
       || !is_symbol("label", &vec[0]) {
        return None;
    }

    let lit = parse_lambda_literal(&vec[2]);

    if lit.is_none() { return None; }
    let mut func = lit.unwrap();
    func.sym = Some( vec[1].clone().unwrap_leaf() );

    Some(func)
}

fn is_symbol(op: &str, expr: &Expr) -> bool {
    if expr.is_atom() {
        let expr_op = expr.get_ref_leaf();
        op.equiv(expr_op)
    } else {
        false
    }
}

fn eval_func_call(env: Env, vec: ~[Expr]) -> EvalResult {
    let num_args = vec.len() - 1;

    let mut vec_iter = vec.move_iter();
    let mut op_expr = vec_iter.next().unwrap();

    // There are two kinds of function calls: lambda calls and label
    // calls. Labels are just lambdas that can cal themselves.
    //
    // We need to distinguish between literal function calls, which
    // take the form:
    //     ((lambda params body) expr1 ... exprn)
    // or 
    //     (label sym ((lambda params body)) expr1 ... exprn)
    //
    // and non-literals, which take the form (expr0 expr1 ... exprn),
    // where expr1 is an expression that evaluates to a function
    // literal. The reason we need to make this distinction is that
    // label and lambda expressions do not evaluate to anything, so 
    // if the operator expression is such a literal we must not eval
    // it. However, if it is not such a literal, we need to eval it
    // to see whether it evaluates to a function literal.

    let mut func_lit = parse_func_literal(&op_expr);
    if func_lit.is_none() {
        op_expr = try!( eval(env.clone(), op_expr) ).val1();

        func_lit = parse_func_literal(&op_expr);
        if func_lit.is_none() {
            return Err(~"Unrecognized expression.");
        }
    }

    let FuncLiteral{params, body, sym} = func_lit.unwrap();
    let mut bindings = HashMap::<~str, Expr>::new();
    if sym.is_some() {
        bindings.insert(sym.unwrap(), op_expr.clone());
    }

    if params.len() != num_args {
        return Err(~"mismatch between number of procedure args and number of args called with.");
    }

    let mut param_iter = params.move_iter();

    for arg in vec_iter {
        let next_param: ~str  = param_iter.next().unwrap();
        debug!("  - eval of {:?} --> {}\n", arg, next_param);
        bindings.insert(next_param, 
                        try!( eval(env.clone(), arg) ).val1());
    }

    let mut new_env = env.clone();
    for (k, v) in bindings.move_iter() {
        new_env.bindings.insert(k, v);
    }

    debug!(" :: arguments have been passed into environment, evaling lambda body\n");
    let val = try!( eval(new_env, body) ).val1();
    Ok( (env, val) )
}
