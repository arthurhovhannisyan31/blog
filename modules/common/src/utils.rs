use crate::constants::QUERY_LIMIT_STEP;
use std::cmp::min;

type Limit = u64;
type Offset = u64;

// TODO Add tests
pub fn get_next_pagination(total: u64, limit: u64) -> (Offset, Limit) {
  let next_offset = min(total, limit);
  let next_limit = min(next_offset + QUERY_LIMIT_STEP, total);

  (next_offset, next_limit)
}
