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
        k_amb: DVec3 { x: 1.0, y: 1.0, z: 1.0 },
        k_dif: DVec3 { x: 1.0, y: 1.0, z: 1.0 },
        k_esp: DVec3 { x: 1.0, y: 1.0, z: 1.0 },
        e: 5.0,
    };

    pub fn new(k_amb: DVec3, k_dif: DVec3, k_esp: DVec3, e: f64) -> Material {
        Material { k_amb, k_dif, k_esp, e }
    }
}