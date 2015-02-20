use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Expression<T> {
    Nil,
    Atom(T),
    List(Vec<Expression<T>>)
}

impl<T: Eq> Expression<T> {
    pub fn is_empty_list(&self) -> bool {
        self.eq(&Expression::empty_list())
    }
}


impl<T> Expression<T> {
    pub fn empty_list() -> Expression<T> {
        Expression::List(vec!())
    }

    pub fn is_nil(&self) -> bool {
        match *self {
            Expression::Nil => true,
            _ => false,
        }
    }

    pub fn is_atom(&self) -> bool {
        match *self {
            Expression::Atom(_) => true,
            _       => false
        }
    }

    pub fn is_list(&self) -> bool {
        !self.is_atom()
    }

    pub fn get_ref_atom<'a>(&'a self) -> &'a T {
        match *self {
            Expression::Atom(ref val) => val,
            _         => panic!("called Expression::get_ref_atom() on non-Atom"),
        }
    }

    pub fn get_ref_list<'a>(&'a self) -> &'a Vec<Expression<T>> {
        match *self {
            Expression::List(ref val) => val,
            _         => panic!("called Expression::get_ref_list() on non-List"),
        }
    }

    pub fn unwrap_atom(self) -> T {
        match self {
            Expression::Atom(val) => val,
            _         => panic!("called Expression::unwrap_atom() on non-Atom"),
        }
    }

    pub fn unwrap_list(self) -> Vec<Expression<T>> {
        match self {
            Expression::List(val) => val,
            _         => panic!("called Expression::unwrap_list() on non-List"),
        }
    }


}

impl<T: fmt::Display> Expression<T> {
    pub fn print(&self) {
        self.print_expr();
        println!("");

    }

    fn print_expr(&self) {
        match *self {
            Expression::Nil => {},
            Expression::Atom(ref val) => { print!("{}", *val); },
            Expression::List(ref vec) => {
                print!("(");
                if vec.len() > 0 {
                    let mut vec_iter = vec.iter();
                    let first = vec_iter.next();
                    first.unwrap().print_expr();

                    for e in vec_iter {
                        print!(" ");
                        e.print_expr();
                    }
                }
                print!(")");
            },
        }
    }
}
