use crate::enemy::Enemy;
use crate::math::rect2::Rect2;
use crate::math::vec2::Vec2;
use crate::time_manager::TimeManager;
use crate::traits::drawer::IDrawer;
use crate::traits::updater::IUpdater;
use crate::utils::double_clamp;
use crate::window::Window;

pub enum MoveDirection {
    Left,
    Right,
    None,
}

impl Clone for MoveDirection {
    fn clone(&self) -> Self {
        match self {
            MoveDirection::Left => MoveDirection::Left,
            MoveDirection::Right => MoveDirection::Right,
            MoveDirection::None => MoveDirection::None,
        }
    }
}

pub enum PlayerEvent {
    Move(MoveDirection),
}

pub struct Player {
    position: Vec2,
    direction: MoveDirection,
}

impl Player {
    pub fn new(initial_player_position: Vec2) -> Self {
        println!("Player.new()");
        Self {
            position: initial_player_position,
            direction: MoveDirection::None,
        }
    }

    pub fn get_speed(&self) -> f64 {
        0.3
    }

    pub fn set_move_direction(&mut self, direction: MoveDirection) {
        self.direction = direction
    }
}

impl IUpdater for Player {
    fn update(&mut self, _enemies: &Vec<Enemy>, time_manager: &TimeManager, window: &Window) {
        let delta = time_manager.get_delta().as_millis() as f64;
        let dir = match self.direction {
            MoveDirection::Left => -1.0,
            MoveDirection::Right => 1.0,
            MoveDirection::None => 0.0,
        };
        let offset = delta * self.get_speed() * dir;
        let new_position = self.position.x + offset;

        let (left, right) = {
            let r = window.get_rect();
            let padding = self.get_rect().w / 2.0;
            (r.left() + padding, r.right() - padding)
        };

        self.position.x = double_clamp(new_position, left, right);
    }
}

impl IDrawer for Player {
    fn get_rect(&self) -> Rect2 {
        let size = 40.0;
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
        // window.draw_rect(rect, Some((255, 255, 255)));
        let pos = Vec2 {
            x: rect.x,
            y: rect.y,
        };
        let size = Vec2 {
            x: rect.w,
            y: rect.h,
        };
        window.draw_player_surface(&pos, Some(&size));
    }
}
