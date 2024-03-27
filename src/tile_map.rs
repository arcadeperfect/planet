

use crate::types::UMap8;

pub type TileMap = Vec<Vec<Tile>>;
pub trait FromUMap<T: PartialEq> {
    fn from_u_map(from: &Vec<Vec<T>>) -> TileMap;
    fn rooms_planet_combiner(planet: &UMap8, rooms: &UMap8) -> TileMap;
}

impl FromUMap<u8> for TileMap {
    fn from_u_map(from: &Vec<Vec<u8>>) -> TileMap {
        from.iter()
            .map(|row| {
                row.iter()
                    .map(|entry| if *entry == 1 { Tile::Wall } else { Tile::Space })
                    .collect()
            })
            .collect()
    }

    fn rooms_planet_combiner(planet: &UMap8, rooms: &UMap8) -> TileMap {
        assert!(planet.len() == rooms.len());
        assert!(planet[0].len() == rooms[0].len());
        assert!(planet.len() == planet[0].len());

        let rows = planet.len();
        let cols = planet.len();
        let placeholder = Tile::Space;

        let mut out: Vec<Vec<Tile>> = vec![vec![placeholder; cols]; rows];

        for (x, row) in planet.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                if *tile == 0 {
                    out[x][y] = Tile::Space;
                }
                if *tile == 1 {
                    if rooms[x][y] == 0 {
                        out[x][y] = Tile::Wall;
                    }
                    if rooms[x][y] == 1 {
                        out[x][y] = Tile::Room(None);
                    }
                }
            }
        }
        out
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    Space,
    Wall,
    Surface,
    Room(Option<u16>),
    RoomEdge(u16),
    RoomCenter(u16),
}

pub trait TileMapDebug {
    fn debug_print_coords(&self);
    fn debug_print(&self);
}

impl TileMapDebug for Vec<Vec<Tile>> {
    fn debug_print_coords(&self) {

        println!("");

        let mut result = String::new();
        result.push_str("tile map: \n");
        for (_y, row) in self.iter().enumerate() {
            for (_x, _tile) in row.iter().enumerate() {
                // let s_x = format!("{:02}", x);
                // let s_y = format!("{:02}", y);
                // result.push_str(&s_x);
                // result.push(',');
                // result.push_str(&s_y);
                // result.push(' ');
                // result.push(tile.x)
            }
            result.push('\n');
        }

        println!("{}", result);
    }

    fn debug_print(&self) {
        let mut result = String::new();
        result.push_str("tile map: \n");
        for (y, column) in self.iter().enumerate() {
            for (x, _) in column.iter().enumerate() {
                let tile = self[x][y];
                let symbol = match tile {
                    Tile::Space => " ".to_string(),
                    Tile::Wall => ".".to_string(),
                    Tile::Surface => "*".to_string(),
                    Tile::Room(value) => match value {
                        Some(v) => (format!("{:1} ", v)).trim().to_string(),
                        None => "!".to_string(),
                    },
                    Tile::RoomEdge(_) => "e".to_string(),
                    Tile::RoomCenter(_) => {

                        "c".to_string()
                    },
                };
                result.push_str(&symbol);
                result.push(' '); // Add space after each symbol
            }
            result.push('\n');
        }

        println!("{}", result);
    }
}

pub trait MapDebug {
    fn debug_print_raw(&self);
    fn debug_print_pretty(&self);
}

impl MapDebug for UMap8 {
    fn debug_print_raw(&self) {
        let mut result = String::new();

        let f = format!("UMap8 debug: \n length: {}\n", self.len());
        result.push_str(&f);

        for row in self.iter() {
            for tile in row.iter() {
                let v = *tile.min(&9);
                let c = char::from_digit(v as u32, 10).unwrap_or(' '); // Clamped at 9 and replaced with space if out of range
                result.push_str(format!("{} ", c).as_str());
            }
            result.push('\n');
        }
        println!("{}", result);
    }

    fn debug_print_pretty(&self) {
        let mut result = String::new();

        let f = format!("UMap8 debug pretty: \n length: {}\n", self.len());
        result.push_str(&f);

        for x in 0..self.len() {
            for y in 0..self[x].len() {
                let value = self[y][x];
                match value {
                    0 => result.push('.'),
                    1 => result.push('#'),
                    _ => result.push('!'),
                }
                result.push(' ');
            }
            result.push('\n');
        }
        println!("{}", result);

        // for row in self.iter() {
        //     for tile in row.iter() {
        //         match tile {
        //             0 => result.push('.'),
        //             1 => result.push('#'),
        //             _ => result.push('!'),
        //         }
        //         result.push(' ');
        //     }
        //     result.push('\n');
        // }
        // println!("{}", result);
    }
}
