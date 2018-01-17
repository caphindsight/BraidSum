#[cfg(test)]
mod test;

use std::collections::HashMap;

/// Laurent Polynomial in one variable (t), with integer coefficients.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Poly {
  coef_map: HashMap<i64, i64>,
}

impl Poly {
  /// Gives the P(t) = 0 polynomial.
  pub fn zero() -> Poly {
    Poly { coef_map: HashMap::new() }
  }

  /// Gives the P(t) = `num` polynomial.
  pub fn number(num: i64) -> Poly {
    let mut coef_map = HashMap::with_capacity(1);
    coef_map.insert(0, num);
    coef_map.shrink_to_fit();
    Poly { coef_map }
  }

  /// Gives the P(t) = t polynomial.
  pub fn identity() -> Poly {
    let mut coef_map = HashMap::with_capacity(1);
    coef_map.insert(1, 1);
    coef_map.shrink_to_fit();
    Poly { coef_map }
  }

  /// Gives the P(t) = t^-1 polynomial.
  pub fn inverse_identity() -> Poly {
    let mut coef_map = HashMap::with_capacity(1);
    coef_map.insert(-1, 1);
    coef_map.shrink_to_fit();
    Poly { coef_map }
  }

  /// Gets the coefficient in front of the t^`exp` power.
  pub fn get_coef(&self, exp: i64) -> i64 {
    self.coef_map.get(&exp).cloned().unwrap_or(0)
  }

  /// Sets the coefficient in front of the t^`exp` power to `coef`.
  pub fn set_coef(&mut self, exp: i64, coef: i64) {
    self.coef_map.insert(exp, coef);
  }

  /// Gets the mirror polynomial for P(t): M(t) = P(t^-1 ).
  pub fn mirror(&self) -> Poly {
    let mut coef_map = HashMap::with_capacity(self.coef_map.len());
    for (k, v) in self.coef_map.iter() {
      coef_map.insert(-(*k), *v);
    }
    Poly { coef_map }
  }
}

impl<'a> std::ops::Add for &'a Poly {
  type Output = Poly;
  fn add(self, rhs: &'a Poly) -> Poly {
    let mut res = self.clone();
    res += rhs;
    res
  }
}

impl<'a> std::ops::AddAssign<&'a Poly> for Poly {
  fn add_assign(&mut self, rhs: &'a Poly) {
    for (k, v) in rhs.coef_map.iter() {
      let current = self.coef_map.get(k).cloned().unwrap_or(0);
      self.coef_map.insert(*k, current + *v);
    }
  }
}

impl<'a> std::ops::Neg for &'a Poly {
  type Output = Poly;
  fn neg(self) -> Poly {
    let mut res = self.clone();
    for (_, v) in res.coef_map.iter_mut() {
      *v = -(*v);
    }
    res
  }
}

impl<'a> std::ops::Sub for &'a Poly {
  type Output = Poly;
  fn sub(self, rhs: &'a Poly) -> Poly {
    let mut res = self.clone();
    res -= rhs;
    res
  }
}

impl<'a> std::ops::SubAssign<&'a Poly> for Poly {
  fn sub_assign(&mut self, rhs: &'a Poly) {
    for (k, v) in rhs.coef_map.iter() {
      let current = self.coef_map.get(k).cloned().unwrap_or(0);
      self.coef_map.insert(*k, current - *v);
    }
  }
}

impl<'a> std::ops::Mul<i64> for &'a Poly {
  type Output = Poly;
  fn mul(self, rhs: i64) -> Poly {
    let mut res = self.clone();
    res *= rhs;
    res
  }
}

impl std::ops::MulAssign<i64> for Poly {
  fn mul_assign(&mut self, rhs: i64) {
    for (_, v) in self.coef_map.iter_mut() {
      *v *= rhs;
    }
  }
}
