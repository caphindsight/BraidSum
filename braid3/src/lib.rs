#[cfg(test)]
mod test;

/// Represents a twist (an atom of braiding) on the set of three strands.
/// `A` stands for forward-slash twisting of the first two strands.
/// `B` stands for forward-slash twisting of the last two strands.
/// `Ainv` and `Binv` stand for inversed `A` and `B` respectively.
#[derive(Eq, Clone, Copy, Debug, PartialEq)]
pub enum Twist {
  A, B, Ainv, Binv
}

/// Represents a braid (a sequence of twist) on the set of three strands.
#[derive(Clone, Debug, PartialEq)]
pub struct Braid {
  pub twists: Vec<Twist>,
}

impl Braid {
  /// Gives the identity in the braid group (braid with no twists).
  pub fn identity() -> Braid {
    Braid { twists: Vec::new() }
  }

  pub fn canonical_len(&self) -> usize {
    self.twists.len()
  }

  pub fn last_twist(&self) -> Option<Twist> {
    self.twists.last().cloned()
  }

  fn ends_with(&self, ending: &Vec<Twist>) -> bool {
    let n = self.twists.len();
    let m = ending.len();
    if n < m {
      return false;
    }
    for i in 0..m {
      if self.twists[n - i - 1] != ending[m - i - 1] {
        return false;
      }
    }
    true
  }

  /// Returns a list of braids (in lexicographic order) which can be obtained
  /// from `self` by adding a single twist to its end. Excludes resulting braids
  /// in non-canonical form (i.e. containing reducible sequences like:
  /// * `A Ainv`, `B Binv`, `Ainv A`, `Binv B`, because they are equal to `1`.
  /// * `B A B`, `Binv Ainv Binv`, because they can be reduced to `A B A` and
  ///   `Ainv Binv Ainv` respectively.
  /// * `Binv A B A`, `A B A Binv`, `B Ainv Binv Ainv`, `Ainv Binv Ainb B`,
  ///    because they can be reduced to `A B`, `B A`, Ainv Binv`, `Binv Ainv`
  ///    respectively.
  pub fn descendants(&self) -> Vec<Braid> {
    let mut res = Vec::<Braid>::with_capacity(4);
    let n = self.twists.len();
    let mut new_twists = Vec::with_capacity(n + 1);
    for i in &self.twists {
      new_twists.push(*i);
    }

    // We can add `A` to the braid unless it ends with:
    // * `Ainv`, in which case `Ainv A` == `1`.
    // * `Binv A B`, in which case `Binv A B A` == `Binv B A B` == `A B`.
    if !self.ends_with(&vec![Twist::Ainv]) &&
       !self.ends_with(&vec![Twist::Binv, Twist::A, Twist::B]) {
      new_twists.push(Twist::A);
      res.push(Braid { twists: new_twists.clone() });
      new_twists.pop();
    }

    // We can add `B` to the braid unless it ends with:
    // * `Binv`, in which case `Binv B` == `1`.
    // * `B A`, in which case `B A B` == `A B A`, and we allow the 2nd form.
    // * `Ainv Binv Ainv`, in which case `Ainv Binv Ainv B` == `Binv Ainv`.
    if !self.ends_with(&vec![Twist::Binv]) &&
       !self.ends_with(&vec![Twist::B, Twist::A]) &&
       !self.ends_with(&vec![Twist::Ainv, Twist::Binv, Twist::Ainv]) {
      new_twists.push(Twist::B);
      res.push(Braid { twists: new_twists.clone() });
      new_twists.pop();
    }

    // We can add `Ainv` to the braid unless it ends with:
    // * `A`, in which case `A Ainv` == `1`.
    // * `B Ainv Binv`, in which case `B Ainv Binv Ainv` == `B Binv Ainv Binv`
    //     == `Ainv Binv`.
    if !self.ends_with(&vec![Twist::A]) &&
       !self.ends_with(&vec![Twist::B, Twist::Ainv, Twist::Binv]) {
      new_twists.push(Twist::Ainv);
      res.push(Braid { twists: new_twists.clone() });
      new_twists.pop();
    }

    // We can add `Binv` to the braid unless it ends with:
    // * `B`, in which case `B Binv` == `1`.
    // * `Binv Ainv`, in which case `Binv Ainv Binv` == `Ainv Binv Ainv`,
    //     and we allow the 2nd form.
    // * `A B A`, in which case `A B A Binv` == `B A`.
    if !self.ends_with(&vec![Twist::B]) &&
       !self.ends_with(&vec![Twist::Binv, Twist::Ainv]) &&
       !self.ends_with(&vec![Twist::A, Twist::B, Twist::A]) {
      new_twists.push(Twist::Binv);
      res.push(Braid { twists: new_twists.clone() });
      new_twists.pop();
    }

    res
  }
}
