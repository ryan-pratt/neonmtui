# neonmtui

Very, very WIP replacement for nmtui I just started. I just didn't like any of the existing tools for controlling NetworkManager from a TUI while working on making my NixOS config feel more like "home," so I decided to make my own. And it's trendy to prefix your project name with "neo"

## Goals

- [ ] edit/(de)activate wifi connections
- [ ] ditto VPN (will start with OpenVPN)
- [ ] ditto bluetooth

## Development

`flake.nix` defines a development shell that ensures all the dependencies are there and everything works together. This prevents weird issues from `rustc` version mismatches and `rustup override` bugs. Not that it matters since this is a one-man project for the forseeable future, but I like this workflow.

I should probably add some better tooling integration in the future, but for now I'm just raw-dogging `cargo` commands.

### The good workflow

- be on a unix system that uses NetworkManager
- install `nix` and `direnv`
- run `direnv allow`

I also plan on tightening up this development shell and figuring out how to mock dbus/NetworkManager (and bluetoothctl or w/e). That'll need to be done before I get to the "edit" part of the goals.

### The other workflow

It's still a normal rust project (just with a couple extra files), so you can also just:

- be on a unix system that uses NetworkManager
- manually install all the packages defined in `flake.nix` (probably use `rustup`)

