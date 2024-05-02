<p align="center">
  <img src="public/logo.png" width="393" height="152" alt="sudo_f4">
</p>

# About <a href="https://www.rust-lang.org/"><img align="right" src="https://img.shields.io/badge/Rust-1.73-F74C00?logo=rust" alt="Rust 1.73" /></a>

You know how some games hook `Alt + F4` on purpose?

They say it's to 'prevent people from rage quitting'. Yeah, that's fucked.

I myself use the shortcut a lot. It's frustrating when it doesn't work.

Here's one of my [favorite comments](https://www.reddit.com/r/RocketLeague/comments/16jcpvw/comment/k27dmxl/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button) on this topic:

> Yeah everyone in the community automatically assumes mal intent and that it's people who want to rage quit mid-game.
>
> No, I just want to exit the game without having to use my mouse and click around on some buttons.
>
> It's easier and faster to alt-F4. That's why it's called a "keyboard shortcut".

Those that rage quit probably use a workaround like task manager anyway, so the hook is pointless.

I have also seen singleplayer games that hook it, so the whole "ruining the experience of other players" argument is invalid in those cases.

`Alt + F4` is an explicit request - the user wants the program to close immediately.

---
⚠ **To hook `Alt + F4` is to assume you understand what the user wants more than the user does.** ⚠
---

Therefore, I present to you **SudoF4**.

# Yay!

No more shitty hooks. Just press `Win + F4` and the current window will be sent into the depths of the void: completely and utterly.

Despite its name, SudoF4 only works on Windows.

Yeah the code ain't special, but it's not about that.

It's about the message. **No one wants shitty `Alt + F4` hooks!**

# Installation

You first need to install [Rust](https://www.rust-lang.org/).

Then clone and install SudoF4:

```sh
git clone https://github.com/UnexomWid/SudoF4

cd SudoF4

cargo install --path .
```

# Usage

Open a new cmd window and run:

```sh
sudo_f4
```

And now you can `Win + F4` all day!

You have to keep SudoF4 running in the background for the shortcut to work.

If it fails to kill some programs, try running cmd as admin.

# License <a href="https://github.com/UnexomWid/SudoF4/blob/master/LICENSE"><img align="right" src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT" /></a>

**SudoF4** was created by [UnexomWid](https://uw.exom.dev). It is licensed under the [MIT](https://github.com/UnexomWid/SudoF4/blob/master/LICENSE) license.