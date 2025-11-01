mod framebuffer;
mod textura;
use minifb::{Key, Window, WindowOptions};
pub const WIDTH: usize = 1024;
pub const HEIGHT: usize = 640;

#[derive(Clone, Copy)]
pub struct Vec2 { pub x: f32, pub y: f32 }
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn sub(self, o: Vec2) -> Vec2 { Vec2::new(self.x - o.x, self.y - o.y) }
    pub fn len(self) -> f32 { (self.x*self.x + self.y*self.y).sqrt() }
}

pub fn clamp(v: f32, a: f32, b: f32) -> f32 { v.min(b).max(a) }
pub fn rgb_to_u32(r: f32, g: f32, b: f32) -> u32 {
    let r = (clamp(r,0.0,1.0)*255.0) as u32;
    let g = (clamp(g,0.0,1.0)*255.0) as u32;
    let b = (clamp(b,0.0,1.0)*255.0) as u32;
    (r << 16) | (g << 8) | b
}
pub fn mix(a: f32, b: f32, t: f32) -> f32 { a*(1.0-t) + b*t }
pub fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = clamp((x - e0)/(e1 - e0), 0.0, 1.0);
    t*t*(3.0 - 2.0*t)
}

pub fn hash(seed: f32) -> f32 {
    let x = (seed * 12345.6789).sin() * 43758.5453;
    x - x.floor()
}
pub fn noise2(x: f32, y: f32) -> f32 { let s = x*12.9898 + y*78.233; hash(s) }
pub fn fbm(x: f32, y: f32) -> f32 {
    let mut amp = 0.5; let mut freq = 1.0; let mut sum = 0.0;
    for _ in 0..5 { sum += amp * noise2(x*freq, y*freq); amp *= 0.5; freq *= 2.0; }
    sum
}

// planet_shader_layers ahora está en textura.rs como planet_texture

fn main() {
    let mut window = Window::new("Sistema Solar Procedural", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let bg_color = rgb_to_u32(0.02,0.02,0.05);
    let mut framebuffer = framebuffer::Framebuffer::new(WIDTH, HEIGHT, bg_color);

    let sun = Vec2::new(WIDTH as f32*0.5, HEIGHT as f32*0.5);
    framebuffer.draw_star(sun, 90.0);

    let orbit_radii = [160.0,230.0,310.0,400.0,490.0];
    for r in orbit_radii { framebuffer.draw_orbit(sun, r); }

    let rocky_positions = [
        Vec2::new(sun.x+160.0, sun.y+30.0),
        Vec2::new(sun.x-230.0, sun.y-50.0),
        Vec2::new(sun.x+310.0, sun.y-40.0)
    ];
    for (i,p) in rocky_positions.iter().enumerate() {
        framebuffer.draw_body(*p,20.0,i,false,false);
    }
    framebuffer.draw_body(Vec2::new(rocky_positions[0].x+40.0, rocky_positions[0].y-20.0),8.0,0,false,false);

    let gas_positions = [
        Vec2::new(sun.x-400.0, sun.y+60.0),
        Vec2::new(sun.x+490.0, sun.y-50.0)
    ];
    framebuffer.draw_body(gas_positions[0],40.0,0,true,true);
    framebuffer.draw_body(gas_positions[1],55.0,1,true,false);

    window.update_with_buffer(framebuffer.as_slice(), WIDTH, HEIGHT).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(framebuffer.as_slice(), WIDTH, HEIGHT).unwrap();
    }
}
