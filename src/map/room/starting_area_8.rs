use crate::dice;
use crate::grid::Grid as Grid;
use crate::grid::Node as Node;
use crate::map::room::place_door as place_door;
use crate::map::room::place_passage as place_passage;
use crate::map::room::place_secret_door as place_secret_door;
use crate::map::room::Room as Room;
use crate::map::room::Wall as Wall;
use crate::tile::Tile as Tile;
use crate::tile::TileIcon as TileIcon;
use crate::tile::TileKind as TileKind;

use rand::Rng;

// https://github.com/Ronatos/rungeon/wiki/Room#starting-area-8
pub fn new() -> Room {
    let wall = Node::Tile(Tile {
        kind: TileKind::Wall,
        icon: TileIcon::Wall
    });
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });
    let exits = vec![Wall::North, Wall::South, Wall::East, Wall::West];

    let mut starting_area8 = Grid::new(8, vec![
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone()
    ]);

    // Repeat exactly 4 times.
    // 1. Select a wall from the remaining empty walls at random.
    // 2. Build a door
    // 3. Remove whichever wall is selected from the list of available walls.

    let mut exits_to_build = 4;
    let mut num_doors = 0;
    let mut num_passages = 0;
    let mut num_secret_doors = 0;
    let mut empty_walls = vec![Wall::North, Wall::South, Wall::East, Wall::West];
    let mut rng = rand::thread_rng();
    while exits_to_build > 0 {
        let wall_index = rng.gen_range(0..empty_walls.len());
        let wall_selection = &empty_walls[wall_index];

        if num_passages != 1 {
            match wall_selection {
                Wall::North => {
                    if dice::roll(12) <= 2 { // This will be a 5ft wide passage
                        starting_area8 = place_passage(starting_area8, Wall::North, 5);
                    }
                    else { // This will be a 10ft wide passage
                        starting_area8 = place_passage(starting_area8, Wall::North, 10);
                    }
                },
                Wall::South => {
                    if dice::roll(12) <= 2 { // This will be a 5ft wide passage
                        starting_area8 = place_passage(starting_area8, Wall::South, 5);
                    }
                    else { // This will be a 10ft wide passage
                        starting_area8 = place_passage(starting_area8, Wall::South, 10);
                    }
                },
                Wall::East => {
                    if dice::roll(12) <= 2 { // This will be a 5ft wide passage
                        starting_area8 = place_passage(starting_area8, Wall::East, 5);
                    }
                    else { // This will be a 10ft wide passage
                        starting_area8 = place_passage(starting_area8, Wall::East, 10);
                    }
                },
                Wall::West => {
                    if dice::roll(12) <= 2 { // This will be a 5ft wide passage
                        starting_area8 = place_passage(starting_area8, Wall::West, 5);
                    }
                    else { // This will be a 10ft wide passage
                        starting_area8 = place_passage(starting_area8, Wall::West, 10);
                    }
                }
            }
            num_passages = num_passages + 1;
        }
        else if num_doors != 2 {
            match wall_selection {
                Wall::North => {
                    starting_area8 = place_door(starting_area8, Wall::North);
                },
                Wall::South => {
                    starting_area8 = place_door(starting_area8, Wall::South);
                },
                Wall::East => {
                    starting_area8 = place_door(starting_area8, Wall::East);
                },
                Wall::West => {
                    starting_area8 = place_door(starting_area8, Wall::West);
                }
            }
            num_doors = num_doors + 1;
        }
        else if num_secret_doors != 1 {
            match wall_selection {
                Wall::North => {
                    starting_area8 = place_secret_door(starting_area8, Wall::North);
                },
                Wall::South => {
                    starting_area8 = place_secret_door(starting_area8, Wall::South);
                },
                Wall::East => {
                    starting_area8 = place_secret_door(starting_area8, Wall::East);
                },
                Wall::West => {
                    starting_area8 = place_secret_door(starting_area8, Wall::West);
                }
            }
            num_secret_doors = num_secret_doors + 1;
        }

        empty_walls.remove(wall_index);
        exits_to_build = exits_to_build - 1;
    }

    Room::new(starting_area8, exits)
}