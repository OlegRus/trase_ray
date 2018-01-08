extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod scene;

use scene::vector::Vector;
use scene::scene::Scene;
use scene::scene::Viewport;
use scene::scene::Sphere;
use scene::scene::Light;
use scene::scene::LightType;

mod window;

use window::window::Window;
use scene::color::Color;

fn main() {
    let width = 768;
    let height = 768;

    let s1 = Sphere::new(Vector::new(-1.6, -0.25, 7.), 0.9, Color::new(0xFF, 0x00, 0x33), 1000., 0.7);
    let s2 = Sphere::new(Vector::new(1.6, -0.25, 7.), 0.9, Color::new(0xFF, 0x99, 0x00), 1000., 0.7);
    let s3 = Sphere::new(Vector::new(0., -0.9, 6.), 0.7, Color::new(0x00, 0xFF, 0x00), 80., 0.45);
    let s4 = Sphere::new(Vector::new(0.4, -5001., 0.), 5000., Color::new(0x00, 0x00, 0xFF), 10., 0.3);
    let spheres = vec! {s1, s2, s3, s4};

    let viewport = Viewport::new(1., width, height);

    let l1 = Light::new(LightType::AMBIENT, 0.1, Vector::new(0., 0., 0.));
    let l2 = Light::new(LightType::DIRECTIONAL, 0.2, Vector::new(-1.5, 0.8, -1.5));
    let l3 = Light::new(LightType::POINT, 0.7, Vector::new(0., 0.5, 6.5));
    let lights = vec! {l1, l2, l3};

    let scene = Scene::new(viewport, spheres, lights);
    let mut window = Window::new(width, height);
    for x in window.get_min_x()..window.get_max_x() + 1 {
        for y in window.get_min_y()..window.get_max_y() + 1 {
            window.set_point(x, y, scene.get_color_for_window_cords(x, y));
        }
    }
    window.present();
    let mut events = window.get_events();
    'lock: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'lock,
                _ => {}
            }
        }
    }
}
