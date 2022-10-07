use crate::c_parser;
use crate::util;
/**
 * HACK:functionに関する構造体を作成したほうがいいかもしれない
 * あったほうがいい属性
 * - indentの深さ
 * - 関数名
 * - 関数で使われる変数名
 * - 本文
**/

/**
 * <<C言語の文法判定方法>>
 * まず大前提として、読み込むC言語はある程度normに準拠している前提なので、forや三項演算子,gotoは使われていない前提
 * 決まった予約語から始まるもの(if,return,break)は判定がしやすい
 * もう一つ、プリプロセッサ命令は判定がしやすい
 * 残るのは関数宣言、変数宣言、関数呼び出し、変数代入
 * - 基本型or自作型(normのルールに準拠していればt_*で始まる)orその他の型(*_t,size_tなど)で始まるのが関数宣言、変数宣言
 * - - 上記二つは()の有無で判定できる（はず）
 * - - 比較的変数宣言の判定のほうが簡単にできる
 * - 上記に該当しない単語で始まるのが関数呼び出し、変数代入
 * - - 関数呼び出しは関数名と(が隣接していて、それに該当しないものは全て変数代入として扱う
 *
 * 判定順序（判定難易度）
 * 1. return,if,else if,else,while,continue,typedef,break,プリプロセッサ命令
 * 2. 変数宣言
 * 3. 関数宣言,プロトタイプ宣言
 * 4. 変数代入,関数呼び出し
 */

/**
 * 関数とは
 * - 型から始まり、最後の;が存在しない
 */
pub fn is_func_def(maybe_func_def: &str) -> bool {
	// 関数宣言より簡単に判定できる文法をあらかじめガードする
	if is_not_func_def(maybe_func_def) {
		return false;
	}
	// 残るのは変数宣言、関数定義、プロトタイプ宣言、関数呼び出し、変数代入
	let maybe_func_def = maybe_func_def.to_string();
	let first_word = &maybe_func_def.replace("\t", " ");
	let first_word = first_word.split(' ').nth(0).unwrap();
	if !maybe_func_def.contains(";")
		&& (util::c_expected_words_before_func().contains(&first_word.to_string())
			|| c_parser::is_custom_type(first_word)
			|| c_parser::is_original_type(first_word))
	{
		return true;
	}
	false
}

pub fn is_prototype_declare(maybe_prototype_declare: &str) -> bool {
	// 関数宣言より簡単に判定できる文法をあらかじめガードする
	if is_not_func_def(maybe_prototype_declare) {
		return false;
	}
	// 残るのは変数宣言、関数定義、プロトタイプ宣言、関数呼び出し、変数代入
	let maybe_prototype_declare = maybe_prototype_declare.to_string();
	let first_word = &maybe_prototype_declare.replace("\t", " ");
	let first_word = first_word.split(' ').nth(0).unwrap();
	if maybe_prototype_declare.contains(";")
		&& (util::c_expected_words_before_func().contains(&first_word.to_string())
			|| c_parser::is_custom_type(first_word)
			|| c_parser::is_original_type(first_word))
	{
		return true;
	}
	false
}

// TODO: 否定形の関数名はよくない
fn is_not_func_def(line: &str) -> bool {
	return is_variable_def(line)
		|| is_return(line)
		|| is_preprocessor(line)
		|| is_if(line)
		|| is_else_if(line)
		|| is_else(line)
		|| is_while(line)
		|| is_continue(line)
		|| is_typedef(line)
		|| is_break(line);
}

// " "でスプリットした時最後の要素以外がC_KEYWORDSである=変数定義である
pub fn is_variable_def(line: &str) -> bool {
	if line.matches(" ").count() == 0 || is_not_variable_def(line) {
		return false;
	}
	let mut word_count = line.matches(" ").count();
	let mut line = line.trim();
	for (i, el) in line.split(" ").enumerate() {
		// 最後の要素の場合
		if i == word_count {
			if el.ends_with(";") && !el.contains("(") && !el.contains(")") {
				return true;
			} else {
				return false;
			}
		// 最後以外の要素の場合
		} else {
			if !(util::C_PRIMITIVE_TYPES.contains(&el)
				|| el == "const" || c_parser::is_custom_type(el)
				|| c_parser::is_original_type(el))
			{
				return false;
			}
		}
	}
	true
}

