use crate::math::Float2;
use crate::math::Float3;
use crate::math::Float4;
use core::f32::consts::PI;

#[test]
fn float3_from() {
    let result = Float3::new(1.5, 2.25, 0.9);
    assert_eq!(result.x, 1.5);
    assert_eq!(result.y, 2.25);
    assert_eq!(result.z, 0.9);
}

#[test]
fn float3_ops_add() {
    let a = Float3::new(1.5, 2.25, 0.9);
    let b = Float3::new(3.1, 2.75, 0.1);
    let result = a + b;
    assert_eq!(result.x, 4.6);
    assert_eq!(result.y, 5.0);
    assert_eq!(result.z, 1.0);
}

#[test]
fn float3_ops_add_f32() {
    let a = Float3::new(1.0, 2.0, 0.5);
    let b = a + 0.5;
    assert_eq!(b, Float3::new(1.5, 2.5, 1.0));
}

#[test]
fn float3_ops_add_assign() {
    let mut result = Float3::new(1.5, 2.25, 0.9);
    let b = Float3::new(3.1, 2.75, 0.1);
    result += b;
    assert_eq!(result.x, 4.6);
    assert_eq!(result.y, 5.0);
    assert_eq!(result.z, 1.0);
}

#[test]
fn float3_ops_add_assign_f32() {
    let mut a = Float3::new(1.0, 2.0, 0.5);
    a += 0.5;
    assert_eq!(a, Float3::new(1.5, 2.5, 1.0));
}

#[test]
fn float3_ops_sub() {
    let a = Float3::new(1.5, 2.25, 2.0);
    let b = Float3::new(3.1, 2.75, 1.0);
    let result = a - b;
    assert_eq!(result.x, -1.5999999);
    assert_eq!(result.y, -0.5);
    assert_eq!(result.z, 1.0);
}

#[test]
fn float3_ops_sub_f32() {
    let a = Float3::new(1.0, 2.0, 4.0);
    let b = a - 0.5;
    assert_eq!(b, Float3::new(0.5, 1.5, 3.5));
}

#[test]
fn float3_ops_sub_assign() {
    let mut result = Float3::new(1.5, 2.25, 4.0);
    let b = Float3::new(3.1, 2.75, 3.0);
    result -= b;
    assert_eq!(result.x, -1.5999999);
    assert_eq!(result.y, -0.5);
    assert_eq!(result.z, 1.0);
}

#[test]
fn float3_ops_sub_assign_f32() {
    let mut a = Float3::new(1.0, 2.0, 4.0);
    a -= 0.5;
    assert_eq!(a, Float3::new(0.5, 1.5, 3.5));
}

#[test]
fn float3_ops_mul() {
    let a = Float3::new(1.5, 2.5, 5.0);
    let b = Float3::new(3.0, 2.0, 5.0);
    let result = a * b;
    assert_eq!(result.x, 4.5);
    assert_eq!(result.y, 5.0);
    assert_eq!(result.z, 25.0);
}

#[test]
fn float3_ops_mul_f32() {
    let a = Float3::new(1.0, 2.0, 3.0);
    let b = a * 0.5;
    assert_eq!(b, Float3::new(0.5, 1.0, 1.5));
}

#[test]
fn float3_ops_mul_assign() {
    let mut result = Float3::new(1.5, 2.5, 5.0);
    let b = Float3::new(3.0, 2.0, 1.5);
    result *= b;
    assert_eq!(result.x, 4.5);
    assert_eq!(result.y, 5.0);
    assert_eq!(result.z, 7.5);
}

#[test]
fn float3_ops_mul_assign_f32() {
    let mut a = Float3::new(1.0, 2.0, 3.0);
    a *= 0.5;
    assert_eq!(a, Float3::new(0.5, 1.0, 1.5));
}

#[test]
fn float3_ops_div() {
    let a = Float3::new(1.5, 2.5, 5.0);
    let b = Float3::new(3.0, 2.0, 2.5);
    let result = a / b;
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 1.25);
    assert_eq!(result.z, 2.0);
}

#[test]
fn float3_ops_div_f32() {
    let a = Float3::new(1.0, 2.0, 5.0);
    let b = a / 0.5;
    assert_eq!(b, Float3::new(2.0, 4.0, 10.0));
}

#[test]
fn float3_ops_div_assign() {
    let mut result = Float3::new(1.5, 2.5, 5.0);
    let b = Float3::new(3.0, 2.0, 2.5);
    result /= b;
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 1.25);
    assert_eq!(result.z, 2.0);
}

