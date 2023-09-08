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
   - After building, you can run the app using the generated binary.

## Usage
`my-notes [SUBCOMMAND] [OPTIONS]`

### Subcommands
- `new` - Create a new note.
  - Usage: `my-notes new <path>`
- `ls` - List notes.
  - Usage: `my-notes ls [path]`
- `edit` - Edit an existing note.
  - Usage: `my-notes edit <path>`
- `rm` - Delete a note.
  - Usage: `my-notes rm <path>`

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

## Notes
- The app uses the default system editor. Set the `EDITOR` environment variable to change it.
- Notes are stored in the `~/.notes` directory.
- The app organizes notes in a tree structure.

## Important
- Deleting a note will permanently remove it and all of its children from the index and your local computer. This action cannot be undone.

## Version
`My Notes` v0.1.0

## Support
For any issues or questions, please open an [issue on GitHub](https://github.com/your-repo/issues).

## License
This app is released under the [MIT License](https://opensource.org/licenses/MIT).

---

*Note: Replace placeholders (`<path>`, `your-repo`) with actual values.*