use crate::enemy::Enemy;
use crate::time_manager::TimeManager;
use crate::window::Window;

pub trait IUpdater {
    fn update(&mut self, enemies: &Vec<Enemy>, time_manager: &TimeManager, window: &Window);
}
