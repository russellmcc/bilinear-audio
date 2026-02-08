use std::ops::RangeInclusive;

#[inline]
#[must_use]
pub fn rescale_points(input: f32, from_low: f32, from_high: f32, to_low: f32, to_high: f32) -> f32 {
    let input = (input - from_low) / (from_high - from_low);
    input * (to_high - to_low) + to_low
}

#[inline]
#[must_use]
pub fn rescale(input: f32, from: RangeInclusive<f32>, to: RangeInclusive<f32>) -> f32 {
    rescale_points(input, *from.start(), *from.end(), *to.start(), *to.end())
}

#[inline]
#[must_use]
pub fn rescale_inverted(input: f32, from: RangeInclusive<f32>, to: RangeInclusive<f32>) -> f32 {
    rescale_points(input, *from.start(), *from.end(), *to.end(), *to.start())
}

#[inline]
#[must_use]
pub fn rescale_clamped(input: f32, from: RangeInclusive<f32>, to: RangeInclusive<f32>) -> f32 {
    let to_low = *to.start();
    let to_high = *to.end();
    rescale_points(input, *from.start(), *from.end(), to_low, to_high).clamp(to_low, to_high)
}

#[inline]
#[must_use]
pub fn rescale_inverted_clamped(
    input: f32,
    from: RangeInclusive<f32>,
    to: RangeInclusive<f32>,
) -> f32 {
    rescale_inverted(input, from.clone(), to.clone()).clamp(*to.start(), *to.end())
}

#[inline]
#[must_use]
pub fn lerp(value_at_zero: f32, value_at_one: f32, t: f32) -> f32 {
    value_at_zero * (1.0 - t) + value_at_one * t
}

#[inline]
#[must_use]
pub fn lerp_clamped(value_at_zero: f32, value_at_one: f32, t: f32) -> f32 {
    lerp(value_at_zero, value_at_one, t.clamp(0.0, 1.0))
}

#[inline]
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn exp2_approx(x: f32) -> f32 {
    // See https://specbranch.com/posts/fast-exp/ for how this works!
    const BASE: f32 = 8_388_608.0; // 2^23
    const K: i32 = 366_393; // Tuning constant - see article above. This shifts curve to minimize maximum error.
    let x_base = x * BASE;
    let x_base_int = x_base as i32;
    let result_int = x_base_int + (127 << 23) - K;
    f32::from_bits(result_int as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_exp2_approx_basic() {
        // This simple test gives us miri coverage of exp2_approx. Full testing is a proptest below.
        assert!((exp2_approx(0.0) - 1.0).abs() < 0.03);
    }

    proptest! {
        #[test]
        #[cfg_attr(miri, ignore)]
        fn test_exp2_approx(x in -60.0f32..=60.0f32) {
            let approx = exp2_approx(x);
            let exact = 2f32.powf(x);
            let relative_error = (approx - exact).abs() / exact.abs();
            prop_assert!(
                relative_error < 0.03,
                "Expected {} to be approximately equal to {} within 3% error (got {:.4}%)",
                approx,
                exact,
                relative_error * 100.0,
            );
        }
    }
}
