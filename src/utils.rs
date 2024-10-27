///Returns greatest common prefix of two strings
fn gcp(s1: String, s2: String) -> String {
    let mut idx = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            break;
        }
        idx += 1
    }
    s1[..idx].to_string()
}

///Returns greatest common prefix of all strings from the vector that contain specified prefix
///If none of the strings contain specified prefix returns it
pub fn complete(mut filenames: Vec<String>, prefix: &String) -> String {
    filenames.retain(|s| s.starts_with(prefix));
    filenames
        .into_iter()
        .reduce(gcp)
        .unwrap_or(prefix.to_string())
}
