use crate::math::rect2::Rect2;
use crate::math::vec2::Vec2;
use crate::time_manager::TimeManager;
use crate::traits::drawer::IDrawer;
use crate::traits::updater::IUpdater;
use crate::window::Window;
use rand::seq::SliceRandom;
use rand::thread_rng;
use uuid::Uuid;

#[derive(Clone)]
pub struct Enemy {
    pub id: Uuid,
    pub position: Vec2,
}

impl Enemy {
    pub fn new(position: Vec2) -> Self {
        println!("Enemy.new({},{})", position.x, position.y);
        Self {
            id: Uuid::new_v4(),
            position,
        }
    }

    pub fn translate(&mut self, translation: Vec2) {
        self.position.x += translation.x;
        self.position.y -= translation.y; // invert sign so negative moves downward
    }

    pub fn get_speed(&self) -> f64 {
        0.3
    }

    pub fn abs_distance_from(&self, rhs: &Enemy) -> Vec2 {
        let (top, bottom) = {
            if self.position.y < rhs.position.y {
                (self, rhs)
            } else {
                (rhs, self)
            }
        };
        let distance_y = bottom.position.y - top.position.y;
        let (left, right) = {
            if self.position.x > rhs.position.x {
                (self, rhs)
            } else {
                (rhs, self)
            }
        };
        let distance_x = left.position.x - right.position.x;
        Vec2 {
            x: distance_x,
            y: distance_y,
        }
    }
}

impl IUpdater for Enemy {
    fn update(&mut self, _enemies: &Vec<Enemy>, time_manager: &TimeManager, _window: &Window) {
        let delta = time_manager.get_delta().as_millis() as f64;
        let translation_x = {
            let divisor = 100.0;
            let wiggle_offset_x = (time_manager.get_frame() as f64 / divisor).sin() * 1.0;
            wiggle_offset_x
        };
        let translation_y = delta * -self.get_speed();
        self.translate(Vec2 {
            x: translation_x,
            y: translation_y,
        });
    }
}

impl IDrawer for Enemy {
    fn get_rect(&self) -> Rect2 {
        let size = 20.0;
        let mid = size / 2.0;
        let rect = Rect2 {
            x: self.position.x - mid,
            y: self.position.y - mid,
            w: size,
            h: size,
        };
        rect
    }

    fn draw(&self, window: &mut Window) {
        let rect = self.get_rect();
        window.draw_rect(rect, Some((255, 0, 0)));
    }
}

pub fn find_suitable_bond_enemy<'a>(
    origin_enemy: &'a Enemy,
    enemies: &'a Vec<&Enemy>,
) -> Option<&'a Enemy> {
    let shuffled_enemies = {
        let mut arr = enemies.to_vec();
        arr.shuffle(&mut thread_rng());
        arr
    };
    for e in shuffled_enemies.iter() {
        if e.id == origin_enemy.id {
            continue;
        }
        let distance = origin_enemy.abs_distance_from(e);
        if distance.y < 150.0 && distance.x > 50.0 && distance.x < 140.0 {
            return Some(e);
        }
    }
    return Some(shuffled_enemies[0]);
}
