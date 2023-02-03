use crate::math::rect2::Rect2;
use crate::window::Window;

pub trait IDrawer {
    fn get_rect(&self) -> Rect2;
    fn draw(&self, window: &mut Window);
}
