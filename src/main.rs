/*
* This file is part of the Rustaplex application (https://github.com/leirn/rustaplex).
* Copyright (c) 2022 Laurent Vromman <leirn@vromman.org>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, version 3.
*
* This program is distributed in the hope that it will be useful, but
* WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
* General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

mod game;
use clap::Parser;
use game::Game;
use log::info;
use std::env;
use std::thread;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Log level. Can be any value within debug, info, warn, error, critical, none
    #[arg(short, long)]
    loglevel: Option<String>,
    // TODO : add custom level file
}

fn main() {
    let args = Args::parse();

    let loglevel = match args.loglevel {
        Some(level) => {
            if level != String::from("debug")
                && level != String::from("info")
                && level != String::from("warn")
                && level != String::from("error")
                && level != String::from("critical")
            {
                String::from("debug")
            } else {
                level
            }
        }
        None => String::from("debug"),
    };

    env::set_var("RUST_LOG", loglevel);
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    info!("Start Rustaplex 0.1");

    let child = thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
        return Game::new().start();
    }).unwrap();

    let matches = child.join().unwrap();

    //let mut game = Box::new(Game::new());
    //game.start();
}
