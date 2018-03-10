use super::Poly;

#[test]
fn literals() {
  assert_eq!(format!("{:?}", Poly::zero()), "Poly { coef_map: {} }");
  assert_eq!(format!("{:?}", Poly::number(42)),
             "Poly { coef_map: {0: 42} }");
  assert_eq!(format!("{:?}", Poly::identity()),
             "Poly { coef_map: {1: 1} }");
  assert_eq!(format!("{:?}", Poly::inverse_identity()),
             "Poly { coef_map: {-1: 1} }");
}

#[test]
fn coef_access() {
  let mut poly = Poly::number(15);
  assert_eq!(poly.get_coef(0), 15);
  assert_eq!(poly.get_coef(1), 0);
  poly.set_coef(1, 40);
  assert_eq!(poly.get_coef(1), 40);
  poly.set_coef(0, 0);
  assert_eq!(poly.get_coef(0), 0);
}

#[test]
fn equality() {
  let mut poly = Poly::zero();
  assert_eq!(&poly, &Poly::zero());
  poly.set_coef(-1, 1);
  assert_ne!(&poly, &Poly::zero());
  assert_eq!(&poly, &Poly::inverse_identity());
  poly.set_coef(5, 0);
  assert_eq!(&poly, &Poly::inverse_identity());
}

#[test]
fn arithmetic() {
  let mut poly1 = Poly::zero();
  poly1 += &Poly::identity();
  poly1 += &Poly::identity();
  poly1 += &(&Poly::inverse_identity() * 2);
  poly1 -= &Poly::number(4);
  
  let mut poly2 = Poly::zero();
  poly2.set_coef(-1, -1);
  poly2.set_coef(0, 2);
  poly2.set_coef(1, -1);
  poly2 *= 2;

  assert_eq!(&poly1, &-&poly2);
}

#[test]
fn multiplication() {
  let mut poly1 = Poly::zero();
  poly1.set_coef(2, 1);
  poly1.set_coef(0, 1);
  
  let mut poly2 = Poly::zero();
  poly2.set_coef(2, 1);
  poly2.set_coef(0, -1);

  let mut poly3 = Poly::zero();
  poly3.set_coef(4, 1);
  poly3.set_coef(0, -1);

  assert_eq!(&(&poly1 * &poly2), &poly3);
}

#[test]
fn mirror() {
  assert_eq!(&Poly::zero().mirror(), &Poly::zero());
  assert_eq!(&Poly::identity().mirror(), &Poly::inverse_identity());

  let mut poly = Poly::zero();
  poly.set_coef(5, -7);
  poly.set_coef(-7, 5);
  let mirr = poly.mirror();
  assert_eq!(mirr.get_coef(-7), 0);
  assert_eq!(mirr.get_coef(-5), -7);
  assert_eq!(mirr.get_coef(5), 0);
  assert_eq!(mirr.get_coef(7), 5);
}
