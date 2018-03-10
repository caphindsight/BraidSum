use super::Braid;

fn get_braids_of_length(n: u32) -> Vec<Braid> {
  if n == 0 {
    return vec![Braid::identity()];
  }
  let prev = get_braids_of_length(n - 1);
  let mut res = Vec::with_capacity(prev.len() * 3);
  for i in &prev {
    res.append(&mut i.descendants());
  }
  res.shrink_to_fit();
  res
}

#[test]
fn braids_of_length_1() {
  let mut braids = get_braids_of_length(1);
  assert_eq!(braids.len(), 4);
  braids.reverse();
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [A] }");
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [B] }");
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [Ainv] }");
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [Binv] }");
}

#[test]
fn braids_of_length_2() {
  let mut braids = get_braids_of_length(2);
  assert_eq!(braids.len(), 12);
  braids.reverse();
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [A, A] }");
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [A, B] }");
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [A, Binv] }");
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [B, A] }");
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [B, B] }");
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [B, Ainv] }");
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [Ainv, B] }");
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [Ainv, Ainv] }");
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [Ainv, Binv] }");
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [Binv, A] }");
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [Binv, Ainv] }");
  assert_eq!(format!("{:?}", braids.pop().unwrap()),
             "Braid { twists: [Binv, Binv] }");
}

#[test]
fn braids_of_length_3() {
  let braids = get_braids_of_length(3);
  assert_eq!(braids.len(), 34);
}

#[test]
fn braids_of_length_4() {
  let braids = get_braids_of_length(4);
  assert_eq!(braids.len(), 92);
}
