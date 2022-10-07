// RETURN_PARENTHESIS対策
// 返り値に括弧をつける

// HACK: return行のみを渡せばもしかしたら処理が早くなる？
pub fn format(file_texts: &Vec<String>) -> Vec<String> {
	let mut new_file_texts: Vec<String> = Vec::new();
	for line in file_texts.iter() {
		// 返り値が()で挟まれてないreturnが見つかった場合
		if line.trim().starts_with("return")
			&& !is_return_parenthesis(line)
			&& does_return_have_value(line)
		{
			let mut new_line = String::new();

			//インデントのtabを追加
			new_line.push_str(indent_tab_str(line).as_str());

			//return文本体を追加
			let return_value = return_value(line);
			new_line.push_str(&format!("return ({});", return_value));
			new_file_texts.push(new_line);
		} else {
			new_file_texts.push(line.to_string());
		}
	}
	new_file_texts
}

// return文を渡されるとreturnされる値の文字列を返す
fn return_value(line: &str) -> String {
	if line.trim() == "return;" {
		return "".to_string();
	}
	line.trim()
		.replacen("return ", "", 1)
		.replace(";", "")
		.replace("\t", "")
}

fn indent_tab_str(line: &str) -> String {
	let mut new_line = String::new();
	for i in 0..line.matches("\t").count() {
		new_line.push('\t');
	}
	new_line
}

// returnの値が()に囲われているかどうかを判定する
fn is_return_parenthesis(line: &str) -> bool {
	if line.contains("return (") && line.contains(");") {
		//最初に見つけた"return("と最後にある");"を削除
		let mut line = line.trim().replacen("return (", "", 1);
		let mut line = line.chars().rev().collect::<String>().replacen(";)", "", 1);
		let mut line = line.chars().rev().collect::<String>();

		// return (1)&&(1)みたいなのでfalseを返すために以下のような多少複雑なロジックになっている
		let mut score: isize = 0;
		for c in line.chars() {
			if c == '(' {
				score += 1;
			} else if c == ')' {
				score -= 1;
			}
			if score < 0 {
				return false;
			}
		}
		return true;
	}
	false
}

fn does_return_have_value(line: &str) -> bool {
	let mut line = line.trim();
	return !(line == "return ;" || line == "return;");
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_util;
	use std::fs::*;
	use std::io::*;

	#[test]
	fn test_format() {
		let (file_texts, expected_texts) = test_util::load_text_from_file("return_parenthesis", 0);
		assert_eq!(
			format(&file_texts),
			expected_texts.clone(),
			"id: {}のテストが失敗しました",
			0
		);
	}

	#[test]
	fn test_return_value() {
		assert_eq!(return_value("return 1;"), "1");
		assert_eq!(
			return_value("return \"test return string\";"),
			"\"test return string\""
		);
		assert_eq!(return_value("return;"), "");
	}

	// return文はindentが0のところには来ないという前提のテスト
	#[test]
	fn test_indent_tab_str() {
		assert_eq!(indent_tab_str("		return 1;"), "\t\t");
		assert_eq!(indent_tab_str("	return 1;",), "\t");
	}

	#[test]
	fn test_is_return_parenthesis() {
		assert_eq!(is_return_parenthesis(r#"return (1);"#), true);
		assert_eq!(is_return_parenthesis(r#"return 1;"#), false);
		assert_eq!(is_return_parenthesis(r#"return (1) + 1;"#), false);
		assert_eq!(is_return_parenthesis(r#"return 1 + (1);"#), false);
		assert_eq!(is_return_parenthesis(r#"return "(1);";"#), false);
		assert_eq!(
			is_return_parenthesis(r#"return (c >= '0') && (c <= '9');"#),
			false
		);
		assert_eq!(
			is_return_parenthesis(r#"return (c >= ('0')) && (c <= '9');"#),
			false
		);
		assert_eq!(
			is_return_parenthesis(r#"return ((c >= '0') && (c <= '9'));"#),
			true
		);
	}
}
