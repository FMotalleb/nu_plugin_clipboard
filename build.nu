use std log


def main [package_file: path] {
    let repo_root = $package_file | path dirname
    let install_root = $env.NUPM_HOME | path join "plugins"

    let name = open ($repo_root | path join "Cargo.toml") | get package.name
    
    if (($nu.os-info.name == "linux") and ($env.XDG_CURRENT_DESKTOP? == "KDE")) {
        log info $"Found (ansi blue)KDE(ansi reset) from env\(`(ansi blue)XDG_CURRENT_DESKTOP(ansi reset)`\) activating (ansi red)enforce-daemon(ansi reset) mode,
        this will cause the copy action to (ansi yellow)use daemon mode(ansi reset) without the (ansi green)`--daemon` \(`-d`\)(ansi reset) flag
        and cause this flag to have inverted functionality (ansi red)\(now using -d will disable daemon mode\)(ansi reset)"
        cargo install --path $repo_root --root $install_root --features enforce-daemon
    } else if ($nu.os-info.name == "linux" and ($env.XDG_SESSION_TYPE? == "wayland")) {
        log info $"wayland was enabled in env\(`XDG_SESSION_TYPE`\): Enabled `use-wayland` feature"
        cargo install --path $repo_root --root $install_root --features use-wayland
    } else {
        cargo install --path $repo_root --root $install_root
    }
    let ext = if ($nu.os-info.name == 'windows') { '.exe' } else { '' }
    nu --commands $"register ($install_root | path join "bin" $name)($ext)"
    log info "do not forget to restart Nushell for the plugin to be fully available!"
}