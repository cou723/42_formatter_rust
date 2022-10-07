use crate::{c_parser, *};
// 渡されたコードを関数ごとに分け、set_tab_indentに渡す
// HACK: 関数長すぎ
pub fn format(file_texts: &Vec<String>) -> Vec<String> {
	let mut new_file_texts: Vec<String> = Vec::new();
	let mut block_depth = 0;
	let mut func_buf: Vec<String> = Vec::new();

	for line in file_texts.iter() {
		// 関数定義がある場合block_depthを1にする
		if c_parser::is_func_def(&line) {
			block_depth = 1;
		}
		// lineが関数の中だった場合
		if block_depth >= 1 {
			// 関数定義以外のコードブロックの開始だった場合
			if is_block_start_other_than_func_def(func_buf.last().unwrap_or(&" ".to_string()))
				&& line.trim().starts_with("{")
			{
				block_depth += 1;
			}
			func_buf.push(line.to_string());
			if line.trim().starts_with("}") {
				// 関数の終わりだった場合
				// TODO: さすがにネストが深すぎる
				if block_depth == 1 {
					block_depth = 0;
					func_buf = set_tab_indent(&mut func_buf);
					new_file_texts.append(&mut func_buf);
					func_buf.clear();
				// 関数以外(if,for,while etc...)のブロックの終わりだった場合
				} else {
					block_depth -= 1;
				}
			}
		} else {
			new_file_texts.push(line.to_string());
		}
	}
	new_file_texts
}

// 関数以外のコードブロックの始まりか？
fn is_block_start_other_than_func_def(line: &str) -> bool {
	(c_parser::is_if(line)
		|| c_parser::is_else_if(line)
		|| c_parser::is_for(line)
		|| c_parser::is_while(line)
		|| c_parser::is_switch(line)
		|| c_parser::is_do(line))
}

fn set_tab_indent(func_texts: &Vec<String>) -> Vec<String> {
	let mut new_func_texts: Vec<String> = Vec::new();
	let mut largest_type_length = largest_type_length_variable(&func_texts);

	for line in func_texts.iter() {
		if c_parser::is_variable_def(&line) && !has_variable_define_used_tab(&line) {
			let mut type_length = type_length(&line);
			let mut insert_tabs = util::get_insert_tab_str(line, largest_type_length, type_length);
			// HACK: line_splitted_spaceって名前良くない気がする
			let mut line_splitted_space: Vec<String> =
				line.split(" ").map(|s| s.to_string()).collect();
			// 最後から二番目にtabを追加する["long","int<here>", "a;"]
			line_splitted_space.rotate_right(1);
			line_splitted_space
				.last_mut()
				.unwrap()
				.push_str(&insert_tabs);
			line_splitted_space.rotate_left(1);

			let mut new_line = line_splitted_space.join(" ");
			let mut new_line = util::replacen_rev(&new_line, ' ', "", 1);
			new_func_texts.push(new_line);
		} else {
			new_func_texts.push(line.to_string());
		}
	}
	new_func_texts
}

fn largest_type_length_variable(file_texts: &Vec<String>) -> usize {
	let mut largest_type_length: usize = 0;
	for line in file_texts.iter() {
		let mut type_length = 0;
		if c_parser::is_variable_def(&line) && !has_variable_define_used_tab(&line) {
			type_length = get_type_length_variable(&line);
			if type_length > largest_type_length {
				largest_type_length = type_length;
			}
		}
	}
	largest_type_length
}

fn get_type_length_variable(line: &str) -> usize {
	let line = line.trim(); // \tを削除
	let mut splitted_line: Vec<&str> = line.split(" ").collect();
	splitted_line.pop(); // 型の文字数取得に先頭の要素（変数名;）はいらないため削除
	splitted_line.join(" ").len()
}

fn type_length(line: &str) -> usize {
	let mut splitted_line: Vec<&str> = line.trim().split(" ").collect();
	splitted_line.pop(); // 型の文字数取得に先頭の要素（変数名;）はいらないため削除
	splitted_line.join(" ").len()
}

fn has_variable_define_used_tab(line: &str) -> bool {
	line.trim().contains("\t")
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
			test_util::load_text_from_file("space_replace_tab_indent", 0);
		assert_eq!(
			format(&file_texts),
			expected_texts.clone(),
			"id: {} のテストが失敗しました",
			0
		);
	}

	#[test]
	fn test_type_length() {
		assert_eq!(type_length("int ax;"), 3);
		assert_eq!(type_length("long int abc;"), 8);
		assert_eq!(type_length("long long int a;"), 13);
	}
}
