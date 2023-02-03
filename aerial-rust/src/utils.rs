use crate::math::rect2::Rect2;
use crate::math::vec2::Vec2;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub fn generate_random_spawn_points(count: u32, rect: Rect2) -> Vec<Vec2> {
    let mut rng = rand::thread_rng();
    let mut positions = vec![];
    for _ in 0..count {
        let x = rng.gen_range(rect.x..(rect.x + rect.w));
        let y = rng.gen_range(rect.y..(rect.y + rect.h));
        positions.push(Vec2 { x, y })
    }
    positions
}

pub fn find_random_item<T: Clone>(items: &Vec<T>, filter: Option<fn(&T) -> bool>) -> Option<&T> {
    if items.len() == 0 {
        return None;
    }
    let mut shuffled_items = items.to_vec();
    shuffled_items.shuffle(&mut thread_rng());
    for item in items.iter() {
        match filter {
            None => {}
            Some(filter_func) => {
                if filter_func(item) {
                    return Some(item);
                }
            }
        }
    }
    Some(&items[0])
}

pub fn double_clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
    if v < min {
        return min;
    }
    if v > max {
        return max;
    }
    return v;
}
