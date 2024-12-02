use std::cmp::Ordering;

pub mod parser;

#[must_use]
pub fn is_safe(v: &[u64], dampening: usize) -> bool {
    is_safe_rec(0, 1, v, dampening, Ordering::Equal)
}

fn is_safe_rec(
    current_idx: usize,
    next_idx: usize,
    v: &[u64],
    dampening: usize,
    old_ord: Ordering,
) -> bool {
    let (Some(&current), Some(&next)) = (v.get(current_idx), v.get(next_idx)) else {
        return true;
    };

    let new_ord = current.cmp(&next);
    let diff = current.abs_diff(next);

    if diff == 0 || diff > 3 {
        dampener(current_idx, next_idx, v, dampening, old_ord)
    } else if matches!(old_ord, Ordering::Equal) || new_ord == old_ord {
        is_safe_rec(next_idx, next_idx + 1, v, dampening, new_ord)
    } else {
        dampener(current_idx, next_idx, v, dampening, old_ord)
    }
}

fn dampener(
    current_idx: usize,
    next_idx: usize,
    v: &[u64],
    dampening: usize,
    ordering: Ordering,
) -> bool {
    if dampening > 0 {
        if current_idx == 0 {
            // Ignore first -> Ignore second
            is_safe_rec(1, 2, v, dampening - 1, Ordering::Equal)
                || is_safe_rec(0, 2, v, dampening - 1, Ordering::Equal)
        } else if current_idx == 1 {
            // Ignore prev -> Ignore current -> Ignore next
            is_safe_rec(1, 2, v, dampening - 1, Ordering::Equal)
                || is_safe_rec(0, 2, v, dampening - 1, Ordering::Equal)
                || is_safe_rec(1, 3, v, dampening - 1, ordering)
        } else {
            // Ignore current -> Ignore next
            is_safe_rec(current_idx - 1, next_idx, v, dampening - 1, ordering)
                || is_safe_rec(current_idx, next_idx + 1, v, dampening - 1, ordering)
        }
    } else {
        false
    }
}
