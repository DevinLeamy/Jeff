# jot

<a href="LICENSE"><img alt="MIT License" src="https://img.shields.io/apm/l/atomic-design-ui.svg?"></a>
<a href="https://github.com/araekiel/jot/releases/tag/v0.1.0"><img alt="Release" src="https://img.shields.io/badge/release-v0.1.0-red"></a>
<a href="https://crates.io/crates/jt"><img alt="Cargo" src="https://img.shields.io/badge/cargo-jt-blue"></a>

<p>
  <a href="#highlights">Highlights</a> •
  <a href="#notes">Notes</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#build-from-source">Build from Source</a> •
  <a href="#authors">Authors</a> •
  <a href="#license">License</a>
</p>


Jot is a CLI alternative for Obsidian built with rust.
<br>
While Obsidian is an excellent knowledge management tool, it proves to be overkill when all I want to do is jot down (pun intended) some notes and manage them locally, plus the GUI can often slow things down. Since Obsidian doesn't have a CLI client or an API that can enable the creation of one, I decided to make a fast and lightweight CLI alternative.
<br>
Jot is aimed at making note management, that can seem tedious on a GUI, (*in The Primeagen's voice*) blaaaazingly fast⚡.  

<img alt="Screenshot" src="assets/imgs/jot.png"/>

## Highlights
- Jot is under active development.
- Jot uses files and folders for notes and vaults just like Obsidian.
- Jot is aimed at reducing the time taken to perform tasks, so, each command is a two letter abbreviation of the word(s) that describe it.  

## Notes
- App data is stored in config and data files in locations generated by the [directories](https://crates.io/crates/directories) crate. It is advised that these files not be messed with, since atm there's no way to automatically fix them.
- App config has two fields: *editor* & *conflict*.
    - *editor* by default is set to *nvim* and conflict to *true*.
    - *conflict* field tells the app if the editor conflicts with it for control over the terminal. Set it to *true* for editors like *nvim* and *false* for editors like *notepad*.

## Installation

### With cargo

Run the following command to install jot with cargo:

```bash
$ cargo install jt
```

### v0.1.0 Executable Download

[Jot v0.1.0](https://github.com/araekiel/jot/releases/download/v0.1.0/jt.exe) (.exe)

## Commands

### `vl`

Use this command to create a vault.
<br>
This command needs an absolute path (as location for vault) that already exists or it will fail.

<img alt="jt-vl-cr" src="assets/gifs/jt-vl-cr.gif">

Pass in no arguments to get a list of vaults.

<img alt="jt-vl" src="assets/gifs/jt-vl.gif">

Use the *-l* flag to get vaults' locations.

<img alt="jt-vl-l" src="assets/gifs/jt-vl-l.gif">

<hr>

### `en`

Use this command to enter a vault.

<img alt="jt-en" src="assets/gifs/jt-en.gif">

<hr>

### `nt`

Use this command to create a note in current folder.

<img alt="jt-nt" src="assets/gifs/jt-nt.gif">

<hr>

### `op`

Use this command to open a note.

<img alt="jt-op" src="assets/gifs/jt-op.gif">

<hr>

### `fd`

Use this command to create a folder in current folder

<img alt="jt-fd" src="assets/gifs/jt-fd.gif">

<hr>

### `cd`

Use this command to change current folder.

<img alt="jt-cd" src="assets/gifs/jt-cd.gif">

<hr>

### `ls`

Use this command to print dir tree of the current folder

<img alt="jt-ls" src="assets/gifs/jt-ls.gif">

<hr>

### `rm`

Remove an item (*vl*, *nt* or *fd*).

<img alt="jt-rm" src="assets/gifs/jt-rm.gif">

<hr>

### `rn` 

Rename an item (*vl*, *nt* or *fd*).

<img alt="jt-rn" src="assets/gifs/jt-rn.gif">

<hr>

### `mv`

Move an item (*vl*, *nt* or *fd*).

<img alt="jt-mv" src="assets/gifs/jt-mv.gif">

<hr>

### `vm` 

Move an item (*nt* or *fd*) to a different vault.

<img alt="jt-vm" src="assets/gifs/jt-vm.gif">

<hr>

### `cf`

Use this command to get the value of a config field.

<img alt="jt-cf" src="assets/gifs/jt-cf.gif">

Pass in a value along with the above command to set the value of the config field.

<img alt="jt-cf-set" src="assets/gifs/jt-cf-set.gif">

<hr>

## Build from Source

### Prerequisites

- Git is need to clone the repository on your machine.
- Cargo is needed to compile the app.

### Installation & Configuration

Clone the repo and cd into the directory: 

```bash
$ git clone https://github.com/araekiel/jot.git
$ cd jot
```

Run the following command to install dependencies and build/compile the app. 

```bash
$ cargo build 
```

Then run the executable created in *target/debug/*.

Or, run the app directly:

```bash
$ cargo run -- *args*
```

Pass in commands and arguments after *'--'*.

## Authors

- **araekiel** - [Github](https://github.com/araekiel)

## License

[MIT License](https://github.com/araekiel/jot/blob/master/LICENSE) | Copyright (c) 2022 Kumar Shashwat
