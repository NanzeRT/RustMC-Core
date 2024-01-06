use num::Float;

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn inverse_lerp<T: Float>(value: T, start: T, end: T) -> T {
    (value - start) / (end - start)
}

pub fn lerp<T: Float>(
    value: T,
    start: T,
    end: T,
) -> T {
    start + value * (end - start)
}

pub fn map<T: Float>(
    value: T,
    old_start: T,
    old_end: T,
    new_start: T,
    new_end: T,
) -> T {
    lerp(inverse_lerp(value, old_start, old_end), new_start, new_end)
}

pub fn clamped_lerp<T: Float>(
    start: T,
    end: T,
    delta: T,
) -> T {
    if delta < T::zero() {
        start
    } else if delta > T::one() {
        end
    } else {
        lerp(delta, start, end)
    }
}

pub fn clamped_map<T: Float>(
    value: T,
    old_start: T,
    old_end: T,
    new_start: T,
    new_end: T,
) -> T {
    clamped_lerp(new_start, new_end, inverse_lerp(value, old_start, old_end))
}
