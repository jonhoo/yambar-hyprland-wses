Enable [Yambar] to show [Hyprland] workspaces.

# Installation

Once installed use the Yambar config further down in the README. You'll need
to adjust the `script` path depending on your installation method.

## Cargo

Assuming you have [Rust installed][Rust], run:

```console
$ cargo install yambar-hyprland-wses
```

Use `/home/yourusernamehere/.cargo/bin/yambar-hyprland-wses` as your script path

## Arch Linux

There is a community maintained package available in the AUR. Assuming you have
[yay], run:

```console
$ yay -S yambar-hyprland-wses
```

Use `/usr/bin/yambar-hyprland-wses` as your script path.

## Nix

There is a community maintained [Nix package] on nixpgs but I could not find
documentation on how to configure it, so you are on your own a little bit here.
Pull requests with documentation on this are welcome :)

# Why?

Yambar [doesn't currently have][nope] a mechanism for displaying
Hyprland workspaces. There is at least one [polling script] out there
that does it, but because it is polling, it feels sluggish and is less
efficient than it could be.

This tool uses the [Hyprland IPC interface][ipc] via the [hyprland
crate] to query and display the workspace list, and then _listen_ for
changes whenever they happen. It does so in a way that is compatible
with the [Yambar script module][script] so that a Yambar configuration
can visualize an always-up-to-date Hyprland workspace list.

# Outputs

The script, when run, outputs the following (using Yambar's script
module output format):

```
workspace_N|string|The Hyprland name of the workspace
workspace_N_windows|int|The number of windows on the workspace
workspace_N_index_on_monitor|int|The index of the workspace on its monitor
workspace_N_monitor|string|The name of the monitor the workspace is on
workspace_N_active|bool|Whether the workspace is active on _a_ monitor
workspace_N_focused|bool|Whether the workspace is active _and_ its monitor has focus
workspace_count|int|The total number of workspaces
```

# Caveats

In Hyprland, empty workspaces [don't exist][empty], and so you'll only
see workspaces that have anything on them appear in your bar. To make
this a little less jarring, this tool will also fill in empty workspaces
up to the the last populated workspace. If you want all the workspaces
to always be listed, you'll want to tweak the suggested Yambar config
slightly (see comments inline below).

Because Yambar's support for plugins is very simple, and its
"[particles]" are fairly limited, the Yambar configuration is quite
verbose and repetitive (see below). For some reason it also seems to
render the particles before the script module has issued its first tags,
so you get a bunch of warnings initially upon launching yambar that then
go away.

I don't currently have a multi-monitor setup, so I haven't tested how
well this works there. It's probably broken. The tool _tries_ to list
out workspaces from left to right based on what monitor they're on, and
it _tries_ to highlight workspaces that are active on non-focused
monitors, but I don't know if any of those bits currently work. Please
let me know!

I haven't set up [`on-click` handlers][click] for the configuration
below. Feel free to do so. You'll probably want to invoke

```bash
hyprctl dispatch workspace N
```

with maybe some special handling on multi-monitor setups? If you make
something good, please do report back!

# Yambar configuration file:

> You must change the path of the script depending on your installation method.
> See installation notes above for more information.

```yaml
    - script:
        path: /home/yourusernamehere/.cargo/bin/yambar-hyprland-wses
        anchors:
          ws_focused: &ws_focused fba922ff
          ws_active: &ws_active ffaa00ff
          ws_empty: &ws_empty 555555ff
          ws_other: &ws_other bbbbbbff
          # you can use `{workspace_N}` here to use the workspace name
          # assigned by Hyprland instead of hard-coding one here. That
          # name is, as far as I can tell, always just the index of the
          # workspace though, so not all that valuable.
          ws_1: &ws_1 "I"
          ws_2: &ws_2 "II"
          ws_3: &ws_3 "III"
          ws_4: &ws_4 "IV"
          ws_5: &ws_5 "V"
          ws_6: &ws_6 "VI"
          ws_7: &ws_7 "VII"
          ws_8: &ws_8 "VIII"
          ws_9: &ws_9 "IX"
        content:
          list:
            spacing: 6
            items:
              - map:
                  default:
                    string: { text: *ws_1, foreground: *ws_other }
                  conditions:
                    workspace_count < 2:
                      # if you replace all of these empty: entries with
                      #
                      #   string: { text: *ws_N, foreground: *ws_empty }
                      #
                      # instead, then you'll always see the full
                      # workspace list. however, you'll be unable to use
                      # `{workspace_N}` as the workspace names in the
                      # anchors list above.
                      empty: {}
                    workspace_1_focused:
                      string: { text: *ws_1, foreground: *ws_focused }
                    workspace_1_active:
                      string: { text: *ws_1, foreground: *ws_active }
                    workspace_1_windows == 0:
                      string: { text: *ws_1, foreground: *ws_empty }
              # All of the maps below are identical to the one above,
              # except with N++.
              - map:
                  default:
                    string: { text: *ws_2, foreground: *ws_other }
                  conditions:
                    workspace_count < 3:
                      empty: {}
                    workspace_2_focused:
                      string: { text: *ws_2, foreground: *ws_focused }
                    workspace_2_active:
                      string: { text: *ws_2, foreground: *ws_active }
                    workspace_2_windows == 0:
                      string: { text: *ws_2, foreground: *ws_empty }
              - map:
                  default:
                    string: { text: *ws_3, foreground: *ws_other }
                  conditions:
                    workspace_count < 4:
                      empty: {}
                    workspace_3_focused:
                      string: { text: *ws_3, foreground: *ws_focused }
                    workspace_3_active:
                      string: { text: *ws_3, foreground: *ws_active }
                    workspace_3_windows == 0:
                      string: { text: *ws_3, foreground: *ws_empty }
              - map:
                  default:
                    string: { text: *ws_4, foreground: *ws_other }
                  conditions:
                    workspace_count < 5:
                      empty: {}
                    workspace_4_focused:
                      string: { text: *ws_4, foreground: *ws_focused }
                    workspace_4_active:
                      string: { text: *ws_4, foreground: *ws_active }
                    workspace_4_windows == 0:
                      string: { text: *ws_4, foreground: *ws_empty }
              - map:
                  default:
                    string: { text: *ws_5, foreground: *ws_other }
                  conditions:
                    workspace_count < 6:
                      empty: {}
                    workspace_5_focused:
                      string: { text: *ws_5, foreground: *ws_focused }
                    workspace_5_active:
                      string: { text: *ws_5, foreground: *ws_active }
                    workspace_5_windows == 0:
                      string: { text: *ws_5, foreground: *ws_empty }
              - map:
                  default:
                    string: { text: *ws_6, foreground: *ws_other }
                  conditions:
                    workspace_count < 7:
                      empty: {}
                    workspace_6_focused:
                      string: { text: *ws_6, foreground: *ws_focused }
                    workspace_6_active:
                      string: { text: *ws_6, foreground: *ws_active }
                    workspace_6_windows == 0:
                      string: { text: *ws_6, foreground: *ws_empty }
              - map:
                  default:
                    string: { text: *ws_7, foreground: *ws_other }
                  conditions:
                    workspace_count < 8:
                      empty: {}
                    workspace_7_focused:
                      string: { text: *ws_7, foreground: *ws_focused }
                    workspace_7_active:
                      string: { text: *ws_7, foreground: *ws_active }
                    workspace_7_windows == 0:
                      string: { text: *ws_7, foreground: *ws_empty }
              - map:
                  default:
                    string: { text: *ws_8, foreground: *ws_other }
                  conditions:
                    workspace_count < 9:
                      empty: {}
                    workspace_8_focused:
                      string: { text: *ws_8, foreground: *ws_focused }
                    workspace_8_active:
                      string: { text: *ws_8, foreground: *ws_active }
                    workspace_8_windows == 0:
                      string: { text: *ws_8, foreground: *ws_empty }
              - map:
                  default:
                    string: { text: *ws_9, foreground: *ws_other }
                  conditions:
                    workspace_count < 10:
                      empty: {}
                    workspace_9_focused:
                      string: { text: *ws_9, foreground: *ws_focused }
                    workspace_9_active:
                      string: { text: *ws_9, foreground: *ws_active }
                    workspace_9_windows == 0:
                      string: { text: *ws_9, foreground: *ws_empty }
```

[Yambar]: https://codeberg.org/dnkl/yambar
[Hyprland]: https://hyprland.org/
[Rust]: https://www.rust-lang.org/learn/get-started
[nope]: https://codeberg.org/dnkl/yambar/issues/279
[polling script]: https://www.reddit.com/r/yambar/comments/13dlodc/hyprland_yambar_script/
[ipc]: https://wiki.hyprland.org/IPC/
[hyprland crate]: https://crates.io/crates/hyprland
[script]: https://codeberg.org/dnkl/yambar/src/branch/master/doc/yambar-modules-script.5.scd
[empty]: https://github.com/hyprwm/Hyprland/issues/2723#issuecomment-1637144432
[particles]: https://codeberg.org/dnkl/yambar/src/branch/master/doc/yambar-particles.5.scd
[click]: https://codeberg.org/dnkl/yambar/src/commit/d6e7710a7ebd0be1f2dba677394f5b30b3e52a4f/doc/yambar-particles.5.scd#L87-L102
[yay]: https://github.com/Jguer/yay
[paru]: https://github.com/Morganamilo/paru
[Nix package]: https://search.nixos.org/packages?show=yambar-hyprland-wses

# License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
