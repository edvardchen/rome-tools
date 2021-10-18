use crate::{format_elements, space_token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::Getter;

impl ToFormatElement for Getter {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let token = formatter.format_token(&self.get_token()?)?;
		let name = formatter.format_node(self.key()?)?;
		let params = formatter.format_node(self.parameters()?)?;
		let body = formatter.format_node(self.body()?)?;
		Some(format_elements![
			token,
			space_token(),
			name,
			params,
			space_token(),
			body
		])
	}
}
