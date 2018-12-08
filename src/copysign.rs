/// Provides copysign intrinsics that don't rely on nightly

use core::intrinsics::transmute;

#[cfg(target_endian = "little")]
pub fn copysignf32(x: f32, y: f32) -> f32 {
    unsafe {
        let sign = transmute::<f32, u32>(x) & (1 << 31);
        let yi = (transmute::<f32, u32>(y) & ((1 << 31) - 1)) | sign;
        transmute::<u32, f32>(yi)
    }
}

#[cfg(target_endian = "little")]
pub fn copysignf64(x: f64, y: f64) -> f64 {
    unsafe {
        let sign = transmute::<f64, u64>(x) & (1 << 63);
        let yi = (transmute::<f64, u64>(y) & ((1 << 63) - 1)) | sign;
        transmute::<u64, f64>(yi)
    }
}

mod tests {
    use super::{copysignf32, copysignf64};

    #[test]
    fn test_copysignf32() {
        assert_eq!(copysignf32(-1.0, 5.0), -5.0);
        assert_eq!(copysignf32(1.0, -5.0), 5.0);
    }

    #[test]
    fn test_copysignf64() {
        assert_eq!(copysignf64(-1.0, 5.0), -5.0);
        assert_eq!(copysignf64(1.0, -5.0), 5.0);
    }
}
