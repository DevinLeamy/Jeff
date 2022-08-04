use clap::{AppSettings, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[clap(global_setting(AppSettings::HidePossibleValuesInHelp))]
#[clap(global_setting(AppSettings::DontCollapseArgsInUsage))]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::ColorNever))]
#[clap(before_help(
    r#"
                                    ________      _____ 
                                    ______(_)_______  /_
                                    _____  /_  __ \  __/
                                    ____  / / /_/ / /_  
                                    ___  /  \____/\__/  
                                    /___/ 
"#
))]
#[clap(help_template(
    "{before-help}{about-with-newline}{usage-heading}\n    {usage}\n\n{all-args}"
))]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// 📁 list and switch vaults or perform fs operations on them.
    #[clap(override_usage(
        "jot vlt \n    jot vlt <vault name> <vault path> \n    jot vlt <SUBCOMMAND>"
    ))]
    VLT {
        /// name for new vault.
        #[clap(value_parser, name = "vault name", requires = "vault path")]
        name: Option<String>,
        /// fs path of new vault.
        #[clap(value_parser, name = "vault path")]
        path: Option<String>,
        #[clap(subcommand)]
        command: Option<VltCommand>,
    },
    /// 📝 list, create, open, rename, move, and delete notes.
    #[clap(override_usage("jot nts\n    jot nts [note name]\n    jot nts <SUBCOMMAND>"))]
    NTS {
        /// name for new note (to be created in current location).
        #[clap(value_parser, name = "note name")]
        name: Option<String>,
        #[clap(subcommand)]
        command: Option<NtsCommand>,
    },
    /// 📂 perform fs operations on directories or display current vault's tree.
    #[clap(override_usage("jot dir\n    jot dir [directory name]\n    jot dir <SUBCOMMAND>"))]
    DIR {
        /// name for new directory (to be created in current location).
        #[clap(value_parser, name = "directory name")]
        name: Option<String>,
        #[clap(subcommand)]
        command: Option<DirCommand>,
    },
    /// 🔀 switch directories within current vault.
    CDR {
        /// path of directory (with current location as root).
        #[clap(value_parser, name = "directory path")]
        path: String,
    },
    /// 🗒️ list and open notes from current vault's history.
    #[clap(override_usage("jot hst\n    jot hst [SUBCOMMAND]"))]
    HST {
        #[clap(subcommand)]
        command: Option<HstCommand>,
        // #[clap(short, action)]
        // open: bool
    },
    /// ⏮️ open last accessed note in the current vault.
    LST,
    /// 🔍 find directories and notes in the current vault.
    FND {
        /// regex query string.
        #[clap(value_parser, name = "query")]
        query: String,
        /// query files (fil) or directories (dir).
        #[clap(value_enum, value_parser, name = "query type")]
        query_type: QueryType,
    },
    /// 📒 list, create and delete memos/quick notes (independent of current vault).
    #[clap(override_usage("jot mem\n    jot mem [content]\n    jot mem <SUBCOMMAND>"))]
    MEM {
        /// content for new memo.
        #[clap(value_parser, name = "content")]
        content: Option<String>,
        #[clap(subcommand)]
        command: Option<MemCommand>,
        // #[clap(short, action)]
        // del: bool
    },
    /// 🆘 show this help message or help for given command.
    Help,
}

#[derive(Subcommand, Debug)]
#[clap(args_conflicts_with_subcommands = true)]
pub enum VltCommand {
    /// 🚪 enter/switch to a vault.
    ENT {
        #[clap(name = "vault name")]
        name: String,
    },
    /// 🚮 delete a vault.
    DEL {
        #[clap(name = "vault name")]
        name: String,
    },
    /// 🔁 rename a vault.
    REN {
        #[clap(name = "current name")]
        name: String,
        #[clap(name = "new name")]
        new_name: String,
    },
    /// 🗃️ move vault to a new location in the fs.
    MOV {
        #[clap(name = "vault name")]
        name: String,
        #[clap(name = "new path")]
        new_path: String,
    },
    /// 🆘 show this help message or help for given command.
    Help,
}

#[derive(Subcommand, Debug)]
#[clap(args_conflicts_with_subcommands = true)]
pub enum NtsCommand {
    /// 📖 open a note with the editor defined in config.
    OPN {
        #[clap(name = "note title")]
        title: String,
    },
    /// 🚮delete a note.
    DEL {
        #[clap(name = "note title")]
        title: String,
    },
    /// 🔁 rename/retitle a note.
    REN {
        #[clap(name = "current title")]
        title: String,
        #[clap(name = "new title")]
        new_title: String,
    },
    /// 🗃️ move note to new location (with the current location as root).
    MOV {
        #[clap(name = "note title")]
        title: String,
        #[clap(name = "new location")]
        new_location: String,
    },
    /// 🗄️ move note to (root of) a different vault.
    MVV {
        #[clap(name = "note title")]
        title: String,
        #[clap(name = "vault name")]
        new_location: String,
    },
    /// 🆘 show this help message or help for given command.
    Help,
}

#[derive(Subcommand, Debug)]
#[clap(args_conflicts_with_subcommands = true)]
pub enum DirCommand {
    /// 🚮 delete a directory.
    DEL {
        #[clap(name = "directory name")]
        name: String,
    },
    /// 🔁 rename a directory.
    REN {
        #[clap(name = "current name")]
        name: String,
        #[clap(name = "new name")]
        new_name: String,
    },
    /// 🗃️ move directory to a new location within current vault.
    MOV {
        #[clap(name = "directory name")]
        name: String,
        #[clap(name = "new location")]
        new_location: String,
    },
    /// 🗄️ move directory to (root of) a different vault.
    MVV {
        #[clap(name = "directory name")]
        title: String,
        #[clap(name = "vault name")]
        new_location: String,
    },
    /// 🆘 show this help message or help for given command.
    Help,
}

#[derive(Subcommand, Debug)]
pub enum HstCommand {
    /// 📖 open a note from history.
    OPN,
    /// 🆘 show this help message or help for given command.
    Help,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum QueryType {
    FIL,
    DIR,
}

#[derive(Subcommand, Debug)]
#[clap(args_conflicts_with_subcommands = true)]
pub enum MemCommand {
    /// 🚮 choose which memo to delete.
    DEL,
    /// 🆘 show this help message or help for given command.
    Help,
}
