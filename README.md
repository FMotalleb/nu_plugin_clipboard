# nu_plugin_clipboard

A [nushell](https://www.nushell.sh/) plugin to copy text into clipboard or get text from it.

* `clipboard copy`: copy a text that's given as input
* `clipboard paste`: returns current text value of clipboard

# Examples

* to copy a string (ONLY string for now)

```bash
~> echo "test value" | clipboard copy 
```

* to use a string that is in clipboard

```bash
~> clipboard paste | echo $in
```

# Installing

* supported features:
  * **use-wayland**: will prioritize wayland api but will falls back to X11 protocol on error

* using [nupm](https://github.com/nushell/nupm)

```bash
git clone https://github.com/FMotalleb/nu_plugin_clipboard.git
nupm install --path nu_plugin_clipboard -f
```

* or compile manually

```bash
git clone https://github.com/FMotalleb/nu_plugin_clipboard.git
cd nu_plugin_clipboard
cargo build -r
register target/release/nu_plugin_clipboard
```

* or using cargo

```bash
cargo install nu_plugin_clipboard
register ~/.cargo/bin/nu_plugin_clipboard
```
