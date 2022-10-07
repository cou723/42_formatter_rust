#![allow(unused)]
use std::env;
use std::error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use structopt::StructOpt;

mod brace_should_eol;
mod c_parser;
mod insert_newline_between_func;
mod no_args_void;
mod preprocessor_indent;
mod replace_tab_before_func;
mod return_parenthesis;
mod space_after_kw;
mod space_replace_tab_indent;
mod test_util;
mod util;

/**
 * やること一覧
 * 済 - NEWLINE_PROCESS_FUNCへの対応: functionの前に一ライン空ける
 * 未 - EMPTY_LINE_FUNCTION: function内の空行を消す
 * 済 - RETURN_PARENTHESISへの対応: returnの後に括弧をつける
 * 済 - NO_ARGS_VOID: 関数に引数がない場合にvoidをつける
 * 済 - SPACE_REPLACE_TAB: 変数宣言の変数の型と変数名の間に適切なtabを挿入しそろえる
 * 済 - SPACE_BEFORE_FUNC: 関数定義の返り値の型と関数名の間をtabにする
 * 済 - BRACE_SHOULD_EOL: 最終行が空行である必要があり、そうではない場合は空行を挿入する
 */

#[derive(StructOpt)]
struct Cli {
	#[structopt(parse(from_os_str))]
	path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn error::Error>> {
	// ファイルを読み込み一行ずつ分割したベクタ(file_texts)を作成
	let args = Cli::from_args();
	let mut file_texts: Vec<String> = Vec::new();
	for result in BufReader::new(File::open(&args.path)?).lines() {
		file_texts.push(result.unwrap().to_string());
	}

	let mut file_path = Path::new(&args.path);
	let mut formatted_file_text;
	// フォーマット
	match file_path.extension() {
		Some(x) => {
			if x == "c" {
				formatted_file_text = c_format(&file_texts);
			} else if x == "h" {
				formatted_file_text = h_format(&file_texts);
			} else {
				return Ok(());
			}
		}
		None => {
			return Ok(());
		}
	}

	// ファイル書き込み
	let mut file = File::create(&args.path).expect("could not create file");
	// 何かしらの不具合でformatted_file_textが空になった場合元のファイルを書き込む
	if formatted_file_text.len() == 0 {
		formatted_file_text = file_texts.join("\n");
	}
	Write::write_all(&mut file, &formatted_file_text.as_bytes()).expect("could not write file");
	Ok(())
}

// TODO: 全てのformat関数の引数を参照にする

fn c_format(file_texts: &Vec<String>) -> String {
	let mut formatted_file_texts = insert_newline_between_func::format(&file_texts);
	let mut formatted_file_texts = return_parenthesis::format(&formatted_file_texts);
	let mut formatted_file_texts = replace_tab_before_func::format(&formatted_file_texts);
	let mut formatted_file_texts = brace_should_eol::format(&formatted_file_texts);
	let mut formatted_file_texts = space_replace_tab_indent::format(&formatted_file_texts);
	let mut formatted_file_texts = no_args_void::format(&formatted_file_texts);
	let mut formatted_file_texts = space_after_kw::format(&formatted_file_texts);
	formatted_file_texts.join("\n")
}

fn h_format(file_texts: &Vec<String>) -> String {
	let mut formatted_file_texts = replace_tab_before_func::format_for_prototype(file_texts);
	let mut formatted_file_texts = preprocessor_indent::format(&formatted_file_texts);
	formatted_file_texts.join("\n")
}
