use std::cmp::min;

use crate::constants::QUERY_LIMIT_STEP;

type Limit = u64;
type Offset = u64;

pub fn get_next_pagination(total: u64, limit: u64) -> (Offset, Limit) {
  let next_offset = min(total, limit);
  let next_limit = min(next_offset + QUERY_LIMIT_STEP, total);

  (next_offset, next_limit)
}

#[cfg(test)]
mod test {
  use crate::utils::get_next_pagination;

  #[test]
  fn test_pagination() {
    let total: u64 = 25;

    let test_cases: Vec<(u64, u64, u64)> =
      vec![(10, 10, 20), (20, 20, 25), (25, 25, 25), (30, 25, 25)];

    for (limit, assert_next_offset, assert_next_limit) in test_cases {
      let (next_offset, next_limit) = get_next_pagination(total, limit);

      assert_eq!(
        (next_offset, next_limit),
        (assert_next_offset, assert_next_limit)
      );
    }
  }
}
