// break;をbreak ;にする
// return;をreturn ;にする
pub fn format(file_texts: &Vec<String>) -> Vec<String> {
	let mut new_file_texts: Vec<String> = Vec::new();
	for line in file_texts.iter() {
		if line.trim() == "break;" || line.trim() == "return;" || line.trim() == "continue;" {
			new_file_texts.push(line.replace(";", " ;"));
		} else {
			new_file_texts.push(line.to_string());
		}
	}
	new_file_texts
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_util;

	#[test]
	fn test_format() {
		let (file_texts, expected_texts) = test_util::load_text_from_file("space_after_kw", 0);
		assert_eq!(
			format(&file_texts),
			expected_texts.clone(),
			"id: {}のテストが失敗しました",
			0
		);
	}
}
