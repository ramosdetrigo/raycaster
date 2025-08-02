use nalgebra::Vector3;

#[derive(Clone, Copy)]
/// Material de um objeto (modelo de iluminação de Phong)
pub struct Material {
    /// Coeficiente ambiente
    pub k_amb: Vector3<f64>,
    /// Coeficiente difuso
    pub k_dif: Vector3<f64>,
    /// Coeficiente especular
    pub k_esp: Vector3<f64>,
    /// Expoente especular (Fator de brilho)
    pub e: f64,
}