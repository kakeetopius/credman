# credman

A simple command-line tool to manage and securely store secrets like login credentials and API keys locally.

## Features

- **Secure local storage**: Store credentials encrypted with SQLCipher
- **Multiple secret types**: Support for login credentials and API keys
- **Password generation**: Auto-generate secure passwords or provide your own
- **Batch operations**: Add multiple secrets at once from a file
- **Flexible retrieval**: Get secrets by name, type, or specific fields
- **Interactive mode**: Select from stored secrets interactively
- **JSON output**: Export secrets in JSON format for scripting
- **Shell completions**: Built-in support for bash, zsh, fish, powershell and elvish shells

## Installation

### Build from source

Requires Rust 1.70+ and development headers for OpenSSL.

```bash
git clone https://github.com/yourusername/credman.git
cd credman
cargo build --release
sudo cp target/release/cman /usr/local/bin/
```

## Quick Start

### Initialize the database

```bash
cman init
```

By default, the database is stored at `~/.creds.db`. You can specify a custom path:

```bash
cman init --path /custom/path/to/database.db
```

Or set the `$CMAN_DBFILE` environment variable:

```bash
export CMAN_DBFILE=/path/to/database.db
```

<details>
<summary>Add a secret</summary>

Add a login credential with auto-generated password:

```bash
cman add github
```

Add with a custom password (interactive prompt):

```bash
cman add github --no-auto
```

Add an API key:

```bash
cman add openai --type api
```

Specify password length:

```bash
cman add github --len 32
```

</details>

<details>
<summary>Retrieve a secret</summary>

Get all details for a credential:

```bash
cman get github
```

Get a specific field:

```bash
cman get github --field user
```

Get multiple secrets interactively:

```bash
cman get --multiple
```

Output as JSON:

```bash
cman get github --json
```

### List all secrets

List all login credentials:

```bash
cman ls
```

List all API keys:

```bash
cman ls --type api
```

Output as JSON:

```bash
cman ls --json
```

</details>

<details>
<summary>Modify a secret</summary>

Change a credential's password:

```bash
cman change github
```

Change a specific field:

```bash
cman change github --field user
```

Change the master password:

```bash
cman change master
```

</details>

<details>
<summary>Delete a secret</summary>

Delete a single credential:

```bash
cman delete github
```

Delete multiple secrets interactively:

```bash
cman delete --multiple
```

</details>

<details>
<summary>Batch Operations</summary>

Add multiple credentials from a file:

```bash
cman add credentials.txt --batch
```

### Batch file format

Each line contains comma-separated fields. The first field specifies the secret type.

**Login format**: `login,secretname,username,password`
**API format**: `api,secretname,username,description,key`

To auto-generate a password for a login, use `?` as the password:

```
login,github,myusername,?
login,gmail,myemail@gmail.com,?
api,openai,user123,my api key,sk-1234567890abcdef
```

</details>

## Commands

| Command            | Description                        |
| ------------------ | ---------------------------------- |
| `cman init`        | Initialize the credential database |
| `cman add`         | Add a new secret                   |
| `cman get`         | Retrieve secret details            |
| `cman change`      | Modify an existing secret          |
| `cman delete`      | Remove a secret permanently        |
| `cman ls`          | List all secrets of a type         |
| `cman completions` | Generate shell completions         |

## Environment Variables

- `$CMAN_DBFILE`: Path to the credential database (defaults to `~/.creds.db`)

## Secret Types

- **Login**: Username and password credentials
- **API**: API keys with optional description

## Security

- Credentials are encrypted using SQLCipher with AES-256
- Master password protects access to the database
- No secrets are logged or written to temporary files
- All operations are performed in-memory when possible

## Requirements

- Rust 1.70+
- OpenSSL development headers
- SQLite 3

## License

MIT
