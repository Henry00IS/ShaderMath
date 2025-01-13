use crate::math::Float2;
use crate::math::Float3;
use crate::math::Float4;
use core::f32::consts::PI;

#[test]
fn float4_from() {
    let result = Float4::new(1.5, 2.25, 0.9, 0.5);
    assert_eq!(result.x, 1.5);
    assert_eq!(result.y, 2.25);
    assert_eq!(result.z, 0.9);
    assert_eq!(result.w, 0.5);
}

#[test]
fn float4_ops_add() {
    let a = Float4::new(1.5, 2.25, 0.9, 0.6);
    let b = Float4::new(3.1, 2.75, 0.1, 0.2);
    let result = a + b;
    assert_eq!(result.x, 4.6);
    assert_eq!(result.y, 5.0);
    assert_eq!(result.z, 1.0);
    assert_eq!(result.w, 0.8);
}

#[test]
fn float4_ops_add_f32() {
    let a = Float4::new(1.0, 2.0, 0.5, 0.2);
    let b = a + 0.5;
    assert_eq!(b, Float4::new(1.5, 2.5, 1.0, 0.7));
}

#[test]
fn float4_ops_add_assign() {
    let mut result = Float4::new(1.5, 2.25, 0.9, 0.5);
    let b = Float4::new(3.1, 2.75, 0.1, 0.5);
    result += b;
    assert_eq!(result.x, 4.6);
    assert_eq!(result.y, 5.0);
    assert_eq!(result.z, 1.0);
    assert_eq!(result.w, 1.0);
}

#[test]
fn float4_ops_add_assign_f32() {
    let mut a = Float4::new(1.0, 2.0, 0.5, 0.2);
    a += 0.5;
    assert_eq!(a, Float4::new(1.5, 2.5, 1.0, 0.7));
}

#[test]
fn float4_ops_sub() {
    let a = Float4::new(1.5, 2.25, 2.0, 2.0);
    let b = Float4::new(3.1, 2.75, 1.0, 4.0);
    let result = a - b;
    assert_eq!(result.x, -1.5999999);
    assert_eq!(result.y, -0.5);
    assert_eq!(result.z, 1.0);
    assert_eq!(result.w, -2.0);
}

#[test]
fn float4_ops_sub_f32() {
    let a = Float4::new(1.0, 2.0, 4.0, 8.0);
    let b = a - 0.5;
    assert_eq!(b, Float4::new(0.5, 1.5, 3.5, 7.5));
}

#[test]
fn float4_ops_sub_assign() {
    let mut result = Float4::new(1.5, 2.25, 4.0, 2.0);
    let b = Float4::new(3.1, 2.75, 3.0, 1.0);
    result -= b;
    assert_eq!(result.x, -1.5999999);
    assert_eq!(result.y, -0.5);
    assert_eq!(result.z, 1.0);
    assert_eq!(result.w, 1.0);
}

#[test]
fn float4_ops_sub_assign_f32() {
    let mut a = Float4::new(1.0, 2.0, 4.0, 8.0);
    a -= 0.5;
    assert_eq!(a, Float4::new(0.5, 1.5, 3.5, 7.5));
}

#[test]
fn float4_ops_mul() {
    let a = Float4::new(1.5, 2.5, 5.0, 2.0);
    let b = Float4::new(3.0, 2.0, 5.0, 2.0);
    let result = a * b;
    assert_eq!(result.x, 4.5);
    assert_eq!(result.y, 5.0);
    assert_eq!(result.z, 25.0);
    assert_eq!(result.w, 4.0);
}

#[test]
fn float4_ops_mul_f32() {
    let a = Float4::new(1.0, 2.0, 3.0, 4.0);
    let b = a * 0.5;
    assert_eq!(b, Float4::new(0.5, 1.0, 1.5, 2.0));
}

#[test]
fn float4_ops_mul_assign() {
    let mut result = Float4::new(1.5, 2.5, 5.0, 3.0);
    let b = Float4::new(3.0, 2.0, 1.5, 3.0);
    result *= b;
    assert_eq!(result.x, 4.5);
    assert_eq!(result.y, 5.0);
    assert_eq!(result.z, 7.5);
    assert_eq!(result.w, 9.0);
}

#[test]
fn float4_ops_mul_assign_f32() {
    let mut a = Float4::new(1.0, 2.0, 3.0, 4.0);
    a *= 0.5;
    assert_eq!(a, Float4::new(0.5, 1.0, 1.5, 2.0));
}

#[test]
fn float4_ops_div() {
    let a = Float4::new(1.5, 2.5, 5.0, 6.0);
    let b = Float4::new(3.0, 2.0, 2.5, 2.0);
    let result = a / b;
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 1.25);
    assert_eq!(result.z, 2.0);
    assert_eq!(result.w, 3.0);
}

#[test]
fn float4_ops_div_f32() {
    let a = Float4::new(1.0, 2.0, 5.0, 6.0);
    let b = a / 0.5;
    assert_eq!(b, Float4::new(2.0, 4.0, 10.0, 12.0));
}

#[test]
fn float4_ops_div_assign() {
    let mut result = Float4::new(1.5, 2.5, 5.0, 6.0);
    let b = Float4::new(3.0, 2.0, 2.5, 2.0);
    result /= b;
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 1.25);
    assert_eq!(result.z, 2.0);
    assert_eq!(result.w, 3.0);
}

#[test]
fn float4_ops_div_assign_f32() {
    let mut a = Float4::new(1.0, 2.0, 5.0, 6.0);
    a /= 0.5;
    assert_eq!(a, Float4::new(2.0, 4.0, 10.0, 12.0));
}

#[test]
fn float4_ops_neg() {
    let result = -Float4::new(1.5, 2.5, 4.1, -2.0);
    assert_eq!(result.x, -1.5);
    assert_eq!(result.y, -2.5);
    assert_eq!(result.z, -4.1);
    assert_eq!(result.w, 2.0);
}

