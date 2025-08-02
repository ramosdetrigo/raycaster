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