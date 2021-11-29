## LinuX Hotkey Daemon

This is a work in progress.

The project aims to combine multiple programs into one
    * [`sxhkd`](https://github.com/baskerville/sxhkd.git)
    * [`xcape`](https://github.com/alols/xcape)
    * `xset`

### Goal
It will allow for:
    * Keybindings and Mouse bindings
        * Mapping to shell commands (i.e., launching a terminal)
        * Remapping one set of keys to another (i.e., `Caps_Lock` to `Escape`)
            * If a key is pressed once, it can be one key
            * If the key is held, it can be another
    * Setting initial key-repeat rate and key-repeat rate when held down
