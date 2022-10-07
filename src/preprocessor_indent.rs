use crate::c_parser;
use crate::util;

// ifndefに合わせてプリプロセッサ命令にインデントを挿入する
// 今のところインデントの深さは1のみ対応している
//  理由は単純に実際使われるときに深さ1以上で使われる事が少なくないから
pub fn format(file_texts: &Vec<String>) -> Vec<String> {
	let mut new_file_texts: Vec<String> = Vec::new();
	let mut is_in_ifndef_block: bool = false;
	for line in file_texts.iter() {
		if c_parser::is_ifndef(line) {
			is_in_ifndef_block = true;
		} else if c_parser::is_endif(line) {
			is_in_ifndef_block = false;
		} else if c_parser::is_preprocessor(line) && is_in_ifndef_block && line.chars().nth(1) != Some(' ') {
			new_file_texts.push(line.replacen("#", "# ", 1));
			continue;
		}
		new_file_texts.push(line.to_string());
	}
	new_file_texts
}

// TODO: test書く
