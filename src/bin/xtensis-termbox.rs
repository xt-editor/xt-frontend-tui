// This file is part of Xtensis.

// Xtensis is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Xtensis is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Xtensis.  If not, see <http://www.gnu.org/licenses/>

extern crate rustc_serialize;
extern crate docopt;
extern crate xt_tui;
extern crate rustbox;
extern crate time;

#[macro_use]
extern crate log;
extern crate fern;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

use xt_tui::logging::init_logger;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_nodaemon: bool,
    flag_version: bool,
    cmd_new: bool,
}

fn print_version() {
    println!("xtensis-tui version: {}", VERSION);
}

fn get_arguments() -> Args {
    Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit())
}

fn init() {
    // Initialise logging
    init_logger();
    trace!("Logging initialised.");
}

fn main() {
    init();

    info!("xtensis-tui version: {} - starting NOW..", VERSION);

    let rustBox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustBox.print(1,
                  1,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  "Welcome to xtensis.");

    rustBox.print(1,
                  3,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  "Press `q` to quit.");

    rustBox.present();

    loop {
        rustBox.present();
        match rustBox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }

}
