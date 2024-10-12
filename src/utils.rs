fn gcp(s1: String, s2: String) -> String {
    let ((_, _), idx) = s1
        .chars()
        .zip(s2.chars())
        .zip(0..s1.len())
        .find(|((c1, c2), _)| c1 != c2)
        .unwrap_or((('0', '0'), s1.len().min(s2.len())));
    s1[..idx].to_string()
}

pub fn complete(mut filenames: Vec<String>, prefix: &String) -> String {
    filenames.retain(|s| s.starts_with(prefix));
    filenames
        .into_iter()
        .reduce(gcp)
        .unwrap_or(prefix.to_string())
}
