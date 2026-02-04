use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

/// A simple tool to manage and securely store secrets like login credentials and API keys locally.
#[derive(Parser, Debug)]
#[command
    (version, about, long_about = None, author = "Kakeeto Pius",
    after_long_help = "Note: cman checks the credential database file from the environment variable $CMAN_DBFILE.\n\
If it is not set , cman defaults to $HOME/.creds.db.",
)]
pub struct CmanArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new database.
    Init(InitArgs),

    /// Add a secret to storage.
    #[command(
        after_long_help = "Rules for batch file:\n1. Each line has comma separated details of a single secret with the type as the first field\n\
        2. For type 'login' the format is login,secretname,username,password\n3. For type 'api' the format is api,secretname,username,description,key\n\
        4. If it is required that a given login credential's password is automatically generated, use ? as a placeholder ie login,secretname,username,?\n\
        \n Note: If the --type argument is not given 'login' is assumed."
    )]
    Add(AddArgs),

    /// Alter details of a particular secret.
    #[command(after_long_help = "Note: If the --type argument is not given 'login' is assumed.")]
    Change(ChangeArgs),

    /// Retrieve details about one or more secrets.
    #[command(after_long_help = "Note: If the --type argument is not given 'login' is assumed.")]
    Get(GetArgs),

    /// Delete one or more secrets permanently from storage. Use with care.
    #[command(after_long_help = "Note: If the --type argument is not given 'login' is assumed.")]
    Delete(DeleteArgs),

    /// List all stored secrets of a particular type.
    #[command(after_long_help = "Note: If the --type argument is not given 'login' is assumed.")]
    Ls(LsArgs),

    /// Generate shell completions
    #[command(hide = true)]
    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Args, Debug)]
pub struct InitArgs {
    //The path to initialse the database.
    #[arg(
        short,
        long,
        long_help = "The path to initialise the database. If not given $CMAN_DBFILE is used if that variable is not set, $HOME/.creds.db is used."
    )]
    pub path: Option<String>,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// The name of the secret to add to storage. Note that the word "master" cannot be used as a
    /// name
    pub secret: String,

    /// The type of Secret.
    #[arg(value_enum, short = 't', long = "type")]
    pub secret_type: Option<SecretType>,

    /// The length of the password to generate. The default is 16 characters.
    #[arg(short = 'l', long = "len")]
    pub passlen: Option<usize>,

    /// Do not automatically generate the password, the user is instead prompted for one.
    #[arg(long = "no-auto")]
    pub no_auto: bool,

    /// The SECRET_NAME is treated as file containing credentials one per line (Use cman add --help for more details).
    #[arg(
        short,
        long,
        long_help = "The SECRET_NAME is treated as file containing credentials one per line."
    )]
    pub batch: bool,
}

#[derive(Args, Debug)]
pub struct ChangeArgs {
    /// The name of the secret to change details for. If the word "master" is given the master password is
    /// what is changed.
    pub secret: Option<String>,

    /// The type of Secret.
    #[arg(value_enum, short = 't', long = "type")]
    pub secret_type: Option<SecretType>,

    /// The field of the secret to change.
    #[arg(value_enum, short, long = "field")]
    pub field: Option<FieldType>,

    /// The length of the password to generate. The default is 16 characters.
    #[arg(short = 'l', long = "len")]
    pub passlen: Option<usize>,

    /// Do not automatically generate a password, the user is instead prompted for one.
    #[arg(long = "no-auto")]
    pub no_auto: bool,
}

#[derive(Args, Debug)]
pub struct GetArgs {
    /// The name(s) of the secret to retrieve details for from storage.
    pub secret: Option<Vec<String>>,

    /// The type of Secret.
    #[arg(value_enum, short = 't', long = "type")]
    pub secret_type: Option<SecretType>,

    /// Get a particular field of the secret(s)
    #[arg(value_enum, short, long = "field")]
    pub field: Option<FieldType>,

    /// Accept mutliple inputs from the interactive menu provided only if no secret name is given
    /// as part of the command line arguments.
    #[arg(short, long)]
    pub multiple: bool,

    /// Do not print prompts or prefixes to stdout. Only the secret's retrieved details are printed.
    #[arg(short, long)]
    pub quiet: bool,

    /// Print the results returned in json form.
    #[arg(short, long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    /// The name(s) of the secret to delete from storage.
    pub secret: Option<Vec<String>>,

    /// The type of Secret.
    #[arg(value_enum, short = 't', long = "type")]
    pub secret_type: Option<SecretType>,

    /// Accept mutliple inputs from the interactive menu provided only if no secret name is given
    /// as part of the command line arguments..
    #[arg(short, long)]
    pub multiple: bool,
}

#[derive(Args, Debug)]
pub struct LsArgs {
    /// The type of Secret.
    #[arg(value_enum, short = 't', long = "type")]
    pub secret_type: Option<SecretType>,

    /// Print the results returned in json form.
    #[arg(short, long)]
    pub json: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum SecretType {
    /// The secret is a login credential.
    Login,

    /// The secret is an API key.
    Api,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum FieldType {
    /// The username for the secret.
    User,

    /// The unique name of the secret.
    Secname,

    /// The Password (LOGIN ONLY)
    Pass,

    /// The description of an api key (API ONLY)
    Desc,

    /// The API Key. (API ONLY)
    Key,
}