#[test]
fn float4_equality() {
    let mut a = Float4::new(20.0, 1.0, 5.0, 3.0);
    let b = a;
    a += b;
    assert_eq!(a.x, 40.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(a.z, 10.0);
    assert_eq!(a.w, 6.0);
    assert_eq!(b.x, 20.0);
    assert_eq!(b.y, 1.0);
    assert_eq!(b.z, 5.0);
    assert_eq!(b.w, 3.0);

    assert_eq!(a == b, false);

    assert_ne!(a, b);
    let a = Float4::new(20.0, 1.0, 5.0, 3.0);
    assert_eq!(a, b);

    assert_eq!(a == b, true);
    assert_eq!(a != b, false);
}

#[test]
fn float4_abs() {
    let a = Float4::new(-PI, -PI * 2.0, -1.5, -11.0);
    let result = a.abs();
    assert_eq!(result.x, PI);
    assert_eq!(result.y, PI * 2.0);
    assert_eq!(result.z, 1.5);
    assert_eq!(result.w, 11.0);
}

#[test]
fn float4_acos() {
    let a = Float4::new(0.5, -0.5, 0.0, -1.0);
    let result = a.acos();
    assert_eq!(
        result,
        Float4::new(1.0471976, 2.0943952, 1.5707964, 3.1415927)
    );
}

#[test]
fn float4_all() {
    let a = Float4::new(0.0, 0.0, 0.0, 0.0);
    let result = a.all();
    assert_eq!(result, false);

    let a = Float4::new(0.1, 0.0, 0.0, 0.0);
    let result = a.all();
    assert_eq!(result, false);

    let a = Float4::new(0.0, 0.1, 0.0, 0.0);
    let result = a.all();
    assert_eq!(result, false);

    let a = Float4::new(0.0, 0.0, 0.1, 0.0);
    let result = a.all();
    assert_eq!(result, false);

    let a = Float4::new(0.0, 0.0, 0.0, 0.1);
    let result = a.all();
    assert_eq!(result, false);

    let a = Float4::new(-0.1, 0.1, 1.0, 0.3);
    let result = a.all();
    assert_eq!(result, true);
}

#[test]
fn float4_any() {
    let a = Float4::new(0.0, 0.0, 0.0, 0.0);
    let result = a.any();
    assert_eq!(result, false);

    let a = Float4::new(0.1, 0.0, 0.0, 0.0);
    let result = a.any();
    assert_eq!(result, true);

    let a = Float4::new(0.0, 0.1, 0.0, 0.0);
    let result = a.any();
    assert_eq!(result, true);

    let a = Float4::new(0.0, 0.0, 0.1, 0.0);
    let result = a.any();
    assert_eq!(result, true);

    let a = Float4::new(0.0, 0.0, 0.0, 0.1);
    let result = a.any();
    assert_eq!(result, true);

    let a = Float4::new(-0.1, 0.1, 1.0, 0.5);
    let result = a.any();
    assert_eq!(result, true);
}

#[test]
fn float4_asin() {
    let a = Float4::new(0.5, -0.5, 0.1, 0.5);
    let result = a.asin();
    assert_eq!(
        result,
        Float4::new(0.5235988, -0.5235988, 0.10016742, 0.5235988)
    );
}

#[test]
fn float4_atan() {
    let a = Float4::new(0.5, -0.5, 1.0, 0.1);
    let result = a.atan();
    assert_eq!(
        result,
        Float4::new(0.4636476, -0.4636476, 0.7853982, 0.09966865)
    );
}

#[test]
fn float4_ceil() {
    let a = Float4::new(0.9, -0.2, 1.2, 2.9);
    let result = a.ceil();
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 2.0);
    assert_eq!(result.w, 3.0);
}

#[test]
fn float4_clamp() {
    let a = Float4::new(0.9, -0.2, 0.6, -0.2);
    let result = a.clamp(0.0, 0.5);
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 0.5);
    assert_eq!(result.w, 0.0);
}

#[test]
fn float4_cos() {
    let a = Float4::new(PI, PI * 2.0, PI * 4.0, -PI);
    let result = a.cos();
    assert_eq!(result.x, -1.0);
    assert_eq!(result.y, 1.0);
    assert_eq!(result.z, 1.0);
    assert_eq!(result.w, -1.0);
}

#[test]
fn float4_cosh() {
    let a = Float4::new(0.9, -0.2, 0.5, 0.4);
    let result = a.cosh();
    assert_eq!(result.x, 1.4330864);
    assert_eq!(result.y, 1.0200667);
    assert_eq!(result.z, 1.127626);
    assert_eq!(result.w, 1.0810723);
}

#[test]
fn float4_degrees() {
    let a = Float4::new(PI, PI * 0.5, PI * 0.25, PI * 2.0);
    let result = a.degrees();
    assert_eq!(result.x, 180.0);
    assert_eq!(result.y, 90.0);
    assert_eq!(result.z, 45.0);
    assert_eq!(result.w, 360.0);
}

#[test]
fn float4_distance() {
    let a = Float4::new(1.0, 1.0, 1.0, 1.0);
    let b = Float4::new(5.0, 5.0, 5.0, 5.0);
    let result = a.distance(&b);
    assert_eq!(result, 8.0);
}

#[test]
fn float4_dot() {
    let a = Float4::new(1.0, 1.0, 0.5, 0.25);
    let b = Float4::new(5.0, 5.0, 0.5, 0.25);
    let result = a.dot(&b);
    assert_eq!(result, 10.3125);
}

#[test]
fn float4_exp() {
    let a = Float4::new(2.0, 4.0, 8.0, 16.0);
    let result = a.exp();
    assert_eq!(result.x, 7.389056);
    assert_eq!(result.y, 54.59815);
    assert_eq!(result.z, 2980.958);
    assert_eq!(result.w, 8886111.0);
}