fn is_not_variable_def(line: &str) -> bool {
	return is_return(line)
		|| is_preprocessor(line)
		|| is_if(line)
		|| is_else_if(line)
		|| is_else(line)
		|| is_while(line)
		|| is_continue(line)
		|| is_typedef(line)
		|| is_break(line);
}

pub fn is_preprocessor(line: &str) -> bool {
	if line.starts_with("#") {
		return true;
	}
	false
}

// MEMO: 内容自体は短いが、再利用性があるため別関数にしている
// NOTE: /**/のタイプのコメントアウトには対応していないが実装が難しいため未対応
pub fn is_comment_out(line: &str) -> bool {
	if line.trim().starts_with("//") || line.trim().starts_with("/*") {
		return true;
	}
	false
}

// NOTE: else ifは判定しない
pub fn is_if(line: &str) -> bool {
	return line.trim().starts_with("if");
}

pub fn is_else_if(line: &str) -> bool {
	return line.trim().starts_with("else if");
}

pub fn is_else(line: &str) -> bool {
	return line.trim() == ("else");
}

pub fn is_typedef(line: &str) -> bool {
	return line.trim().starts_with("typedef");
}

pub fn is_break(line: &str) -> bool {
	return line.trim().starts_with("break");
}

pub fn is_for(line: &str) -> bool {
	return line.trim().starts_with("for");
}

pub fn is_switch(line: &str) -> bool {
	return line.trim().starts_with("switch");
}

pub fn is_while(line: &str) -> bool {
	return line.trim().starts_with("while");
}

pub fn is_do(line: &str) -> bool {
	return line.trim().starts_with("do");
}

pub fn is_return(line: &str) -> bool {
	return line.trim().starts_with("return");
}

pub fn is_continue(line: &str) -> bool {
	return line.trim().starts_with("continue");
}

pub fn is_ifndef(line: &str) -> bool {
	return line.trim().starts_with("#ifndef");
}

pub fn is_endif(line: &str) -> bool {
	return line.trim().starts_with("#endif");
}

pub fn is_custom_type(line: &str) -> bool {
	for c in line.chars() {
		if !(c.is_ascii_alphabetic() || c == '_') {
			return false;
		}
	}
	return line.trim().ends_with("_t");
}

pub fn is_original_type(line: &str) -> bool {
	for c in line.chars() {
		if !(c.is_ascii_alphabetic() || c == '_') {
			return false;
		}
	}
	return line.trim().starts_with("t_");
}

