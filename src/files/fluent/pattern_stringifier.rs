use fluent_syntax::ast::{Expression, InlineExpression, Pattern, PatternElement, Variant, VariantKey};
use itertools::Itertools;

fn variant_as_str(variant: &Variant<&str>) -> String {
    let value = pattern_as_str(&variant.value);
    let key = match &variant.key {
        &VariantKey::Identifier { name } => name,
        &VariantKey::NumberLiteral { value } => value,
    };

    if variant.default {
        format!("  *[{}] {}\n", key, value)
    } else {
        format!("  [{}] {}\n", key, value)
    }
}

fn inline_expr_as_str(expr: &InlineExpression<&str>) -> String {
    match expr {
        InlineExpression::StringLiteral { value } => format!("{:?}", value),
        InlineExpression::NumberLiteral { value } => value.to_string(),
        InlineExpression::FunctionReference { id, arguments } => {
            let args = arguments.positional.iter()
                .map(inline_expr_as_str)
                .chain(arguments.named.iter().map(|a| format!("{}: {}", a.name.name, inline_expr_as_str(&a.value))))
                .join(", ");

            format!("{}({})", id.name, args)
        }
        InlineExpression::MessageReference { id, attribute } => {
            attribute.as_ref().map_or_else(|| id.name.to_string(), |a| format!("{}.{}", id.name, a.name))
        }
        InlineExpression::TermReference { id, attribute, arguments } => String::new(),
        InlineExpression::VariableReference { id } => id.name.to_string(),
        InlineExpression::Placeable { expression } => expression_as_str(expression),
    }
}

fn expression_as_str(expression: &Expression<&str>) -> String {
    match expression {
        Expression::Select { selector, variants } => {
            let selector = inline_expr_as_str(selector);
            let variants = variants.iter().map(variant_as_str).join("");

            format!("{{ {} ->\n{}}}", selector, variants)
        }
        Expression::Inline(expr) => format!("{{ {} }}", inline_expr_as_str(expr)),
    }
}

fn pattern_element_as_str(elt: &PatternElement<&str>) -> String {
    match elt {
        PatternElement::TextElement { value } => value.to_string(),
        PatternElement::Placeable { expression } => expression_as_str(&expression),
    }
}

pub(super) fn pattern_as_str(p: &Pattern<&str>) -> String {
    p.elements.iter().map(pattern_element_as_str).join("")
}
