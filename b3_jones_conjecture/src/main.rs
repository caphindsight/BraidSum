extern crate poly;
use poly::Poly;

extern crate braid3;
use braid3::Braid;
use braid3::Twist;

// Data used by the dynamical programming algorithm to efficietly calculate the
// Jones polynomial for descendant braids. For more info on how the algorithm
// works and {A,B,C,D,E}-type joinings please read the pdf paper.
struct BraidData {
  // The element of braid group for which data is stored.
  braid: Braid,

  // Kauffman bracket for the knot obtained by A-type joining of strands.
  kauffman_a: Poly,

  // Kauffman bracket for the knot obtained by B-type joining of strands.
  kauffman_b: Poly,

  // Kauffman bracket for the knot obtained by C-type joining of strands.
  kauffman_c: Poly,

  // Kauffman bracket for the knot obtained by D-type joining of strands.
  kauffman_d: Poly,

  // Kauffman bracket for the knot obtained by E-type joining of strands.
  kauffman_e: Poly,

  // Writhe of the corresponding knot (always A-type joining).
  writhe: i64,

  // Jones polynomial of the corresponding knot (always A-type joining).
  jones: Poly,
}

// Calculate Jones polynomial given the Kauffman bracket and the writhe number.
fn calc_jones(kauffman: &Poly, writhe: i64) -> Poly {
  let mut writhe_poly = Poly::zero();
  writhe_poly.set_coef(-3 * writhe, 1 - 2 * (writhe % 2));
  kauffman * (&writhe_poly)
}

// Kauffman bracket for the unknot: `-(t^{-2} + t^2)`.
fn kauffman_unknot() -> Poly {
    let mut res = Poly::zero();
    res.set_coef(-2, -1);
    res.set_coef(2,  -1);
    res
}

impl BraidData {
  // Braid data for the identity element of the braid group.
  fn identity_braid() -> BraidData {
    let kauffman_unknot_1 = kauffman_unknot();
    
    // Kauffman bracket for unknot, squared.
    let mut kauffman_unknot_2 = kauffman_unknot_1.clone();
    kauffman_unknot_2 *= &kauffman_unknot_1;
    
    // Kauffman bracket for unknot, cubed.
    let mut kauffman_unknot_3 = kauffman_unknot_2.clone();
    kauffman_unknot_3 *= &kauffman_unknot_1;

    BraidData {
      braid: Braid::identity(),
      
      // A-type joining of three untwisted strands gives 3 unknots.
      kauffman_a: kauffman_unknot_3.clone(),
      
      // B-type joining of three untwisted strands gives 2 unknots.
      kauffman_b: kauffman_unknot_2.clone(),
      
      // C-type joining of three untwisted strands gives 2 unknots.
      kauffman_c: kauffman_unknot_2.clone(),
      
      // D-type joining of three untwisted strands gives 1 unknots.
      kauffman_d: kauffman_unknot_1.clone(),
      
      // E-type joining of three untwisted strands gives 1 unknots.
      kauffman_e: kauffman_unknot_1.clone(),

      writhe: 0,
      jones: calc_jones(&kauffman_unknot_3, 0),
    }
  }