#[test]
fn float3_ops_div_assign_f32() {
    let mut a = Float3::new(1.0, 2.0, 5.0);
    a /= 0.5;
    assert_eq!(a, Float3::new(2.0, 4.0, 10.0));
}

#[test]
fn float3_ops_neg() {
    let result = -Float3::new(1.5, 2.5, 4.1);
    assert_eq!(result.x, -1.5);
    assert_eq!(result.y, -2.5);
    assert_eq!(result.z, -4.1);
}

#[test]
fn float3_equality() {
    let mut a = Float3::new(20.0, 1.0, 5.0);
    let b = a;
    a += b;
    assert_eq!(a.x, 40.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(a.z, 10.0);
    assert_eq!(b.x, 20.0);
    assert_eq!(b.y, 1.0);
    assert_eq!(b.z, 5.0);

    assert_eq!(a == b, false);

    assert_ne!(a, b);
    let a = Float3::new(20.0, 1.0, 5.0);
    assert_eq!(a, b);

    assert_eq!(a == b, true);
    assert_eq!(a != b, false);
}

#[test]
fn float3_abs() {
    let a = Float3::new(-PI, -PI * 2.0, -1.5);
    let result = a.abs();
    assert_eq!(result.x, PI);
    assert_eq!(result.y, PI * 2.0);
    assert_eq!(result.z, 1.5);
}

#[test]
fn float3_acos() {
    let a = Float3::new(0.5, -0.5, 0.0);
    let result = a.acos();
    assert_eq!(result, Float3::new(1.0471976, 2.0943952, 1.5707964));
}

#[test]
fn float3_all() {
    let a = Float3::new(0.0, 0.0, 0.0);
    let result = a.all();
    assert_eq!(result, false);

    let a = Float3::new(0.1, 0.0, 0.0);
    let result = a.all();
    assert_eq!(result, false);

    let a = Float3::new(0.0, 0.1, 0.0);
    let result = a.all();
    assert_eq!(result, false);

    let a = Float3::new(0.0, 0.0, 0.1);
    let result = a.all();
    assert_eq!(result, false);

    let a = Float3::new(-0.1, 0.1, 1.0);
    let result = a.all();
    assert_eq!(result, true);
}

#[test]
fn float3_any() {
    let a = Float3::new(0.0, 0.0, 0.0);
    let result = a.any();
    assert_eq!(result, false);

    let a = Float3::new(0.1, 0.0, 0.0);
    let result = a.any();
    assert_eq!(result, true);

    let a = Float3::new(0.0, 0.1, 0.0);
    let result = a.any();
    assert_eq!(result, true);

    let a = Float3::new(0.0, 0.0, 0.1);
    let result = a.any();
    assert_eq!(result, true);

    let a = Float3::new(-0.1, 0.1, 1.0);
    let result = a.any();
    assert_eq!(result, true);
}

#[test]
fn float3_asin() {
    let a = Float3::new(0.5, -0.5, 0.1);
    let result = a.asin();
    assert_eq!(result, Float3::new(0.5235988, -0.5235988, 0.10016742));
}

#[test]
fn float3_atan() {
    let a = Float3::new(0.5, -0.5, 1.0);
    let result = a.atan();
    assert_eq!(result, Float3::new(0.4636476, -0.4636476, 0.7853982));
}

#[test]
fn float3_ceil() {
    let a = Float3::new(0.9, -0.2, 1.2);
    let result = a.ceil();
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 2.0);
}

#[test]
fn float3_clamp() {
    let a = Float3::new(0.9, -0.2, 0.6);
    let result = a.clamp(0.0, 0.5);
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 0.5);
}

#[test]
fn float3_cos() {
    let a = Float3::new(PI, PI * 2.0, PI * 4.0);
    let result = a.cos();
    assert_eq!(result.x, -1.0);
    assert_eq!(result.y, 1.0);
    assert_eq!(result.z, 1.0);
}

#[test]
fn float3_cosh() {
    let a = Float3::new(0.9, -0.2, 0.5);
    let result = a.cosh();
    assert_eq!(result.x, 1.4330864);
    assert_eq!(result.y, 1.0200667);
    assert_eq!(result.z, 1.127626);
}

#[test]
fn float3_degrees() {
    let a = Float3::new(PI, PI * 0.5, PI * 0.25);
    let result = a.degrees();
    assert_eq!(result.x, 180.0);
    assert_eq!(result.y, 90.0);
    assert_eq!(result.z, 45.0);
}

