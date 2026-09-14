# Zellij Idempotent Fullscreen

## About

A Zellij plugin that provides **idempotent fullscreen actions**, allowing you to explicitly set the zoom state to `normal`, `fullscreen`, or `no-ui-fullscreen`.

### Why?

By default, Zellij only exposes `toggle-fullscreen` and `toggle-no-ui-fullscreen`. These actions are state-dependent (toggling based on current view), and are therefore awkward to use in automation or scripting environments.

## installation

### Prebuilt binary

Add to zellij config:

```kdl
// ~/.config/zellij/config.kdl
plugins {
    idempotent_fullscreen location="https://github.com/Torsteinws/zellij-idempotent-fullscreen/releases/download/0.1.0/idempotent-fullscreen.wasm"
}

load_plugins {
    idempotent_fullscreen
}
```

### Build From source

Prerequisites:

- [Rust and cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- `wasm32-wasip1` – can be added with `rustup target add wasm32-wasip1`

Build plugin

```console
git clone https://github.com/Torsteinws/zellij-idempotent-fullscreen
cd zellij-idempotent-fullscreen
cargo build --release
mkdir -p ~/.config/zellij/plugins/
mv target/wasm32-wasip1/release/idempotent-fullscreen.wasm ~/.config/zellij/plugins/
```

Load plugin in zellij config

```kdl
// ~/.config/zellij/config.kdl
plugins {
    idempotent_fullscreen location="file:~/.config/zellij/plugins/idempotent-fullscreen.wasm"
}

load_plugins {
    idempotent_fullscreen
}
```

## Usage

### CLI

Set the focused pane to be normal sized:

```console
zellij action pipe --name "set-normal-screen" --plugin idempotent_fullscreen
```

Set to fullscreen:

```console
zellij action pipe --name "set-fullscreen" --plugin idempotent_fullscreen
```

Set to fullscreen and hide all UI elements:

```console
zellij action pipe --name "set-no-ui-fullscreen" --plugin idempotent_fullscreen
```

### Keybinds

```kdl
// ~/.config/zellij/config.kdl
keybinds {
    shared_except "locked" {

        bind "Ctrl z" {
            MessagePlugin "idempotent_fullscreen" {
                name "set-normal-screen";
            };
        }

        bind "Ctrl Shift z" {
            MessagePlugin "idempotent_fullscreen" {
                name "set-fullscreen";
            };
        }

        bind "Ctrl Shift Alt z" {
            MessagePlugin "idempotent_fullscreen" {
                name "set-no-ui-fullscreen";
            };
        }
    }
}
```

## Development

1. Start zellij,
2. Run:

    ```
    zellij -l zellij.kdl
    ```

The layout includes floating panes for:

- The plugin instance.
- A convenience script to reload the plugin.
