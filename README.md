<p align="center">
  <img src="public/logo.png" width="393" height="152" alt="sudo_f4">
</p>

# About <a href="https://www.rust-lang.org/"><img align="right" src="https://img.shields.io/badge/Rust-1.63-F74C00?logo=rust" alt="Rust 1.63" /></a>

You know how some games hook `Alt + F4` on purpose?

They say it's to 'prevent people from rage quitting'. Yeah, that's fucked.

Most people just use it as a shortcut, and it's frustrating when it doesn't work.

Those that rage quit probably use task manager anyway, so the hook is pointless.

`Alt + F4` is an explicit request - the user wants the program to close immediately.

To hook `Alt + F4` is to assume you understand what the user wants more than the user does.

Therefore, I present to you **SudoF4**.

# Yay!

No more shitty hooks. Just press `Win + F4` and the current window will be sent into the depths of the void: completely and utterly.

Despite its name, SudoF4 only works on Windows.

Yeah the code ain't great, but it's not about that.

It's about the message. No one wants shitty `Alt + F4` hooks!

# Installation

You first need to install [Rust](https://www.rust-lang.org/).

Then clone/download SudoF4 and run this in the dir:

```sh
cargo install --path .
```

# Usage

Open a new cmd window and run:

```sh
sudo_f4
```

And now you can `Win + F4` all day!

You have to keep SudoF4 running in the background for the shortcut to work.

# License <a href="https://github.com/UnexomWid/SudoF4/blob/master/LICENSE"><img align="right" src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT" /></a>

**SudoF4** was created by [UnexomWid](https://uw.exom.dev). It is licensed under the [MIT](https://github.com/UnexomWid/SudoF4/blob/master/LICENSE) license.