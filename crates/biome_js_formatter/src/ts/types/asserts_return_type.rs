use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::TsAssertsReturnType;
use biome_js_syntax::TsAssertsReturnTypeFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsAssertsReturnType;

impl FormatNodeRule<TsAssertsReturnType> for FormatTsAssertsReturnType {
    fn fmt_fields(&self, node: &TsAssertsReturnType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAssertsReturnTypeFields {
            parameter_name,
            asserts_token,
            predicate,
        } = node.as_fields();
        write![
            f,
            [
                asserts_token.format(),
                space(),
                parameter_name.format(),
                predicate.is_some().then_some(space()),
                predicate.format()
            ]
        ]
    }

    fn needs_parentheses(&self, item: &TsAssertsReturnType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsAssertsReturnType {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}
