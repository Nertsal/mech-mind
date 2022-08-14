use super::*;

use model::*;
use render::Render;

#[allow(dead_code)]
pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    render: Render,
    model: Model,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            render: Render::new(geng, assets),
            model: Model::new(assets),
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        self.render.draw(&self.model, framebuffer);
    }

    fn handle_event(&mut self, event: geng::Event) {
        #[allow(clippy::single_match)]
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::G => {
                    self.model.spawn_unit(
                        self.model.templates.tank.clone(),
                        vec2(0.0, 5.0).map(Coord::new),
                        Faction::Mech,
                    );
                }
                geng::Key::H => {
                    self.model.spawn_unit(
                        self.model.templates.blighter.clone(),
                        vec2(10.0, 5.0).map(Coord::new),
                        Faction::Alien,
                    );
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as _);
        self.model.update(delta_time);
    }
}
