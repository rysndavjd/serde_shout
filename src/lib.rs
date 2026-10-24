#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;
#[cfg(any(feature = "std", test))]
extern crate std;

mod de;
mod error;
mod ser;

#[cfg(feature = "alloc")]
pub use crate::ser::{to_string, to_vec};
pub use crate::{
    // de::{Deserializer, from_str},
    error::SerError,
    ser::{
        BashCompactFormatter, BashPrettyFormatter, ExportCompactFormatter, ExportPrettyFormatter,
        Formatter, PosixCompactFormatter, PosixPrettyFormatter, Serializer, to_writer,
    },
};

#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("`std` and `no_std` are mutually exclusive");

#[cfg(not(any(feature = "std", feature = "no_std")))]
compile_error!("must enable either `std` or `no_std`");
