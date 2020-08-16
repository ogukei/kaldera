

#[derive(Debug)]
pub enum InputEvent {
    MoveDelta(f32, f32),
    Key(InputKeyEvent),
}

#[derive(Debug)]
pub struct InputKeyEvent {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub is_shift: bool,
    pub is_control: bool,
}
