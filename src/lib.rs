//! # Pretty Assertions
//!
//! When writing tests in Rust, you'll probably use `assert_eq!(a, b)` _a lot_.
//!
//! If such a test fails, it will present all the details of `a` and `b`.
//! But you have to spot the differences yourself, which is not always straightforward,
//! like here:
//!
//! ![standard assertion](https://raw.githubusercontent.com/colin-kiegel/rust-pretty-assertions/v0.6.1/examples/standard_assertion.png)
//!
//! Wouldn't that task be _much_ easier with a colorful diff?
//!
//! ![pretty assertion](https://raw.githubusercontent.com/colin-kiegel/rust-pretty-assertions/v0.6.1/examples/pretty_assertion.png)
//!
//! Yep — and you only need **one line of code** to make it happen:
//!
//! ```rust,ignore
//! use pretty_assertions::{assert_eq, assert_ne};
//! ```
//!
//! <details>
//! <summary>Show the example behind the screenshots above.</summary>
//!
//! ```rust,ignore
//! // 1. add the `pretty_assertions` dependency to `Cargo.toml`.
//! // 2. insert this line at the top of each module, as needed
//! use pretty_assertions::{assert_eq, assert_ne};
//!
//! fn main() {
//!     #[derive(Debug, PartialEq)]
//!     struct Foo {
//!         lorem: &'static str,
//!         ipsum: u32,
//!         dolor: Result<String, String>,
//!     }
//!
//!     let x = Some(Foo { lorem: "Hello World!", ipsum: 42, dolor: Ok("hey".to_string())});
//!     let y = Some(Foo { lorem: "Hello Wrold!", ipsum: 42, dolor: Ok("hey ho!".to_string())});
//!
//!     assert_eq!(x, y);
//! }
//! ```
//! </details>
//!
//! ## Tip
//!
//! Specify it as [`[dev-dependencies]`](http://doc.crates.io/specifying-dependencies.html#development-dependencies)
//! and it will only be used for compiling tests, examples, and benchmarks.
//! This way the compile time of `cargo build` won't be affected!
//!
//! Also add `#[cfg(test)]` to your `use` statements, like this:
//!
//! ```rust,ignore
//! #[cfg(test)]
//! use pretty_assertions::{assert_eq, assert_ne};
//! ```
//!
//! ## Note
//!
//! * Since `Rust 2018` edition, you need to declare
//!   `use pretty_assertions::{assert_eq, assert_ne};` per module.
//!   Before you would write `#[macro_use] extern crate pretty_assertions;`.
//! * The replacement is only effective in your own crate, not in other libraries
//!   you include.
//! * `assert_ne` is also switched to multi-line presentation, but does _not_ show
//!   a diff.

extern crate ansi_term;
extern crate diffus;

#[cfg(windows)]
extern crate ctor;
#[cfg(windows)]
extern crate output_vt100;

mod format_changeset;

use std::fmt::{self, Debug, Display};

use crate::format_changeset::format_changeset;
pub use ansi_term::Style;

#[cfg(windows)]
use ctor::*;
#[cfg(windows)]
#[ctor]
fn init() {
    output_vt100::try_init().ok(); // Do not panic on fail
}

/// A comparison of two values.
///
/// Where both values implement `Debug`, the comparison can be displayed as a pretty diff.
///
/// ```
/// use pretty_assertions::Comparison;
///
/// print!("{}", Comparison::new(&123, &134));
/// ```
///
/// The values may have different types, although in practice they are usually the same.
pub struct Comparison<'a, TLeft, TRight> {
    left: &'a TLeft,
    right: &'a TRight,
}

impl<'a, TLeft, TRight> Comparison<'a, TLeft, TRight> {
    pub fn new(left: &'a TLeft, right: &'a TRight) -> Comparison<'a, TLeft, TRight> {
        Comparison { left, right }
    }
}

impl<'a, TLeft, TRight> Display for Comparison<'a, TLeft, TRight>
where
    TLeft: Debug,
    TRight: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // To diff arbitary types, render them as debug strings
        let left_debug = format!("{:#?}", self.left);
        let right_debug = format!("{:#?}", self.right);
        // And then diff the debug output
        format_changeset(f, &left_debug, &right_debug)
    }
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr , $right:expr,) => ({
        assert_eq!($left, $right)
    });
    ($left:expr , $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!("assertion failed: `(left == right)`\
                          \n\
                          \n{}\
                          \n",
                           $crate::Comparison::new(left_val, right_val))
                }
            }
        }
    });
    ($left:expr , $right:expr, $($arg:tt)*) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!("assertion failed: `(left == right)`: {}\
                          \n\
                          \n{}\
                          \n",
                           format_args!($($arg)*),
                           $crate::Comparison::new(left_val, right_val))
                }
            }
        }
    });
}

#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr) => ({
        assert_ne!(@ $left, $right, "", "");
    });
    ($left:expr, $right:expr,) => ({
        assert_ne!(@ $left, $right, "", "");
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        assert_ne!(@ $left, $right, ": ", $($arg)+);
    });
    (@ $left:expr, $right:expr, $maybe_semicolon:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                  let left_dbg = format!("{:?}", *left_val);
                  let right_dbg = format!("{:?}", *right_val);
                  if left_dbg != right_dbg {

                      panic!("assertion failed: `(left != right)`{}{}\
                            \n\
                            \n{}\
                            \n{}: According to the `PartialEq` implementation, both of the values \
                              are partially equivalent, even if the `Debug` outputs differ.\
                            \n\
                            \n",
                             $maybe_semicolon,
                             format_args!($($arg)+),
                             $crate::Comparison::new(left_val, right_val),
                             $crate::Style::new()
                                 .bold()
                                 .underline()
                                 .paint("Note"))
                  }

                  panic!("assertion failed: `(left != right)`{}{}\
                        \n\
                        \n{}:\
                        \n{:#?}\
                        \n\
                        \n",
                         $maybe_semicolon,
                         format_args!($($arg)+),
                         $crate::Style::new().bold().paint("Both sides"),
                         left_val)
                }
            }
        }
    });
}
