use std::fmt::Show;

#[deriving(Eq, Clone)]
pub enum Tree<T> {
    Nil,
    Leaf(T),
    Branch(~[Tree<T>])
}

impl<T> Tree<T> {
    pub fn empty_branch() -> Tree<T> {
        Branch(~[])
    }

    pub fn is_nil(&self) -> bool {
        match *self {
            Nil     => true,
            _       => false
        }
    }

    pub fn is_leaf(&self) -> bool {
        match *self {
            Leaf(_) => true,
            _       => false
        }
    }

    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }

    pub fn get_ref_leaf<'a>(&'a self) -> &'a T {
        match *self {
            Leaf(ref val) => val,
            _         => fail!("called Tree<T>::get_ref_leaf() on non-Leaf"),
        }
    }

    pub fn get_ref_branch<'a>(&'a self) -> &'a ~[Tree<T>] {
        match *self {
            Branch(ref val) => val,
            _         => fail!("called Tree<T>::get_ref_branch() on non-Branch"),
        }
    }

    pub fn unwrap_leaf(self) -> T {
        match self {
            Leaf(val) => val,
            _         => fail!("called Tree<T>::unwrap_leaf() on non-Leaf"),
        }
    }

    pub fn unwrap_branch(self) -> ~[Tree<T>] {
        match self {
            Branch(val) => val,
            _         => fail!("called Tree<T>::unwrap_branch() on non-Branch"),
        }
    }


}

impl<T: Show> Tree<T> {
    pub fn print(&self) {
        self.print_tree();
        println!("");

    }

    fn print_tree(&self) {
        match *self {
            Nil => {},
            Leaf(ref val) => { print!("{}", *val); },
            Branch(ref vec) => {
                print!("(");
                if vec.len() > 0 {
                    let mut vec_iter = vec.iter();
                    let first = vec_iter.next();
                    first.unwrap().print_tree();

                    for e in vec_iter {
                        print!(" ");
                        e.print_tree();
                    }
                }
                print!(")");
            },
        }
    }
}
