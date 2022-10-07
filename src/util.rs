use crate::c_parser;

pub const C_KEYWORDS: [&str; 32] = [
	"auto", "break", "case", "char", "const", "continue", "default", "do", "double", "else",
	"enum", "extern", "float", "for", "goto", "if", "int", "long", "register", "return", "short",
	"signed", "sizeof", "static", "struct", "switch", "typedef", "union", "unsigned", "void",
	"volatile", "while",
];

pub const C_PRIMITIVE_TYPES: [&str; 9] = [
	"char", "double", "float", "int", "long", "short", "signed", "unsigned", "void",
];

pub const C_MODIFIERS: [&str; 2] = ["const", "static"];

pub fn c_expected_words_before_func() -> Vec<String> {
	let mut expected_words: Vec<String> = Vec::new();
	for word in C_PRIMITIVE_TYPES.iter() {
		expected_words.push(word.to_string());
	}
	for word in C_MODIFIERS.iter() {
		expected_words.push(word.to_string());
	}
	expected_words
}

//渡された文字列にC_KEYWORDSの要素があるか判定するboolを返す関数
pub fn is_c_type_contain(line: &str) -> bool {
	for c_type in C_KEYWORDS.iter() {
		if line.contains(c_type) {
			return true;
		}
	}
	false
}

// lineに"("と")"が含まれているか判定する
pub fn is_parenthesis(line: &str) -> bool {
	line.contains("(") && line.contains(")")
}

// lineに"="と";"が含まれていないか判定する
pub fn is_not_equal_semicolon(line: &str) -> bool {
	!line.contains("=") && !line.contains(";")
}

/**
 * sの後ろから1番目からx番目をdestからsrcに置き換える
 */
pub fn replacen_rev(s: &String, dest: char, src: &str, x: usize) -> String {
	s.chars()
		.rev()
		.collect::<String>()
		.replacen(dest, &src.to_string(), x)
		.chars()
		.rev()
		.collect::<String>()
}

// 型と名前の間のスペースをtabにする
pub fn replace_space_to_tab_func(func_def: &String) -> String {
	let mut type_end_index = get_index_between_types_and_name(func_def);
	func_def
		.replacen(" ", "\t", type_end_index) //  _ _ _ _\t\t :type_end_index=3
		.replacen("\t", " ", type_end_index - 1) // \t\t\t _\t\t :type_end_index=3
		.to_string()
}

pub fn replace_space_to_tab_func_indent(func_def: &String, largest_type_len: usize) -> String {
	let mut type_end_index = get_index_between_types_and_name(func_def);
	let insert_tab_str =
		get_insert_tab_str(func_def, largest_type_len, get_type_length_func(func_def));
	let insert_tab_str = insert_tab_str.as_str();
	func_def
		.replacen(" ", &insert_tab_str, type_end_index) //  _ _ _ _\t\t :type_end_index=3
		.replacen(&insert_tab_str, " ", type_end_index - 1) // \t\t\t _\t\t :type_end_index=3
		.to_string()
}

pub fn get_index_between_types_and_name(func_def: &String) -> usize {
	let mut type_end_index: usize = 0;
	for (i, el) in func_def.split(" ").enumerate() {
		if !(c_expected_words_before_func().contains(&el.to_string())
			|| c_parser::is_custom_type(el)
			|| c_parser::is_original_type(el))
		{
			type_end_index = i;
			break;
		}
	}
	type_end_index
}

//挿入するタブを与えられた最大型長と現在の型長から求め、文字列をして返す
pub fn get_insert_tab_str(line: &str, largest_type_length: usize, type_length: usize) -> String {
	let need_space = calc_need_space(&line, largest_type_length, type_length);
	let mut insert_tabs = String::new();
	if need_space % 4 == 0 {
		for _ in 0..need_space / 4 {
			insert_tabs.push('\t');
		}
	} else {
		for _ in 0..(need_space / 4) + 1 {
			insert_tabs.push('\t');
		}
	}
	insert_tabs
}

