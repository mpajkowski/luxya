use super::{
	helpers::*,
	interpret::eval_expression,
	interpreter_env::InterpreterEnvironment,
	types::*,
};
use crate::{
	ast::{expr::*, stmt::*},
	env::*,
};

use std::{collections::HashMap, rc::Rc};


#[inline(always)]
pub fn expression_statement<E, T>(
	expr_evaluator: fn(&Expr, &E) -> Result<T, RuntimeError>,
	v: &ExpressionValue,
	env: &E,
) -> Result<InterpreterStmtValue<T>, RuntimeError>
where
	E: EnvironmentWrapper<T>,
{
	expr_evaluator(&v.expression, env)?;

	Ok(InterpreterStmtValue::Noop)
}

#[inline(always)]
pub fn print_statement<E, T>(
	expr_evaluator: fn(&Expr, &E) -> Result<T, RuntimeError>,
	v: &PrintValue,
	env: &E,
) -> Result<InterpreterStmtValue<T>, RuntimeError>
where
	T: std::fmt::Display,
	E: EnvironmentWrapper<T>,
{
	let evaluated = expr_evaluator(&v.expression, env)?;

	println!("{}", evaluated);

	Ok(InterpreterStmtValue::Noop)
}

#[inline(always)]
pub fn declaration_statement<E>(
	expr_evaluator: fn(&Expr, &E) -> Result<InterpreterValue, RuntimeError>,
	v: &DeclarationValue,
	env: &E,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError>
where
	E: EnvironmentWrapper<InterpreterValue>,
{
	let value = v
		.initializer
		.as_ref()
		.map_or(Ok(InterpreterValue::Nil), |initializer| {
			expr_evaluator(&initializer, env)
		})?;

	env.declare(
		assume_identifier(&v.name).to_owned(),
		DeclaredValue {
			mutable: v.mutable,
			value,
		},
	);

	Ok(InterpreterStmtValue::Noop)
}

#[inline(always)]
pub fn block_statement<E, T>(
	stmts_evaluator: fn(
		&[Stmt],
		&E,
	) -> Result<InterpreterStmtValue<T>, RuntimeError>,
	v: &BlockValue,
	env: &E,
) -> Result<InterpreterStmtValue<T>, RuntimeError>
where
	E: EnvironmentWrapper<T>,
{
	let new_scope = env.fork();

	stmts_evaluator(&v.statements, &new_scope)
}

#[inline(always)]
pub fn if_statement<E>(
	expr_evaluator: fn(&Expr, &E) -> Result<InterpreterValue, RuntimeError>,
	stmt_evaluator: fn(
		&Stmt,
		&E,
	) -> Result<
		InterpreterStmtValue<InterpreterValue>,
		RuntimeError,
	>,
	v: &IfValue,
	env: &E,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError>
where
	E: EnvironmentWrapper<InterpreterValue>,
{
	if expr_evaluator(&v.condition, env)? == InterpreterValue::True {
		stmt_evaluator(&v.then, env)
	} else if let Some(otherwise) = &v.otherwise {
		stmt_evaluator(otherwise, env)
	} else {
		Ok(InterpreterStmtValue::Noop)
	}
}

pub fn for_statement<E, T>(
	expr_evaluator: fn(&Expr, &E) -> Result<InterpreterValue, RuntimeError>,
	stmt_evaluator: fn(
		&Stmt,
		&E,
	) -> Result<InterpreterStmtValue<T>, RuntimeError>,
	v: &ForValue,
	env: &E,
) -> Result<InterpreterStmtValue<T>, RuntimeError>
where
	E: EnvironmentWrapper<T>,
{
	// these branches look sooo sketchy, but it's an optimization for
	// condition-less loops
	if let Some(condition) = &v.condition {
		while expr_evaluator(condition, env)? == InterpreterValue::True {
			let e = stmt_evaluator(&v.body, env)?;

			match e {
				InterpreterStmtValue::Break(_) => break,
				InterpreterStmtValue::Continue(_) => {
					if let Some(c) = &v.closer {
						stmt_evaluator(c, env)?;
					}

					continue;
				}
				InterpreterStmtValue::Noop => (),
				InterpreterStmtValue::Return { .. } => {
					return Ok(e);
				}
			}

			if let Some(c) = &v.closer {
				stmt_evaluator(c, env)?;
			}
		}
	} else {
		loop {
			let e = stmt_evaluator(&v.body, env)?;

			match e {
				InterpreterStmtValue::Break(_) => break,
				InterpreterStmtValue::Continue(_) => {
					if let Some(c) = &v.closer {
						stmt_evaluator(c, env)?;
					}

					continue;
				}
				InterpreterStmtValue::Noop => (),
				InterpreterStmtValue::Return { .. } => {
					return Ok(e);
				}
			}

			if let Some(c) = &v.closer {
				stmt_evaluator(c, env)?;
			}
		}
	}

	Ok(InterpreterStmtValue::Noop)
}

#[inline(always)]
pub fn return_statement<E>(
	expr_evaluator: fn(&Expr, &E) -> Result<InterpreterValue, RuntimeError>,
	v: &ReturnValue,
	env: &E,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError>
where
	E: EnvironmentWrapper<InterpreterValue>,
{
	Ok(InterpreterStmtValue::Return {
		value: v
			.expression
			.as_ref()
			.map_or(Ok(InterpreterValue::Nil), |e| expr_evaluator(e, env))?,
		keyword: v.keyword.clone(),
	})
}

#[inline(always)]
pub fn break_statement<T>(
	v: &BreakValue,
) -> Result<InterpreterStmtValue<T>, RuntimeError> {
	Ok(InterpreterStmtValue::Break(v.keyword.clone()))
}

#[inline(always)]
pub fn continue_statement<T>(
	v: &ContinueValue,
) -> Result<InterpreterStmtValue<T>, RuntimeError> {
	Ok(InterpreterStmtValue::Continue(v.keyword.clone()))
}

pub fn class_statement(
	v: &ClassValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	let name = assume_identifier(&v.name);

	let mut methods = HashMap::new();

	let mut constructor = None;

	for method in &v.methods {
		// TODO: unwrap using unwrapping macro like assume_expr(Function)
		let fv = if let Expr::Function(v) = method {
			v
		} else {
			unreachable!(
				"Method should be a function expression. Parser fucked up"
			)
		};

		let fun = construct_lox_defined_function(fv, env);

		let name = assume_identifier(fv.name.as_ref().expect("Method name"));

		if name == "constructor" {
			constructor = Some(Rc::new(fun));
		} else {
			methods.insert(name.to_owned(), fun);
		}
	}

	let superclass = if let Some(expr) = &v.superclass {
		let evaluated = eval_expression(expr, env)?;

		if !matches!(evaluated, InterpreterValue::Class { .. }) {
			return Err(RuntimeError {
				message: format!(
					"Cannot inherit from {}",
					evaluated.to_human_readable()
				),
				token: v.name.clone(),
			});
		}

		Some(Rc::new(eval_expression(expr, env)?))
	} else {
		None
	};

	env.declare(
		name.to_owned(),
		DeclaredValue {
			mutable: false,
			value: InterpreterValue::Class {
				superclass,
				constructor,
				name: Rc::from(name),
				methods: Rc::new(methods),
			},
		},
	);

	Ok(InterpreterStmtValue::Noop)
}