  // Calculates BraidData for descendants of the element of the braid group.
  fn descendants(&self) -> Vec<BraidData> {
    let mut res = Vec::with_capacity(4);
    for braid in self.braid.descendants().into_iter() {
      match braid.last_twist().unwrap() {
        Twist::A => {
          let kauffman_a =
            &(&Poly::identity() * &self.kauffman_a) +
            &(&Poly::inverse_identity() * &self.kauffman_b);

          let kauffman_b =
            &(&Poly::identity() * &self.kauffman_b) +
            &(&(&Poly::inverse_identity() * &self.kauffman_b) *
              &kauffman_unknot());
          
          let kauffman_c =
            &(&Poly::identity() * &self.kauffman_c) +
            &(&Poly::inverse_identity() * &self.kauffman_d);

          let kauffman_d =
            &(&Poly::identity() * &self.kauffman_d) +
            &(&(&Poly::inverse_identity() * &self.kauffman_d) *
              &kauffman_unknot());
          
          let kauffman_e =
            &(&Poly::identity() * &self.kauffman_e) +
            &(&Poly::inverse_identity() * &self.kauffman_b);

          let writhe = self.writhe + 1;
          let jones = calc_jones(&kauffman_a, writhe);

          res.push(BraidData {
            braid,
            kauffman_a,
            kauffman_b,
            kauffman_c,
            kauffman_d,
            kauffman_e,
            writhe,
            jones,
          });
        },
 
        Twist::B => {
          let kauffman_a =
            &(&Poly::identity() * &self.kauffman_a) +
            &(&Poly::inverse_identity() * &self.kauffman_c);

          let kauffman_b =
            &(&Poly::identity() * &self.kauffman_b) +
            &(&Poly::inverse_identity() * &self.kauffman_e);
          
          let kauffman_c =
            &(&Poly::identity() * &self.kauffman_c) +
            &(&(&Poly::inverse_identity() * &self.kauffman_c) *
              &kauffman_unknot());

          let kauffman_d =
            &(&Poly::identity() * &self.kauffman_d) +
            &(&Poly::inverse_identity() * &self.kauffman_c);
          
          let kauffman_e =
            &(&Poly::identity() * &self.kauffman_e) +
            &(&(&Poly::inverse_identity() * &self.kauffman_e) *
              &kauffman_unknot());

          let writhe = self.writhe + 1;
          let jones = calc_jones(&kauffman_a, writhe);

          res.push(BraidData {
            braid,
            kauffman_a,
            kauffman_b,
            kauffman_c,
            kauffman_d,
            kauffman_e,
            writhe,
            jones,
          });
        },
        
        Twist::Ainv => {
          let kauffman_a =
            &(&Poly::inverse_identity() * &self.kauffman_a) +
            &(&Poly::identity() * &self.kauffman_b);

          let kauffman_b =
            &(&Poly::inverse_identity() * &self.kauffman_b) +
            &(&(&Poly::identity() * &self.kauffman_b) *
              &kauffman_unknot());
          
          let kauffman_c =
            &(&Poly::inverse_identity() * &self.kauffman_c) +
            &(&Poly::identity() * &self.kauffman_d);

          let kauffman_d =
            &(&Poly::inverse_identity() * &self.kauffman_d) +
            &(&(&Poly::identity() * &self.kauffman_d) *
              &kauffman_unknot());
          
          let kauffman_e =
            &(&Poly::inverse_identity() * &self.kauffman_e) +
            &(&Poly::identity() * &self.kauffman_b);

          let writhe = self.writhe - 1;
          let jones = calc_jones(&kauffman_a, writhe);

          res.push(BraidData {
            braid,
            kauffman_a,
            kauffman_b,
            kauffman_c,
            kauffman_d,
            kauffman_e,
            writhe,
            jones,
          });
        },
        
        Twist::Binv => {
          let kauffman_a =
            &(&Poly::inverse_identity() * &self.kauffman_a) +
            &(&Poly::identity() * &self.kauffman_c);

          let kauffman_b =
            &(&Poly::inverse_identity() * &self.kauffman_b) +
            &(&Poly::identity() * &self.kauffman_e);
          
          let kauffman_c =
            &(&Poly::inverse_identity() * &self.kauffman_c) +
            &(&(&Poly::identity() * &self.kauffman_c) *
              &kauffman_unknot());

          let kauffman_d =
            &(&Poly::inverse_identity() * &self.kauffman_d) +
            &(&Poly::identity() * &self.kauffman_c);
          
          let kauffman_e =
            &(&Poly::inverse_identity() * &self.kauffman_e) +
            &(&(&Poly::identity() * &self.kauffman_e) *
              &kauffman_unknot());

          let writhe = self.writhe - 1;
          let jones = calc_jones(&kauffman_a, writhe);

          res.push(BraidData {
            braid,
            kauffman_a,
            kauffman_b,
            kauffman_c,
            kauffman_d,
            kauffman_e,
            writhe,
            jones,
          });
        },
      }
    }
    res.shrink_to_fit();
    res
  }
}

// Reduced BraidData (with only the braid and its Jones polynomial left).
struct BraidJones {
  braid: Braid,
  jones: Poly,
}

// Recursive function. Accepts the maximal canonical length of the braid,
// returns a pair of (bj, bd), where:
// * bj is a list of all BraidJones structures for all braids with canonical
//   length <= n.
// * bd is a list of all BraidData structures for all braids with canonical
//   length of exactly n.
fn calc_braid_jones_rec(n: u32) -> (Vec<BraidJones>, Vec<BraidData>) {
  if n == 0 {
    let bdata = BraidData::identity_braid();
    let bjones = BraidJones {
      braid: bdata.braid.clone(),
      jones: bdata.jones.clone(),
    };
    return (vec![bjones], vec![bdata]);
  }

  let (mut bjones, bdata) = calc_braid_jones_rec(n - 1);
  let mut new_bdata = Vec::with_capacity(bdata.len() * 3);
  for i in bdata.into_iter() {
    for d in i.descendants().into_iter() {
      bjones.push(BraidJones {
        braid: d.braid.clone(),
        jones: d.jones.clone(),
      });
      new_bdata.push(d);
    }
  }
  new_bdata.shrink_to_fit();

  (bjones, new_bdata)
}

// Wrapper around `calc_braid_jones_rec`. Only returns the BraidJones objects.
fn calc_braid_jones(n: u32) -> Vec<BraidJones> {
  let (res, _) = calc_braid_jones_rec(n);
  res
}

fn main() {
  let n = 14;  // Upper limit on canonical braid length.
  let bj = calc_braid_jones(n);
  let mut last_zero_index_change_braid_len = -1_i64;
  for i in bj.iter() {
    // println!("{:?}\t  {:?}", &i.braid, &i.jones);
    if i.jones.get_coef(0) != 0 {
      last_zero_index_change_braid_len = i.braid.canonical_len() as i64;
    }
  }
  println!("Total braids: {}", bj.len());
  println!("Last zero-index change braid len: {}",
           last_zero_index_change_braid_len);
  // Seems to be unbounded, grows with `n` :(
}
