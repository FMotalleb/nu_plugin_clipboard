# 📋 nu_plugin_clipboard

A [nushell](https://www.nushell.sh/) plugin for interacting with the clipboard, allowing you to copy/paste text, objects, and tables.

## ✨ Features

- **`clipboard copy`**: Copies input text to the clipboard.
  - **Daemon Mode** (Linux only): Since version **0.105.2**, using env variables will try to detect display server.
    If there is any issue with copy/paste command
    This config will override this behavior. If you need to override this, please file an issue:
    
    ```bash
    $env.config.plugins.clipboard.NO_DAEMON = true
    ```

  - To make this setting permanent, add it to your `config env`.

- **`clipboard paste`**: Retrieves the current clipboard content.

## ⚠️ Important

If you face the error `Error: × Clipboard Error: The clipboard contents were not available in the requested format...`
Try disabling the daemon mode, as mentioned in [#20](https://github.com/FMotalleb/nu_plugin_clipboard/issues/20).

## 📌 Usage Examples

### Copying a string

```bash
"test value" | clipboard copy 
```

### Using clipboard content

```bash
clipboard paste
```

### Copying tables and objects

- Tables and objects are internally converted to **JSON**.
- When pasting, `clipboard paste` tries to parse JSON into a table or object.
- If parsing fails, the content is returned as a string.

```bash
$env | clipboard copy
clipboard paste

ps | clipboard copy
clipboard paste
```

## 🔧 Installation

### 🚀 Recommended: Using [nupm](https://github.com/nushell/nupm)

This method automatically handles dependencies and features:

```bash
git clone https://github.com/FMotalleb/nu_plugin_clipboard.git
nupm install --path nu_plugin_clipboard -f
```

### ⚙️ Supported Features

- **`use-wayland`**: Prioritizes the Wayland API, but falls back to X11 if needed.

### 🛠️ Manual Compilation

```bash
git clone https://github.com/FMotalleb/nu_plugin_clipboard.git
cd nu_plugin_clipboard
cargo build -r
plugin add target/release/nu_plugin_clipboard
```

### 📦 Install via Cargo (using git)

```bash
cargo install --git https://github.com/FMotalleb/nu_plugin_clipboard.git
plugin add ~/.cargo/bin/nu_plugin_clipboard
```

### 📦 Install via Cargo (crates.io) _Not Recommended_

- Since I live in Iran and crates.io won't let me update my packages like a normal person, most of the time crates.io is outdated.

```bash
cargo install nu_plugin_clipboard
plugin add ~/.cargo/bin/nu_plugin_clipboard
```