fn calc_need_space(line: &str, largest_type_length: usize, type_length: usize) -> usize {
	if largest_type_length % 4 == 0 {
		return largest_type_length - type_length + 4;
	} else {
		return 4 - (largest_type_length % 4) + largest_type_length - type_length;
	}
}

pub fn get_type_length_func(line: &String) -> usize {
	let mut line = line.trim();
	let mut type_length = 0;
	for word in line.split(" ") {
		if c_expected_words_before_func().contains(&word.to_string())
			|| c_parser::is_custom_type(word)
			|| c_parser::is_original_type(word)
		{
			type_length += (word.len() + 1);
		} else {
			break;
		}
	}
	type_length - 1
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_replace_space_to_tab_func_indent() {
		assert_eq!(
			replace_space_to_tab_func_indent(&"int main()".to_string(), 3),
			"int	main()"
		);
		assert_eq!(
			replace_space_to_tab_func_indent(&"int main()".to_string(), 4),
			"int		main()"
		);
		assert_eq!(
			replace_space_to_tab_func_indent(&"char main()".to_string(), 4),
			"char	main()"
		);
		assert_eq!(
			replace_space_to_tab_func_indent(&"size_t main()".to_string(), 6),
			"size_t	main()"
		);
		assert_eq!(
			replace_space_to_tab_func_indent(&"int main()".to_string(), 5),
			"int		main()"
		);
	}

	#[test]
	fn test_get_index_between_types_and_name() {
		assert_eq!(
			get_index_between_types_and_name(&"int main()".to_string()),
			1
		);
		assert_eq!(
			get_index_between_types_and_name(&"unsigned int main()".to_string()),
			2
		);
		assert_eq!(
			get_index_between_types_and_name(&"const unsigned int main()".to_string()),
			3
		);
	}

	#[test]
	fn test_replace_space_to_tab() {
		assert_eq!(
			replace_space_to_tab_func(&r#"int main()"#.to_string()),
			r#"int	main()"#
		);
		assert_eq!(
			replace_space_to_tab_func(&r#"int main(int x)"#.to_string()),
			r#"int	main(int x)"#
		);
		assert_eq!(
			replace_space_to_tab_func(&r#"unsigned int main(int x)"#.to_string()),
			r#"unsigned int	main(int x)"#
		);
	}

	#[test]
	fn test_calc_need_space() {
		assert_eq!(calc_need_space("int a;", 3, 3), 1);
		assert_eq!(calc_need_space("long int x", 8, 8), 4);
		assert_eq!(calc_need_space("int minus;", 7, 3), 5);
		assert_eq!(calc_need_space("int minus;", 8, 3), 9);
	}

	#[test]
	fn test_get_insert_tab_str() {
		assert_eq!(get_insert_tab_str("int a;", 3, 3), "\t");
		assert_eq!(get_insert_tab_str("int a;", 4, 3), "\t\t");
		assert_eq!(get_insert_tab_str("unsigned int a;", 12, 12), "\t");
	}

	#[test]
	fn test_get_type_length_func() {
		let test_str = "int main()".to_string();
		let result = get_type_length_func(&test_str);
		assert_eq!(result, 3, "({})が{}と評価されました", test_str, result);
		let test_str = "unsigned int main()".to_string();
		let result = get_type_length_func(&test_str);
		assert_eq!(
			result,
			"unsigned int".len(),
			"({})が{}と評価されました",
			test_str,
			result
		);
		let test_str = "const unsigned int main()".to_string();
		let result = get_type_length_func(&test_str);
		assert_eq!(
			result,
			"const unsigned int".len(),
			"({})が{}と評価されました",
			test_str,
			result
		);
		let test_str = "const unsigned int *main()".to_string();
		let result = get_type_length_func(&test_str);
		assert_eq!(
			result,
			"const unsigned int".len(),
			"({})が{}と評価されました",
			test_str,
			result
		);
		let test_str = "void *ft_calloc(size_t	count, size_t size);".to_string();
		let result = get_type_length_func(&test_str);
		assert_eq!(
			result,
			"void".len(),
			"({})が{}と評価されました",
			test_str,
			result
		);
	}
}
