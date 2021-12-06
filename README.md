## LinuX Hotkey Daemon

The project aims to combine multiple programs into one
    - [`sxhkd`](https://github.com/baskerville/sxhkd.git)
        - Mapping to shell commands (i.e., launching a terminal)
    - [`xcape`](https://github.com/alols/xcape)
        - Remapping one set of keys to another (i.e., `Caps_Lock` to `Escape`)
            - If a key is pressed once, it can be one key
            - If the key is held, it can be another
    - `xset`
        - Set `autorepeat_delay` and `autorepeat_interval` of key presses

### TODO
- [ ] Mouse bindings
- [ ] Key press vs release
- [ ] Key bindings to other keys
- [ ] Account for `Lock` modifiers
- [ ] Key sequences instead of just key presses (i.e., `alt-l alt-e`)
- [ ] Dynamic reloading of configuration file
- [x] Key bindings to shell commands
- [x] Setting `autorepeat_delay` and `autorepeat_interval`
