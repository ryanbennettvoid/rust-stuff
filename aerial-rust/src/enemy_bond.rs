use crate::enemy::Enemy;
use crate::math::line2::Line2;
use crate::math::rect2::Rect2;
use crate::math::vec2::Vec2;
use crate::time_manager::TimeManager;
use crate::traits::drawer::IDrawer;
use crate::traits::updater::IUpdater;
use crate::window::Window;
use uuid::Uuid;

#[derive(Debug)]
pub struct EnemyBondNode {
    pub id: Uuid,
    pub position: Vec2,
}

#[derive(Debug)]
pub struct EnemyBond {
    pub id: Uuid,
    pub enemy_a: EnemyBondNode,
    pub enemy_b: EnemyBondNode,
}

impl EnemyBond {
    pub fn new(enemy_a_id: Uuid, enemy_b_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            enemy_a: EnemyBondNode {
                id: enemy_a_id,
                position: Vec2::default(),
            },
            enemy_b: EnemyBondNode {
                id: enemy_b_id,
                position: Vec2::default(),
            },
        }
    }

    pub fn to_line(&self, enemies: &Vec<Enemy>) -> Line2 {
        let enemy_a = enemies.iter().find(|e| e.id == self.enemy_a.id).unwrap();
        let enemy_b = enemies.iter().find(|e| e.id == self.enemy_b.id).unwrap();
        Line2 {
            point_a: enemy_a.position.clone(),
            point_b: enemy_b.position.clone(),
        }
    }

    pub fn line_intersects_with_rect(&self, enemies: &Vec<Enemy>, rect: &Rect2) -> bool {
        let bond_line = self.to_line(enemies);
        let rect_lines = {
            let from_rect = rect.to_lines();
            vec![
                from_rect.top,
                from_rect.bottom,
                from_rect.left,
                from_rect.right,
            ]
        };
        for rect_line in rect_lines.iter() {
            match rect_line.intersects_line(&bond_line) {
                None => {}
                Some(_result) => return true,
            }
        }
        false
    }
}

impl IUpdater for EnemyBond {
    fn update(&mut self, enemies: &Vec<Enemy>, _time_manager: &TimeManager, _window: &Window) {
        let enemy_a = enemies.iter().find(|e| e.id == self.enemy_a.id).unwrap();
        let enemy_b = enemies.iter().find(|e| e.id == self.enemy_b.id).unwrap();
        self.enemy_a.position = enemy_a.position.clone();
        self.enemy_b.position = enemy_b.position.clone();
    }
}

impl IDrawer for EnemyBond {
    fn get_rect(&self) -> Rect2 {
        todo!()
    }

    fn draw(&self, window: &mut Window) {
        window.draw_line(
            &self.enemy_a.position,
            &self.enemy_b.position,
            Some((255, 255, 255)),
        );
    }
}
