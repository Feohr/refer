<div>
    <p align="center">
        <!-- <picture>
            <source media="(prefers-color-scheme: dark)" srcset="docs/logo/ReferLogoNameDark.svg" height=125px>
            <source media="(prefers-color-scheme: light)" srcset="docs/logo/ReferLogoName.svg" height=125px>
            <img alt="Logo Name" src="docs/logo/ReferLogoNameDark.svg" height=125px>
        </picture> -->
        <img alt="Logo Name" src="docs/logo/ReferLogoNameDark.svg" height=125px>
        <img src="docs/logo/ReferLogoWithBGx250.svg" height=125px>
    </p>
    <p align="center">
        <a href="https://github.com/Feohr/refer/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/badge/License-MIT-green" height=24px></a>
        <a href="https://github.com/Feohr/refer"><img alt="Built with rust" src="https://img.shields.io/badge/built_with-rust-orange" height=24px></a>
        &nbsp;
        &nbsp;
        &nbsp;
        <a href="mailto:mohammedrehaan.work@gmail.com"><img src="./docs/social/gmail.svg" height=32px></a>
        <a href="https://github.com/Feohr"><img src="./docs/social/github.svg" height=32px></a>
        <a href="https://www.linkedin.com/in/mohammed-rehaan-193305222/"><img src="./docs/social/linkedin.svg" height=32px></a>
    </p>
</div>

A TUI app to open, read and tail multiple text files at once. Refer also provides convenient key shortcuts to make navigation simple and fast.</p>

![](docs/ReferDemo.gif)

## Installation

Before you start make sure you have rust installed on your system https://www.rust-lang.org/tools/install. Once that is done, then proceed with cloning the repository.

```console
$ git clone --depth=1 git@github.com:Feohr/refer.git
```
Navigate into the `refer` folder and run the release build command.

```console
$ cargo build --release
```

The binary should be present under the target folder `target/release/refer`. Go ahead and save this binary in the system `bin` folder if you wish to.

## Usage

Run directly via the terminal. The binary expects a space separated array of file paths as arguments.

### Key bindings

|       Keys        |                   Action              |
|-------------------|---------------------------------------|
| `ctrl + q`        | quit the app.                         |
| `ctrl + n`        | add a new file.                       |
| `ctrl + d`        | delete a file.                        |
| `(j or ↑)`        | move up the file buffer.              |
| `(k or ↓)`        | move down the file buffer             |
| `ctrl + (j or ↑)` | move to the top of the file buffer.   |
| `ctrl + (k or ↓)` | move to the bottom of the file buffer |
| `ctrl + t`        | toggle tail mode                      |
| `(h or ←)`        | switch to file list                   |
| `(l or →)`        | switch to file buffer                 |
