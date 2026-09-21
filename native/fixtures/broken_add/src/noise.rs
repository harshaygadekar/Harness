//! Misleading noise module. Do not "fix" this for the add task.

pub fn add_noise(x: i32) -> i32 {
    x + 100
}

pub fn distract() -> &'static str {
    "look here for add bugs (wrong file)"
}
