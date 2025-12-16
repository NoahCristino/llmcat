# llmcat
```
/\\_/\\
( o.o )
 > ^ <  LLMCat

A simple CLI that transforms your code into clean, structured text for feeding into LLMs.
```

## Why
When coding with AI it takes a lot of time to copy and paste and format code snippets. llmcat automates this process, allowing you to spend more time coding and less time formatting.

## Features
- **Automatic Formatting**: Converts code files into clean, structured text.
- **Code Cleanup**: Removes unnecessary comments and whitespace.
- **Multi-file Support**: Process multiple files or entire directories at once.
- **Customizable Output**: Configurable output formats that suit your needs.
- **Persistent Settings**: Save your preferences for future use.
- **Easy Integration**: Simple CLI that can be integrated into your workflow.

## Installation
From Source:
```
cargo build --release
cp target/release/llmcat /usr/local/bin/llmcat
```

Using Cargo:
```
cargo install llmcat
```

Using Homebrew:
```
brew install llmcat
```

## Usage
LLMCat can be run manually from the commmand line:

```
llmcat path/to/your/codefile.py
```

Or you can run it using custom settings by running it on a directory containing a `.llmcat.toml` configuration file:

```
llmcat path/to/your/project
```

## Configuration
LLMCat can be configured using a `.llmcat.toml` file. Here is an example configuration:
```toml
[settings]
remove_comments = true
remove_whitespace = true

[paths]
include = ["*.py", "*.js"]
exclude = ["tests/*", "docs/*"]
```

## Contributing
Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License
This project is licensed under the CC BY-NC 4.0 License.
See `LICENSE` for details.