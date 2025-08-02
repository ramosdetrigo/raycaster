use glam::DVec3;

use crate::{lights::Light, Intersection, Scene, Ray};

pub struct Point {
    pub pos: DVec3,
    pub color: DVec3,
    pub intensity: f64,
}

impl Point {
    pub fn new(pos: DVec3, color: DVec3, intensity: f64) -> Point {
        Point { pos, color, intensity }
    }

    pub fn color_at(&self, intersection: &Intersection, v: DVec3, scene: &Scene) -> DVec3 {
        let light_direction = (self.pos - intersection.p).normalize();
        let light_ray = Ray::new(intersection.p, light_direction);
        
        // Testa se existe um objeto entre o objeto renderizado e a luz
        // pra ver se tem sombra
        let shadow_intersection = scene.objects.iter().find(|object| {
            // Checa se a interseção existe e se ela está dentro de um threshold de erro    
            let intersection = object.intersects(&light_ray);
            intersection.is_some() && intersection.unwrap().t.abs() > 1e-6
        });

        if shadow_intersection.is_none() {
            let l = light_direction; // vetor unitário apontando na direção da luz
            let n = intersection.normal;
            let mat = intersection.material;
            let light_intensity = self.color * self.intensity;
                    
            let r = 2.0 * l.dot(n)*n - l; // vetor l refletido na normal
            let nl = n.dot(l); // normal escalar l
            let rv = r.dot(-v); // r escalar v

            // O check > 0.0 previne o bug de iluminação no "lado escuro da esfera"
            let mut ieye = DVec3::ZERO;
            if nl > 0.0 { ieye += mat.k_dif * nl * light_intensity; } // Reflexão difusa
            if rv > 0.0 { ieye += mat.k_esp * rv.powf(mat.e) * light_intensity; } // Reflexão especular
            
            ieye
        } else {
            DVec3::ZERO
        }
    }
}

impl Light for Point {
    fn color_at(&self, intersection: &Intersection, v: DVec3, scene: &Scene) -> DVec3 {
        self.color_at(intersection, v, scene)
    }
}
