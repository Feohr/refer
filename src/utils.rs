///Returns greatest common prefix of two strings
fn gcp(s1: String, s2: String) -> String {
    if let Some((idx, (_, _))) = s1
        .chars()
        .zip(s2.chars())
        .enumerate()
        .find(|(_, (c1, c2))| c1 != c2)
    {
        s1[..idx].to_string()
    } else {
        s1[..s1.len().min(s2.len())].to_string()
    }
}

///Returns greatest common prefix of all strings from the vector that contain specified prefix
///If none of the strings contain specified prefix returns prefix
pub fn complete(mut filenames: Vec<String>, prefix: &String) -> String {
    filenames.retain(|s| s.starts_with(prefix));
    filenames
        .into_iter()
        .reduce(gcp)
        .unwrap_or(prefix.to_string())
}
