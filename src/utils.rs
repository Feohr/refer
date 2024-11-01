/*
 * MIT License
 *
 * Copyright (c) 2024 Mohammed Rehaan and contributors
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * */

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
