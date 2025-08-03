use glam::DVec3;

#[derive(Clone, Copy)]
/// Material de um objeto (modelo de iluminação de Phong)
pub struct Material {
    /// Coeficiente ambiente
    pub k_amb: DVec3,
    /// Coeficiente difuso
    pub k_dif: DVec3,
    /// Coeficiente especular
    pub k_esp: DVec3,
    /// Expoente especular (Fator de brilho)
    pub e: f64,
}

impl Material {
    pub const WHITE: Material = Material {
        k_amb: DVec3::splat(0.8),
        k_dif: DVec3::splat(0.8),
        k_esp: DVec3::splat(0.8),
        e: 15.0,
    };

    pub const RED: Material = Material {
        k_amb: DVec3::new(0.8, 0.3, 0.3),
        k_dif: DVec3::new(0.8, 0.3, 0.3),
        k_esp: DVec3::new(0.8, 0.3, 0.3),
        e: 15.0,
    };

    pub const GREEN: Material = Material {
        k_amb: DVec3::new(0.3, 0.8, 0.3),
        k_dif: DVec3::new(0.3, 0.8, 0.3),
        k_esp: DVec3::new(0.3, 0.8, 0.3),
        e: 15.0,
    };

    pub const BLUE: Material = Material {
        k_amb: DVec3::new(0.3, 0.3, 0.8),
        k_dif: DVec3::new(0.3, 0.3, 0.8),
        k_esp: DVec3::new(0.3, 0.3, 0.8),
        e: 15.0,
    };

    pub fn new(k_amb: DVec3, k_dif: DVec3, k_esp: DVec3, e: f64) -> Material {
        Material { k_amb, k_dif, k_esp, e }
    }
}