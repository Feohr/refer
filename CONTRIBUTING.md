# How to contribute

## :heart: **Firstly, thank you for deciding to contribute to the project**:grey_exclamation: :heart:

Please take a few moments and read the steps below to learn how to get started with your first contribution:

## Create a branch

Fork our repository into your own [_Github_](https://www.github.com) account, and create
a local clone. Once done and you have the code locally on the disk, you can get started. Try
not work directly on the [_main_](https://www.github.com/Feohr/refer/tree/main) branch,
but create a separate branch for each issue you are working on. There is no set rule on the
naming of branches but, it is good practice to add the intent of the branch in the prefix. For
example, a feature branch would be `feature/<feature-description>`. Correspondingly, **bugfix**,
**docs**, **hotfix** and **release** are valid branch prefixes.

## Style guide

To preface, these rules shall not be heavily imposed upon contributors.
They are entirely optional and with exception to general sensible practices can be ignored.

- Follow the [_rust style guidlines_](https://doc.rust-lang.org/nightly/style-guide/) or
simply use [_cargo fmt_](https://github.com/rust-lang/rustfmt) in combination with [_cargo
clippy_](https://doc.rust-lang.org/clippy/usage.html).

- When importing items to use within the module, ordering them accordinly helps maintain code readablity:

    ```rust
    // core std imports
    use std::fs::File;

    // external library imports
    use thiserror::Error;

    // local module imports
    use crate::io::FileList;
    ```

- It is always better to avoid over nesting. Anything beyond 4 levels of nesting
should require rewriting. There are several ways to avoid over-nesting.
Please have a look at this [`video`](https://www.youtube.com/watch?v=CFRhGnuXG-4) to learn more.

- Variable names should be short and understandable. They should explain what that variable
is used for. Try not to make them too short as well.

    ```rust
    // Variable names that are not ideal.
    let r = Resource::new();
    let rsrc = Resource::new();

    // Instead add an understandable variable name.
    let resource = Resource::new();
    ```
- One shouldn't require more than 2 words to describe a variable, wouldn't you agree? A
variable can acquire meaning based on the context hence, it is not necessary to make the
variable name too specific if the meaning can be drawn from the context easily.

    ```rust
    // This variable name would suffice given the context.
    let resource = Resource::new();
    // This kind of name could be counter-productive.
    let main_resource = Resource::new();

    // It is also better to use abbreviations if their meaning can be easily derived.
    // For example, instead of this:
    fn foo(resource: &Resource);
    // This is better to use if there are no other 'res' variables and it cannot be confused
    // for something else like `result`.
    fn foo(res: &Resource);
    ```

- You may not declare a function inside a function.

- Rust has some neat pattern matching syntax. Do use them.

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

- Using `expect` instead of `panic` is almost always better; Returning an error is the best way to handle issues.

- Not a strict requirement but you may add code comments
where necessary. In case of doc comments follow the [_rust doc comment
conventions_](https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#appendix-a-full-conventions-text). It is perfectly okay to not add any comments at all as long as your code is readable. If there are parts to your code that are unreadable i.e. require comments, you may be reminded during the PR. If you wish to write doc comments for the whole project, please go ahead by all means.

## Create pull request

Once done with the code, create a pull request and reference the issue that it addresses,
if any. Squash the commits to one and add a short, meaningful message.

## Submit a new issue

In case, you have found a bug in the software, have a new feature idea that you believe would improve refer or might want to tweak the documentation, please navigate to the [*issues*](https://github.com/Feohr/refer/issues) page and submit a new issue.

## Notice

Unless you explicitly state otherwise, any contribution submitted for inclusion in the work by you shall be licensed as stated in the [_project_](https://raw.githubusercontent.com/Feohr/refer/refs/heads/main/LICENSE), without any additional terms or conditions.
