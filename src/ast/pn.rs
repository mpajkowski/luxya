use crate::ast::expr::*;

#[allow(dead_code)]
pub fn pn_stringify_tree(expr: &Expr) -> String {
	match expr {
		Expr::Binary(v) => {
			pn_gen(&v.operator.token_type.to_string(), &[&v.left, &v.right])
		}
		Expr::Unary(v) => {
			pn_gen(&v.operator.token_type.to_string(), &[&v.right])
		}
		Expr::Grouping(v) => pn_gen("group", &[&v.expression]),
		Expr::Literal(v) => match v {
			LiteralValue::String(s) => format!("{:?}", s),
			LiteralValue::Number(n) => format!("{}", n),
			LiteralValue::True => "true".into(),
			LiteralValue::False => "false".into(),
			LiteralValue::Nil => "nil".into(),
		},
		Expr::Identifier(v) => v.name.token_type.to_string(),
		Expr::Assignment(v) => pn_gen(&format!("= {}", v.name), &[&v.value]),
		Expr::Call(v) => pn_gen(
			&format!("call {}", pn_stringify_tree(&v.calee)),
			v.arguments.iter().collect::<Vec<&Expr>>().as_slice(),
		),
	}
}

fn pn_gen(name: &str, exprs: &[&Expr]) -> String {
	let mut res = format!("({}", name);

	exprs.iter().for_each(|expr| {
		res += " ";
		res += &pn_stringify_tree(expr);
	});

	res + ")"
}
