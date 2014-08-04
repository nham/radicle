use super::{Expr, Env, HashMap, Tree, Nil, Atom, List};

pub type EvalResult = Result<Expr, &'static str>;

/// The heart and soul of Radicle.
pub fn eval(env: &mut Env, expr: Expr) -> EvalResult {
    debug!(" :: Entered eval, expr = {:?}\n", expr);
    match expr {
        Nil => Ok(Nil),
        Atom(ref s) => {
            let res = env.find_copy(s);
            if res.is_none() {
                Err("Symbol not found.")
            } else {
                Ok(res.unwrap())
            }
        },
        List(vec) => {
            if vec.len() == 0 {
                return Err("No procedure to call. TODO: a better error message?");
            }

            if is_symbol("quote", &vec[0]) {
                if vec.len() != 2 {
                    Err("`quote` expects exactly one argument.")
                } else {
                    Ok(vec[1].clone())
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

fn eval_atom(env: &mut Env, vec: Vec<Expr>) -> EvalResult {
    if vec.len() != 2 {
        Err("`atom` expects exactly one argument.")
    } else {
        let val = try!( eval(env, vec[1].clone() ) );
        if val.is_atom() || val.is_empty_list() {
            Ok( Atom("t".to_string()) )
        } else {
            Ok( Tree::empty_branch() )
        }
    }
}


fn eval_eq(env: &mut Env, vec: Vec<Expr>) -> EvalResult {

    if vec.len() != 3 {
        Err("`eq` expects exactly two arguments.")
    } else {
        let val1 = try!( eval(env, vec[1].clone()) );
        let val2 = try!( eval(env, vec[2].clone() ) );
        if (val1.is_empty_list() && val2.is_empty_list())
           || (val1.is_atom() && val2.is_atom() && val1.eq(&val2)) {
            Ok( Atom("t".to_string()) )
        } else {
            Ok( Tree::empty_branch() )
        }
    }
}


fn eval_first(env: &mut Env, vec: Vec<Expr>) -> EvalResult {

    if vec.len() != 2 {
        Err("`first` expects exactly one argument.")
    } else {
        let val = try!( eval(env, vec[1].clone() ) );
        if val.is_list() && !val.is_empty_list() {
            let list = val.unwrap_branch();
            Ok( list[0].clone() )
        } else {
            debug!("argument is {:?}\n", val);
            Err("`first`'s argument must be a non-empty list")
        }
    }
}

fn eval_rest(env: &mut Env, vec: Vec<Expr>) -> EvalResult {

    if vec.len() != 2 {
        Err("`rest` expects exactly one argument.")
    } else {
        let val = try!( eval(env, vec[1].clone() ) );
        if val.is_list() && !val.is_empty_list() {
            let mut list = val.unwrap_branch();
            list.shift();
            Ok( List(list) )
        } else {
            Err("`rest`'s argument must be a non-empty list")
        }
    }
}

fn eval_cons(env: &mut Env, vec: Vec<Expr>) -> EvalResult {

    if vec.len() != 3 {
        Err("`cons` expects exactly two arguments.")
    } else {
        let val1 = try!( eval(env, vec[1].clone()) );
        let val2 = try!( eval(env, vec[2].clone()) );

        if val2.is_list() {
            let mut list = val2.unwrap_branch();
            list.unshift(val1);
            Ok( List(list) )
        } else {
            Err("`cons`'s second argument must be a list")
        }
    }
}

fn eval_cond(env: &mut Env, vec: Vec<Expr>) -> EvalResult {
    let mut i = 1;
    while i < vec.len() {
        if !vec[i].is_list() {
            return Err("Invalid argument to `cond`");
        }

        let arg = vec[i].clone();
        let list = arg.unwrap_branch();

        if list.len() != 2 {
            return Err("Invalid argument to `cond`");
        } else {
            let res = eval(env, list[0].clone());
            let val = try!(res);

            if val.eq( &Atom("t".to_string()) ) {
                return eval(env, list[1].clone() );
            }
        }

        i += 1;
    }

    Ok(Nil)
}


fn eval_defun(env: &mut Env, vec: Vec<Expr>) -> EvalResult {
    if vec.len() != 4 {
        Err("`defun` expects exactly three arguments.")
    } else {

        if !vec[1].is_atom() {
            return Err("First argument to `defun` must be a symbol");
        }

        {
            let params = vec[2].get_ref_branch();
            for p in params.iter() {
                if !p.is_atom() {
                    return Err("Second argument to `defun` must be a list of params");
                } 
            }
        }

        let func_name = vec[1].clone();
        let params = vec[2].clone();
        let body = vec[3].clone();

        let label_expr = List( vec!(Atom("label".to_string()), 
                                    func_name,
                                    List( vec!(Atom("lambda".to_string()), params, body) ))
                             );
        env.bindings.insert(vec[1].clone().unwrap_leaf(), label_expr);
        Ok(Nil)
    }
}


struct FuncLiteral {
    params: Vec<String>,
    body: Expr,
    sym: Option<String>, // lambdas will have None, labels will have Some
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
    let mut plist = vec!();

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

fn eval_func_call(env: &mut Env, vec: Vec<Expr>) -> EvalResult {
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
        op_expr = try!( eval(env, op_expr) );

        func_lit = parse_func_literal(&op_expr);
        if func_lit.is_none() {
            return Err("Unrecognized expression.");
        }
    }

    let FuncLiteral{params, body, sym} = func_lit.unwrap();
    let mut bindings = HashMap::<String, Expr>::new();
    if sym.is_some() {
        bindings.insert(sym.unwrap(), op_expr.clone());
    }

    if params.len() != num_args {
        return Err("mismatch between number of procedure args and number of args called with.");
    }

    let mut param_iter = params.move_iter();

    for arg in vec_iter {
        let next_param: String  = param_iter.next().unwrap();
        debug!("  - eval of {:?} --> {}\n", arg, next_param);
        bindings.insert(next_param, 
                        try!( eval(env, arg) ));
    }

    let mut new_env = env.clone();
    for (k, v) in bindings.move_iter() {
        new_env.bindings.insert(k, v);
    }

    debug!(" :: arguments have been passed into environment, evaling lambda body\n");
    let val = try!( eval(&mut new_env, body) );
    Ok(val)
}