#[test]
fn float4_exp2() {
    let a = Float4::new(2.0, 4.0, 8.0, 16.0);
    let result = a.exp2();
    assert_eq!(result.x, 4.0);
    assert_eq!(result.y, 16.0);
    assert_eq!(result.z, 256.0);
    assert_eq!(result.w, 65536.0);
}

#[test]
fn float4_floor() {
    let a = Float4::new(0.9, -0.2, 1.2, 0.1);
    let result = a.floor();
    assert_eq!(result.x, 0.0);
    assert_eq!(result.y, -1.0);
    assert_eq!(result.z, 1.0);
    assert_eq!(result.w, 0.0);
}

#[test]
fn float4_fmod() {
    let a = Float4::new(0.2, 2.0, 4.0, 6.0);
    let b = Float4::new(2.0, 4.0, 6.0, 4.0);
    let result = a.fmod(&b);
    assert_eq!(result.x, 0.2);
    assert_eq!(result.y, 2.0);
    assert_eq!(result.z, 4.0);
    assert_eq!(result.w, 2.0);
}

#[test]
fn float4_frac() {
    let a = Float4::new(24.50, 8.25, 1.75, 0.8);
    let result = a.frac();
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 0.25);
    assert_eq!(result.z, 0.75);
    assert_eq!(result.w, 0.8);
}

#[test]
fn float4_ldexp() {
    let value = Float4::new(1.5, 2.5, 1.0, 0.5);
    let exponent = Float4::new(2.0, -1.0, 1.0, 0.5);
    let result = value.ldexp(&exponent);
    assert_eq!(result.x, 6.0);
    assert_eq!(result.y, 1.25);
    assert_eq!(result.z, 2.0);
    assert_eq!(result.w, 0.5);
}

#[test]
fn float4_length() {
    let a = Float4::new(1.0, 1.0, 1.0, 1.0);
    let result = a.length();
    assert_eq!(result, 2.0);
}

#[test]
fn float4_lerp() {
    let a = Float4::new(0.0, 0.1, 0.5, 1.0);
    let b = Float4::new(2.0, 4.1, 1.0, 2.0);
    assert_eq!(a.lerp(&b, 0.5), Float4::new(1.0, 2.1, 0.75, 1.5));
    assert_eq!(a.lerp(&b, 1.5), Float4::new(3.0, 6.1, 1.25, 2.5));
}

#[test]
fn float4_log() {
    let a = Float4::new(1.0, 2.0, 4.0, 8.0);
    assert_eq!(a.log(), Float4::new(0.0, 0.6931472, 1.3862944, 2.0794415));
}

#[test]
fn float4_log10() {
    let a = Float4::new(1.0, 10.0, 20.0, 3.0);
    assert_eq!(a.log10(), Float4::new(0.0, 1.0, 1.30103, 0.47712126));
}

#[test]
fn float4_log2() {
    let a = Float4::new(1.0, 2.0, 4.0, 8.0);
    assert_eq!(a.log2(), Float4::new(0.0, 1.0, 2.0, 3.0));
}

#[test]
fn float4_mad() {
    let a = Float4::new(2.0, 2.0, 5.0, 2.0);
    let b = Float4::new(4.0, 5.0, 5.0, 2.0);
    let c = Float4::new(0.5, 0.25, 5.0, 3.0);
    assert_eq!(a.mad(&b, &c), Float4::new(8.5, 10.25, 30.0, 7.0));
}

#[test]
fn float4_max() {
    let a = Float4::new(2.0, 2.0, 2.0, 2.0);
    let b = Float4::new(4.0, 1.0, 5.0, 1.0);
    assert_eq!(a.max(&b), Float4::new(4.0, 2.0, 5.0, 2.0));
}

#[test]
fn float4_min() {
    let a = Float4::new(2.0, 2.0, 2.0, 2.0);
    let b = Float4::new(4.0, 1.0, 5.0, 1.0);
    assert_eq!(a.min(&b), Float4::new(2.0, 1.0, 2.0, 1.0));
}

#[test]
fn float4_normalize() {
    let a = Float4::new(2.0, 1.0, 3.0, 5.0);
    assert_eq!(
        a.normalize(),
        Float4::new(0.32025632, 0.16012816, 0.48038447, 0.80064076)
    );
    let a = Float4::new(0.0, 0.0, 0.0, 0.0);
    assert_eq!(a.normalize(), Float4::new(0.0, 0.0, 0.0, 0.0));
}

#[test]
fn float4_pow() {
    let a = Float4::new(2.0, 1.0, 5.0, 4.0);
    assert_eq!(a.pow(2.0), Float4::new(4.0, 1.0, 25.0, 16.0));
}

#[test]
fn float4_radians() {
    let a = Float4::new(180.0, 90.0, 45.0, 270.0);
    let result = a.radians();
    assert_eq!(result.x, PI);
    assert_eq!(result.y, PI * 0.5);
    assert_eq!(result.z, PI * 0.25);
    assert_eq!(result.w, PI * 1.5);
}

#[test]
fn float4_rcp() {
    let a = Float4::new(2.0, 4.0, 8.0, 16.0);
    let result = a.rcp();
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 0.25);
    assert_eq!(result.z, 0.125);
    assert_eq!(result.w, 0.0625);
    let a = Float4::new(2.0, 0.0, 8.0, 16.0);
    let result = a.rcp_safe();
    assert_eq!(result.y, 0.0);
}

#[test]
fn float4_reflect() {
    let incident = Float4::new(1.0, -1.0, 1.0, 0.0);
    let normal = Float4::new(0.0, 1.0, 0.0, 0.0);
    let result = incident.reflect(&normal);
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 1.0);
    assert_eq!(result.z, 1.0);
    assert_eq!(result.w, 0.0);
}

