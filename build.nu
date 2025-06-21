use std log

let messages = {
   "use-wayland" : $"Found (ansi blue)wayland(ansi reset) in env\(`(ansi blue)XDG_SESSION_TYPE(ansi reset)`\): activating `(ansi green)use-wayland(ansi reset)` feature"
   "debug" : $"Debug mode is enabled: activating `(ansi green)debug(ansi reset)` feature"
}
def append-feature [active: bool,feature: string] {
    if $active {
        $in | append $feature
    } else {
        $in
    }
}

def main [package_file: path = nupm.nuon] {
    let repo_root = (ls -f $package_file | first | get name | path dirname)
    let install_root = $env.NUPM_HOME | path join "plugins"

    let name = open ($repo_root | path join "Cargo.toml") | get package.name
    let debug = (([no,yes] | input list "Enable debug mode") == "yes")
    let use_wayland = ($nu.os-info.name == "linux" and ($env.XDG_SESSION_TYPE? == "wayland"))
    let features = []
        | append-feature $use_wayland use-wayland
        | append-feature $debug debug
    for feature in $features { 
        let message = $messages | get $feature
        if $message != null {
            log info $message
        }
    }

    let channel = $repo_root
        | path join rust-toolchain.toml
        | open $in
        | get toolchain?.channel?
        | default stable

    let cmd = $"cargo +($channel) install --path ($repo_root) --root ($install_root) --features=($features | str join ",")"
    log info $"building plugin using: (ansi blue)($cmd)(ansi reset)"
    nu -c $cmd
    let ext: string = if ($nu.os-info.name == 'windows') { '.exe' } else { '' }
    plugin add $"($install_root | path join "bin" $name)($ext)"
    log info "do not forget to restart Nushell for the plugin to be fully available!"
}
