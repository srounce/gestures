> This fork repo mainly focus on the performance of three-figure-dragging feature.

> for more tech details, please vist this discussion: https://github.com/riley-martin/gestures/discussions/6

> My own compiled binaries(maybe not too synchronized with the latest source code）and configuration can be found at https://github.com/ferstar/gestures/releases

> OR, you can install(compile) it with cargo by yourself like this: cargo install --git https://github.com/ferstar/gestures.git

> ~~NOTE: ONLY WORKS ON X11, NO WAYLAND SUPPORT YET!~~
> Now support both Xorg and Wayland(with the help of ydotool power)

---
# Gestures
## About
This is a program for intercepting touchpad gestures and executing commands based on them.
Unlike some alternatives, it directly uses the libinput api rather than parsing the output
of `libinput debug-events`.

## Features
`gestures` is able to handle libinput swipe events; not only vertical and horizontal but diagonal
as well.
- [x] Handle libinput events
  - [x] Swipe events; vertical, horizontal and diagonal
  - [x] Pinch events
  - [x] Hold events
  - [x] Rotate events
  - [x] Continuous and one-shot events
- [x] Config file

## Configuration
See [config.md](./config.md) for configuration instructions.

## Installation
### Platforms
Linux. The testing workflow runs on Ubuntu and I test it myself on ~~Artix Linux~~ Nixos, but it should work on any distro if it uses the
`libinput` touchpad driver rather than the older `synaptics` driver.  
Note: If your DE/WM has its own touchpad gestures system, it may need to be disabled to
prevent conflicts.
### Nix
If you are using flakes, simply add `gestures.url = "github:ferstar/gestures";` to your flake inputs
and add `inputs.gestures.packages.${system}.gestures` to your `home.packages` or `environment.systemPackages`. You can also create a service in
`systemd.user.services`.

### Dependencies
You may need to install `libudev` and `libinput`, or their equivalant for your distro, and possibly the `dev` versions as well.

### With Cargo
If you have cargo installed, simply use `cargo install --git https://github.com/ferstar/gestures.git`

### Manual installation
- Clone the repo
  - `git clone https://github.com/ferstar/gestures && cd gestures`

- Build
  - `cargo build --release`

- Copy `./target/release/gestures` to a convenient place and execute it

### Autostart
#### Systemd
Drop [examples/gestures.service](./examples/gestures.service) into `~/.config/systemd/user/gestures.service`
and modify it for your system (mainly the "$HOME" environment variable and the `ExecStart` will need changed).
To have it start automatically, run `systemctl --user enable --now gestures.service`.

#### Other init systems
I haven't used any other init systems, but the service is quite simple so it should be easy to modify
for other systems.

## Alternatives
Here are some alternatives with similar features.

- [libinput-gestures](https://github.com/bulletmark/libinput-gestures)
Parses output of `libinput debug-events` rather than using libinput api.
- [gebaar](https://github.com/Coffee2CodeNL/gebaar-libinput)
Only supports swipe gestures
- [gebaar-libinput-fork](https://github.com/osleg/gebaar-libinput-fork)
Fork of gebaar which supports other gestures
- [fusuma](https://github.com/iberianpig/fusuma)
Also parses `libinput debug-events` output
