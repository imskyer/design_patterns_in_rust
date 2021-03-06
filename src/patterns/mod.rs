// Design Patterns by GoF

pub use self::command::command;
pub use self::iterator::iterator;
pub use self::memento::memento;
pub use self::observer::observer;
pub use self::state::state;
pub use self::strategy::strategy;
pub use self::template_method::template_method;
pub use self::visitor::visitor;
pub use self::mediator::mediator;
pub use self::interpreter::interpreter;
pub use self::builder::builder;
pub use self::prototype::prototype;
pub use self::factory::factory;
pub use self::chain_of_responsibility::cor;
pub use self::proxy::proxy;
pub use self::decorator::decorator;

mod command;
mod iterator;
mod memento;
mod observer;
mod state;
mod strategy;
mod template_method;
mod visitor;
mod mediator;
mod interpreter;
mod builder;
mod prototype;
mod factory;
mod chain_of_responsibility;
mod proxy;
mod decorator;
