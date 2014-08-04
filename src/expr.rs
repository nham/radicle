use std::fmt::Show;

#[deriving(PartialEq, Clone)]
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
        List(vec!())
    }

    pub fn is_nil(&self) -> bool {
        match *self {
            Nil => true,
            _ => false,
        }
    }

    pub fn is_atom(&self) -> bool {
        match *self {
            Atom(_) => true,
            _       => false
        }
    }

    pub fn is_list(&self) -> bool {
        !self.is_atom()
    }

    pub fn get_ref_atom<'a>(&'a self) -> &'a T {
        match *self {
            Atom(ref val) => val,
            _         => fail!("called Expression::get_ref_atom() on non-Atom"),
        }
    }

    pub fn get_ref_list<'a>(&'a self) -> &'a Vec<Expression<T>> {
        match *self {
            List(ref val) => val,
            _         => fail!("called Expression::get_ref_list() on non-List"),
        }
    }

    pub fn unwrap_atom(self) -> T {
        match self {
            Atom(val) => val,
            _         => fail!("called Expression::unwrap_atom() on non-Atom"),
        }
    }

    pub fn unwrap_list(self) -> Vec<Expression<T>> {
        match self {
            List(val) => val,
            _         => fail!("called Expression::unwrap_list() on non-List"),
        }
    }


}

impl<T: Show> Expression<T> {
    pub fn print(&self) {
        self.print_expr();
        println!("");

    }

    fn print_expr(&self) {
        match *self {
            Nil => {},
            Atom(ref val) => { print!("{}", *val); },
            List(ref vec) => {
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
