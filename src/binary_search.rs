use std::cmp;

fn find(nums: &[u32], n: u32) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }

    let mut l = 0;
    let mut r = nums.len();

    while l < r {
        let mid = (l + r) / 2;
        match nums[mid].cmp(&n) {
            cmp::Ordering::Equal => return Some(mid),
            cmp::Ordering::Less => l = mid + 1,
            cmp::Ordering::Greater => r = mid,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bs() {
        assert_eq!(find(&[1,2,3,4,5], 4), Some(3));
        assert_eq!(find(&[1,2,3,4,5], 5), Some(4));
    }
}
