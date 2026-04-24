# 🐱 llmcat

```
/\\_/\\
( o.o )
 > ^ <  LLMCat

A simple CLI that transforms your code into clean, structured text for feeding into LLMs.
```

## 🤔 Why LLMCat?

Copying, pasting, and formatting code for AI tools is annoying and slow.
LLMCat automates the boring parts so you can stay focused on actually building things.

## ✨ Features

- ⚡ Automatic Formatting — Turns messy code into clean, structured text
- 🧹 Code Cleanup — Removes unnecessary comments and whitespace
- 📁 Multi-file Support — Works on single files or entire directories
- 🎛️ Customizable Output — Tailor formatting to your workflow
- 💾 Persistent Settings — Save configs and reuse them anytime
- 🔌 Easy Integration — Lightweight CLI that fits anywhere

## 🚀 Installation

### 🍎 macOS & 🐧 Linux (One-liner)

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/NoahCristino/llmcat/releases/download/v1.3.1/llmcat-installer.sh | sh
```

### 🍺 macOS (Homebrew)

```
brew install NoahCristino/tap/llmcat
```

### 🪟 Windows (PowerShell)

```
powershell -ExecutionPolicy Bypass -c "irm https://github.com/NoahCristino/llmcat/releases/download/v1.3.1/llmcat-installer.ps1 | iex"
```

### 🛠️ From Source

```
git clone https://github.com/NoahCristino/llmcat.git
cd llmcat
cargo build --release
cargo install --path .
```

See [releases](https://github.com/NoahCristino/llmcat/releases) for more installation options.

## ▶️ Usage

Run LLMCat on a single file:

```
llmcat file.py
```

Run LLMCat on a directory:

```
llmcat ./src
```

## ⚙️ Configuration

LLMCat can be configured using a `.llmcat.toml` file. There is an example in `examples/code`:

```toml
[settings]
remove_comments = true
remove_whitespace = true

[paths]
include = ["*.py", "*.js"]
exclude = ["tests/*", "docs/*"]
```

Run:

```
llmcat .
```

Example output:

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

Then:
Press c 📋 to copy output
Press q ❌ to q

## 📏 Rules

- ✅ Files must match [paths].include
- 🚫 Files matching [paths].exclude are ignored
- 🔒 .llmcat.toml is always excluded
- ⚙️ [settings] controls processing behavior

## 🤝 Contributing

Got ideas or improvements?

- Open an issue 💡
- Submit a PR 🚀

## License

MIT License — see `LICENSE` for details.
