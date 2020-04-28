use crate::kal_ref::KalRef;
use crate::new_interpreter::{Interpreter, Key, Value};

use std::collections::HashMap;

#[allow(dead_code)]
fn eval_file(path: &str) -> Value {
    let text =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not read file {:?}", path));
    let ast = crate::kal_grammar::BlockInnerParser::new()
        .parse(&text)
        .unwrap_or_else(|_| panic!("Failed to parse file {:?}.", path));
    let mut runtime = Interpreter::new();

    runtime.eval(ast)
}

macro_rules! test {
    {release_mode_only, $test_name:ident, $expected_val:expr } => {
        #[cfg(not(debug_assertions))]
        #[test]
        pub fn $test_name() {
            let val = eval_file(&format!("examples/{}.kal", stringify!($test_name)));
            let expected = $expected_val;
            assert!(val == expected, format!("Assertion failed: got {:?}, expected {:?}.", val, expected));
        }
    };
    {$test_name:ident, $expected_val:expr} => {
        #[test]
        pub fn $test_name() {
            let val = eval_file(&format!("examples/{}.kal", stringify!($test_name)));
            let expected = $expected_val;
            assert!(val == expected, format!("Assertion failed: got {:?}, expected {:?}.", val, expected));
        }
    };
}

#[cfg(not(debug_assertions))]
#[test]
fn big_file() {
    let size = 4_000_000_i64;
    let let_statements = "let num=num+1;".repeat(size as usize);
    let text = format!("let num = 0; {} num", let_statements);
    let ast = crate::kal_grammar::BlockInnerParser::new()
        .parse(&text)
        .unwrap();
    let mut runtime = Interpreter::new();
    let val = runtime.eval(ast);
    assert!(val == Value::Int(size));
}

test! { empty_file, Value::Null }

test! { let_expr_basic, Value::Int(42) }

test! { fn_add_one, Value::Int(6) }

test! { fn_nameless, Value::Int(452) }

test! { fn_nested, Value::Int(4) }

test! { fn_chained, Value::Int(23) }

test! { fn_null, Value::Null }

test! { fn_multiple_statements, Value::Int(100) }

test! { fn_recursive_factorial, Value::Int(120) }

test! { if_true, Value::Int(71) }

test! { if_false, Value::Int(72) }

test! { if_comparison, Value::Int(0) }

test! { if_else_if, Value::Int(77) }

test! { if_without_else, Value::Null }

test! { comparison_true, Value::Bool(true) }

test! { comparison_false, Value::Bool(false) }

test! { release_mode_only, big_recursive, Value::Int(1_000_000) }

test! { object_empty, Value::Object(KalRef::new(HashMap::new())) }

test! { object_simple,
    {
        let mut obj = HashMap::new();
        obj.insert(Key::Str("cat".to_owned()), Value::Int(1));
        Value::Object(KalRef::new(obj))
    }
}

test! { object_access, Value::Int(2) }

test! { object_nested, Value::Int(22) }

test! { boolean_and, Value::Bool(false) }

test! { boolean_or, Value::Bool(true) }

test! { boolean_xor, Value::Bool(true) }

test! { boolean_precedence, Value::Bool(true) }

test! { symbol, Value::Symbol(0) } // first symbol is always 0

test! { symbol_as_value, Value::Symbol(1) } // second symbol is always 1

test! { symbol_equality, Value::Bool(false) }

test! { trailing_commas, Value::Int(2) }

test! { list, Value::List(KalRef::new(vec![Value::Int(1), Value::Int(2), Value::Int(3)])) }

test! { list_index, Value::Int(29) }

test! { list_negative_index, Value::Int(53) }

test! { list_spread, Value::List(KalRef::new(vec![Value::Int(1), Value::Int(2), Value::Int(3), Value::Int(4), Value::Int(5), Value::Int(6)])) }

test! { int, Value::Int(5) }

test! { int_negative, Value::Int(-5) }

test! { num_addition, Value::Int(8) }

test! { num_subtraction, Value::Int(-2) }

test! { num_multiplication, Value::Int(15) }

test! { num_negative_subtraction, Value::Int(14) }

test! { num_division, Value::Int(2) }

test! { mut_num, Value::Int(2) }

test! { mut_multi_let, Value::Int(999) }

test! { mut_increment, Value::Int(61) }

test! { handle_resume, Value::Int(9) }

test! { handle_multiple_resume, Value::Int(81) }

test! { handle_no_resume, Value::Int(3) }

test! { handle_nested, Value::Int(55) }

test! { handle_nested_resume, Value::Int(20) }

#[test]
fn handle_implicit() {
    let val = eval_file("examples/handle_implicit.kal");
    match val {
        Value::Effect(effect) => assert!(effect.value == Value::Int(4)),
        _ => panic!("Expected an effect value, got something else."),
    }
}

#[test]
fn handle_empty() {
    let val = eval_file("examples/handle_empty.kal");
    match val {
        Value::Effect(effect) => assert!(effect.value == Value::Bool(true)),
        _ => panic!("Expected an effect value, got something else."),
    }
}