// 与えられたファイルを関数ごとに分割して返す関数
// HACK: 関数長すぎ
// TODO: 未使用関数のため削除要検討
pub fn split_per_func(file_texts: &Vec<String>) -> Vec<Vec<String>> {
	let mut splitted_per_func_lines: Vec<Vec<String>> = Vec::new();
	// HACK: フラグを使わない方法があればそっちの方がよさそう
	let mut is_func_scope = false;
	let mut is_in_statement = false;
	for line in file_texts.iter() {
		if is_comment_out(&line) {
			continue;
		}
		if is_func_def(&line) {
			is_func_scope = true;
			splitted_per_func_lines.push(vec![line.to_string()]);
		} else if is_if(&line)
			|| is_else_if(&line)
			|| is_for(&line)
			|| is_while(&line)
			|| is_switch(&line)
			|| is_do(&line)
		{
			is_in_statement = true;
		} else if is_func_scope {
			splitted_per_func_lines
				.last_mut()
				.unwrap()
				.push(line.to_string());
			// 関数の終わりだった場合フラグ整理
			// HACK: ifの入れ子はよくない
			if line.trim().starts_with("}") {
				if is_in_statement {
					is_in_statement = false;
				} else {
					is_func_scope = false;
				}
			}
		} else {
			continue;
		}
	}
	splitted_per_func_lines
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_util;

	#[test]
	fn test_is_variable_def() {
		test_util::syntax_test_include(is_variable_def, false);
		test_util::syntax_test_define(is_variable_def, false);
		test_util::syntax_test_ifdef(is_variable_def, false);
		test_util::syntax_test_func_def(is_variable_def, false);
		test_util::syntax_test_prototype_declare(is_variable_def, false);
		test_util::syntax_test_func_call(is_variable_def, false);
		test_util::syntax_test_return(is_variable_def, false);
		test_util::syntax_test_variable_def(is_variable_def, true);
		test_util::syntax_test_if(is_variable_def, false);
		test_util::syntax_test_else_if(is_variable_def, false);
		test_util::syntax_test_else(is_variable_def, false);
		test_util::syntax_test_while(is_variable_def, false);
		test_util::syntax_test_continue(is_variable_def, false);
		test_util::syntax_test_typedef(is_variable_def, false);
		test_util::syntax_test_break(is_variable_def, false);
	}
	#[test]
	fn test_is_func_def() {
		test_util::syntax_test_include(is_func_def, false);
		test_util::syntax_test_define(is_func_def, false);
		test_util::syntax_test_ifdef(is_func_def, false);
		test_util::syntax_test_func_def(is_func_def, true);
		test_util::syntax_test_prototype_declare(is_func_def, false);
		test_util::syntax_test_func_call(is_func_def, false);
		test_util::syntax_test_return(is_func_def, false);
		test_util::syntax_test_variable_def(is_func_def, false);
		test_util::syntax_test_if(is_func_def, false);
		test_util::syntax_test_else_if(is_func_def, false);
		test_util::syntax_test_else(is_func_def, false);
		test_util::syntax_test_while(is_func_def, false);
		test_util::syntax_test_continue(is_func_def, false);
		test_util::syntax_test_typedef(is_func_def, false);
		test_util::syntax_test_break(is_func_def, false);
	}
	#[test]
	fn test_is_prototype_declare() {
		test_util::syntax_test_include(is_prototype_declare, false);
		test_util::syntax_test_define(is_prototype_declare, false);
		test_util::syntax_test_ifdef(is_prototype_declare, false);
		test_util::syntax_test_func_def(is_prototype_declare, false);
		test_util::syntax_test_prototype_declare(is_prototype_declare, true);
		test_util::syntax_test_func_call(is_prototype_declare, false);
		test_util::syntax_test_return(is_prototype_declare, false);
		test_util::syntax_test_variable_def(is_prototype_declare, false);
		test_util::syntax_test_if(is_prototype_declare, false);
		test_util::syntax_test_else_if(is_prototype_declare, false);
		test_util::syntax_test_else(is_prototype_declare, false);
		test_util::syntax_test_while(is_prototype_declare, false);
		test_util::syntax_test_continue(is_prototype_declare, false);
		test_util::syntax_test_typedef(is_prototype_declare, false);
		test_util::syntax_test_break(is_prototype_declare, false);
	}
	#[test]
	fn test_is_return() {
		test_util::syntax_test_include(is_return, false);
		test_util::syntax_test_define(is_return, false);
		test_util::syntax_test_ifdef(is_return, false);
		test_util::syntax_test_func_def(is_return, false);
		test_util::syntax_test_prototype_declare(is_return, false);
		test_util::syntax_test_func_call(is_return, false);
		test_util::syntax_test_return(is_return, true);
		test_util::syntax_test_variable_def(is_return, false);
		test_util::syntax_test_if(is_return, false);
		test_util::syntax_test_else_if(is_return, false);
		test_util::syntax_test_else(is_return, false);
		test_util::syntax_test_while(is_return, false);
		test_util::syntax_test_continue(is_return, false);
		test_util::syntax_test_typedef(is_return, false);
		test_util::syntax_test_break(is_return, false);
	}
	#[test]
	fn test_is_preprocessor() {
		test_util::syntax_test_include(is_preprocessor, true);
		test_util::syntax_test_define(is_preprocessor, true);
		test_util::syntax_test_ifdef(is_preprocessor, true);
		test_util::syntax_test_func_def(is_preprocessor, false);
		test_util::syntax_test_prototype_declare(is_preprocessor, false);
		test_util::syntax_test_func_call(is_preprocessor, false);
		test_util::syntax_test_return(is_preprocessor, false);
		test_util::syntax_test_variable_def(is_preprocessor, false);
		test_util::syntax_test_if(is_preprocessor, false);
		test_util::syntax_test_else_if(is_preprocessor, false);
		test_util::syntax_test_else(is_preprocessor, false);
		test_util::syntax_test_while(is_preprocessor, false);
		test_util::syntax_test_continue(is_preprocessor, false);
		test_util::syntax_test_typedef(is_preprocessor, false);
		test_util::syntax_test_break(is_preprocessor, false);
	}
	#[test]
	fn test_is_if() {
		test_util::syntax_test_include(is_if, false);
		test_util::syntax_test_define(is_if, false);
		test_util::syntax_test_ifdef(is_if, false);
		test_util::syntax_test_func_def(is_if, false);
		test_util::syntax_test_prototype_declare(is_if, false);
		test_util::syntax_test_func_call(is_if, false);
		test_util::syntax_test_return(is_if, false);
		test_util::syntax_test_variable_def(is_if, false);
		test_util::syntax_test_if(is_if, true);
		test_util::syntax_test_else_if(is_if, false);
		test_util::syntax_test_else(is_if, false);
		test_util::syntax_test_while(is_if, false);
		test_util::syntax_test_continue(is_if, false);
		test_util::syntax_test_typedef(is_if, false);
		test_util::syntax_test_break(is_if, false);
	}
	#[test]
	fn test_is_else_if() {
		test_util::syntax_test_include(is_else_if, false);
		test_util::syntax_test_define(is_else_if, false);
		test_util::syntax_test_ifdef(is_else_if, false);
		test_util::syntax_test_func_def(is_else_if, false);
		test_util::syntax_test_prototype_declare(is_else_if, false);
		test_util::syntax_test_func_call(is_else_if, false);
		test_util::syntax_test_return(is_else_if, false);
		test_util::syntax_test_variable_def(is_else_if, false);
		test_util::syntax_test_if(is_else_if, false);
		test_util::syntax_test_else_if(is_else_if, true);
		test_util::syntax_test_else(is_else_if, false);
		test_util::syntax_test_while(is_else_if, false);
		test_util::syntax_test_continue(is_else_if, false);
		test_util::syntax_test_typedef(is_else_if, false);
		test_util::syntax_test_break(is_else_if, false);
	}
	#[test]
	fn test_is_else() {
		test_util::syntax_test_include(is_else, false);
		test_util::syntax_test_define(is_else, false);
		test_util::syntax_test_ifdef(is_else, false);
		test_util::syntax_test_func_def(is_else, false);
		test_util::syntax_test_prototype_declare(is_else, false);
		test_util::syntax_test_func_call(is_else, false);
		test_util::syntax_test_return(is_else, false);
		test_util::syntax_test_variable_def(is_else, false);
		test_util::syntax_test_if(is_else, false);
		test_util::syntax_test_else_if(is_else, false);
		test_util::syntax_test_else(is_else, true);
		test_util::syntax_test_while(is_else, false);
		test_util::syntax_test_continue(is_else, false);
		test_util::syntax_test_typedef(is_else, false);
		test_util::syntax_test_break(is_else, false);
	}
	#[test]
	fn test_is_while() {
		test_util::syntax_test_include(is_while, false);
		test_util::syntax_test_define(is_while, false);
		test_util::syntax_test_ifdef(is_while, false);
		test_util::syntax_test_func_def(is_while, false);
		test_util::syntax_test_prototype_declare(is_while, false);
		test_util::syntax_test_func_call(is_while, false);
		test_util::syntax_test_return(is_while, false);
		test_util::syntax_test_variable_def(is_while, false);
		test_util::syntax_test_if(is_while, false);
		test_util::syntax_test_else_if(is_while, false);
		test_util::syntax_test_else(is_while, false);
		test_util::syntax_test_while(is_while, true);
		test_util::syntax_test_continue(is_while, false);
		test_util::syntax_test_typedef(is_while, false);
		test_util::syntax_test_break(is_while, false);
	}
	#[test]
	fn test_is_continue() {
		test_util::syntax_test_include(is_continue, false);
		test_util::syntax_test_define(is_continue, false);
		test_util::syntax_test_ifdef(is_continue, false);
		test_util::syntax_test_func_def(is_continue, false);
		test_util::syntax_test_prototype_declare(is_continue, false);
		test_util::syntax_test_func_call(is_continue, false);
		test_util::syntax_test_return(is_continue, false);
		test_util::syntax_test_variable_def(is_continue, false);
		test_util::syntax_test_if(is_continue, false);
		test_util::syntax_test_else_if(is_continue, false);
		test_util::syntax_test_else(is_continue, false);
		test_util::syntax_test_while(is_continue, false);
		test_util::syntax_test_continue(is_continue, true);
		test_util::syntax_test_typedef(is_continue, false);
		test_util::syntax_test_break(is_continue, false);
	}
	#[test]
	fn test_is_typedef() {
		test_util::syntax_test_include(is_typedef, false);
		test_util::syntax_test_define(is_typedef, false);
		test_util::syntax_test_ifdef(is_typedef, false);
		test_util::syntax_test_func_def(is_typedef, false);
		test_util::syntax_test_prototype_declare(is_typedef, false);
		test_util::syntax_test_func_call(is_typedef, false);
		test_util::syntax_test_return(is_typedef, false);
		test_util::syntax_test_variable_def(is_typedef, false);
		test_util::syntax_test_if(is_typedef, false);
		test_util::syntax_test_else_if(is_typedef, false);
		test_util::syntax_test_else(is_typedef, false);
		test_util::syntax_test_while(is_typedef, false);
		test_util::syntax_test_continue(is_typedef, false);
		test_util::syntax_test_typedef(is_typedef, true);
		test_util::syntax_test_break(is_typedef, false);
	}
	#[test]
	fn test_is_break() {
		test_util::syntax_test_include(is_break, false);
		test_util::syntax_test_define(is_break, false);
		test_util::syntax_test_ifdef(is_break, false);
		test_util::syntax_test_func_def(is_break, false);
		test_util::syntax_test_prototype_declare(is_break, false);
		test_util::syntax_test_func_call(is_break, false);
		test_util::syntax_test_return(is_break, false);
		test_util::syntax_test_variable_def(is_break, false);
		test_util::syntax_test_if(is_break, false);
		test_util::syntax_test_else_if(is_break, false);
		test_util::syntax_test_else(is_break, false);
		test_util::syntax_test_while(is_break, false);
		test_util::syntax_test_continue(is_break, false);
		test_util::syntax_test_typedef(is_break, false);
		test_util::syntax_test_break(is_break, true);
	}

	#[test]
	fn test_is_custom_type() {
		assert_eq!(is_custom_type("int"), false);
		assert_eq!(is_custom_type("size_t"), true);
		assert_eq!(is_custom_type("t_list"), false);
	}

	#[test]
	fn test_is_original_type() {
		assert_eq!(is_original_type("int"), false);
		assert_eq!(is_original_type("size_t"), false);
		assert_eq!(is_original_type("t_list"), true);
	}

	#[test]
	fn test_split_per_func() {
		let (expected_vec, actual_vec) = variables_set1();
		//println!("actual_vec:{:?}", actual_vec);
		let buf = split_per_func(&actual_vec);
		//println!("buf:{:?}", buf);
		assert_eq!(buf, expected_vec);
	}

	fn variables_set1() -> (Vec<Vec<String>>, Vec<String>) {
		let mut expected_vec: Vec<Vec<String>> = Vec::new();
		expected_vec.push(
			vec![
				"int	is_number(char c)",
				"{",
				"	return ((c >= '0') && (c <= '9'));",
				"}",
			]
			.iter()
			.map(|s| s.to_string())
			.collect(),
		);

		expected_vec.push(
			vec![
				"int	is_p_m(char c)",
				"{",
				"	return (c == '+' || c == '-');",
				"}",
			]
			.iter()
			.map(|s| s.to_string())
			.collect(),
		);

		let actual_vec: Vec<String> = r#"include<"stdio.h">

int	is_number(char c)
{
	return ((c >= '0') && (c <= '9'));
}

int	is_p_m(char c)
{
	return (c == '+' || c == '-');
}
	"#
		.split("\n")
		.map(|x| x.to_string())
		.collect();

		(expected_vec, actual_vec)
	}
}
