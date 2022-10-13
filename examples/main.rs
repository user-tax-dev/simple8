use simple8::{pack, unpack};

pub fn main() {
  let max = (1u64 << 60) - 1;
  let values = [max, 2, 76, 3, 7, 2];
  dbg!(max);
  let mut r = 0;
  let res = pack(&values, &mut r).unwrap();
  dbg!((res, r));
  dbg!(unpack(vec![r].into_iter()));
}
