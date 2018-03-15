extern crate sdl2;

use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::rect::Point;
use sdl2::EventPump;

use scene::color::Color;

pub struct Window {
    sdl: Sdl,
    canvas: Canvas<sdl2::video::Window>,
    width: u32,
    height: u32,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video.window("trace_ray", width, height)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas()
            .build()
            .unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let min_x = -(width as i32 / 2);
        let max_x = width as i32 / 2;
        let min_y = -(height as i32 / 2);
        let max_y = height as i32 / 2;
        Window { sdl, canvas, width, height, min_x, max_x, min_y, max_y }
    }

    pub fn get_max_x(&self) -> i32 {
        self.max_x
    }

    pub fn get_min_x(&self) -> i32 {
        self.min_x
    }

    pub fn get_max_y(&self) -> i32 {
        self.max_y
    }

    pub fn get_min_y(&self) -> i32 {
        self.min_y
    }

    pub fn get_events(&mut self) -> EventPump {
        self.sdl.event_pump().unwrap()
    }

    pub fn set_point(&mut self, x: i32, y: i32, color: Color) {
        let dot = self.to_buffer_cord(Dot::new(x, y));
        if dot.x < 0 || dot.x >= self.width as i32 {
            return;
        }
        if dot.y < 0 || dot.y >= self.height as i32 {
            return;
        }
        let x = dot.x;
        let y = dot.y;
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(color.r, color.g, color.b));
        self.canvas.draw_point(Point::new(x, y)).unwrap();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    fn to_buffer_cord(&self, dot: Dot) -> Dot {
        let width: i32 = self.width as i32;
        let height: i32 = self.height as i32;
        let x = dot.x + width / 2;
        let y = height - (dot.y + height / 2);
        Dot::new(x, y)
    }
}

pub struct Dot {
    pub x: i32,
    pub y: i32,
}

impl Dot {
    pub fn new(x: i32, y: i32) -> Dot {
        Dot { x, y }
    }
}