#[test]
fn float4_refract() {
    let incident = Float4::new(1.0, -1.0, 0.0, 0.0);
    let normal = Float4::new(0.0, 1.0, 0.0, 0.0);
    let eta = 0.5;
    let result = incident.refract(&normal, eta);
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, -1.0);
    assert_eq!(result.z, 0.0);
    assert_eq!(result.w, 0.0);
}

#[test]
fn float4_round() {
    let a = Float4::new(0.9, -0.2, 1.2, 0.5);
    let result = a.round();
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 1.0);
    assert_eq!(result.w, 1.0);
}

#[test]
fn float4_rsqrt() {
    let a = Float4::new(1.0, 4.0, 8.0, 3.0);
    let result = a.rsqrt();
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 0.5);
    assert_eq!(result.z, 0.35355338);
    assert_eq!(result.w, 0.57735026);
}

#[test]
fn float4_saturate() {
    let a = Float4::new(2.9, -0.2, 1.2, 0.1);
    let result = a.saturate();
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 1.0);
    assert_eq!(result.w, 0.1);
}

#[test]
fn float4_sign() {
    let a = Float4::new(2.9, -0.2, 0.3, -1.0);
    let result = a.sign();
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, -1.0);
    assert_eq!(result.z, 1.0);
    assert_eq!(result.w, -1.0);
}

#[test]
fn float4_sin() {
    let a = Float4::new(PI, PI * 2.0, 1.0, 90.0);
    let result = a.sin();
    assert_eq!(result.x, -8.742278e-8);
    assert_eq!(result.y, 1.7484555e-7);
    assert_eq!(result.z, 0.84147096);
    assert_eq!(result.w, 0.89399666);
}

#[test]
fn float4_sinh() {
    let a = Float4::new(0.9, -0.2, 1.0, 0.5);
    let result = a.sinh();
    assert_eq!(result.x, 1.0265167);
    assert_eq!(result.y, -0.20133601);
    assert_eq!(result.z, 1.1752012);
    assert_eq!(result.w, 0.5210953);
}

#[test]
fn float4_smoothstep() {
    let a = Float4::new(0.0, 0.0, 0.0, 0.0);
    let b = Float4::new(1.0, 1.0, 1.0, 1.0);
    let c = Float4::new(0.5, 1.5, 2.5, 0.5);
    let result = c.smoothstep(&a, &b);
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 1.0);
    assert_eq!(result.z, 1.0);
    assert_eq!(result.w, 0.5);
}

#[test]
fn float4_sqrt() {
    let a = Float4::new(4.0, 9.0, 16.0, 9.0);
    let result = a.sqrt();
    assert_eq!(result.x, 2.0);
    assert_eq!(result.y, 3.0);
    assert_eq!(result.z, 4.0);
    assert_eq!(result.w, 3.0);
}

#[test]
fn float4_step() {
    let a = Float4::new(0.5, 0.8, 3.0, 5.0);
    let b = Float4::new(0.3, 1.0, 4.0, 4.0);
    let result = a.step(&b);
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 0.0);
    assert_eq!(result.w, 1.0);
}

#[test]
fn float4_tan() {
    let a = Float4::new(PI / 4.0, PI / 6.0, PI, 0.5);
    let result = a.tan();
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 0.57735026);
    assert_eq!(result.z, 8.742278e-8);
    assert_eq!(result.w, 0.5463025);
}

#[test]
fn float4_tanh() {
    let a = Float4::new(PI / 4.0, PI / 6.0, PI / 8.0, PI / 16.0);
    let result = a.tanh();
    assert_eq!(result.x, 0.65579426);
    assert_eq!(result.y, 0.4804728);
    assert_eq!(result.z, 0.37368476);
    assert_eq!(result.w, 0.19386457);
}

#[test]
fn float4_trunc() {
    let a = Float4::new(25.2, 4.81, 1.02, 8.18);
    let result = a.trunc();
    assert_eq!(result.x, 25.0);
    assert_eq!(result.y, 4.0);
    assert_eq!(result.z, 1.0);
    assert_eq!(result.w, 8.0);
}

