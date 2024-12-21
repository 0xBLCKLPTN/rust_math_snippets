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
}