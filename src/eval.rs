use super::{Expr, Env, HashMap, MoveItems, Nil, Atom, List};

type EnvExpr = (Env, Expr);
type EvalResult = Result<EnvExpr, ~str>;

/// The heart and soul of Radicle.
pub fn eval(mut env: Env, expr: Expr) -> EvalResult {
    debug!("\n :: Entered eval, expr = \n{}\n", expr);
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
        List([]) => Err(~"No procedure to call. TODO: a better error message?"),
        List(vec) => {
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
            } else if is_symbol("car", &vec[0]) {
                eval_car(env, vec)
            } else if is_symbol("cdr", &vec[0]) {
                eval_cdr(env, vec)
            } else if is_symbol("cons", &vec[0]) {
                eval_cons(env, vec)
            } else if is_symbol("cond", &vec[0]) {
                eval_cond(env, vec)
            } else {

                let num_args = vec.len() - 1;
                let res = prepare_lambda(env.clone(), vec);

                if res.is_err() {
                    return Err( res.unwrap_err() );
                }

                let (mut lambda, mut bindings, args) = res.unwrap();


                let mut lambda_iter = lambda.move_iter();
                lambda_iter.next(); // discard "lambda" symbol, not needed
                let params: ~[Expr] = lambda_iter.next().unwrap().unwrap_branch();
                let lambda_body: Expr = lambda_iter.next().unwrap();

                if params.len() != num_args {
                    return Err(~"mismatch between number of procedure args and number of args called with.");
                }

                let new_binds = populate_bindings(args, params, env.clone(), bindings);
                if new_binds.is_err() {
                    return Err( new_binds.unwrap_err() );
                }

                for (k, v) in new_binds.unwrap().move_iter() {
                    env.bindings.insert(k, v);
                }

                debug!("\n :: arguments have been passed into environment, evaling lambda body\n");
                eval(env, lambda_body)

            }
        }
    }
}

fn eval_atom(env: Env, vec: ~[Expr]) -> EvalResult {
    if vec.len() != 2 {
        Err(~"`atom` expects exactly one argument.")
    } else {
        let val = if_ok!( eval(env.clone(), vec[1]) ).n1();
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
        let val1 = if_ok!( eval(env.clone(), vec[1].clone()) ).n1();
        let val2 = if_ok!( eval(env.clone(), vec[2]) ).n1();
        if (val1.is_empty_list() && val2.is_empty_list())
           || (val1.is_atom() && val2.is_atom() && val1.eq(&val2)) {
            Ok( (env, Atom(~"t")) )
        } else {
            Ok( (env, List(~[])) )
        }
    }
}


fn eval_car(env: Env, vec: ~[Expr]) -> EvalResult {

    if vec.len() != 2 {
        Err(~"`car` expects exactly one argument.")
    } else {
        let val = if_ok!( eval(env.clone(), vec[1]) ).n1();
        if val.is_list() && !val.is_empty_list() {
            let list = val.unwrap_branch();
            Ok( (env, list[0]) )
        } else {
            Err(~"`car`'s argument must be a non-empty list")
        }
    }
}

fn eval_cdr(env: Env, vec: ~[Expr]) -> EvalResult {

    if vec.len() != 2 {
        Err(~"`cdr` expects exactly one argument.")
    } else {
        let val = if_ok!( eval(env.clone(), vec[1]) ).n1();
        if val.is_list() && !val.is_empty_list() {
            let mut list = val.unwrap_branch();
            list.shift();
            Ok( (env.clone(), List(list)) )
        } else {
            Err(~"`cdr`'s argument must be a non-empty list")
        }
    }
}

fn eval_cons(env: Env, vec: ~[Expr]) -> EvalResult {

    if vec.len() != 3 {
        Err(~"`cons` expects exactly two arguments.")
    } else {
        let val1 = if_ok!( eval(env.clone(), vec[1].clone()) ).n1();
        let val2 = if_ok!( eval(env.clone(), vec[2]) ).n1();

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
            let val = if_ok!(res).n1();

            if val.eq( &Atom(~"t") ) {
                return eval(env, list[1]);
            }
        }

        i += 1;
    }

    Ok( (env, Nil) )
}


fn is_func_literal(expr: &Expr) -> bool {
    is_lambda_literal(expr) || is_label_literal(expr)
}

fn is_lambda_literal(expr: &Expr) -> bool {
    if !expr.is_list() {
        return false;
    }

    let vec = expr.get_ref_branch();

    if vec.len() != 3 
       || !vec[1].is_list() 
       || !is_symbol("lambda", &vec[0]) {
        return false;
    }

    let params = vec[1].get_ref_branch();

    for p in params.iter() {
        if !p.is_atom() {
            return false;
        }
    }

    true
}

fn is_label_literal(expr: &Expr) -> bool {
    if !expr.is_list() {
        return false;
    }

    let vec = expr.get_ref_branch();

    if vec.len() != 3 
       || !vec[1].is_atom() 
       || !is_symbol("label", &vec[0]) 
       || !is_lambda_literal(&vec[2]) {
        return false;
    }

    true
}

fn is_symbol(op: &str, expr: &Expr) -> bool {
    if expr.is_atom() {
        let expr_op = expr.get_ref_leaf();
        op.equiv(expr_op)
    } else {
        false
    }
}

fn prepare_lambda(env: Env, vec: ~[Expr])
    -> Result<(~[Expr], HashMap<~str, Expr>, MoveItems<Expr>), ~str> {

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

    if !is_func_literal(&op_expr) {
        let res = eval(env, op_expr);

        if res.is_err() {
            return Err( res.unwrap_err() );
        } else {
            op_expr = res.unwrap().n1();

            if !is_func_literal(&op_expr) {
                return Err(~"Unrecognized expression.");
            }
        }
    }

    // If we've made it here, op_expr is either a label or lambda literal
    let lambda: ~[Expr];
    let mut bindings = HashMap::<~str, Expr>::new();

    if is_label_literal(&op_expr) {
        let label_expr = op_expr.clone();

        let label = op_expr.unwrap_branch();
        let mut label_iter = label.move_iter();
        label_iter.next(); //discard "label" symbol

        let func_sym = label_iter.next().unwrap().unwrap_leaf();

        bindings.insert(func_sym, label_expr);

        lambda = label_iter.next().unwrap().unwrap_branch();
    } else {
        lambda = op_expr.unwrap_branch();
    }

    Ok( (lambda, bindings, vec_iter) )
}


/// takes a vector of expressions and a vector of Atoms, evals each expression
/// and inserts it into a provided hashMap (with the Atom as the key)
fn populate_bindings(mut args: MoveItems<Expr>, params: ~[Expr],
     env: Env, mut bindings: HashMap<~str, Expr>) 
    -> Result<HashMap<~str, Expr>, ~str> {

    let mut param_iter = params.move_iter();

    debug!("\n :: iterating through args now and passing them into bindings\n");
    for arg in args {
        debug!("  -- {}", arg);
        let res = eval(env.clone(), arg);

        if res.is_err() {
            return Err( res.unwrap_err() );
        } else {

            let next_param: Expr  = param_iter.next().unwrap();
            bindings.insert(next_param.unwrap_leaf(),
                            res.unwrap().n1());
        }

    }

    Ok( bindings )
}