#[test]
fn float4_swizzle() {
    let x = 1.0;
    let y = 2.0;
    let z = 3.0;
    let w = 4.0;
    let a = Float4::new(x, y, z, w);

    assert_eq!(a.xx(), Float2::new(x, x));
    assert_eq!(a.xy(), Float2::new(x, y));
    assert_eq!(a.xz(), Float2::new(x, z));
    assert_eq!(a.xw(), Float2::new(x, w));
    assert_eq!(a.yx(), Float2::new(y, x));
    assert_eq!(a.yy(), Float2::new(y, y));
    assert_eq!(a.yz(), Float2::new(y, z));
    assert_eq!(a.yw(), Float2::new(y, w));
    assert_eq!(a.zx(), Float2::new(z, x));
    assert_eq!(a.zy(), Float2::new(z, y));
    assert_eq!(a.zz(), Float2::new(z, z));
    assert_eq!(a.zw(), Float2::new(z, w));
    assert_eq!(a.wx(), Float2::new(w, x));
    assert_eq!(a.wy(), Float2::new(w, y));
    assert_eq!(a.wz(), Float2::new(w, z));
    assert_eq!(a.ww(), Float2::new(w, w));

    assert_eq!(a.xxx(), Float3::new(x, x, x));
    assert_eq!(a.xxy(), Float3::new(x, x, y));
    assert_eq!(a.xxz(), Float3::new(x, x, z));
    assert_eq!(a.xxw(), Float3::new(x, x, w));
    assert_eq!(a.xyx(), Float3::new(x, y, x));
    assert_eq!(a.xyy(), Float3::new(x, y, y));
    assert_eq!(a.xyz(), Float3::new(x, y, z));
    assert_eq!(a.xyw(), Float3::new(x, y, w));
    assert_eq!(a.xzx(), Float3::new(x, z, x));
    assert_eq!(a.xzy(), Float3::new(x, z, y));
    assert_eq!(a.xzz(), Float3::new(x, z, z));
    assert_eq!(a.xzw(), Float3::new(x, z, w));
    assert_eq!(a.xwx(), Float3::new(x, w, x));
    assert_eq!(a.xwy(), Float3::new(x, w, y));
    assert_eq!(a.xwz(), Float3::new(x, w, z));
    assert_eq!(a.xww(), Float3::new(x, w, w));
    assert_eq!(a.yxx(), Float3::new(y, x, x));
    assert_eq!(a.yxy(), Float3::new(y, x, y));
    assert_eq!(a.yxz(), Float3::new(y, x, z));
    assert_eq!(a.yxw(), Float3::new(y, x, w));
    assert_eq!(a.yyx(), Float3::new(y, y, x));
    assert_eq!(a.yyy(), Float3::new(y, y, y));
    assert_eq!(a.yyz(), Float3::new(y, y, z));
    assert_eq!(a.yyw(), Float3::new(y, y, w));
    assert_eq!(a.yzx(), Float3::new(y, z, x));
    assert_eq!(a.yzy(), Float3::new(y, z, y));
    assert_eq!(a.yzz(), Float3::new(y, z, z));
    assert_eq!(a.yzw(), Float3::new(y, z, w));
    assert_eq!(a.ywx(), Float3::new(y, w, x));
    assert_eq!(a.ywy(), Float3::new(y, w, y));
    assert_eq!(a.ywz(), Float3::new(y, w, z));
    assert_eq!(a.yww(), Float3::new(y, w, w));
    assert_eq!(a.zxx(), Float3::new(z, x, x));
    assert_eq!(a.zxy(), Float3::new(z, x, y));
    assert_eq!(a.zxz(), Float3::new(z, x, z));
    assert_eq!(a.zxw(), Float3::new(z, x, w));
    assert_eq!(a.zyx(), Float3::new(z, y, x));
    assert_eq!(a.zyy(), Float3::new(z, y, y));
    assert_eq!(a.zyz(), Float3::new(z, y, z));
    assert_eq!(a.zyw(), Float3::new(z, y, w));
    assert_eq!(a.zzx(), Float3::new(z, z, x));
    assert_eq!(a.zzy(), Float3::new(z, z, y));
    assert_eq!(a.zzz(), Float3::new(z, z, z));
    assert_eq!(a.zzw(), Float3::new(z, z, w));
    assert_eq!(a.zwx(), Float3::new(z, w, x));
    assert_eq!(a.zwy(), Float3::new(z, w, y));
    assert_eq!(a.zwz(), Float3::new(z, w, z));
    assert_eq!(a.zww(), Float3::new(z, w, w));
    assert_eq!(a.wxx(), Float3::new(w, x, x));
    assert_eq!(a.wxy(), Float3::new(w, x, y));
    assert_eq!(a.wxz(), Float3::new(w, x, z));
    assert_eq!(a.wxw(), Float3::new(w, x, w));
    assert_eq!(a.wyx(), Float3::new(w, y, x));
    assert_eq!(a.wyy(), Float3::new(w, y, y));
    assert_eq!(a.wyz(), Float3::new(w, y, z));
    assert_eq!(a.wyw(), Float3::new(w, y, w));
    assert_eq!(a.wzx(), Float3::new(w, z, x));
    assert_eq!(a.wzy(), Float3::new(w, z, y));
    assert_eq!(a.wzz(), Float3::new(w, z, z));
    assert_eq!(a.wzw(), Float3::new(w, z, w));
    assert_eq!(a.wwx(), Float3::new(w, w, x));
    assert_eq!(a.wwy(), Float3::new(w, w, y));
    assert_eq!(a.wwz(), Float3::new(w, w, z));
    assert_eq!(a.www(), Float3::new(w, w, w));

    assert_eq!(a.xxxx(), Float4::new(x, x, x, x));
    assert_eq!(a.xxxy(), Float4::new(x, x, x, y));
    assert_eq!(a.xxxz(), Float4::new(x, x, x, z));
    assert_eq!(a.xxxw(), Float4::new(x, x, x, w));
    assert_eq!(a.xxyx(), Float4::new(x, x, y, x));
    assert_eq!(a.xxyy(), Float4::new(x, x, y, y));
    assert_eq!(a.xxyz(), Float4::new(x, x, y, z));
    assert_eq!(a.xxyw(), Float4::new(x, x, y, w));
    assert_eq!(a.xxzx(), Float4::new(x, x, z, x));
    assert_eq!(a.xxzy(), Float4::new(x, x, z, y));
    assert_eq!(a.xxzz(), Float4::new(x, x, z, z));
    assert_eq!(a.xxzw(), Float4::new(x, x, z, w));
    assert_eq!(a.xxwx(), Float4::new(x, x, w, x));
    assert_eq!(a.xxwy(), Float4::new(x, x, w, y));
    assert_eq!(a.xxwz(), Float4::new(x, x, w, z));
    assert_eq!(a.xxww(), Float4::new(x, x, w, w));
    assert_eq!(a.xyxx(), Float4::new(x, y, x, x));
    assert_eq!(a.xyxy(), Float4::new(x, y, x, y));
    assert_eq!(a.xyxz(), Float4::new(x, y, x, z));
    assert_eq!(a.xyxw(), Float4::new(x, y, x, w));
    assert_eq!(a.xyyx(), Float4::new(x, y, y, x));
    assert_eq!(a.xyyy(), Float4::new(x, y, y, y));
    assert_eq!(a.xyyz(), Float4::new(x, y, y, z));
    assert_eq!(a.xyyw(), Float4::new(x, y, y, w));
    assert_eq!(a.xyzx(), Float4::new(x, y, z, x));
    assert_eq!(a.xyzy(), Float4::new(x, y, z, y));
    assert_eq!(a.xyzz(), Float4::new(x, y, z, z));
    assert_eq!(a.xyzw(), Float4::new(x, y, z, w));
    assert_eq!(a.xywx(), Float4::new(x, y, w, x));
    assert_eq!(a.xywy(), Float4::new(x, y, w, y));
    assert_eq!(a.xywz(), Float4::new(x, y, w, z));
    assert_eq!(a.xyww(), Float4::new(x, y, w, w));
    assert_eq!(a.xzxx(), Float4::new(x, z, x, x));
    assert_eq!(a.xzxy(), Float4::new(x, z, x, y));
    assert_eq!(a.xzxz(), Float4::new(x, z, x, z));
    assert_eq!(a.xzxw(), Float4::new(x, z, x, w));
    assert_eq!(a.xzyx(), Float4::new(x, z, y, x));
    assert_eq!(a.xzyy(), Float4::new(x, z, y, y));
    assert_eq!(a.xzyz(), Float4::new(x, z, y, z));
    assert_eq!(a.xzyw(), Float4::new(x, z, y, w));
    assert_eq!(a.xzzx(), Float4::new(x, z, z, x));
    assert_eq!(a.xzzy(), Float4::new(x, z, z, y));
    assert_eq!(a.xzzz(), Float4::new(x, z, z, z));
    assert_eq!(a.xzzw(), Float4::new(x, z, z, w));
    assert_eq!(a.xzwx(), Float4::new(x, z, w, x));
    assert_eq!(a.xzwy(), Float4::new(x, z, w, y));
    assert_eq!(a.xzwz(), Float4::new(x, z, w, z));
    assert_eq!(a.xzww(), Float4::new(x, z, w, w));
    assert_eq!(a.xwxx(), Float4::new(x, w, x, x));
    assert_eq!(a.xwxy(), Float4::new(x, w, x, y));
    assert_eq!(a.xwxz(), Float4::new(x, w, x, z));
    assert_eq!(a.xwxw(), Float4::new(x, w, x, w));
    assert_eq!(a.xwyx(), Float4::new(x, w, y, x));
    assert_eq!(a.xwyy(), Float4::new(x, w, y, y));
    assert_eq!(a.xwyz(), Float4::new(x, w, y, z));
    assert_eq!(a.xwyw(), Float4::new(x, w, y, w));
    assert_eq!(a.xwzx(), Float4::new(x, w, z, x));
    assert_eq!(a.xwzy(), Float4::new(x, w, z, y));
    assert_eq!(a.xwzz(), Float4::new(x, w, z, z));
    assert_eq!(a.xwzw(), Float4::new(x, w, z, w));
    assert_eq!(a.xwwx(), Float4::new(x, w, w, x));
    assert_eq!(a.xwwy(), Float4::new(x, w, w, y));
    assert_eq!(a.xwwz(), Float4::new(x, w, w, z));
    assert_eq!(a.xwww(), Float4::new(x, w, w, w));
    assert_eq!(a.yxxx(), Float4::new(y, x, x, x));
    assert_eq!(a.yxxy(), Float4::new(y, x, x, y));
    assert_eq!(a.yxxz(), Float4::new(y, x, x, z));
    assert_eq!(a.yxxw(), Float4::new(y, x, x, w));
    assert_eq!(a.yxyx(), Float4::new(y, x, y, x));
    assert_eq!(a.yxyy(), Float4::new(y, x, y, y));
    assert_eq!(a.yxyz(), Float4::new(y, x, y, z));
    assert_eq!(a.yxyw(), Float4::new(y, x, y, w));
    assert_eq!(a.yxzx(), Float4::new(y, x, z, x));
    assert_eq!(a.yxzy(), Float4::new(y, x, z, y));
    assert_eq!(a.yxzz(), Float4::new(y, x, z, z));
    assert_eq!(a.yxzw(), Float4::new(y, x, z, w));
    assert_eq!(a.yxwx(), Float4::new(y, x, w, x));
    assert_eq!(a.yxwy(), Float4::new(y, x, w, y));
    assert_eq!(a.yxwz(), Float4::new(y, x, w, z));
    assert_eq!(a.yxww(), Float4::new(y, x, w, w));
    assert_eq!(a.yyxx(), Float4::new(y, y, x, x));
    assert_eq!(a.yyxy(), Float4::new(y, y, x, y));
    assert_eq!(a.yyxz(), Float4::new(y, y, x, z));
    assert_eq!(a.yyxw(), Float4::new(y, y, x, w));
    assert_eq!(a.yyyx(), Float4::new(y, y, y, x));
    assert_eq!(a.yyyy(), Float4::new(y, y, y, y));
    assert_eq!(a.yyyz(), Float4::new(y, y, y, z));
    assert_eq!(a.yyyw(), Float4::new(y, y, y, w));
    assert_eq!(a.yyzx(), Float4::new(y, y, z, x));
    assert_eq!(a.yyzy(), Float4::new(y, y, z, y));
    assert_eq!(a.yyzz(), Float4::new(y, y, z, z));
    assert_eq!(a.yyzw(), Float4::new(y, y, z, w));
    assert_eq!(a.yywx(), Float4::new(y, y, w, x));
    assert_eq!(a.yywy(), Float4::new(y, y, w, y));
    assert_eq!(a.yywz(), Float4::new(y, y, w, z));
    assert_eq!(a.yyww(), Float4::new(y, y, w, w));
    assert_eq!(a.yzxx(), Float4::new(y, z, x, x));
    assert_eq!(a.yzxy(), Float4::new(y, z, x, y));
    assert_eq!(a.yzxz(), Float4::new(y, z, x, z));
    assert_eq!(a.yzxw(), Float4::new(y, z, x, w));
    assert_eq!(a.yzyx(), Float4::new(y, z, y, x));
    assert_eq!(a.yzyy(), Float4::new(y, z, y, y));
    assert_eq!(a.yzyz(), Float4::new(y, z, y, z));
    assert_eq!(a.yzyw(), Float4::new(y, z, y, w));
    assert_eq!(a.yzzx(), Float4::new(y, z, z, x));
    assert_eq!(a.yzzy(), Float4::new(y, z, z, y));
    assert_eq!(a.yzzz(), Float4::new(y, z, z, z));
    assert_eq!(a.yzzw(), Float4::new(y, z, z, w));
    assert_eq!(a.yzwx(), Float4::new(y, z, w, x));
    assert_eq!(a.yzwy(), Float4::new(y, z, w, y));
    assert_eq!(a.yzwz(), Float4::new(y, z, w, z));
    assert_eq!(a.yzww(), Float4::new(y, z, w, w));
    assert_eq!(a.ywxx(), Float4::new(y, w, x, x));
    assert_eq!(a.ywxy(), Float4::new(y, w, x, y));
    assert_eq!(a.ywxz(), Float4::new(y, w, x, z));
    assert_eq!(a.ywxw(), Float4::new(y, w, x, w));
    assert_eq!(a.ywyx(), Float4::new(y, w, y, x));
    assert_eq!(a.ywyy(), Float4::new(y, w, y, y));
    assert_eq!(a.ywyz(), Float4::new(y, w, y, z));
    assert_eq!(a.ywyw(), Float4::new(y, w, y, w));
    assert_eq!(a.ywzx(), Float4::new(y, w, z, x));
    assert_eq!(a.ywzy(), Float4::new(y, w, z, y));
    assert_eq!(a.ywzz(), Float4::new(y, w, z, z));
    assert_eq!(a.ywzw(), Float4::new(y, w, z, w));
    assert_eq!(a.ywwx(), Float4::new(y, w, w, x));
    assert_eq!(a.ywwy(), Float4::new(y, w, w, y));
    assert_eq!(a.ywwz(), Float4::new(y, w, w, z));
    assert_eq!(a.ywww(), Float4::new(y, w, w, w));
    assert_eq!(a.zxxx(), Float4::new(z, x, x, x));
    assert_eq!(a.zxxy(), Float4::new(z, x, x, y));
    assert_eq!(a.zxxz(), Float4::new(z, x, x, z));
    assert_eq!(a.zxxw(), Float4::new(z, x, x, w));
    assert_eq!(a.zxyx(), Float4::new(z, x, y, x));
    assert_eq!(a.zxyy(), Float4::new(z, x, y, y));
    assert_eq!(a.zxyz(), Float4::new(z, x, y, z));
    assert_eq!(a.zxyw(), Float4::new(z, x, y, w));
    assert_eq!(a.zxzx(), Float4::new(z, x, z, x));
    assert_eq!(a.zxzy(), Float4::new(z, x, z, y));
    assert_eq!(a.zxzz(), Float4::new(z, x, z, z));
    assert_eq!(a.zxzw(), Float4::new(z, x, z, w));
    assert_eq!(a.zxwx(), Float4::new(z, x, w, x));
    assert_eq!(a.zxwy(), Float4::new(z, x, w, y));
    assert_eq!(a.zxwz(), Float4::new(z, x, w, z));
    assert_eq!(a.zxww(), Float4::new(z, x, w, w));
    assert_eq!(a.zyxx(), Float4::new(z, y, x, x));
    assert_eq!(a.zyxy(), Float4::new(z, y, x, y));
    assert_eq!(a.zyxz(), Float4::new(z, y, x, z));
    assert_eq!(a.zyxw(), Float4::new(z, y, x, w));
    assert_eq!(a.zyyx(), Float4::new(z, y, y, x));
    assert_eq!(a.zyyy(), Float4::new(z, y, y, y));
    assert_eq!(a.zyyz(), Float4::new(z, y, y, z));
    assert_eq!(a.zyyw(), Float4::new(z, y, y, w));
    assert_eq!(a.zyzx(), Float4::new(z, y, z, x));
    assert_eq!(a.zyzy(), Float4::new(z, y, z, y));
    assert_eq!(a.zyzz(), Float4::new(z, y, z, z));
    assert_eq!(a.zyzw(), Float4::new(z, y, z, w));
    assert_eq!(a.zywx(), Float4::new(z, y, w, x));
    assert_eq!(a.zywy(), Float4::new(z, y, w, y));
    assert_eq!(a.zywz(), Float4::new(z, y, w, z));
    assert_eq!(a.zyww(), Float4::new(z, y, w, w));
    assert_eq!(a.zzxx(), Float4::new(z, z, x, x));
    assert_eq!(a.zzxy(), Float4::new(z, z, x, y));
    assert_eq!(a.zzxz(), Float4::new(z, z, x, z));
    assert_eq!(a.zzxw(), Float4::new(z, z, x, w));
    assert_eq!(a.zzyx(), Float4::new(z, z, y, x));
    assert_eq!(a.zzyy(), Float4::new(z, z, y, y));
    assert_eq!(a.zzyz(), Float4::new(z, z, y, z));
    assert_eq!(a.zzyw(), Float4::new(z, z, y, w));
    assert_eq!(a.zzzx(), Float4::new(z, z, z, x));
    assert_eq!(a.zzzy(), Float4::new(z, z, z, y));
    assert_eq!(a.zzzz(), Float4::new(z, z, z, z));
    assert_eq!(a.zzzw(), Float4::new(z, z, z, w));
    assert_eq!(a.zzwx(), Float4::new(z, z, w, x));
    assert_eq!(a.zzwy(), Float4::new(z, z, w, y));
    assert_eq!(a.zzwz(), Float4::new(z, z, w, z));
    assert_eq!(a.zzww(), Float4::new(z, z, w, w));
    assert_eq!(a.zwxx(), Float4::new(z, w, x, x));
    assert_eq!(a.zwxy(), Float4::new(z, w, x, y));
    assert_eq!(a.zwxz(), Float4::new(z, w, x, z));
    assert_eq!(a.zwxw(), Float4::new(z, w, x, w));
    assert_eq!(a.zwyx(), Float4::new(z, w, y, x));
    assert_eq!(a.zwyy(), Float4::new(z, w, y, y));
    assert_eq!(a.zwyz(), Float4::new(z, w, y, z));
    assert_eq!(a.zwyw(), Float4::new(z, w, y, w));
    assert_eq!(a.zwzx(), Float4::new(z, w, z, x));
    assert_eq!(a.zwzy(), Float4::new(z, w, z, y));
    assert_eq!(a.zwzz(), Float4::new(z, w, z, z));
    assert_eq!(a.zwzw(), Float4::new(z, w, z, w));
    assert_eq!(a.zwwx(), Float4::new(z, w, w, x));
    assert_eq!(a.zwwy(), Float4::new(z, w, w, y));
    assert_eq!(a.zwwz(), Float4::new(z, w, w, z));
    assert_eq!(a.zwww(), Float4::new(z, w, w, w));
    assert_eq!(a.wxxx(), Float4::new(w, x, x, x));
    assert_eq!(a.wxxy(), Float4::new(w, x, x, y));
    assert_eq!(a.wxxz(), Float4::new(w, x, x, z));
    assert_eq!(a.wxxw(), Float4::new(w, x, x, w));
    assert_eq!(a.wxyx(), Float4::new(w, x, y, x));
    assert_eq!(a.wxyy(), Float4::new(w, x, y, y));
    assert_eq!(a.wxyz(), Float4::new(w, x, y, z));
    assert_eq!(a.wxyw(), Float4::new(w, x, y, w));
    assert_eq!(a.wxzx(), Float4::new(w, x, z, x));
    assert_eq!(a.wxzy(), Float4::new(w, x, z, y));
    assert_eq!(a.wxzz(), Float4::new(w, x, z, z));
    assert_eq!(a.wxzw(), Float4::new(w, x, z, w));
    assert_eq!(a.wxwx(), Float4::new(w, x, w, x));
    assert_eq!(a.wxwy(), Float4::new(w, x, w, y));
    assert_eq!(a.wxwz(), Float4::new(w, x, w, z));
    assert_eq!(a.wxww(), Float4::new(w, x, w, w));
    assert_eq!(a.wyxx(), Float4::new(w, y, x, x));
    assert_eq!(a.wyxy(), Float4::new(w, y, x, y));
    assert_eq!(a.wyxz(), Float4::new(w, y, x, z));
    assert_eq!(a.wyxw(), Float4::new(w, y, x, w));
    assert_eq!(a.wyyx(), Float4::new(w, y, y, x));
    assert_eq!(a.wyyy(), Float4::new(w, y, y, y));
    assert_eq!(a.wyyz(), Float4::new(w, y, y, z));
    assert_eq!(a.wyyw(), Float4::new(w, y, y, w));
    assert_eq!(a.wyzx(), Float4::new(w, y, z, x));
    assert_eq!(a.wyzy(), Float4::new(w, y, z, y));
    assert_eq!(a.wyzz(), Float4::new(w, y, z, z));
    assert_eq!(a.wyzw(), Float4::new(w, y, z, w));
    assert_eq!(a.wywx(), Float4::new(w, y, w, x));
    assert_eq!(a.wywy(), Float4::new(w, y, w, y));
    assert_eq!(a.wywz(), Float4::new(w, y, w, z));
    assert_eq!(a.wyww(), Float4::new(w, y, w, w));
    assert_eq!(a.wzxx(), Float4::new(w, z, x, x));
    assert_eq!(a.wzxy(), Float4::new(w, z, x, y));
    assert_eq!(a.wzxz(), Float4::new(w, z, x, z));
    assert_eq!(a.wzxw(), Float4::new(w, z, x, w));
    assert_eq!(a.wzyx(), Float4::new(w, z, y, x));
    assert_eq!(a.wzyy(), Float4::new(w, z, y, y));
    assert_eq!(a.wzyz(), Float4::new(w, z, y, z));
    assert_eq!(a.wzyw(), Float4::new(w, z, y, w));
    assert_eq!(a.wzzx(), Float4::new(w, z, z, x));
    assert_eq!(a.wzzy(), Float4::new(w, z, z, y));
    assert_eq!(a.wzzz(), Float4::new(w, z, z, z));
    assert_eq!(a.wzzw(), Float4::new(w, z, z, w));
    assert_eq!(a.wzwx(), Float4::new(w, z, w, x));
    assert_eq!(a.wzwy(), Float4::new(w, z, w, y));
    assert_eq!(a.wzwz(), Float4::new(w, z, w, z));
    assert_eq!(a.wzww(), Float4::new(w, z, w, w));
    assert_eq!(a.wwxx(), Float4::new(w, w, x, x));
    assert_eq!(a.wwxy(), Float4::new(w, w, x, y));
    assert_eq!(a.wwxz(), Float4::new(w, w, x, z));
    assert_eq!(a.wwxw(), Float4::new(w, w, x, w));
    assert_eq!(a.wwyx(), Float4::new(w, w, y, x));
    assert_eq!(a.wwyy(), Float4::new(w, w, y, y));
    assert_eq!(a.wwyz(), Float4::new(w, w, y, z));
    assert_eq!(a.wwyw(), Float4::new(w, w, y, w));
    assert_eq!(a.wwzx(), Float4::new(w, w, z, x));
    assert_eq!(a.wwzy(), Float4::new(w, w, z, y));
    assert_eq!(a.wwzz(), Float4::new(w, w, z, z));
    assert_eq!(a.wwzw(), Float4::new(w, w, z, w));
    assert_eq!(a.wwwx(), Float4::new(w, w, w, x));
    assert_eq!(a.wwwy(), Float4::new(w, w, w, y));
    assert_eq!(a.wwwz(), Float4::new(w, w, w, z));
    assert_eq!(a.wwww(), Float4::new(w, w, w, w));
}
