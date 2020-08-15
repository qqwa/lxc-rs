//! Safe and ergonomic wrapper for [lxc](https://github.com/lxc/lxc) library.
//!
//! # Examples
//! ```rust
//! # fn inner_main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut c = lxc_rs::Container::new("playtime", lxc_rs::Path::Default)?;
//! if c.is_running() {
//!     println!("Container is running");
//! }
//! # Ok(())
//! # }
//! ```

mod container;
mod error;
mod functions;
mod helper;

pub use container::Container;
pub use error::Error;
pub use functions::*;
pub use helper::Path;

type LxcResult<T> = std::result::Result<T, Error>;
type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
