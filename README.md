# nu_plugin_clipboard

A [nushell](https://www.nushell.sh/) plugin to copy text into clipboard or get text from it.

* `clipboard copy`: copy a text that's given as input
  * `--{disable or enable}-daemon` (`-d`): spawn a daemon that manages clipboard (if copy is not working try using this flag)
* `clipboard paste`: returns current text value of clipboard

## Examples

* to copy a string (ONLY string for now)

```bash
~> echo "test value" | clipboard copy 
```

* to use a string that is in clipboard

```bash
~> clipboard paste | echo $in
```

* in order to copy tables please convert them to text format like JSON, YAML, ...
  * you are able to paste them as tables again using `clipboard paste | from json`

```bash
~> $env | to json | clipboard copy
~> clipboard paste | from json

~> ps | to json | clipboard copy
~> clipboard paste | from json
```

## Installing

* using [nupm](https://github.com/nushell/nupm) **Recommended!**
  * this way you don't need to mess with features and it will install required features

```bash
git clone https://github.com/FMotalleb/nu_plugin_clipboard.git
nupm install --path nu_plugin_clipboard -f
```

* supported features:
  * **use-wayland**: will prioritize wayland api but will falls back to X11 protocol on error
  * **enforce-daemon**: force copy command to spawn a daemon and revert the functionality of `--daemon` flag

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
