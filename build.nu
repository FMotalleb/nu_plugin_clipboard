use std log

let messages = {
    "enforce-daemon" : $"Found (ansi blue)($env.XDG_CURRENT_DESKTOP)(ansi reset) from env\(`(ansi blue)XDG_CURRENT_DESKTOP(ansi reset)`\) activating `(ansi red)enforce-daemon(ansi reset)` mode,
    this will cause the copy action to (ansi yellow)use daemon mode(ansi reset) without the (ansi green)`--daemon` \(`-d`\)(ansi reset) flag
    and cause `--daemon` flag to have inverted functionality (ansi red)\(now using -d will disable daemon mode\)(ansi reset)",
    "use-wayland" : $"Found (ansi blue)wayland(ansi reset) in env\(`(ansi blue)XDG_SESSION_TYPE(ansi reset)`\): activating `(ansi green)use-wayland(ansi reset)` feature"
}

def main [package_file: path] {
    let repo_root = $package_file | path dirname
    let install_root = $env.NUPM_HOME | path join "plugins"

    let name = open ($repo_root | path join "Cargo.toml") | get package.name
    let features = [] 
        | if ($nu.os-info.name == "linux") { $in | append enforce-daemon } else { $in } 
        | if ($nu.os-info.name == "linux" and ($env.XDG_SESSION_TYPE? == "wayland")) {$in | append use-wayland } else { $in }

    for feature in $features { 
        let message = $messages | get $feature
        if $message != null {
            log info $message
        }
    }
    let cmd = $"cargo install --path ($repo_root) --root ($install_root) --features=($features | str join ",")"
    log info $"building plugin using: (ansi blue)($cmd)(ansi reset)"
    nu -c $cmd
    let ext: string = if ($nu.os-info.name == 'windows') { '.exe' } else { '' }
    nu --commands $"register ($install_root | path join "bin" $name)($ext)"
    log info "do not forget to restart Nushell for the plugin to be fully available!"
}