use std::f32;
use scene::vector::Vector;
use scene::color::Color;

pub struct Scene {
    camera: Vector,
    viewport: Viewport,
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new(viewport: Viewport, spheres: Vec<Sphere>, lights: Vec<Light>) -> Scene {
        let camera = Vector::new(0., 0., 0.);
        Scene { camera, viewport, spheres, lights }
    }

    pub fn get_color_for_window_cords(&self, x: i32, y: i32) -> Color {
        let view_vec = self.viewport.get_vector_on_viewport(x, y);
        let direction = view_vec - self.camera;
        self.trace_ray(self.camera, direction, 1., f32::INFINITY, 4)
    }

    fn trace_ray(&self, point: Vector, direction: Vector, min: f32, max: f32, depth: i32) -> Color {
        let (sphere, coeff) = self.calculate_closest_sphere_and_coeff(point, direction, min, max);
        let mut color = sphere.color;
        if color.is_black() {
            return Color::black();
        }
        let point_on_sphere: Vector = point + direction * coeff;
        let light_coeff = self.calculate_light_coeff(point_on_sphere, &sphere, direction);
        color = color * light_coeff;;
        if depth <= 0 || sphere.reflective <= 0. {
            return color;
        }
        let normal = sphere.get_normal(point_on_sphere);
        let reflect_rey = Scene::calc_reflection_vector(normal, direction * (-1.));
        color * (1. - sphere.reflective) + self.trace_ray(point_on_sphere, reflect_rey, 0.001, max, depth - 1) * sphere.reflective
    }

    fn calculate_closest_sphere_and_coeff(&self, point: Vector, direction: Vector, min: f32, max: f32) -> (Sphere, f32) {
        let mut closest_coeff = max;
        let mut selected_sphere = Sphere::zero();
        for sphere in &self.spheres {
            let from_sphere_vec = point - sphere.center;
            let rr = sphere.radius * sphere.radius;
            let a = direction * direction;
            let b = 2.0 * (from_sphere_vec * direction);
            let c = from_sphere_vec * from_sphere_vec - rr;
            let disc = b * b - 4. * a * c;
            if disc < 0. {
                continue;
            }
            let disc = disc.sqrt();
            let x1 = (-b + disc) / (2. * a);
            let x2 = (-b - disc) / (2. * a);
            let coeff = match x1 > x2 {
                true => x2,
                false => x1,
            };
            if coeff < closest_coeff && coeff > min {
                selected_sphere = *sphere;
                closest_coeff = coeff;
            }
        }
        (selected_sphere, closest_coeff)
    }

    fn calculate_light_coeff(&self, point: Vector, sphere: &Sphere, view_point: Vector) -> f32 {
        let mut light_coeff: f32 = 0.;
        for light in &self.lights {
            match light.get_type() {
                &LightType::AMBIENT => {
                    light_coeff += light.get_intensity();
                }
                &LightType::DIRECTIONAL | &LightType::POINT => {
                    let direction = match light.get_type() {
                        &LightType::DIRECTIONAL => light.get_position(),
                        &LightType::POINT => light.get_position() - point,
                        _ => panic!("panic"),
                    };
                    if !self.point_in_shadow(point, direction, light) {
                        let normal = sphere.get_normal(point);
                        light_coeff += light.get_intensity() * Scene::calculate_cos(direction.normalized(), normal);
                        let reflection = Scene::calc_reflection_vector(normal, direction);
                        light_coeff += light.get_intensity() * Scene::calculate_cos(reflection, view_point * (-1.)).powf(sphere.specular);
                    }
                }
            }
        }
        if light_coeff > 1. {
            light_coeff = 1.;
        }
//        println!("coeff: {:?}", light_coeff);
        light_coeff
    }

    fn point_in_shadow(&self, point: Vector, direction: Vector, light: &Light) -> bool {
        let start_coeff = match light.get_type() {
            &LightType::DIRECTIONAL => f32::INFINITY,
            &LightType::POINT => 1.,
            _ => 0.,
        };
        let (_, coeff) = self.calculate_closest_sphere_and_coeff(point, direction, 0., start_coeff);
        coeff < start_coeff
    }

    fn calc_reflection_vector(normal: Vector, light_direction: Vector) -> Vector {
        let len = normal * light_direction;
        let vec_n = normal * len;
        let reflection = (vec_n * 2.) - light_direction;
        reflection
    }

    fn calculate_cos(vec1: Vector, vec2: Vector) -> f32 {
        let mut cos = (vec1 * vec2) / (vec1.get_len() * vec2.get_len());
        if cos < 0. {
            cos = 0.;
        }
        cos
    }
}

pub struct Viewport {
    size: f32,
    width_coeff: f32,
    height_coeff: f32,
}

impl Viewport {
    pub fn new(size: f32, window_width: u32, window_height: u32) -> Viewport {
        let width_coeff = size / window_width as f32;
        let height_coeff = size / window_height as f32;
        Viewport { size, width_coeff, height_coeff }
    }

    fn get_vector_on_viewport(&self, x: i32, y: i32) -> Vector {
        let x = self.width_coeff * x as f32;
        let y = self.height_coeff * y as f32;
        Vector::new(x, y, self.size)
    }
}

#[derive(Clone, Copy)]
pub struct Sphere {
    center: Vector,
    pub radius: f32,
    color: Color,
    specular: f32,
    reflective: f32,
}

impl Sphere {
    pub fn new(center: Vector, radius: f32, color: Color, specular: f32, reflective: f32) -> Sphere {
        Sphere { center, radius, color, specular, reflective }
    }

    pub fn zero() -> Sphere {
        Sphere::new(Vector::null_vec(), 0., Color::black(), 0., 0.)
    }

    pub fn get_normal(&self, point: Vector) -> Vector {
        let normal = point - self.center;
        normal.normalized()
    }
}

pub enum LightType {
    AMBIENT,
    DIRECTIONAL,
    POINT,
}

pub struct Light {
    light_type: LightType,
    intensity: f32,
    position: Vector,
}

impl Light {
    pub fn new(light_type: LightType, intensity: f32, position: Vector) -> Light {
        Light { light_type, intensity, position }
    }

    pub fn get_type(&self) -> &LightType {
        &self.light_type
    }

    pub fn get_intensity(&self) -> f32 {
        self.intensity
    }

    pub fn get_position(&self) -> Vector {
        self.position
    }
}