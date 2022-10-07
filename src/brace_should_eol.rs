pub fn format(file_texts: &Vec<String>) -> Vec<String> {
	let mut new_file_texts: Vec<String> = Vec::new();
	// 最後にちゃんと空行が入っている場合はtrue;
	if file_texts.last().unwrap().is_empty() {
		return file_texts.clone();
	} else {
		new_file_texts = file_texts.clone();
		new_file_texts.push("".to_string());
	}
	new_file_texts
}
