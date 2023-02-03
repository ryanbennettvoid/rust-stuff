use std::borrow::Borrow;
use crate::AudioManager;
use crate::enemy::Enemy;
use crate::math::rect2::Rect2;
use crate::time_manager::TimeManager;
use crate::traits::drawer::IDrawer;
use crate::traits::updater::IUpdater;
use crate::window::Window;

pub struct Score {
    score: u64
}

impl Score {
    pub fn new() -> Self {
        Self {
            score: 0,
        }
    }

    fn calculate_score(&self, enemies: &Vec<Enemy>, window: &Window) -> u64 {
        let mut score = 0;
        for enemy in enemies.iter() {
            if enemy.get_rect().top() > window.get_rect().bottom() {
                score += 1;
            }
        }
        score
    }

    pub fn get_score(&self) -> u64 {
        return self.score;
    }
}

impl IUpdater for Score {
    fn update(&mut self, enemies: &Vec<Enemy>, _time_manager: &TimeManager, window: &Window) {
        let old_score = self.score;
        self.score = self.calculate_score(enemies, window);
        if self.score != old_score {
            window.get_audio_manager().play_blip();
        }
    }
}

impl IDrawer for Score {
    fn get_rect(&self) -> Rect2 {
        todo!()
    }

    fn draw(&self, window: &mut Window) {
        window.draw_text(format!("{}", self.score));
    }
}
