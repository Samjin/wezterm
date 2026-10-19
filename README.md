# Wez's Terminal

<img height="128" alt="WezTerm Icon" src="https://raw.githubusercontent.com/wezterm/wezterm/main/assets/icon/wezterm-icon.svg" align="left"> *A GPU-accelerated cross-platform terminal emulator and multiplexer written by <a href="https://github.com/wez">@wez</a> and implemented in <a href="https://www.rust-lang.org/">Rust</a>*

User facing docs and guide at: https://wezterm.org/

![Screenshot](docs/screenshots/two.png)

*Screenshot of wezterm on macOS, running vim*

## Installation

https://wezterm.org/installation

## WezTerm Space Development Build

This fork adds an always-visible space panel on the left side of the window. It
lists workspaces vertically and supports switching workspaces, creating a new
workspace with `+`, renaming a workspace with right-click, and resizing the
panel by dragging its right edge. The regular tab bar remains to the right of
the space panel.

### Build and run

Build the release binaries and make the launcher available on your `PATH`:

```sh
cargo build --release
mkdir -p /path/to/bin
ln -sfn /path/to/wezterm/scripts/wezterm-space /path/to/bin/wezterm-space
```

Replace `/path/to/wezterm` and `/path/to/bin` with your local paths. The
launcher can then be started with:

```sh
wezterm-space
```

The launcher uses the local overlay at
`$XDG_CONFIG_HOME/wezterm/wezterm-space.lua`, or
`$HOME/.config/wezterm/wezterm-space.lua` when `XDG_CONFIG_HOME` is unset.
The overlay is optional and should remain local when it contains machine-
specific paths or settings. Do not use `/private/tmp` for the permanent copy.

The `wezterm-space` CLI delegates GUI commands to a sibling `wezterm-gui` binary.
If you link `target/release/wezterm-space` directly instead of using the launcher,
link both release binaries into the same bin directory:

```sh
ln -sfn /path/to/wezterm/target/release/wezterm-space /path/to/bin/wezterm-space
ln -sfn /path/to/wezterm/target/release/wezterm-gui /path/to/bin/wezterm-gui
```

The launcher connects to the `space` Unix mux domain. `connect` starts a new
GUI client for that mux rather than reusing an official WezTerm GUI process.
`--class wezterm-space-dev` keeps the space window separate from
the official installation.

### Update from upstream

Save, commit, or stash any current work before rebasing. Then update the
space branch and rebuild:

```sh
git fetch upstream
git rebase upstream/main
cargo build --release
git push --force-with-lease origin wezterm-space
```

The launcher continues to use the rebuilt binary. If the branch is shared,
merge instead of rebasing and push normally:

```sh
git fetch upstream
git merge upstream/main
cargo build --release
git push origin wezterm-space
```

## Getting help

This is a spare time project, so please bear with me.  There are a couple of channels for support:

* You can use the [GitHub issue tracker](https://github.com/wezterm/wezterm/issues) to see if someone else has a similar issue, or to file a new one.
* Start or join a thread in our [GitHub Discussions](https://github.com/wezterm/wezterm/discussions); if you have general
  questions or want to chat with other wezterm users, you're welcome here!
* There is a [Matrix room via Element.io](https://matrix.to/#/#wezterm:matrix.org)
  for (potentially!) real time discussions.

The GitHub Discussions and Element/Gitter rooms are better suited for questions
than bug reports, but don't be afraid to use whichever you are most comfortable
using and we'll work it out.

## Supporting the Project

If you use and like WezTerm, please consider sponsoring it: your support helps
to cover the fees required to maintain the project and to validate the time
spent working on it!

[Read more about sponsoring](https://wezterm.org/sponsor.html).

* [![Sponsor WezTerm](https://img.shields.io/github/sponsors/wez?label=Sponsor%20WezTerm&logo=github&style=for-the-badge)](https://github.com/sponsors/wez)
* [Patreon](https://patreon.com/WezFurlong)
* [Ko-Fi](https://ko-fi.com/wezfurlong)
* [Liberapay](https://liberapay.com/wez)
