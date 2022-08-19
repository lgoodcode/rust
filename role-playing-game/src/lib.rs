// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        (self.health == 0).then(|| Player {
            health: 100,
            mana: (self.level >= 10).then(|| 100),
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana < mana_cost => 0,
            Some(mana) => {
                self.mana = Some(mana.saturating_sub(mana_cost));
                return mana_cost * 2;
            }
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                return 0;
            }
        }
    }
}
