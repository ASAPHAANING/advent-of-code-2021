#[derive(Debug, Copy, Clone)]
pub enum Command {
    NOOP,
    FORWARD(i32),
    DOWN(i32),
    UP(i32),
}

pub trait Commandable {
    fn navigate(&self, cmd: Command) -> Self;
    fn get_position(&self) -> i32;
}
