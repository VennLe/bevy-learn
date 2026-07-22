pub mod core_plugin;

// 重新导出常用类型，让外部可以直接通过 player::Player 访问
pub use core_plugin::*;