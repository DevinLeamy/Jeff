# *Jeff*

<p>
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#build">Build</a> •
  <a href="#notes">Notes</a>
</p>

***Jeff*** is a command line note management app, similar to Obsidian.


## Installation

#### ***Install with cargo:***

```bash
$ cargo install jf
```

## Usage

#### ***Create a vault using the following command:***

```bash
$ jf vault newvault ~/vaults 
```

Here, ***newvault*** is the name of the vault, and '***~/vault***' is the location where it will be created (this location should be an absolute fs path and exist already or `jeff` will throw an error).

Providing no arguments to `vault` command will list all vaults.

```bash
$ jf vault
```

Adding the '***-l***' flag will list all vaults with their locations. 

```bash
$ jf vault -l
```

At this moment only ***newvault*** will be listed.

#### ***Enter into the vault:***

```bash
$ jf enter newvault
```

`enter` command is also used to switch to other vaults.


#### ***Create notes and folders***

```bash
$ jf note newnote
```

```bash
$ jf folder newfolder
```

`note` and `folder`, both work similarly and create the corresponding items in ***current folder***. When a vault is first created, the ***current folder*** is set to its root.

#### ***Create templates***

To avoid having to write the boilerplate for things like weeks notes, `jeff` provides note templates.

```bash
# list all templates
$ jf template
```

```bash
# create and edit a new template "weekly_note"
$ jf template weekly_note
```

Notes can then be created using a template by supplying `--template (-t)` along with the name of the template.
```bash
# create a new note from the "weekly_note" template
$ jf note reflection --template weekly_note

# or equivalently
$ jf nt reflection -t weekly_note
```

#### ***Create and edit daily note in the current vault***

Daily notes are stored as `YYYY-MM-DD.md` at the top-level the current vault. `jf today` will edit any existing daily note. If no daily note exists, 
you will be prompted to create one.

```bash
# Create daily note YYYY-MM-DD? (y/n)
$ jf today 
```


#### ***Change folder***

```bash
$ jf chdir newfolder
```

`chdir` command will switch the current folder to the location mentioned. 
<br>
Relative path to location from current folder has to be provided. Standard fs paths are accepted as valid input, like `../folder1/somefolder/`.

```bash
$ jf chdir ..
```

This will switch back to the root of vault.

#### ***Print dir tree of current folder***

```bash
$ jf list
```

When needed `list` command will print the dir tree of current folder.
All notes will be highlighted in ***yellow***,
vaults will be highlighted in ***red***, and folders will appear ***blue***.

This is what the dir tree will look like with this vault's root as the current folder.

```bash
newvault        # red 
├── newfolder   # blue 
└── newnote     # yellow 
```
The highlight colors can be configured using `jf config <item-name>-color <color>`. Colors can also be set using `jf config <item-name>-color`, which displays a select containing all available options.


Set vault color to red
```bash
jf config vault-color red
```
Set folder color using selection
```bash
jf config folder-color
```


#### ***Fs operations***

Command `remove`  works as its name suggests, on all items (vaults, notes, or folders).

```
$ jf remove note newnote 
```

Commands `rename` and `move` are used similarly but take one additional argument each.

Command `rename` takes the new name as its third argument.

```bash
$ jf rename note newnote somenewnote
```

Command `move` takes the new location as its third argument.

For vaults, path rules are same as `vault` command and for other items, path rules are same as `chdir` command.

```bash
$ jf move note newnote /newfolder/
```

These commands take the item type (***vault***, ***note***, or ***folder***) as their first argument.

Command `vmove` is similar to `move`, but it moves an item (***note*** or ***folder***) from the current folder of the current vault to the root of a different vault, and takes the name of this vault as an argument in place of location.

```bash
$ jf vmove note newnote somevault 
```

Every keyword used so far (commands and item names) is interchangeable with its two letter alias, e.g. `move` command can also be written as:

```
$ jf mv nt newnote /newfolder/
```

#### ***Handle Jeff's config***

```bash
$ jf config 
```
Will display the current configuration. Add additional arguments, `jf config <config-type> <config-value` to set specific values 

```bash
$ jf config editor vim 
```

#### ***Get Help***

Run ***jf*** without a command, or with `help` command or ***-h*** flag for main help message.   

```bash
$ jf
```

Use `help` command or ***-h*** flag with a command to get corresponding help.

```
$ jf help vault
```

```
$ jf vault -h
```

### Build

Clone the repo and cd into the directory: 

```bash
$ git clone https://github.com/DevinLeamy/Jeff
$ cd Jeff
```

Run the following command to install dependencies and build/compile the program. 

```bash
$ cargo build 
```

Then run the executable created in '***target/debug/***' (or add it to your path).

Or, run the tool directly:

```bash
$ cargo run -- *args*
```

Pass in commands and arguments after '***--***'.

### Test

All tests can be run using 
```bash
$ cargo test -- --test-threads=1
```

## Notes
***Jeff*** was bootstrapped by [jot](https://crates.io/crates/jf), but eventually became a full rewrite with new features, a different design, and open-source maintenance in mind. Contributions are welcome!
