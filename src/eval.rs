use super::Interpreter;
use std::{
    fmt::{self, Debug},
    rc::Rc,
};

pub trait Eval: Debug {
    fn eval(self: Rc<Self>, int: &mut Interpreter);
    fn short_name(&self) -> &str;
}

pub trait IntoEval<T: ?Sized> {
    fn into_eval(self: Rc<Self>) -> Rc<T>;
}

impl<'a, T: Eval + 'a> IntoEval<dyn Eval + 'a> for T {
    fn into_eval(self: Rc<Self>) -> Rc<dyn Eval + 'a> {
        self
    }
}

pub trait UnimplementedEval: Debug {
    fn short_name(&self) -> &str;
}

impl<T: UnimplementedEval> Eval for T {
    fn eval(self: Rc<Self>, _: &mut Interpreter) {
        unimplemented!("unimplemented -- {} -- unimplemented", self.short_name())
    }
    fn short_name(&self) -> &str {
        self.short_name()
    }
}

pub struct Custom<T: Fn(&mut Interpreter)> {
    name: &'static str,
    function: T,
}
impl<T: Fn(&mut Interpreter)> Custom<T> {
    pub fn new(name: &'static str, function: T) -> Self {
        Custom { name, function }
    }
}
impl<T: Fn(&mut Interpreter)> fmt::Debug for Custom<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Custom")
            .field("name", &self.name)
            .finish()
    }
}
impl<T: Fn(&mut Interpreter)> Eval for Custom<T> {
    fn eval(self: Rc<Self>, int: &mut Interpreter) {
        // TODO: Allow FnOnce functions in Custom by using Rc::try_unwrap here.
        (self.function)(int)
    }
    fn short_name(&self) -> &str {
        self.name
    }
}
