use crate::math::Float2;
use crate::math::Float3;
use core::f32::consts::PI;

#[test]
fn float2_from() {
    let result = Float2::new(1.5, 2.25);
    assert_eq!(result.x, 1.5);
    assert_eq!(result.y, 2.25);
}

#[test]
fn float2_ops_add() {
    let a = Float2::new(1.5, 2.25);
    let b = Float2::new(3.1, 2.75);
    let result = a + b;
    assert_eq!(result.x, 4.6);
    assert_eq!(result.y, 5.0);
}

#[test]
fn float2_ops_add_assign() {
    let mut result = Float2::new(1.5, 2.25);
    let b = Float2::new(3.1, 2.75);
    result += b;
    assert_eq!(result.x, 4.6);
    assert_eq!(result.y, 5.0);
}

#[test]
fn float2_ops_sub() {
    let a = Float2::new(1.5, 2.25);
    let b = Float2::new(3.1, 2.75);
    let result = a - b;
    assert_eq!(result.x, -1.5999999);
    assert_eq!(result.y, -0.5);
}

#[test]
fn float2_ops_sub_assign() {
    let mut result = Float2::new(1.5, 2.25);
    let b = Float2::new(3.1, 2.75);
    result -= b;
    assert_eq!(result.x, -1.5999999);
    assert_eq!(result.y, -0.5);
}

#[test]
fn float2_ops_mul() {
    let a = Float2::new(1.5, 2.5);
    let b = Float2::new(3.0, 2.0);
    let result = a * b;
    assert_eq!(result.x, 4.5);
    assert_eq!(result.y, 5.0);
}

#[test]
fn float2_ops_mul_assign() {
    let mut result = Float2::new(1.5, 2.5);
    let b = Float2::new(3.0, 2.0);
    result *= b;
    assert_eq!(result.x, 4.5);
    assert_eq!(result.y, 5.0);
}

#[test]
fn float2_ops_div() {
    let a = Float2::new(1.5, 2.5);
    let b = Float2::new(3.0, 2.0);
    let result = a / b;
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 1.25);
}

#[test]
fn float2_ops_div_assign() {
    let mut result = Float2::new(1.5, 2.5);
    let b = Float2::new(3.0, 2.0);
    result /= b;
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 1.25);
}

#[test]
fn float2_neg() {
    let result = -Float2::new(1.5, 2.5);
    assert_eq!(result.x, -1.5);
    assert_eq!(result.y, -2.5);
}

#[test]
fn float2_sin() {
    let a = Float2::new(PI, PI * 2.0);
    let result = a.sin();
    assert_eq!(result.x, -8.742278e-8);
    assert_eq!(result.y, 1.7484555e-7);
}

#[test]
fn float2_acos() {
    let a = Float2::new(0.5, -0.5);
    let result = a.acos();
    assert_eq!(result, Float2::new(1.0471976, 2.0943952));
}

#[test]
fn float2_asin() {
    let a = Float2::new(0.5, -0.5);
    let result = a.asin();
    assert_eq!(result, Float2::new(0.5235988, -0.5235988));
}

#[test]
fn float2_cos() {
    let a = Float2::new(PI, PI * 2.0);
    let result = a.cos();
    assert_eq!(result.x, -1.0);
    assert_eq!(result.y, 1.0);
}

#[test]
fn float2_abs() {
    let a = Float2::new(-PI, -PI * 2.0);
    let result = a.abs();
    assert_eq!(result.x, PI);
    assert_eq!(result.y, PI * 2.0);
}

#[test]
fn float2_all() {
    let a = Float2::new(0.0, 0.0);
    let result = a.all();
    assert_eq!(result, false);

    let a = Float2::new(0.1, 0.0);
    let result = a.all();
    assert_eq!(result, false);

    let a = Float2::new(0.0, 0.1);
    let result = a.all();
    assert_eq!(result, false);

    let a = Float2::new(-0.1, 0.1);
    let result = a.all();
    assert_eq!(result, true);
}

#[test]
fn float2_any() {
    let a = Float2::new(0.0, 0.0);
    let result = a.any();
    assert_eq!(result, false);

    let a = Float2::new(0.1, 0.0);
    let result = a.any();
    assert_eq!(result, true);

    let a = Float2::new(0.0, 0.1);
    let result = a.any();
    assert_eq!(result, true);

    let a = Float2::new(-0.1, 0.1);
    let result = a.any();
    assert_eq!(result, true);
}

#[test]
fn float2_equality() {
    let mut a = Float2::new(20.0, 1.0);
    let b = a;
    a += b;
    assert_eq!(a.x, 40.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(b.x, 20.0);
    assert_eq!(b.y, 1.0);

    assert_eq!(a == b, false);

    assert_ne!(a, b);
    let a = Float2::new(20.0, 1.0);
    assert_eq!(a, b);

    assert_eq!(a == b, true);
    assert_eq!(a != b, false);
}

#[test]
fn float2_swizzle()
{
    let x = 1.0;
    let y = 2.0;
    let a = Float2::new(x, y);
    assert_eq!(a.xx(), Float2::new(x, x));
    assert_eq!(a.xy(), Float2::new(x, y));
    assert_eq!(a.yx(), Float2::new(y, x));
    assert_eq!(a.yy(), Float2::new(y, y));
    assert_eq!(a.xxx(), Float3::new(x, x, x));
    assert_eq!(a.xxy(), Float3::new(x, x, y));
    assert_eq!(a.xyx(), Float3::new(x, y, x));
    assert_eq!(a.xyy(), Float3::new(x, y, y));
    assert_eq!(a.yxx(), Float3::new(y, x, x));
    assert_eq!(a.yxy(), Float3::new(y, x, y));
    assert_eq!(a.yyx(), Float3::new(y, y, x));
    assert_eq!(a.yyy(), Float3::new(y, y, y));
}