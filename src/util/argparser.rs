use clap::{Args, Parser, Subcommand, ValueEnum};

/// A simple tool to manage and securely store secrets like login credentials and API keys locally.
#[derive(Parser, Debug)]
#[command
    (version, about, long_about = None, 
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

    /// Adds the given secret to storage.
    #[command(after_long_help = "Rules for batch file:\n1. Each line has comma separated details of a single secret with the type as the first field\n\
        2. For type 'login' the format is login,secretname,username,password\n3. For type 'api' the format is api,secretname,username,description,key\n\
        4. If it is required that a given login credential's password is automatically generated, use ? as a placeholder ie login,secretname,username,?\n\
        \n Note: If the --type argument is not given 'login' is assumed.")]

    Add(AddArgs),

    /// Alters details of a particular secret.
    #[command(after_long_help = "Note: If the --type argument is not given 'login' is assumed.")]
    Change(ChangeArgs),

    /// Retrieves details about a particular secret.
    #[command(after_long_help = "Note: If the --type argument is not given 'login' is assumed.")]
    Get(GetArgs),

    /// Deletes a given secret permanently from storage. Use with care.
    #[command(after_long_help = "Note: If the --type argument is not given 'login' is assumed.")]
    Delete(DeleteArgs),

    /// Lists all stored secrets of a particular type.
    #[command(after_long_help = "Note: If the --type argument is not given 'login' is assumed.")]
    Ls(LsArgs),
}

#[derive(Args, Debug)]
pub struct InitArgs {
    //The path to initialse the database. 
    #[arg(short, long, long_help = "The path to initialise the database. If not given $CMAN_DBFILE is used else $HOME/.creds.db")]
    pub path: Option<String>
}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// The name of the secret to add to storage. Note that the word "master" cannot be used as a
    /// name
    pub secret: String,

    /// The type of Secret.
    #[arg(value_enum, short = 't', long = "type")]
    pub secret_type: Option<SecretType>,

    /// If set, the SECRET_NAME is treated as file containing credentials one per line (Use cman add --help for more details).
    #[arg(short, long, long_help = "If set, the SECRET_NAME is treated as file containing credentials one per line.")]
    pub batch: bool,

    /// If set, it specifies that the password should be prompted from the user instead of
    /// automatically generating one.
    #[arg(long = "no-auto")]
    pub no_auto: bool,

    /// The length of the password to generate. The default is 16 characters.
    #[arg(short = 'l', long = "len")]
    pub passlen: Option<usize>,
}

#[derive(Args, Debug)]
pub struct ChangeArgs {
    /// The name of the secret to change details for. If the word "master" is given the master password is
    /// what is changed.
    pub secret: String,

    /// The type of Secret.
    #[arg(value_enum, short = 't', long = "type")]
    pub secret_type: Option<SecretType>,

    /// The field to change.
    #[arg(value_enum, short, long = "field")]
    pub field: Option<FieldType>,

    /// If set, it specifies that the password should be prompted from the user instead of
    /// automatically generating one.
    #[arg(long = "no-auto")]
    pub no_auto: bool,

    /// The length of the password to generate. The default is 16 characters.
    #[arg(short = 'l', long = "len")]
    pub passlen: Option<usize>,
}

#[derive(Args, Debug)]
pub struct GetArgs {
    /// The name of the secret to retrieve details for from storage.
    pub secret: String,

    /// The type of Secret.
    #[arg(value_enum, short = 't', long = "type")]
    pub secret_type: Option<SecretType>,

    /// An optional field to get. If not set all details of the secret are retrieved.
    #[arg(value_enum, short, long = "field")]
    pub field: Option<FieldType>,

    /// If set, no prompts or prefixes are printed to stdout only the secret's retrieved details are printed.
    #[arg(short, long)]
    pub quiet: bool,
    
    /// If set, the results are returned in json form.
    #[arg(short, long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    /// The name of the secret to delete from storage.
    pub secret: String,

    /// The type of Secret.
    #[arg(value_enum, short = 't', long = "type")]
    pub secret_type: Option<SecretType>,
}

#[derive(Args, Debug)]
pub struct LsArgs {
    /// The type of Secret.
    #[arg(value_enum, short = 't', long = "type")]
    pub secret_type: Option<SecretType>,

    /// If set, the results are returned in json form.
    #[arg(short, long)]
    pub json: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum SecretType {
    /// Specifies that the secret type is a login credential.
    Login,

    /// Specifies that the secret type is an API key.
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


