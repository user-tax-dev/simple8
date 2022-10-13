use simple8::{pack, unpack};

pub fn main() {
  let values = [2, 76, 3, (u64::max_value() >> 4) - 1, 7, 2];
  let mut r = 0;
  let res = pack(&values, &mut r).unwrap();
  dbg!((res, r));
  dbg!(unpack(vec![r].into_iter()));
}
