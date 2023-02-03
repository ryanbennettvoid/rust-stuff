use std::borrow::Borrow;
use crate::enemy::{find_suitable_bond_enemy, Enemy};
use crate::enemy_bond::EnemyBond;
use crate::math::rect2::Rect2;
use crate::math::vec2::Vec2;
use crate::player::{MoveDirection, Player, PlayerEvent};
use crate::score::Score;
use crate::time_manager::TimeManager;
use crate::traits::drawer::IDrawer;
use crate::traits::updater::IUpdater;
use crate::utils::{find_random_item, generate_random_spawn_points};
use crate::window::{Window, WindowResolution};
use crate::AudioManager;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::cell::RefCell;
use std::process::exit;
use std::time::{Duration, Instant};

pub struct Game<'a> {
    time_manager: &'a mut TimeManager,
    audio_manager: &'a AudioManager<'a>,
    window: &'a mut Window<'a>,
    player: Player,
    enemies: Vec<Enemy>,
    enemy_bonds: Vec<EnemyBond>,
    player_events_buf: RefCell<Vec<PlayerEvent>>,
    game_over_at: Option<Instant>,
    fade_progress: Option<f64>,
    score: Score,
}

impl<'a> Game<'a> {
    pub fn init(
        time_manager: &'a mut TimeManager,
        audio_manager: &'a AudioManager<'a>,
        window: &'a mut Window<'a>,
    ) -> Self {
        println!("Game.new()");

        let (player, enemies, enemy_bonds) =
            Game::create_player_and_enemies(window.get_resolution());

        Self {
            time_manager,
            audio_manager,
            window,
            player,
            enemies,
            enemy_bonds,
            player_events_buf: RefCell::new(vec![]),
            game_over_at: None,
            fade_progress: None,
            score: Score::new(),
        }
    }

    // TODO: refactor this so all rounds use the same code path
    pub fn reset(&mut self) {
        let (player, enemies, enemy_bonds) =
            Game::create_player_and_enemies(self.window.get_resolution());
        self.player = player;
        self.enemies = enemies;
        self.enemy_bonds = enemy_bonds;
        self.game_over_at = None;
        self.fade_progress = None;
        self.score = Score::new();
    }

    fn create_player_and_enemies(
        window_resolution: &WindowResolution,
    ) -> (Player, Vec<Enemy>, Vec<EnemyBond>) {
        let (window_width, window_height) = window_resolution.to_floats();

        let initial_player_position = Vec2 {
            x: window_width / 2.0,
            y: window_height * (9.0 / 10.0),
        };
        let player = Player::new(initial_player_position);

        let vertical_scale = 10.0;
        let game_rect = Rect2 {
            x: 0.0,
            y: -(window_height * vertical_scale), // shift position above the screen
            w: window_width,
            h: window_height * vertical_scale, // make taller
        };

        let enemies: Vec<Enemy> = {
            let spawn_points = generate_random_spawn_points(100, game_rect);
            spawn_points
                .iter()
                .map(|point| Enemy::new(point.clone()))
                .collect()
        };

        let enemy_bonds: Vec<EnemyBond> = {
            let mut results = vec![];
            // iterate through game rect sections
            for idx in 1..=vertical_scale as i32 {
                // get all enemies in rect
                let area_rect = Rect2 {
                    x: 0.0,
                    y: -(window_height * idx as f64),
                    w: window_width,
                    h: window_height,
                };
                let enemies_in_rect: Vec<&Enemy> = enemies
                    .iter()
                    .filter(|e| area_rect.contains_point(&e.position))
                    .collect();
                // println!("enemies_in_rect: {}", enemies_in_rect.len());
                let enemy_a = find_random_item(&enemies_in_rect, None).unwrap();
                let enemy_b = find_suitable_bond_enemy(enemy_a, &enemies_in_rect).unwrap();
                let enemy_bond = EnemyBond::new(enemy_a.id, enemy_b.id);
                // println!(
                //     "enemy_bond: {:?} <> {:?}",
                //     enemy_bond.enemy_a, enemy_bond.enemy_b
                // );
                results.push(enemy_bond);
            }
            results
        };

        (player, enemies, enemy_bonds)
    }

