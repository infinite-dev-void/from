#[inline]
pub fn array_contains<const L: usize, T>(arr: [T; L], e: T) -> bool
where
    T: PartialEq,
{
    for elem in arr {
        if elem == e {
            return true;
        };
    }

    false
}

#[inline]
pub fn array_not_contains<const L: usize, T>(arr: [T; L], e: T) -> bool
where
    T: PartialEq,
{
    for elem in arr {
        if elem == e {
            return false;
        };
    }

    true
}

#[inline]
pub fn num_of_f32_fracs(f: f32) -> usize {
    if !f.is_finite() {
        return 0;
    };

    match f.to_string().split_once('.') {
        Some((_, fracs)) => fracs.len(),

        None => 0,
    }
}

#[inline]
pub fn num_of_f64_fracs(f: f64) -> usize {
    if !f.is_finite() {
        return 0;
    };

    match f.to_string().split_once('.') {
        Some((_, fracs)) => fracs.len(),

        None => 0,
    }
}