#[test]
fn float3_distance() {
    let a = Float3::new(1.0, 1.0, 1.0);
    let b = Float3::new(5.0, 5.0, 5.0);
    let result = a.distance(&b);
    assert_eq!(result, 6.928203);
}

#[test]
fn float3_dot() {
    let a = Float3::new(1.0, 1.0, 0.5);
    let b = Float3::new(5.0, 5.0, 0.5);
    let result = a.dot(&b);
    assert_eq!(result, 10.25);
}

#[test]
fn float3_exp() {
    let a = Float3::new(2.0, 4.0, 8.0);
    let result = a.exp();
    assert_eq!(result.x, 7.389056);
    assert_eq!(result.y, 54.59815);
    assert_eq!(result.z, 2980.958);
}

#[test]
fn float3_exp2() {
    let a = Float3::new(2.0, 4.0, 8.0);
    let result = a.exp2();
    assert_eq!(result.x, 4.0);
    assert_eq!(result.y, 16.0);
    assert_eq!(result.z, 256.0);
}

#[test]
fn float3_floor() {
    let a = Float3::new(0.9, -0.2, 1.2);
    let result = a.floor();
    assert_eq!(result.x, 0.0);
    assert_eq!(result.y, -1.0);
    assert_eq!(result.z, 1.0);
}

#[test]
fn float3_fmod() {
    let a = Float3::new(0.2, 2.0, 4.0);
    let b = Float3::new(2.0, 4.0, 6.0);
    let result = a.fmod(&b);
    assert_eq!(result.x, 0.2);
    assert_eq!(result.y, 2.0);
    assert_eq!(result.z, 4.0);
}

#[test]
fn float3_frac() {
    let a = Float3::new(24.50, 8.25, 1.75);
    let result = a.frac();
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 0.25);
    assert_eq!(result.z, 0.75);
}

#[test]
fn float3_ldexp() {
    let value = Float3::new(1.5, 2.5, 1.0);
    let exponent = Float3::new(2.0, -1.0, 1.0);
    let result = value.ldexp(&exponent);
    assert_eq!(result.x, 6.0);
    assert_eq!(result.y, 1.25);
    assert_eq!(result.z, 2.0);
}

#[test]
fn float3_length() {
    let a = Float3::new(1.0, 1.0, 1.0);
    let result = a.length();
    assert_eq!(result, 1.7320508);
}

#[test]
fn float3_lerp() {
    let a = Float3::new(0.0, 0.1, 0.5);
    let b = Float3::new(2.0, 4.1, 1.0);
    assert_eq!(a.lerp(&b, 0.5), Float3::new(1.0, 2.1, 0.75));
    assert_eq!(a.lerp(&b, 1.5), Float3::new(3.0, 6.1, 1.25));
}

#[test]
fn float3_log() {
    let a = Float3::new(1.0, 2.0, 4.0);
    assert_eq!(a.log(), Float3::new(0.0, 0.6931472, 1.3862944));
}

#[test]
fn float3_log10() {
    let a = Float3::new(1.0, 10.0, 20.0);
    assert_eq!(a.log10(), Float3::new(0.0, 1.0, 1.30103));
}

#[test]
fn float3_log2() {
    let a = Float3::new(1.0, 2.0, 4.0);
    assert_eq!(a.log2(), Float3::new(0.0, 1.0, 2.0));
}

#[test]
fn float3_mad() {
    let a = Float3::new(2.0, 2.0, 5.0);
    let b = Float3::new(4.0, 5.0, 5.0);
    let c = Float3::new(0.5, 0.25, 5.0);
    assert_eq!(a.mad(&b, &c), Float3::new(8.5, 10.25, 30.0));
}

#[test]
fn float3_max() {
    let a = Float3::new(2.0, 2.0, 2.0);
    let b = Float3::new(4.0, 1.0, 5.0);
    assert_eq!(a.max(&b), Float3::new(4.0, 2.0, 5.0));
}

#[test]
fn float3_min() {
    let a = Float3::new(2.0, 2.0, 2.0);
    let b = Float3::new(4.0, 1.0, 5.0);
    assert_eq!(a.min(&b), Float3::new(2.0, 1.0, 2.0));
}

#[test]
fn float3_normalize() {
    let a = Float3::new(2.0, 1.0, 3.0);
    assert_eq!(a.normalize(), Float3::new(0.5345225, 0.26726124, 0.8017837));
    let a = Float3::new(0.0, 0.0, 0.0);
    assert_eq!(a.normalize(), Float3::new(0.0, 0.0, 0.0));
}

