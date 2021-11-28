#[derive(Debug)]
pub enum Event {
    MousePress(IPoint, mouse::Buttons, key::Mods),
    MouseRelease(IPoint, mouse::Buttons, key::Mods),
    MouseMove(IPoint, mouse::Buttons, key::Mods),

    KeyPress(key::Sym, key::Code, String),
    KeyRelease(key::Sym, key::Code, String),
}
