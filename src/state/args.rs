use crate::enums::{ConfigType, Item as ItemType, VaultItem as VaultItemType};
use clap::{AppSettings, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(global_setting(AppSettings::HidePossibleValuesInHelp))]
#[clap(global_setting(AppSettings::DontCollapseArgsInUsage))]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::ColorNever))]
#[clap(help_template("\x1b[0;34m________      _____ 
______(_)_______  /_
_____  /_  __ \\  __/
____  / / /_/ / /_  
___  /  \\____/\\__/  
/___/
\x1b[0m

\x1b[0;34mv0.2.1\x1b[0m | crafted with ❤️ by \x1b[0;34maraekiel\x1b[0m and \x1b[0;34mdevinleamy\x1b[0m


usage: jt <command>

\x1b[0;34mcommands:\x1b[0m

create items
    \x1b[0;34mvault\x1b[0m, \x1b[0;34mvl\x1b[0m       create a vault or list vaults
    create items in current folder
        \x1b[0;34mnote\x1b[0m, \x1b[0;34mnt\x1b[0m        create a note 
        \x1b[0;34mfolder\x1b[0m, \x1b[0;34mfd\x1b[0m      create a folder

interact with items
    \x1b[0;34menter\x1b[0m, \x1b[0;34men\x1b[0m       enter a vault
    \x1b[0;34mopen\x1b[0m, \x1b[0;34mop\x1b[0m        open a note from current folder
    \x1b[0;34mchdir\x1b[0m, \x1b[0;34mcd\x1b[0m       change folder within current vault
    \x1b[0;34mlist\x1b[0m, \x1b[0;34mls\x1b[0m        print dir tree of current folder
    \x1b[0;34mtoday\x1b[0m, \x1b[0;34mto\x1b[0m       edit daily note

perform fs operations on items
    \x1b[0;34mremove\x1b[0m, \x1b[0;34mrm\x1b[0m      remove an item 
    \x1b[0;34mrename\x1b[0m, \x1b[0;34mrn\x1b[0m      rename an item 
    \x1b[0;34mmove\x1b[0m, \x1b[0;34mmv\x1b[0m        move an item to a new location
    \x1b[0;34mvmove\x1b[0m, \x1b[0;34mvm\x1b[0m       move an item to a different vault

config
    \x1b[0;34mconfig\x1b[0m, \x1b[0;34mcf\x1b[0m      set and get config values
    \x1b[0;34mtemplate\x1b[0m, \x1b[0;34mtp\x1b[0m    create or edit a note template

get help 
    use \x1b[0;34mhelp\x1b[0m or \x1b[0;34m-h\x1b[0m and \x1b[0;34m--help\x1b[0m flags along with a command to get corresponding help"))]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// create a vault or list vaults
    #[clap(override_usage(
        "jt vault\n    jt vault -l\n    jt vault <vault name> <vault location>"
    ))]
    #[clap(alias = "vl")]
    Vault {
        /// show vaults' location
        #[clap(parse(from_flag), short = 'l', long = "location")]
        show_loc: bool,
        /// name for new vault
        #[clap(value_parser, name = "vault name")]
        name: Option<String>,
        /// absolute path to location of new vault
        #[clap(value_parser, name = "vault location")]
        location: Option<PathBuf>,
    },
    /// create or edit the daily note
    /// format: YYYY-MM-DD
    #[clap(override_usage("jt today"))]
    #[clap(alias = "to")]
    Today,
    /// enter a vault.
    #[clap(alias = "en")]
    Enter {
        /// name of the vault to enter
        #[clap(value_parser, name = "vault name")]
        name: String,
    },
    /// create a note
    #[clap(override_usage(
        "jt note\n    jt note <note name>\n    jt note <note name> -t <template name>"
    ))]
    #[clap(alias = "nt")]
    Note {
        /// name for new note (to be created in the current folder)
        #[clap(value_parser, name = "note name")]
        name: String,
        /// create new note from an existing template
        #[clap(parse(from_flag), short = 't', long = "template")]
        from_template: bool,
        /// name of the note template
        #[clap(value_parser, name = "template name")]
        template_name: Option<String>,
    },
    /// create or edit a template, or list existing templates
    #[clap(override_usage("jt tempate\n    jt template [template-name]"))]
    #[clap(alias = "tp")]
    Template {
        /// name of the template
        #[clap(value_parser, name = "template name")]
        name: Option<String>,
    },
    /// creates an alias for a note
    // #[clap(override_usage(
    //     "jt alias\n    jt alias <note name> -r\n    jt alias <note name> <alias>"
    // ))]
    // #[clap(alias = "al")]
    // Alias {
    //     /// name of the note being given an alias
    //     #[clap(value_parser, name = "note name")]
    //     name: String,
    //     /// remove alias from a note
    //     #[clap(parse(from_flag), short = 'r', long = "remove", name = "remove")]
    //     remove_alias: bool,
    //     /// alias being given to the note
    //     #[clap(value_parser, name = "alias", required_unless_present("remove"))]
    //     maybe_alias: Option<String>,
    // },
    /// open a note (from the current folder)
    #[clap(alias = "op")]
    Open {
        /// name of note to be opened
        #[clap(value_parser, name = "note name")]
        name: String,
    },
    /// create a folder
    #[clap(override_usage("jt folder\n    jt folder [folder name]"))]
    #[clap(alias = "fd")]
    Folder {
        /// name for new folder (to be created in the current folder)
        #[clap(value_parser, name = "folder name")]
        name: String,
    },
    /// change folder within current vault
    #[clap(alias = "cd")]
    Chdir {
        /// path to folder to switch to (from current folder)
        #[clap(value_parser, name = "folder path")]
        path: PathBuf,
    },
    /// remove an item
    #[clap(alias = "rm")]
    Remove {
        /// remove a vault (or vl) | note (or nt) | folder (or fd)
        #[clap(value_enum, value_parser, name = "item type")]
        item_type: ItemType,
        /// name of item to be removed
        #[clap(value_parser, name = "name")]
        name: String,
    },
    /// rename an item
    #[clap(alias = "rn")]
    Rename {
        /// rename a vault (or vl) | note (or nt) | folder (or fd)
        #[clap(value_enum, value_parser, name = "item type")]
        item_type: ItemType,
        /// name of item to be renamed
        #[clap(value_parser, name = "name")]
        name: String,
        /// new name of item
        #[clap(value_parser, name = "new name")]
        new_name: String,
    },
    /// move an item
    #[clap(alias = "mv")]
    Move {
        /// move a vault (or vl) | note (or nt) | folder (or fd)
        #[clap(value_enum, value_parser, name = "item type")]
        item_type: ItemType,
        /// name of item to be moved
        #[clap(value_parser, name = "name")]
        name: String,
        /// path to new location of item (current folder as root in case of note or folder).
        #[clap(value_parser, name = "new location")]
        new_location: PathBuf,
    },
    /// move notes and folders to a different vault
    #[clap(alias = "vm")]
    Vmove {
        /// move a note (or nt) | folder (or fd).
        #[clap(value_enum, value_parser, name = "item type")]
        item_type: VaultItemType,
        /// name of item to be moved
        #[clap(value_parser, name = "name")]
        name: String,
        /// name of vault to move the item to
        #[clap(value_parser, name = "vault name")]
        vault_name: String,
    },
    /// list tree of current folder
    #[clap(alias = "ls")]
    List,
    /// display or set a config item
    #[clap(override_usage(
        "jt config\njt config [config type]\n    jt config [config type] [config value]"
    ))]
    #[clap(alias = "cf")]
    Config {
        /// name of config item to display or set
        #[clap(value_enum, value_parser, name = "config type")]
        config_type: Option<ConfigType>,
        /// pass a value if config needs to be updated
        #[clap(value_parser, name = "config value")]
        value: Option<String>,
    },
    /// 🆘 show this help message or help for given command.
    Help,
}
