use std log


def main [package_file: path] {
    let repo_root = $package_file | path dirname
    let install_root = $env.NUPM_HOME | path join "plugins"

    let name = open ($repo_root | path join "Cargo.toml") | get package.name
    
    # if (($nu.os-info.name == "linux") and ($env.XDG_SESSION_TYPE? != "wayland")) {
    #     log info $"Wayland was missing considering env\(`($env.XDG_SESSION_TYPE)`\) as x11 and forcing x11 feature"
    #     cargo install --path $repo_root --root $install_root --features force-x11
    # } else 
    if ($nu.os-info.name == "linux" and ($env.XDG_SESSION_TYPE? == "wayland")) {
        log info $"wayland was enabled in env\(`XDG_SESSION_TYPE`\): Enabled `use-wayland` feature"
        cargo install --path $repo_root --root $install_root --features use-wayland
    } else {
        cargo install --path $repo_root --root $install_root
    }
    let ext = if ($nu.os-info.name == 'windows') { '.exe' } else { '' }
    nu --commands $"register ($install_root | path join "bin" $name)($ext)"
    log info "do not forget to restart Nushell for the plugin to be fully available!"
}