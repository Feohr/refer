# How to contribute

## Create a branch

Firstly fork our repository into your own [_Github_](https://www.github.com) account, and create
a local clone. Once done and you have the code locally on the disk, you can get started. Try
not work directly on the [_main_](https://www.github.com/Feohr/refer/tree/main) branch,
but create a separate branch for each issue you are working on. There is no set rule on the
naming of branches but, it is good practice to add the intent of the branch in the prefix. For
example, a feature branch would be `feature/<feature-description>`. Correspondingly, **bugfix**,
**docs**, **hotfix** and **release** are valid branch prefixes. Notice how there is no use of `_`
or `camelCase`-ing in the branch name, only `-`.

## Style guide

- Follow the [_rust style guidlines_](https://doc.rust-lang.org/nightly/style-guide/) or
simply use [_cargo fmt_](https://github.com/rust-lang/rustfmt) in combination with [_cargo
clippy_](https://doc.rust-lang.org/clippy/usage.html).

- When importing items to use within the module, order them accordinly:

    ```rust
    // core std imports
    use std::fs::File;

    // external library imports
    use thiserror::Error;

    // local module imports
    use crate::io::FileList;
    ```

- Avoid over nesting. Anything beyond 4 levels of nesting requires
rewriting. There are several ways to avoid over-nesting. Have a look at this
[`video`](https://www.youtube.com/watch?v=CFRhGnuXG-4) to learn more.

- Keep variable names short and understandable. Variable name should explain what that variable
is used for. Don't make variable name too short.

    ```rust
    // Bad variable names.
    let r = Resource::new();
    let rsrc = Resource::new();

    // Instead add an understandable variable name.
    let resource = Resource::new();
    ```
- If you require more than 2 words to describe a variable, refactor and change the name. A
variable can acquire meaning based on the context hence, it is not necessary to make the
variable name too specific if the meaning can be drawn from context easily.

    ```rust
    // This variable name would suffice given the context.
    let resource = Resource::new();
    // This kind of name would be counter-productive.
    let main_resource = Resource::new();

    // It is also better to use abbreviations if their meaning can be easily derived.
    // For example, instead of this:
    fn foo(resource: &Resource);
    // This is better to use if there are no other 'res' variables and it cannot be confused
    // for something else like `result`.
    fn foo(res: &Resource);
    ```

- **DO NOT** declare a function inside a function. Just... why?

- Rust has some neat pattern matching syntax. Use it.

    ```rust
    // Instead of this.
    let x = positions.x;
    let y = positions.y;

    // This is better.
    let (x, y) = positions;
    ```
    ```rust
    // Instead of this.
    let x = match result {
        Ok(value) => value,
        Err(_) => return,
    };

    // This is better.
    let Ok(x) = result else {
        return
    };
    ```
    ```rust
    // Instead of this.
    if y.is_some() {
        let y = y.expect("Error while accessing Option value on y");
        ...
    }

    // This is better.
    if let Some(y) = y {
        ...
    }
    ```

- Avoid panicking. Use `expect` at the very least. Return error when possible.

- A PR should not have any warnings and no `todo` macros.

- Not a requirement but you can add code comments
where necessary. In case of doc comments follow the [_rust doc comment
conventions_](https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#appendix-a-full-conventions-text).
It is perfectly okay to not add any comments at all as long as your code is readable. If there
are parts to your code that are unreadable i.e. require comments, you would be informed during
the PR. If you wish to write doc comments for the whole project, please go ahead by all means.

## Create pull request

Once done with the code, create a pull request and reference the issue that it addresses,
if any. Squash the commits to one and add a short, meaningful message.

Issues can have either `patch` or `minor` flag associated with them; which imposes upon the
contributor the responsibility to increment the corresponding version in `Cargo.toml` adhering
to the [_SemVer_](https://semver.org) convention.

## Notice

Unless you explicitly state otherwise, any contribution submitted for inclusion in the work
by you shall be licensed as stated in the
[_project_](https://raw.githubusercontent.com/Feohr/refer/refs/heads/main/LICENSE), without
any additional terms or conditions.
