// textura.rs
use crate::{Vec2, fbm, clamp, mix, smoothstep};

pub fn planet_texture(pos: Vec2, center: Vec2, radius: f32, planet_index: usize, is_gas: bool) -> (f32, f32, f32, f32) {
    let p = pos.sub(center);
    let dist = p.len();
    let t = dist / radius;
    if t > 1.0 { return (0.0, 0.0, 0.0, 0.0); }

    // Paletas mejoradas
    let (base_r, base_g, base_b) = match (is_gas, planet_index % 4) {
        (false, 0) => (0.55, 0.48, 0.38), // Tierra
        (false, 1) => (0.38, 0.42, 0.60), // Azul
        (false, 2) => (0.65, 0.62, 0.5),  // Gris
        (false, 3) => (0.7, 0.5, 0.3),    // Marrón
        (true, 0) => (0.95, 0.7, 0.25),   // Amarillo
        (true, 1) => (0.45, 0.7, 0.95),   // Azul claro
        (true, 2) => (0.7, 0.9, 0.6),     // Verde
        _ => (0.6, 0.6, 0.6),
    };

    let base_light = 1.0 - 0.5 * t;
    let angle = (p.y / radius) * std::f32::consts::PI;
    let band_coord = angle;
    let bands = band_coord.sin() * 0.5 + 0.5;
    let nx = p.x / radius * 2.0;
    let ny = p.y / radius * 2.0;
    let storm = fbm(nx * 3.0, ny * 3.0);
    let rim = smoothstep(0.85, 1.0, t);

    let (mut r, mut g, mut b) = (base_r, base_g, base_b);
    if is_gas {
        // Textura suave y fluida, sin estratos ni cráteres
        let band_t = mix(0.5, 1.0, bands * 0.8 + 0.2 * storm);
        r *= band_t * base_light;
        g *= mix(band_t, 1.0, 0.7) * base_light;
        b *= mix(1.0, band_t, 0.7) * base_light;
        // Añade variación suave tipo "tormenta" pero sin detalles rocosos
        let swirl = fbm(nx * 1.5 + storm * 2.0, ny * 1.5 - storm * 2.0);
        r = mix(r, r * 1.2 + 0.10, swirl * 0.3);
        g = mix(g, g * 1.1 + 0.08, swirl * 0.2);
        b = mix(b, b * 1.05 + 0.05, swirl * 0.15);
    } else {
        // ...existing code...
        let strata = smoothstep(-0.7, 0.7, (p.x / radius) * 3.2 + fbm(nx * 2.2, ny * 2.2));
        let strata_t = mix(0.85, 1.25, strata);
        r *= strata_t * base_light;
        g *= mix(0.92, 1.15, strata) * base_light;
        b *= mix(0.82, 1.08, strata * 0.8) * base_light;
        let crater = fbm(nx * 6.5, ny * 6.5);
        let crater_mask = smoothstep(0.58, 0.64, crater);
        r = mix(r, r * 0.45, crater_mask * 0.95);
        g = mix(g, g * 0.45, crater_mask * 0.95);
        b = mix(b, b * 0.45, crater_mask * 0.95);
    }

    // Borde más suave y con halo
    let rim_strength = rim * 0.8;
    r = mix(r, 1.0, rim_strength * 0.38);
    g = mix(g, 1.0, rim_strength * 0.28);
    b = mix(b, 1.0, rim_strength * 0.18);
    let edge_dark = mix(1.0, 0.72, t * 0.85);
    r *= edge_dark;
    g *= edge_dark;
    b *= edge_dark;

    // Contraste y saturación extra
    let sat = 1.08;
    r = clamp(r * sat, 0.0, 1.0);
    g = clamp(g * sat, 0.0, 1.0);
    b = clamp(b * sat, 0.0, 1.0);

    (r, g, b, 1.0)
}
