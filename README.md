# llmcat

```
/\\_/\\
( o.o )
 > ^ <  LLMCat

A simple CLI that transforms your code into clean, structured text for feeding into LLMs.
```

## Why LLMCat?

Copying, pasting, and formatting code for AI tools takes time.
LLMCat automates this, letting you focus on coding instead of formatting.

## ✨ Features

- **Automatic Formatting**: Converts code files into clean, structured text.
- **Code Cleanup**: Removes unnecessary comments and whitespace.
- **Multi-file Support**: Process multiple files or entire directories at once.
- **Customizable Output**: Configurable output formats that suit your needs.
- **Persistent Settings**: Save your preferences for future use.
- **Easy Integration**: Simple CLI that can be integrated into your workflow.

## 🚀 Installation

### macOS & Linux (One-liner)

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/NoahCristino/llmcat/releases/download/v1.2.0/llmcat-installer.sh | sh
```

### macOS (Homebrew)

```
brew install NoahCristino/tap/llmcat
```

### Windows (PowerShell)

```
powershell -ExecutionPolicy Bypass -c "irm https://github.com/NoahCristino/llmcat/releases/download/v1.2.0/llmcat-installer.ps1 | iex"
```

### From Source

```
git clone https://github.com/NoahCristino/llmcat.git
cd llmcat
cargo build --release
cargo install --path .
```

## Usage

Run LLMCat on a single file:

```
llmcat file.py
```

Run LLMCat on a directory:

```
llmcat ./src
```

## Configuration

LLMCat can be configured using a `.llmcat.toml` file. There is an example in `examples/code`:

```toml
[settings]
remove_comments = true
remove_whitespace = true

[paths]
include = ["*.py", "*.js"]
exclude = ["tests/*", "docs/*"]
```

When you run:

```
llmcat .
```

in the `examples/code` directory, LLMCat will process `hello.js` and `world.py`, but will ignore `docs/info.md` and `.llmcat.toml` itself, resulting in output like:

```
Using config: /home/user/llmcat/examples/code/.llmcat.toml

===== FILE: hello.js [8 lines] =====
function fib(n) {
  if (n <= 1) {
    return n;
  }
  return fib(n - 1) + fib(n - 2);
}

console.log(fib(10));

===== FILE: world.py [2 lines] =====
def foo(bar):
    print(f"Foo {bar}")
c - copy | q - quit
:
```

You can then press `c` to copy the output to your clipboard, or `q` to quit.

Rules:

- Files are collected based on the `[paths].include` patterns.
- Files matching the `[paths].exclude` patterns are ignored, and `.llmcat.toml` itself is always excluded.
- Settings in the `[settings]` section control how files are processed.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License.
See `LICENSE` for details.
