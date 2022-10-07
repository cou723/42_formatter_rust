// NEWLINE_PROCESS_FUNCへの対応
// 関数と関数の間に空行がなかった場合空行を挿入する

use crate::c_parser;

pub fn format(file_texts: &Vec<String>) -> Vec<String> {
	let mut new_file_texts: Vec<String> = Vec::new();
	for (i, line) in file_texts.iter().enumerate() {
		new_file_texts.push(line.to_string());
		let next_line = match file_texts.get(i + 1) {
			Some(x) => x,
			None => break,
		};
		// 関数の終わりの行でかつ次の行が空行ではない場合、空行を挿入する
		if line.starts_with("}") && next_line != "" {
			new_file_texts.push(String::from(""));
		}
	}
	new_file_texts
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_util;

	const TEST_NAME: &str = "newline_between_func";
	#[test]
	fn test_format_wrapper() {
		// TODO: パニック時のメッセージ
		test_format("default1");
	}

	fn test_format(test_file_name: &str) {
		test_util::file_compare_test(TEST_NAME, test_file_name, format);
	}
}
