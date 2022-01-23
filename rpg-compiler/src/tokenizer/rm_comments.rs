use regex::Regex;

pub fn rm_comments(code: &str) -> String {
    let lines = code.lines();
    let comment_regex = Regex::new("#").unwrap();
    let uncommented = lines.into_iter().map(|line| {
        let mut l = line;
        if let Some(index) = comment_regex.find(line) {
            l = &l[0..index.start()];
        }
        l.to_string()
    }).collect::<Vec<String>>().join("\n");
    uncommented
}