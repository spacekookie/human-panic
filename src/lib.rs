#![feature(external_doc)]
#![doc(include = "../README.md")]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

extern crate console;
extern crate failure;
extern crate unindent;

use console::style;
use std::panic::PanicInfo;
use unindent::unindent;

/// Setup the human panic hook that will make all panics
/// as beautiful as your shitty code.
#[macro_export]
macro_rules! setup_panic {
  () => {
    use std::env;
    use std::panic::{self, PanicInfo};
    use human_panic::*;
    let _version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    let authors = env!("CARGO_PKG_AUTHORS");
    let homepage = env!("CARGO_PKG_HOMEPAGE");

    panic::set_hook(Box::new(move |info: &PanicInfo| {
      print_msg(_version, name, authors, homepage);
      handle_dump(info);
    }));
  };
}

/// Utility function to print a pretty message for our human users
pub fn print_msg(_version: &str, name: &str, authors: &str, homepage: &str) {
  let mut msg = unindent(&format!(r#"
      Well, this is embarrasing.

      {} had a problem and crashed. To help us diagnose the problem you can send us a crash report.

      We have generated a report file at "<reports not generated yet>". Submit an issue or email with the subject of "{} Crash Report" and include the report as an attachment.
    "#, name, name));
  msg.push_str("\n");

  if !homepage.is_empty() {
    msg.push_str(&format!("- Homepage: {}\n", homepage));
  }
  if !authors.is_empty() {
    msg.push_str(&format!("- Authors: {}\n", authors));
  }
  msg.push_str("\nWe take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.\n");
  msg.push_str("\nThank you kindly!");
  eprintln!("{}", style(msg).red());
}

/// Utility function which will handle dumping information to disk
/// TODO: Implement
pub fn handle_dump(_panic_info: &PanicInfo) {}
