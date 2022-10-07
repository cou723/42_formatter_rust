// SPACE_REPLACE_FUNC対策
// functionの名前と返り値の型にスペースがあった場合norm違反のためtabに置き換える

use crate::util;
use crate::{c_parser, *};

// 関数定義の返り値と関数名の間をspaceからtabにする
pub fn format(file_texts: &Vec<String>) -> Vec<String> {
	let mut new_file_texts: Vec<String> = Vec::new();
	for line in file_texts.iter() {
		// 関数定義でかつ、返り値と関数名の間がスペースの場合
		if c_parser::is_func_def(&line) && has_func_define_used_space(&line) {
			new_file_texts.push(util::replace_space_to_tab_func(&line));
		} else {
			new_file_texts.push(line.to_string());
		}
	}
	new_file_texts
}

pub fn format_for_prototype(file_texts: &Vec<String>) -> Vec<String> {
	let mut new_file_texts: Vec<String> = Vec::new();
	let mut largest_type_length = largest_type_length_in_header(file_texts);
	for line in file_texts.iter() {
		// 関数定義でかつ、返り値と関数名の間がスペースの場合
		if c_parser::is_prototype_declare(&line) && has_func_define_used_space(&line) {
			new_file_texts.push(util::replace_space_to_tab_func_indent(
				&line,
				largest_type_length,
			));
		} else {
			new_file_texts.push(line.to_string());
		}
	}
	new_file_texts
}

fn largest_type_length_in_header(file_texts: &Vec<String>) -> usize {
	let mut largest_type_length: usize = 0;
	for line in file_texts.iter() {
		let mut type_length = 0;
		if c_parser::is_prototype_declare(&line) && has_func_define_used_space(&line) {
			type_length = util::get_type_length_func(&line);
			if type_length > largest_type_length {
				largest_type_length = type_length;
			}
		}
	}
	largest_type_length
}

// 関数定義の返り値と関数名の間がスペースかどうかを判定する
// MEMO: 必ずlineが関数定義であることを前提としている
fn has_func_define_used_space(line: &str) -> bool {
	!line.contains("\t")
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_util;
	use std::fs::*;
	use std::io::*;

	#[test]
	fn test_format() {
		let (file_texts, expected_texts) =
			test_util::load_text_from_file("replace_tab_before_func", 0);
		assert_eq!(
			format(&file_texts),
			expected_texts.clone(),
			"id: {} のテストが失敗しました",
			0
		);
	}

	#[test]
	fn test_has_func_define_used_space() {
		assert_eq!(has_func_define_used_space(r#"int	main()"#), false);
		assert_eq!(has_func_define_used_space(r#"int main()"#), true);
		// static関数は未対応
		// assert_eq!(has_func_define_used_space(r#"static int	main()"#), true);
	}

	#[test]
	fn test_largest_type_length_in_header() {
		assert_eq!(
			largest_type_length_in_header(&vec![
				r#"int main();"#.to_string(),
				r#"char main();"#.to_string(),
				r#"size_t main();"#.to_string(),
				r#"unsigned char main();"#.to_string(),
				r#"void *main();"#.to_string()
			]),
			"unsigned char".len()
		);
	}
}
