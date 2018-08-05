// Design Patterns by GoF

pub use self::command::command;
pub use self::state::state;
pub use self::strategy::strategy;
pub use self::template_method::template_method;
pub use self::memento::memento;
pub use self::observer::observer;

mod command;
mod state;
mod strategy;
mod template_method;
mod memento;
mod observer;
