// NO_ARGS_VOID対策
// 引数がない関数定義にvoidを挿入する

use crate::c_parser::*;

pub fn format(file_texts:&Vec<String>) -> Vec<String> {
	let mut new_file_texts: Vec<String> = Vec::new();
	for line in file_texts.iter() {
		// 引数がないの関数定義を見つけたとき
		if (is_func_def(&line) || is_prototype_declare(&line)) && line.contains("()") {
			let mut new_line = insert_void_to_func_def(&line);
			new_file_texts.push(new_line);
		} else {
			new_file_texts.push(line.to_string());
		}
	}
	new_file_texts
}

fn insert_void_to_func_def(line: &String) -> String {
	let mut new_line = String::new();
	for c in line.chars() {
		new_line.push(c);
		if c == '(' {
			new_line.push_str("void");
		}
	}
	new_line
}

// test
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_insert_void_to_func_def() {
		assert_eq!(insert_void_to_func_def(&"int x()".to_string()),"int x(void)");
		assert_eq!(insert_void_to_func_def(&"long int x()".to_string()),"long int x(void)");
	}
}
