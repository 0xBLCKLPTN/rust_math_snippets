mod vector;
mod matrix;
use vector::*;
use matrix::*;

fn main() {
    let v2_zeroes = Vector2::<f32>::zeroes();
    let v2_ones = Vector2::<f32>::ones();
    let v3_zeroes = Vector3::<f32>::zeroes();
    let v3_ones = Vector3::<f32>::ones();

    let v2_a = Vector2 { x: 1.0, y: 2.0 };
    let v2_b = Vector2 { x: 3.0, y: 4.0 };
    let v3_a = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    let v3_b = Vector3 { x: 4.0, y: 5.0, z: 6.0 };

    let v2_add = v2_a.add(&v2_b);
    let v2_sub = v2_a.sub(&v2_b);
    let v2_scale = v2_a.scale(2.0);
    let v2_dot = v2_a.dot(&v2_b);
    let v2_mag = v2_a.magnitude();

    let v3_add = v3_a.add(&v3_b);
    let v3_sub = v3_a.sub(&v3_b);
    let v3_scale = v3_a.scale(2.0);
    let v3_dot = v3_a.dot(&v3_b);
    let v3_mag = v3_a.magnitude();

    println!("Vector2 zeroes: ({}, {})", v2_zeroes.x, v2_zeroes.y);
    println!("Vector2 ones: ({}, {})", v2_ones.x, v2_ones.y);
    println!("Vector3 zeroes: ({}, {}, {})", v3_zeroes.x, v3_zeroes.y, v3_zeroes.z);
    println!("Vector3 ones: ({}, {}, {})", v3_ones.x, v3_ones.y, v3_ones.z);

    println!("Vector2 add: ({}, {})", v2_add.x, v2_add.y);
    println!("Vector2 sub: ({}, {})", v2_sub.x, v2_sub.y);
    println!("Vector2 scale: ({}, {})", v2_scale.x, v2_scale.y);
    println!("Vector2 dot: {}", v2_dot);
    println!("Vector2 magnitude: {}", v2_mag);

    println!("Vector3 add: ({}, {}, {})", v3_add.x, v3_add.y, v3_add.z);
    println!("Vector3 sub: ({}, {}, {})", v3_sub.x, v3_sub.y, v3_sub.z);
    println!("Vector3 scale: ({}, {}, {})", v3_scale.x, v3_scale.y, v3_scale.z);
    println!("Vector3 dot: {}", v3_dot);
    println!("Vector3 magnitude: {}", v3_mag);
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::Float;
    
    #[test]
    fn test_vector2_zeroes() {
        let v = Vector2::<f32>::zeroes();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
    }
    
    #[test]
    fn test_vector2_ones() {
        let v = Vector2::<f32>::ones();
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
    }
    
    #[test]
    fn test_vector2_add() {
        let v1 = Vector2 { x: 1.0, y: 2.0 };
        let v2 = Vector2 { x: 3.0, y: 4.0 };
        let result = v1.add(&v2);
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }
    
    #[test]
    fn test_vector2_sub() {
        let v1 = Vector2 { x: 3.0, y: 4.0 };
        let v2 = Vector2 { x: 1.0, y: 2.0 };
        let result = v1.sub(&v2);
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 2.0);
    }
    
    #[test]
    fn test_vector2_scale() {
        let v = Vector2 { x: 1.0, y: 2.0 };
        let result = v.scale(2.0);
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
    }
    
    #[test]
    fn test_vector2_dot() {
        let v1 = Vector2 { x: 1.0, y: 2.0 };
        let v2 = Vector2 { x: 3.0, y: 4.0 };
        let result = v1.dot(&v2);
        assert_eq!(result, 11.0);
    }
    
    #[test]
    fn test_vector2_magnitude() {
        let v = Vector2 { x: 3.0, y: 4.0 };
        let result = v.magnitude();
        assert!((result - 5.0).abs() < 1e-6);
    }
    
    #[test]
    fn test_vector3_zeroes() {
        let v = Vector3::<f32>::zeroes();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }
    
    #[test]
    fn test_vector3_ones() {
        let v = Vector3::<f32>::ones();
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 1.0);
    }
    
    #[test]
    fn test_vector3_add() {
        let v1 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
        let result = v1.add(&v2);
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }
    
    #[test]
    fn test_vector3_sub() {
        let v1 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
        let v2 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
        let result = v1.sub(&v2);
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 3.0);
    }
    
    #[test]
    fn test_vector3_scale() {
        let v = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
        let result = v.scale(2.0);
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 6.0);
    }
    
    #[test]
    fn test_vector3_dot() {
        let v1 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
        let result = v1.dot(&v2);
        assert_eq!(result, 32.0);
    }
    
    #[test]
    fn test_vector3_magnitude() {
        let v = Vector3 { x: 1.0, y: 2.0, z: 2.0 };
        let result = v.magnitude();
        assert!((result - 3.0).abs() < 1e-6);
    }
    
    #[test]
    fn test_matrix_zeroes() {
        let m = Matrix::<f32>::zeroes(2, 2);
        assert_eq!(m.data, vec![0.0, 0.0, 0.0, 0.0]);
    }
    
    #[test]
    fn test_matrix_ones() {
        let m = Matrix::<f32>::ones(2, 2);
        assert_eq!(m.data, vec![1.0, 1.0, 1.0, 1.0]);
    }
    
    #[test]
    fn test_matrix_add() {
        let m1 = Matrix {
            rows: 2,
            cols: 2,
            data: vec![1.0, 2.0, 3.0, 4.0],
        };
        let m2 = Matrix {
            rows: 2,
            cols: 2,
            data: vec![5.0, 6.0, 7.0, 8.0],
        };
        let result = m1.add(&m2);
        assert_eq!(result.data, vec![6.0, 8.0, 10.0, 12.0]);
    }
    
    #[test]
    fn test_matrix_sub() {
        let m1 = Matrix {
            rows: 2,
            cols: 2,
            data: vec![5.0, 6.0, 7.0, 8.0],
        };
        let m2 = Matrix {
            rows: 2,
            cols: 2,
            data: vec![1.0, 2.0, 3.0, 4.0],
        };
        let result = m1.sub(&m2);
        assert_eq!(result.data, vec![4.0, 4.0, 4.0, 4.0]);
    }
    
    #[test]
    fn test_matrix_scale() {
        let m = Matrix {
            rows: 2,
            cols: 2,
            data: vec![1.0, 2.0, 3.0, 4.0],
        };
        let result = m.scale(2.0);
        assert_eq!(result.data, vec![2.0, 4.0, 6.0, 8.0]);
    }
    
    #[test]
    fn test_matrix_mul() {
        let m1 = Matrix {
            rows: 2,
            cols: 2,
            data: vec![1.0, 2.0, 3.0, 4.0],
        };
        let m2 = Matrix {
            rows: 2,
            cols: 2,
            data: vec![5.0, 6.0, 7.0, 8.0],
        };
        let result = m1.mul(&m2);
        assert_eq!(result.data, vec![19.0, 22.0, 43.0, 50.0]);
    }
    
    #[test]
    fn test_matrix_transpose() {
        let m = Matrix {
            rows: 2,
            cols: 3,
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        };
        let result = m.transpose();
        assert_eq!(result.data, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
    }
}
