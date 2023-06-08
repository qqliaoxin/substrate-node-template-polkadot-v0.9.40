// 配置要升级的版本
pub const VERSION: u16 = 0;
pub use v0 as version; // 当前版本

pub mod conn;
pub mod v0;
pub mod v1;
pub mod v2;
