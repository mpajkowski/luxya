use super::{interpreter_env::InterpreterEnvironment, types::*};
use crate::{env::*, token::*};

use std::{
	cell::{RefCell, RefMut},
	rc::Rc,
};


pub const NATIVE_FUNCTION_NAMES: [&str; 7] =
	["str", "typeof", "number", "len", "chars", "push", "extend"];

struct FunctionDefinition<'a> {
	name: &'a str,
	arity: usize,
	fun: NativeFunctionSignature,
}


fn native_str(
	_keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> InterpreterValue {
	let input = &args[0];

	if let InterpreterValue::String(_) = input {
		input.clone()
	} else {
		InterpreterValue::String(Rc::from(input.to_string()))
	}
}

fn native_typeof(
	_keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> InterpreterValue {
	InterpreterValue::String(Rc::from(args[0].to_human_readable()))
}

fn native_number(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let input = &args[0];

	match input {
		InterpreterValue::Number(_) => Ok(input.clone()),
		InterpreterValue::String(s) => {
			Ok(InterpreterValue::Number(s.parse().map_err(|_| {
				RuntimeError {
					message: format!("Couldn't parse `{}` to number", s),
					token: keyword.clone(),
				}
			})?))
		}
		_ => Err(RuntimeError {
			message: format!(
				"Can't parse {} to number",
				input.to_human_readable()
			),
			token: keyword.clone(),
		}),
	}
}

fn native_len(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	match &args[0] {
		InterpreterValue::String(s) => {
			Ok(InterpreterValue::Number(s.len() as f64))
		}
		InterpreterValue::List(l) => {
			let l_borrow = l.borrow();

			Ok(InterpreterValue::Number(l_borrow.len() as f64))
		}
		_ => Err(RuntimeError {
			message: format!(
				"Can't get length of {}",
				&args[0].to_human_readable()
			),
			token: keyword.clone(),
		}),
	}
}

fn native_chars(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let val = &args[0];

	match val {
		InterpreterValue::String(s) => Ok(InterpreterValue::List(Rc::new(
			RefCell::new(s.chars().map(InterpreterValue::Char).collect()),
		))),
		_ => Err(RuntimeError {
			message: format!(
				"Can't extract chars out of {}",
				val.to_human_readable(),
			),
			token: keyword.clone(),
		}),
	}
}

fn native_push(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let mut l_borrow = unwrap_list(&args[0], &keyword, 0)?;

	l_borrow.push(args[1].clone());

	drop(l_borrow);

	Ok(args[0].clone())
}

fn native_extend(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let second_items = unwrap_list(&args[1], keyword, 1)?
		.iter()
		.cloned()
		.collect::<Vec<InterpreterValue>>();

	unwrap_list(&args[0], keyword, 0)?.extend(second_items);

	Ok(args[0].clone())
}

fn declarator(env: &InterpreterEnvironment, funs: &[FunctionDefinition]) {
	funs.iter().for_each(|fd| {
		env.declare(
			fd.name.to_owned(),
			DeclaredValue {
				mutable: true,
				value: InterpreterValue::Function {
					fun: Rc::new(InterpreterFunction::Native {
						arity: fd.arity,
						fun: fd.fun,
					}),
					enclosing_env: env.clone(),
				},
			},
		);
	})
}

pub fn declare_native_functions(env: &InterpreterEnvironment) {
	declarator(
		env,
		&[
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[0],
				arity: 1,
				fun: |_k, _e, args| Ok(native_str(_k, _e, args)),
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[1],
				arity: 1,
				fun: |_k, _e, args| Ok(native_typeof(_k, _e, args)),
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[2],
				arity: 1,
				fun: native_number,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[3],
				arity: 1,
				fun: native_len,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[4],
				arity: 1,
				fun: native_chars,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[5],
				arity: 2,
				fun: native_push,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[6],
				arity: 2,
				fun: native_extend,
			},
		],
	);
}

#[inline(always)]
fn unwrap_list<'a>(
	value: &'a InterpreterValue,
	blame: &Token,
	arg_num: usize,
) -> Result<RefMut<'a, Vec<InterpreterValue>>, RuntimeError> {
	if let InterpreterValue::List(l) = &value {
		Ok(l.borrow_mut())
	} else {
		Err(RuntimeError {
			message: format!("Argument {} must be of type list", arg_num),
			token: blame.clone(),
		})
	}
}