    pub fn run(&mut self) {
        self.audio_manager.play_music();
        loop {
            self.time_manager.on_frame_start();
            self.update();
            self.render();
            self.time_manager.on_frame_end();
        }
    }

    fn update(&mut self) {
        match self.game_over_at {
            None => {}
            Some(at) => {
                let total_fade_duration = Duration::from_secs(1);
                let fade_progress = Instant::now().duration_since(at);
                let percentage =
                    fade_progress.as_millis() as f64 / total_fade_duration.as_millis() as f64;
                if percentage >= 1.0 {
                    self.reset();
                    return;
                }
                self.fade_progress = Some(percentage);
            }
        }

        if self.game_over_at != None {
            return;
        }

        self.player_events_buf.borrow_mut().clear();

        self.window.poll_events(|event| match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                exit(0);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                self.player_events_buf
                    .borrow_mut()
                    .push(PlayerEvent::Move(MoveDirection::Left));
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                self.player_events_buf
                    .borrow_mut()
                    .push(PlayerEvent::Move(MoveDirection::Right));
            }
            Event::KeyUp {
                keycode: Some(Keycode::Left),
                ..
            }
            | Event::KeyUp {
                keycode: Some(Keycode::Right),
                ..
            } => self
                .player_events_buf
                .borrow_mut()
                .push(PlayerEvent::Move(MoveDirection::None)),
            _ => {}
        });

        // process events
        for event in self.player_events_buf.borrow().iter() {
            match event {
                PlayerEvent::Move(direction) => {
                    self.player.set_move_direction(direction.clone());
                }
            }
        }

        // update enemies


        // TODO: don't copy vec?
        let enemies_copy = self.enemies.to_vec();
        for enemy in self.enemies.iter_mut() {
            enemy.update(&enemies_copy, &self.time_manager, &self.window);

            // check if player is touching enemy
            if self.game_over_at == None && self.player.get_rect().contains_rect(&enemy.get_rect())
            {
                self.game_over_at = Some(Instant::now());
                continue;
            }
        }

        // update enemy bonds
        for enemy_bond in self.enemy_bonds.iter_mut() {
            enemy_bond.update(&self.enemies, &self.time_manager, &self.window);

            // check if player is touching enemy bond
            if enemy_bond.line_intersects_with_rect(&self.enemies, &self.player.get_rect()) {
                self.game_over_at = Some(Instant::now());
                continue;
            }
        }

        // update player
        self.player
            .update(&self.enemies, &self.time_manager, &self.window);

        // update score
        self.score
            .update(&self.enemies, &self.time_manager, &self.window);

        // end game when all enemies have passed by
        if self.score.get_score() >= self.enemies.len() as u64 {
            self.game_over_at = Some(Instant::now());
        }
    }

    fn render(&mut self) {
        self.window.clear_canvas();

        // draw enemy bonds
        for enemy_bond in self.enemy_bonds.iter() {
            (enemy_bond as &dyn IDrawer).draw(&mut self.window);
        }

        // draw enemies
        for enemy in self.enemies.iter() {
            if self.window.get_rect().contains_rect(&enemy.get_rect()) {
                (enemy as &dyn IDrawer).draw(&mut self.window);
            }
        }

        // draw player
        (&self.player as &dyn IDrawer).draw(&mut self.window);

        // draw score
        (&self.score as &dyn IDrawer).draw(&mut self.window);
        //
        // fade on game over
        match self.fade_progress {
            None => {}
            Some(fade_amount) => self.window.fade_overlay(fade_amount),
        }

        self.window.present();
    }
}
