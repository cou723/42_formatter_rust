use std::fs::*;
use std::io::*;

pub fn file_compare_test(
	test_name: &str,
	test_file_name: &str,
	test_fn: fn(&Vec<String>) -> Vec<String>,
) {
	let mut test_file_texts: Vec<String> = load_test_file(test_name, test_file_name);
	let mut expected_file_texts: Vec<String> = load_expected_file(test_name, test_file_name);
	assert_eq!(
		expected_file_texts,
		test_fn(&test_file_texts),
		"<<expected>>\n{}\n<<actual>>\n{}\n",
		expected_file_texts.join("\n"),
		test_fn(&test_file_texts).join("\n")
	);
}

fn load_test_file(test_name: &str, test_file_name: &str) -> Vec<String> {
	let file_path = format!("./test/{}/{}.c", &test_name, &test_file_name);
	load_file_texts_vec(file_path.as_str())
}

fn load_expected_file(test_name: &str, test_file_name: &str) -> Vec<String> {
	let file_path = format!("./test/{}/{}_expected.c", &test_name, &test_file_name);
	load_file_texts_vec(file_path.as_str())
}

fn load_file_texts_vec(path: &str) -> Vec<String> {
	let mut file_texts: Vec<String> = Vec::new();
	for line in
		BufReader::new(File::open(path).expect(format!("file({}) not found", path).as_str()))
			.lines()
	{
		file_texts.push(line.unwrap());
	}
	file_texts
}

// ファイルを読み込み加工前のファイルテキストと加工後のファイルテキストを返す
pub fn load_text_from_file(test_name: &str, test_file_id: usize) -> (Vec<String>, Vec<String>) {
	// 読み込んだ加工前ファイルのVector
	let mut file_texts: Vec<String> = Vec::new();
	// 読み込んだ加工後ファイルのVector
	let mut expected_texts: Vec<String> = Vec::new();
	let test_file = File::open(format!("./test/{}/{}.c", test_name, test_file_id)).unwrap();
	for result in BufReader::new(test_file).lines() {
		file_texts.push(result.unwrap().to_string());
	}
	let test_file =
		File::open(format!("./test/{}/expected_{}.c", test_name, test_file_id)).unwrap();
	for result in BufReader::new(test_file).lines() {
		expected_texts.push(result.unwrap().to_string());
	}
	return (file_texts, expected_texts);
}

// syntax_test_*関数で使われるラッパー関数
pub fn check_syntax(
	test_fn: fn(line: &str) -> bool,
	arg_str: &str,
	expect_value: bool,
	error_str: &str,
) {
	assert_eq!(
		test_fn(arg_str),
		expect_value,
		"{}({})が{}と判定されました",
		error_str,
		arg_str,
		!expect_value
	);
}

