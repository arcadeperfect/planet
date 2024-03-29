use crate::{tile_map::{Status, Tile}, types::{FMap, UMap8}};







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
                        Status::Designated(v) => format!("{:1} ", v).trim().to_string(),
                        Status::Undesignated => "!".to_string(),

                        // Some(v) => (format!("{:1} ", v)).trim().to_string(),
                        // None => "!".to_string(),
                    },
                    Tile::RoomEdge(_) => "e".to_string(),
                    Tile::RoomCenter(_) => {

                        "c".to_string()
                    },
                    Tile::Tunnel(_) => "@".to_string()
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
    fn debug_print(&self);
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

    fn debug_print(&self) {
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


impl MapDebug for FMap {
    fn debug_print_raw(&self) {
        todo!()
        // let mut result = String::new();
        // result.push_str("FMap debug: \n");
        // for row in self.iter() {
        //     for tile in row.iter() {
        //         let v = *tile.min(&9);
        //         let c = char::from_digit(v as u32, 10).unwrap_or(' '); // Clamped at 9 and replaced with space if out of range
        //         result.push_str(format!("{} ", c).as_str());
        //     }
        //     result.push('\n');
        // }
        // println!("{}", result);
    }   

    fn debug_print(&self) {
        let mut result = String::new();
        result.push_str("FMap debug pretty: \n");
        for x in 0..self.len() {
            for y in 0..self[x].len() {
                let value = self[y][x];

                let s = format!("{:.2} ", value);
              

                result.push_str(&s);
            }
            result.push('\n');
        }
        println!("{}", result);
    }
}