# My Notes App

## Overview
`My Notes` is a simple command-line note-taking app designed to help you organize and manage your notes efficiently. It provides essential functionalities like creating, editing, listing, and deleting notes.

## Getting Started
To use `My Notes`, follow these steps:

1. **Installation**
   - Clone the repository to your local machine.
   - Ensure you have Rust and Cargo installed.

2. **Building the App**
   - Navigate to the project directory.
   - Run `cargo build --release` to compile the application.

3. **Running the App**
   - After building, you can run the app using the generated binary.ß

## Usage
`my-notes [SUBCOMMAND] [OPTIONS]`

### Subcommands
- `new` - Create a new note.
  - Usage: `my-notes new <path>`
  - Arguments:
    - `-c` `--content` : string - add inline content
    - `-b` `--blank`: boolean - mark as a blank note
- `ls` - List notes.
  - Usage: `my-notes ls [path]`
- `edit` - Edit an existing note.
  - Usage: `my-notes edit <path>`
- `rm` - Delete a note.
  - Usage: `my-notes rm <path>`
  - Arguments
    - `-f` `--force` : boolean - force delete without confirmation
- `search` - Find notes by keyword.
  - Usage: `my-notes search <keyword>`
- `echo` - Print the contents of a note.
  - Usage: `my-notes echo <path>`ß
- `help` - Display help information.
  - Usage: `my-notes help [SUBCOMMAND]`
- `version` - Display version information.
  - Usage: `my-notes version`
- `view` - View a note with rendered markdown.
  - Usage: `my-notes view <path>`
- `completions` - Generate shell completion scripts.
  - Usage: `my-notes completions <bash|zsh|fish|powershell|elvish>`

### Examples
- Create a new note:
  ```
  my-notes new my-note
  ```
- List all notes:
  ```
  my-notes ls
  ```
- List notes in a specific path:
  ```
  my-notes ls my-note.subnote
  ```
- Edit a note:
  ```
  my-notes edit my-note
  ```
- Delete a note (with confirmation):
  ```
  my-notes rm my-note
  ```
- Delete a note (without confirmation):
  ```
  my-notes rm my-note --force
  ```
- Find notes by keyword:
  ```
  my-notes search keyword
  ```
- Print the contents of a note:
  ```
  my-notes echo my-note
  ```
- View a note with rendered markdown:
  ```
  my-notes view my-note
  ```
- Display version information:
  ```
  my-notes version
  ```
- Display help information:
  ```
  my-notes help
  ```
- Generate bash completions:
  ```
  my-notes completions bash > /usr/share/bash-completion/completions/my-notes
  ```
  Note names will now tab-complete for commands like `view`, `edit`, and `rm`.

## Notes
- The app uses the default system editor. Set the `EDITOR` environment variable to change it.
- Notes are stored in the `~/.notes` directory.
- The app organizes notes in a tree structure.

## Using Search Registers
- After searching for a keyword with the `search` subcommand, the search results are stored in the `.register` file.
- You can now use the template `reg:<index>` to reference the value in the search register at a specific index. This will help avoid needing to type out the full path of the note.

```bash
my-notes search keyword

#Output
this.is.a.note_that_contains_keyword
this.is.another.note_with_key_word

my-notes edit reg:0 # edit this.is.a.note_that_contains_keyword
my-notes edit reg:1 # edit this.is.another.note_with_key_word

my-notes view reg:0 # view this.is.a.note_that_contains_keyword
my-notes view reg:1 # view this.is.another.note_with_key_word
```

note: The search register is overwritten every time a new search is performed.
## Important
- Deleting a note will permanently remove it and all of its children from the index and your local computer. This action cannot be undone.

## Version
`My Notes` v0.1.0

## Support
For any issues or questions, please open an [issue on GitHub](https://github.com/DeForestt/my_notes/issues).

## License
This app is released under the [MIT License](https://opensource.org/licenses/MIT).

---

*Note: Replace placeholders (`<path>`) with actual values.*