pub fn syntax_test_include(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(test_fn, r#"#include <stdio.h>"#, expect_value, "include文");
	check_syntax(
		test_fn,
		r#"#include "\"libft.h\""#,
		expect_value,
		"include文",
	);
}
pub fn syntax_test_define(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(
		test_fn,
		r#"#define MAX 100"#,
		expect_value,
		"マクロ（define文）",
	);
	check_syntax(
		test_fn,
		r#"#define minus(x) (x - 1)"#,
		expect_value,
		"マクロ（define文）",
	);
}
pub fn syntax_test_ifdef(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(test_fn, r#"#ifndef LIBFT_H"#, expect_value, "ifndef文");
	check_syntax(test_fn, r#"#ifdef LIBFT_H"#, expect_value, "ifdef文");
	check_syntax(test_fn, r#"#endif"#, expect_value, "endif文");
}
pub fn syntax_test_func_def(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(test_fn, "int main()", expect_value, "関数宣言");
	check_syntax(
		test_fn,
		"int main(int a)",
		expect_value,
		"引数のある関数宣言",
	);
	check_syntax(
		test_fn,
		"static int main(int a)",
		expect_value,
		"引数のある関数宣言",
	);
	check_syntax(
		test_fn,
		"unsigned int ft_strlcpy(char *dest, char *src, unsigned int size)",
		expect_value,
		"引数のある関数宣言",
	);
	check_syntax(
		test_fn,
		r#"int main(int ac,char *av[])"#,
		expect_value,
		"引数のある関数定義",
	);
	check_syntax(
		test_fn,
		r#"size_t main(int ac,char *av[])"#,
		expect_value,
		"返り値が基本型ではない関数定義",
	);
	check_syntax(
		test_fn,
		r#"t_list main(int ac,char *av[])"#,
		expect_value,
		"自作型の関数定義",
	);
}
pub fn syntax_test_prototype_declare(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(test_fn, r#"int main();"#, expect_value, "プロトタイプ宣言");
	check_syntax(
		test_fn,
		r#"int main(int ac,char *av[]);"#,
		expect_value,
		"引数のあるプロトタイプ宣言",
	);
	check_syntax(
		test_fn,
		r#"size_t main(int ac,char *av[]);"#,
		expect_value,
		"返り値が基本型ではない関数定義",
	);
	check_syntax(
		test_fn,
		r#"t_list main(int ac,char *av[]);"#,
		expect_value,
		"自作型の関数定義",
	);
	check_syntax(
		test_fn,
		r#"void *ft_calloc(size_t count, size_t size);"#,
		expect_value,
		"返り値がvoid*の関数定義",
	);
}
pub fn syntax_test_func_call(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(test_fn, r#"	test();"#, expect_value, "関数呼び出し");
	check_syntax(
		test_fn,
		r#"	test("test string", 100);"#,
		expect_value,
		"関数呼び出し",
	);
	check_syntax(
		test_fn,
		r#"	test("auto", "break", "case", "char", "const", "continue", "default", "do", "double", "else", "enum", "extern", "float", "for", "goto", "if", "int", "long", "register", "return", "short","signed", "sizeof", "static", "struct", "switch", "typedef", "union", "unsigned", "void","volatile", "while");"#,
		expect_value,
		"",
	);
}
pub fn syntax_test_return(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(test_fn, r#"	return n;"#, expect_value, "return文");
	check_syntax(test_fn, r#"	return n || m;"#, expect_value, "return文");
	check_syntax(
		test_fn,
		r#"	return (n + 1) * (m + 1);"#,
		expect_value,
		"return文",
	);
	check_syntax(test_fn, r#"	return (n);"#, expect_value, "括弧付きreturn文");
	check_syntax(
		test_fn,
		r#"	return (n || m);"#,
		expect_value,
		"括弧付きreturn文",
	);
	check_syntax(
		test_fn,
		r#"	return ((n + 1) * (m + 1));"#,
		expect_value,
		"括弧付きreturn文",
	);
}
pub fn syntax_test_variable_def(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(test_fn, r#"	int a;"#, expect_value, "変数宣言");
	check_syntax(test_fn, r#"	unsigned long a;"#, expect_value, "変数宣言");
	check_syntax(test_fn, r#"	void *p;"#, expect_value, "ポインタ変数宣言");
	check_syntax(test_fn, r#"	const void *p;"#, expect_value, "定数宣言");
	check_syntax(test_fn, r#"	char s[12];"#, expect_value, "配列の変数宣言");
	check_syntax(test_fn, r#"	size_t a;"#, expect_value, "size_t型の変数宣言");
	check_syntax(test_fn, r#"	t_list list;"#, expect_value, "自作型の変数宣言");
	check_syntax(test_fn, r#"	t_list *l;"#, expect_value, "自作型の変数宣言");
}
pub fn syntax_test_if(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(test_fn, r#"	if(x < 100)"#, expect_value, "if文");
}
pub fn syntax_test_else_if(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(test_fn, r#"	else if(x < 100)"#, expect_value, "else if文");
}
pub fn syntax_test_else(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(test_fn, r#"	else"#, expect_value, "else文");
}
pub fn syntax_test_while(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(test_fn, r#"	while(*s != NULL)"#, expect_value, "while文");
}
pub fn syntax_test_continue(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(test_fn, r#"	continue;"#, expect_value, "continue文");
}
pub fn syntax_test_typedef(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(
		test_fn,
		r#"typedef struct point {"#,
		expect_value,
		"typedef文",
	);
}
pub fn syntax_test_break(test_fn: fn(line: &str) -> bool, expect_value: bool) {
	check_syntax(test_fn, r#"	break;"#, expect_value, "break文");
}
