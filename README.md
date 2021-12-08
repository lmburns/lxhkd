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
- [ ] Key sequences instead of just key presses (i.e., `alt-l alt-e`)
- [ ] Support ranges `{a-e}` and options `{a,c}`
- [ ] Dynamic reloading of configuration file
- [x] Account for `Lock` modifiers
- [x] Key bindings to shell commands
- [x] Setting `autorepeat_delay` and `autorepeat_interval`

### Configuration
```yaml
### The shell to run the commands in
shell: zsh
### The timeout between key presses
timeout: 300

### The delay in which keys begin to repeat
autorepeat_delay: 300
### The speed in which keys repeat after the delay
 autorepeat_interval: 50

### The file to write the PID to
pid_file: /run/user/1000/lxhkd.pid
### Whether contents should be written to a file
log_to_file: true
### The file to write the log to
log_dir: ${XDG_CONFIG_HOME}/lxhkd/log

### The mapping of keys to shell commands
bindings:
    # Can use any of the modifiers listed below
   super + t: notify-send -a lxhkd "it" "worked"
   # Can use keysym code in hex
   super + [0x61]: notify-send -a lxhkd "this binding" "is super +  a"
   # Can use keysym code
   super + [97]: notify-send -a lxhkd "this binding" "is super +  a"

   # Can use key release events with the tilde `~`
   super + ~t: notify-send -a lxhkd "it" "worked"
   super + ~[0x61]: notify-send -a lxhkd "this binding" "is super +  a"
   super + ~[97]: notify-send -a lxhkd "this binding" "is super +  a"

   # Can use mouse buttons
   shift + mouse2: notify-send -a lxhkd "this binding" "is a mouse button"
   # Can use mouse button release events
   shift + ~mouse2: notify-send -a lxhkd "this binding" "is a mouse button"

   # Can use ranges
   super + {a-c}: notify-send -a lxhkd "this binding" "expands to a, b, and c"
   super + ~{a-c}: notify-send -a lxhkd "this binding" "expands to a, b, and c"

   # Can use option
   super + {a,c}: notify-send -a lxhkd "this binding" "expands to a and c"
   super + ~{a,c}: notify-send -a lxhkd "this binding" "expands to a and c"

### The mappings of keys to other keybindings
remaps:
   Caps_Lock: Hyper_L

### Mappings of modifiers to one key when pressed & another when held down
xcape:
   Caps_Lock: Escape
```

#### Modifiers
The variants of each modifier are listed below. The first three that are lowercase are for convenience,
and the capitalized ones are what the `keysym` strings actually are in the [`KeysymHash`](src/keys/keysym.rs).
The linked file contains all available `keysym`s
```
# Alt
alt,   lalt,      ralt,       Alt_L,     Alt_R,

# Shift
shift, lshift,    rshift,     Shift_L,   Shift_R,

# Super
super, lsuper,    rsuper,     Super_L,   Super_R,

# Meta
meta,  lmeta,     rmeta,      Meta_L,    Meta_R,

# Control
ctrl,  lctrl,     rctrl,      Control_L, Control_R,

# Hyper. This is not a combination of `alt`, `super`, `control`, `shift`
# Instead, this is its own modifier with its own mask
hyper, lhyper,    rhyper,     Hyper_L,   Hyper_R,

# Mod keys. Can be seen with `xmodmap` command
mod1,  mod2,      mod3,       mod4,      mod5,

# Lock modifiers
lock,  Caps_Lock, Shift_Lock, Num_Lock,  Scroll_Lock,

# This is a shortcut for `ctrl` + `shift` + `alt`
# This is not it's own custom modifier and is instead just a combination of the
# three mentioned keys' masks
meh
```