#[test]
fn float3_pow() {
    let a = Float3::new(2.0, 1.0, 5.0);
    assert_eq!(a.pow(2.0), Float3::new(4.0, 1.0, 25.0));
}

#[test]
fn float3_radians() {
    let a = Float3::new(180.0, 90.0, 45.0);
    let result = a.radians();
    assert_eq!(result.x, PI);
    assert_eq!(result.y, PI * 0.5);
    assert_eq!(result.z, PI * 0.25);
}

#[test]
fn float3_rcp() {
    let a = Float3::new(2.0, 4.0, 8.0);
    let result = a.rcp();
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 0.25);
    let a = Float3::new(2.0, 0.0, 8.0);
    let result = a.rcp_safe();
    assert_eq!(result.y, 0.0);
}

#[test]
fn float3_reflect() {
    let incident = Float3::new(1.0, -1.0, 1.0);
    let normal = Float3::new(0.0, 1.0, 0.0);
    let result = incident.reflect(&normal);
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 1.0);
    assert_eq!(result.z, 1.0);
}

#[test]
fn float3_refract() {
    let incident = Float3::new(1.0, -1.0, 0.0);
    let normal = Float3::new(0.0, 1.0, 0.0);
    let eta = 0.5;
    let result = incident.refract(&normal, eta);
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, -1.0);
    assert_eq!(result.z, 0.0);
}

#[test]
fn float3_round() {
    let a = Float3::new(0.9, -0.2, 1.2);
    let result = a.round();
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 1.0);
}

#[test]
fn float3_rsqrt() {
    let a = Float3::new(1.0, 4.0, 8.0);
    let result = a.rsqrt();
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 0.5);
    assert_eq!(result.z, 0.35355338);
}

#[test]
fn float3_saturate() {
    let a = Float3::new(2.9, -0.2, 1.2);
    let result = a.saturate();
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 1.0);
}

#[test]
fn float3_sign() {
    let a = Float3::new(2.9, -0.2, 0.3);
    let result = a.sign();
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, -1.0);
    assert_eq!(result.z, 1.0);
}

#[test]
fn float3_sin() {
    let a = Float3::new(PI, PI * 2.0, 1.0);
    let result = a.sin();
    assert_eq!(result.x, -8.742278e-8);
    assert_eq!(result.y, 1.7484555e-7);
    assert_eq!(result.z, 0.84147096);
}

#[test]
fn float3_sinh() {
    let a = Float3::new(0.9, -0.2, 1.0);
    let result = a.sinh();
    assert_eq!(result.x, 1.0265167);
    assert_eq!(result.y, -0.20133601);
    assert_eq!(result.z, 1.1752012);
}

#[test]
fn float3_smoothstep() {
    let a = Float3::new(0.0, 0.0, 0.0);
    let b = Float3::new(1.0, 1.0, 1.0);
    let c = Float3::new(0.5, 1.5, 2.5);
    let result = c.smoothstep(&a, &b);
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 1.0);
    assert_eq!(result.y, 1.0);
}

#[test]
fn float3_sqrt() {
    let a = Float3::new(4.0, 9.0, 16.0);
    let result = a.sqrt();
    assert_eq!(result.x, 2.0);
    assert_eq!(result.y, 3.0);
    assert_eq!(result.z, 4.0);
}

#[test]
fn float3_step() {
    let a = Float3::new(0.5, 0.8, 3.0);
    let b = Float3::new(0.3, 1.0, 4.0);
    let result = a.step(&b);
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 0.0);
}

#[test]
fn float3_tan() {
    let a = Float3::new(PI / 4.0, PI / 6.0, PI);
    let result = a.tan();
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 0.57735026);
    assert_eq!(result.z, 8.742278e-8);
}

#[test]
fn float3_tanh() {
    let a = Float3::new(PI / 4.0, PI / 6.0, PI / 8.0);
    let result = a.tanh();
    assert_eq!(result.x, 0.65579426);
    assert_eq!(result.y, 0.4804728);
    assert_eq!(result.z, 0.37368476);
}

#[test]
fn float3_trunc() {
    let a = Float3::new(25.2, 4.81, 1.02);
    let result = a.trunc();
    assert_eq!(result.x, 25.0);
    assert_eq!(result.y, 4.0);
    assert_eq!(result.z, 1.0);
}

