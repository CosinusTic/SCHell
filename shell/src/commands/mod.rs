mod basic;
mod system;
pub use basic::{echo, greet, grep, ls};
pub use system::{get_RAM_size, l_procfiles, my_pid};
