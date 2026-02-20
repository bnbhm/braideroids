pub mod level1;
pub mod level2;
pub mod level3;
pub mod menu;

pub enum Level {
    Lvl1,
    Lvl2,
    Lvl3,
}

struct LevelStruct {
    level_initialised: bool,
}

impl Level {
    fn level_init(&self) {
        todo!();
    }

    fn level_loop(&self) {
        todo!();
    }
}

/* impl Level for Level {
    fn level_init(&self) {
        match self {
            GameLevel::Lvl1 => {
                lvl1_init();
            }
            GameLevel::Lvl2 => {
                lvl2_init();
            }
        }
    }

    fn level_loop(&self) {}
} */