#[test]
fn float3_swizzle() {
    let x = 1.0;
    let y = 2.0;
    let z = 3.0;
    let a = Float3::new(x, y, z);
    assert_eq!(a.xx(), Float2::new(x, x));
    assert_eq!(a.xy(), Float2::new(x, y));
    assert_eq!(a.xz(), Float2::new(x, z));
    assert_eq!(a.yx(), Float2::new(y, x));
    assert_eq!(a.yy(), Float2::new(y, y));
    assert_eq!(a.yz(), Float2::new(y, z));
    assert_eq!(a.zx(), Float2::new(z, x));
    assert_eq!(a.zy(), Float2::new(z, y));
    assert_eq!(a.zz(), Float2::new(z, z));

    assert_eq!(a.xxx(), Float3::new(x, x, x));
    assert_eq!(a.xxy(), Float3::new(x, x, y));
    assert_eq!(a.xxz(), Float3::new(x, x, z));
    assert_eq!(a.xyx(), Float3::new(x, y, x));
    assert_eq!(a.xyy(), Float3::new(x, y, y));
    assert_eq!(a.xyz(), Float3::new(x, y, z));
    assert_eq!(a.xzx(), Float3::new(x, z, x));
    assert_eq!(a.xzy(), Float3::new(x, z, y));
    assert_eq!(a.xzz(), Float3::new(x, z, z));
    assert_eq!(a.yxx(), Float3::new(y, x, x));
    assert_eq!(a.yxy(), Float3::new(y, x, y));
    assert_eq!(a.yxz(), Float3::new(y, x, z));
    assert_eq!(a.yyx(), Float3::new(y, y, x));
    assert_eq!(a.yyy(), Float3::new(y, y, y));
    assert_eq!(a.yyz(), Float3::new(y, y, z));
    assert_eq!(a.yzx(), Float3::new(y, z, x));
    assert_eq!(a.yzy(), Float3::new(y, z, y));
    assert_eq!(a.yzz(), Float3::new(y, z, z));
    assert_eq!(a.zxx(), Float3::new(z, x, x));
    assert_eq!(a.zxy(), Float3::new(z, x, y));
    assert_eq!(a.zxz(), Float3::new(z, x, z));
    assert_eq!(a.zyx(), Float3::new(z, y, x));
    assert_eq!(a.zyy(), Float3::new(z, y, y));
    assert_eq!(a.zyz(), Float3::new(z, y, z));
    assert_eq!(a.zzx(), Float3::new(z, z, x));
    assert_eq!(a.zzy(), Float3::new(z, z, y));
    assert_eq!(a.zzz(), Float3::new(z, z, z));

    assert_eq!(a.xxxx(), Float4::new(x, x, x, x));
    assert_eq!(a.xxxy(), Float4::new(x, x, x, y));
    assert_eq!(a.xxxz(), Float4::new(x, x, x, z));
    assert_eq!(a.xxyx(), Float4::new(x, x, y, x));
    assert_eq!(a.xxyy(), Float4::new(x, x, y, y));
    assert_eq!(a.xxyz(), Float4::new(x, x, y, z));
    assert_eq!(a.xxzx(), Float4::new(x, x, z, x));
    assert_eq!(a.xxzy(), Float4::new(x, x, z, y));
    assert_eq!(a.xxzz(), Float4::new(x, x, z, z));
    assert_eq!(a.xyxx(), Float4::new(x, y, x, x));
    assert_eq!(a.xyxy(), Float4::new(x, y, x, y));
    assert_eq!(a.xyxz(), Float4::new(x, y, x, z));
    assert_eq!(a.xyyx(), Float4::new(x, y, y, x));
    assert_eq!(a.xyyy(), Float4::new(x, y, y, y));
    assert_eq!(a.xyyz(), Float4::new(x, y, y, z));
    assert_eq!(a.xyzx(), Float4::new(x, y, z, x));
    assert_eq!(a.xyzy(), Float4::new(x, y, z, y));
    assert_eq!(a.xyzz(), Float4::new(x, y, z, z));
    assert_eq!(a.xzxx(), Float4::new(x, z, x, x));
    assert_eq!(a.xzxy(), Float4::new(x, z, x, y));
    assert_eq!(a.xzxz(), Float4::new(x, z, x, z));
    assert_eq!(a.xzyx(), Float4::new(x, z, y, x));
    assert_eq!(a.xzyy(), Float4::new(x, z, y, y));
    assert_eq!(a.xzyz(), Float4::new(x, z, y, z));
    assert_eq!(a.xzzx(), Float4::new(x, z, z, x));
    assert_eq!(a.xzzy(), Float4::new(x, z, z, y));
    assert_eq!(a.xzzz(), Float4::new(x, z, z, z));
    assert_eq!(a.yxxx(), Float4::new(y, x, x, x));
    assert_eq!(a.yxxy(), Float4::new(y, x, x, y));
    assert_eq!(a.yxxz(), Float4::new(y, x, x, z));
    assert_eq!(a.yxyx(), Float4::new(y, x, y, x));
    assert_eq!(a.yxyy(), Float4::new(y, x, y, y));
    assert_eq!(a.yxyz(), Float4::new(y, x, y, z));
    assert_eq!(a.yxzx(), Float4::new(y, x, z, x));
    assert_eq!(a.yxzy(), Float4::new(y, x, z, y));
    assert_eq!(a.yxzz(), Float4::new(y, x, z, z));
    assert_eq!(a.yyxx(), Float4::new(y, y, x, x));
    assert_eq!(a.yyxy(), Float4::new(y, y, x, y));
    assert_eq!(a.yyxz(), Float4::new(y, y, x, z));
    assert_eq!(a.yyyx(), Float4::new(y, y, y, x));
    assert_eq!(a.yyyy(), Float4::new(y, y, y, y));
    assert_eq!(a.yyyz(), Float4::new(y, y, y, z));
    assert_eq!(a.yyzx(), Float4::new(y, y, z, x));
    assert_eq!(a.yyzy(), Float4::new(y, y, z, y));
    assert_eq!(a.yyzz(), Float4::new(y, y, z, z));
    assert_eq!(a.yzxx(), Float4::new(y, z, x, x));
    assert_eq!(a.yzxy(), Float4::new(y, z, x, y));
    assert_eq!(a.yzxz(), Float4::new(y, z, x, z));
    assert_eq!(a.yzyx(), Float4::new(y, z, y, x));
    assert_eq!(a.yzyy(), Float4::new(y, z, y, y));
    assert_eq!(a.yzyz(), Float4::new(y, z, y, z));
    assert_eq!(a.yzzx(), Float4::new(y, z, z, x));
    assert_eq!(a.yzzy(), Float4::new(y, z, z, y));
    assert_eq!(a.yzzz(), Float4::new(y, z, z, z));
    assert_eq!(a.zxxx(), Float4::new(z, x, x, x));
    assert_eq!(a.zxxy(), Float4::new(z, x, x, y));
    assert_eq!(a.zxxz(), Float4::new(z, x, x, z));
    assert_eq!(a.zxyx(), Float4::new(z, x, y, x));
    assert_eq!(a.zxyy(), Float4::new(z, x, y, y));
    assert_eq!(a.zxyz(), Float4::new(z, x, y, z));
    assert_eq!(a.zxzx(), Float4::new(z, x, z, x));
    assert_eq!(a.zxzy(), Float4::new(z, x, z, y));
    assert_eq!(a.zxzz(), Float4::new(z, x, z, z));
    assert_eq!(a.zyxx(), Float4::new(z, y, x, x));
    assert_eq!(a.zyxy(), Float4::new(z, y, x, y));
    assert_eq!(a.zyxz(), Float4::new(z, y, x, z));
    assert_eq!(a.zyyx(), Float4::new(z, y, y, x));
    assert_eq!(a.zyyy(), Float4::new(z, y, y, y));
    assert_eq!(a.zyyz(), Float4::new(z, y, y, z));
    assert_eq!(a.zyzx(), Float4::new(z, y, z, x));
    assert_eq!(a.zyzy(), Float4::new(z, y, z, y));
    assert_eq!(a.zyzz(), Float4::new(z, y, z, z));
    assert_eq!(a.zzxx(), Float4::new(z, z, x, x));
    assert_eq!(a.zzxy(), Float4::new(z, z, x, y));
    assert_eq!(a.zzxz(), Float4::new(z, z, x, z));
    assert_eq!(a.zzyx(), Float4::new(z, z, y, x));
    assert_eq!(a.zzyy(), Float4::new(z, z, y, y));
    assert_eq!(a.zzyz(), Float4::new(z, z, y, z));
    assert_eq!(a.zzzx(), Float4::new(z, z, z, x));
    assert_eq!(a.zzzy(), Float4::new(z, z, z, y));
    assert_eq!(a.zzzz(), Float4::new(z, z, z, z));
}
