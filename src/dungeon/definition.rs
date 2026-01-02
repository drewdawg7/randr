use std::collections::HashMap;

use crate::{dungeon::enums::RoomType, entities::mob::MobId};



#[derive(Debug)]
pub struct Dungeon {
    pub name: String,
    pub rooms: Vec<Vec<DungeonRoom>>,
    pub mob_table: HashMap<MobId, i32>,
}


impl Dungeon {

    pub fn get_neighbors(&self, room: &DungeonRoom) -> Vec<Option<&DungeonRoom>> {
        let x = room.x;
        let y = room.y;
        vec![
            self.get_room(x - 1, y),
            self.get_room(x, y - 1),
            self.get_room(x + 1, y),
            self.get_room(x, y + 1)
        ]
    }

    pub fn get_room(&self, x: i32, y: i32) -> Option<&DungeonRoom> {
        if x < 0 || (x >= (self.rooms.len() as i32)) {  return None }
        else if y < 0 || y > (self.rooms[x as usize].len() as i32) {return None;}
        Some(&self.rooms[x as usize][y as usize])
        
    }

//    pub fn intiate_fight(&self, &player) {

  //  }
}

#[derive(Debug)]
pub struct DungeonRoom {
   room_type: RoomType,
   is_cleared: bool,
   x: i32,
   y: i32,
}
