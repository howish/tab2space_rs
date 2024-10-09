# Tab2Space

Tab2Space is a Rust command-line tool that converts tabs to spaces in text files or entire folders. It is designed for developers who need to standardize the formatting of their source code or text files. This tool supports various file types commonly used in programming, including `.c`, `.cc`, `.h`, `.py`, and `.cs`.

## Features
- Convert tabs to spaces in a single file or all files within a folder.
- Specify the width of a tab (default is 4 spaces).
- Safe mode to avoid overwriting original files.
- Option to remove trailing spaces.
- Supports multiple file extensions.

## Requirements
- Rust (for compiling the project). You can install Rust by visiting [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Installation
1. Clone the repository:
   ```sh
   git clone <repository-url>
   cd tab2space_rs
   ```
2. Build the project using Cargo:
   ```sh
   cargo build --release
   ```
3. The executable will be located in the `target/release` directory:
   ```sh
   ./target/release/tab2space
   ```

## Usage
### Command Line Arguments
- **path**: The path of the target file or folder (required).
- **-f, --is_folder**: Add this flag if the target is a folder.
- **-o, --overwrite**: Use unsafe mode to overwrite the original files.
- **-s, --rtw**: Remove trailing spaces from lines.
- **-w, --tab_width**: Width of a tab in spaces (default: 4).

### Examples
1. Convert tabs to spaces in a single file:
   ```sh
   ./target/release/tab2space path/to/file -w 4
   ```
2. Convert all files in a folder (safe mode, won't overwrite original files):
   ```sh
   ./target/release/tab2space path/to/folder -f
   ```
3. Convert all files in a folder and overwrite the original files:
   ```sh
   ./target/release/tab2space path/to/folder -f -o
   ```
4. Convert a file and remove trailing spaces:
   ```sh
   ./target/release/tab2space path/to/file -s
   ```

## How It Works
- **Safe Mode**: By default, Tab2Space runs in safe mode, meaning it creates a new file with `_notab` appended to the original file name, instead of overwriting it.
- **Tab Conversion**: The tool converts tabs to a specified number of spaces, which can be set using the `--tab_width` argument.
- **File Filtering**: When processing folders, only files with extensions `.c`, `.cc`, `.h`, `.py`, and `.cs` are targeted. You can modify this list in the source code if needed.

## Contributing
Contributions are welcome! Feel free to submit a pull request or open an issue if you find a bug or have a feature request.

## License
This project is licensed under the MIT License. See the `LICENSE.md` file for details.
