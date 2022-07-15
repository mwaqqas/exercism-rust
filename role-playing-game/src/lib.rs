// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => match self.level {
                0..=9 => Some(Player {
                    health: 100,
                    mana: None,
                    level: self.level,
                }),
                _ => Some(Player {
                    health: 100,
                    mana: Some(100),
                    level: self.level,
                }),
            },
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                match self.health > mana_cost {
                    true => {
                        self.health = self.health - mana_cost;
                    }
                    false => {
                        self.health = 0;
                    }
                }
                return 0;
            }

            Some(current_mana) => match mana_cost > current_mana {
                true => {
                    return 0;
                }
                false => {
                    self.mana = Some(current_mana - mana_cost);
                    return mana_cost * 2;
                }
            },
        }
    }
}
