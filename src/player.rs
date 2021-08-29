use rltk::{VirtualKeyCode, Rltk, Point, console};
use specs::prelude::*;
use crate::{CombatStats, RunState};

use super::{Position, Viewshed, Player, Map, State};
use std::cmp::{min, max};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let combats_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        
        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combats_stats.get(*potential_target);
            match target {
                None => {}
                Some(t) => {
                    // todo: implement attacj
                    console::log("player attack");
                    return;
                }
            }

        }
        // todo: check other code according to original
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => { return RunState::Paused } // Nothing happened
        Some(key) => match key {
            // moving straight
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 | 
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            
            VirtualKeyCode::Right | 
            VirtualKeyCode::Numpad6 | 
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up | 
            VirtualKeyCode::Numpad8 | 
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            
            VirtualKeyCode::Down | 
            VirtualKeyCode::Numpad2 | 
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            
            // moving in diagonals
            VirtualKeyCode::Numpad9 |
            VirtualKeyCode::E => try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad7 |
            VirtualKeyCode::Q => try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad3 |
            VirtualKeyCode::C => try_move_player(1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 |
            VirtualKeyCode::Z => try_move_player(-1, 1, &mut gs.ecs),

            _ => { return  RunState::Paused }
        },
    }

    RunState::Running
}