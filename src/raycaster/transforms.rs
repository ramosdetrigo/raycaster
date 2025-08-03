use glam::{DVec3, DMat4};

/// Creates a rotation matrix (DMat4) that rotates around a given axis by a given angle (in radians).
pub fn rotation_matrix_from_axis_angle(axis: DVec3, angle_rad: f64) -> DMat4 {
    let axis = axis.normalize();
    let (x, y, z) = (axis.x, axis.y, axis.z);
    let cos_theta = angle_rad.cos();
    let sin_theta = angle_rad.sin();
    let one_minus_cos = 1.0 - cos_theta;

    let m00 = cos_theta + x * x * one_minus_cos;
    let m01 = x * y * one_minus_cos - z * sin_theta;
    let m02 = x * z * one_minus_cos + y * sin_theta;

    let m10 = y * x * one_minus_cos + z * sin_theta;
    let m11 = cos_theta + y * y * one_minus_cos;
    let m12 = y * z * one_minus_cos - x * sin_theta;

    let m20 = z * x * one_minus_cos - y * sin_theta;
    let m21 = z * y * one_minus_cos + x * sin_theta;
    let m22 = cos_theta + z * z * one_minus_cos;

    DMat4::from_cols_array(&[
        m00, m10, m20, 0.0,
        m01, m11, m21, 0.0,
        m02, m12, m22, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}
