# llmcat
```
/\\_/\\
( o.o )
 > ^ <  LLMCat

A simple CLI that transforms your code into clean, structured text for feeding into LLMs.
```

## Why
Copying, pasting, and formatting code for AI tools takes time.
LLMCat automates this, letting you focus on coding instead of formatting.

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
Run LLMCat on a single file:
```
llmcat path/to/your/codefile.py
```

Run LLMCat on a directory using a `.llmcat.toml` configuration:
```
llmcat path/to/your/project
```

## Configuration
LLMCat can be configured using a `.llmcat.toml` file. There is an example under `examples/code`:
```
.
├── docs
│   └── info.md
├── hello.js
├── .llmcat.toml
└── world.py
```

`.llmcat.toml`:
```toml
[settings]
remove_comments = true
remove_whitespace = true

[paths]
include = ["*.py", "*.js"]
exclude = ["tests/*", "docs/*"]
```

```
llmcat examples/code
```
```
Using config: ~/llmcat/examples/code/.llmcat.toml

===== FILE: hello.js =====
function fib(n) {
  if (n <= 1) {
    return n;
  }
  return fib(n - 1) + fib(n - 2);
}

console.log(fib(10));

===== FILE: world.py =====
def foo(bar):
    print(f"Foo {bar}")
```


Rules:
- Files are collected based on the `[paths].include` patterns.
- Files matching the `[paths].exclude` patterns are ignored, and `.llmcat.toml` itself is always excluded.
- Settings in the `[settings]` section control how files are processed.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License
This project is licensed under the CC BY-NC 4.0 License.
See `LICENSE` for details.