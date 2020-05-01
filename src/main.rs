use new_interpreter::Interpreter;

#[macro_use]
extern crate lalrpop_util;

#[cfg(test)]
mod tests;

mod ast;
mod new_interpreter;

lalrpop_mod!(#[allow(clippy::all)] pub kal_grammar);

fn main() {
    let ast = kal_grammar::BlockInnerParser::new()
        .parse(include_str!("../examples/handle_implicit.kal"))
        .unwrap_or_else(|err| panic!("Failed to parse file, {:?}", err));

    let mut runtime = Interpreter::new();

    let result = runtime.eval(ast);

    println!("{:#?}", result);
}
