use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsImportCallExpression;

impl ToFormatElement for JsImportCallExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.import_token()?)?,
			formatter.format_node(self.arguments()?)?,
		])
	}
}