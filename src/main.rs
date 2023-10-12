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
use game::Game;
use log::info;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    info!("Start Rustaplex 0.1");

    let mut game = Game::new();
    game.start();
}
