/// Rescales a normalized value into a given range
///
/// Given a value in range `[0, 1]` the function will rescale it to `[min, max]`
#[inline]
pub fn rescale_normalized_value(val: f32, min: f32, max: f32) -> f32 {
    val * (max - min) + min
}

/// Collection of common numerical functions
///
/// Useful for gain correction in certain cases
pub mod numerical_functions {
    /// Cubic function
    ///
    /// `ax^2 + bx + c`
    #[inline]
    pub fn cubic(x: f32, a: f32, b: f32, c: f32) -> f32 {
        x.mul_add(a, b).mul_add(x, c)
    }

    /// Quadratic function
    ///
    /// `ax^3 + bx^2 + cx + d`
    #[inline]
    pub fn quadratic(x: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
        // a * x.powi(3) + b * x.powi(2) + c * x + d
        x.mul_add(a, b).mul_add(x, c).mul_add(x, d)
    }

    /// Quartic function
    ///
    /// `ax^4 + bx^3 + cx^2 + dx + e`
    #[inline]
    pub fn quartic(x: f32, a: f32, b: f32, c: f32, d: f32, e: f32) -> f32 {
        x.mul_add(a, b).mul_add(x, c).mul_add(x, d).mul_add(x, e)
    }
}
/// A collection of functions that change the distribution of param values
///
/// Those functions, when given value in range `[0, 1]`, will produce a number between `[0, 1]`, unless stated otherwise, with curves
/// depending on the function and passed parameters
///
/// Some suffixes and their meanings
///
/// | Suffix   | Note                                                               |
/// |----------|--------------------------------------------------------------------|
/// | reversed | function is mirrored by `x = 0.5`                                  |
/// | unscaled | function will produce a range `[0, a]`with `a` depending on inputs |
/// | default  | function with some prebuild constants                              |
pub mod rescalers {
    /// Sqrt
    #[inline]
    pub fn sqrt(x: f32) -> f32 {
        x.sqrt()
    }

    /// Reversed sqrt
    #[inline]
    pub fn sqrt_reversed(x: f32) -> f32 {
        1_f32 - x.sqrt()
    }

    /// Arctan based function
    ///
    /// `f(x) = (a + arctan((x - 0.2) * pi)) / b` with `a` scaled so `f(0) = 0` and `b` scaled so the value range is `[0, 1]`
    ///
    /// Produces sharply inclined graph that tapers off around `0.5`
    #[inline]
    pub fn arctan(x: f32) -> f32 {
        (((x - 0.2_f32) * std::f32::consts::PI).atan() + 0.56098) * 0.57042137
    }

    /// Arctan based function, mirrored around `x = 0.5`
    #[inline]
    pub fn arctan_reversed(x: f32) -> f32 {
        1_f32 - arctan(x)
    }

    /// Arctan based function, not scaled, the value range is `[0, 1.753]`
    ///
    /// Slightly cheaper then [`arctan`]
    #[inline]
    pub fn arctan_unscaled(x: f32) -> f32 {
        ((x - 0.2_f32) * std::f32::consts::PI).atan() + 0.56098
    }

    /// Arctan based function, not scaled, the value range is `[0, 1.753]`, it is also mirrored around `x = 0.8765`
    #[inline]
    pub fn arctan_reversed_unscaled(x: f32) -> f32 {
        1.19211 - ((x - 0.2_f32) * std::f32::consts::PI).atan()
    }

    /// Ln based function
    ///
    /// `f(x) = ln(ax + 1) / ln(a + 1)` with a being a user defined parameter, that sets the curve of the function. Smaller a
    /// results in the function rising quicker in the start
    #[inline]
    pub fn ln(x: f32, a: f32) -> f32 {
        (x / (a * std::f32::consts::E)).ln_1p() / (1_f32 / (a * std::f32::consts::E)).ln_1p()
    }

    /// Ln based function, mirrored around `x = 0.5`
    #[inline]
    pub fn ln_reversed(x: f32, a: f32) -> f32 {
        1_f32 - ln(x, a)
    }

    /// Ln based function, unscaled, the value range is `[0, ln(1 / (a * e) + 1)]`
    ///
    /// Slightly cheaper then [`ln`]
    #[inline]
    pub fn ln_unscaled(x: f32, a: f32) -> f32 {
        (x / (a * std::f32::consts::E)).ln_1p()
    }

    /// Ln based function, the value range is `[0, ln(1 / (a * e) + 1)]`, it is mirrored by `x = ln(1 / (a * e) + 1) / 2`
    #[inline]
    pub fn reversed_unscaled(x: f32, a: f32) -> f32 {
        (1_f32 / (a * std::f32::consts::E)).ln_1p() - (x / (a * std::f32::consts::E)).ln_1p()
    }

    /// Ln based function, with `a = 0.01`
    ///
    /// Slightly cheaper then [`ln`]
    #[inline]
    pub fn ln_default(x: f32) -> f32 {
        (x / (0.01_f32 * std::f32::consts::E)).ln_1p() * 0.275331145
    }

    /// Ln based function, with `a = 0.01`, mirrored by `x = 0.5`
    ///
    /// Slightly cheaper then [`ln`]
    #[inline]
    pub fn ln_reversed_default(x: f32) -> f32 {
        1_f32 - (x / (0.01_f32 * std::f32::consts::E)).ln_1p() * 0.275331145
    }

    /// Ln based function, with `a = 0.01`, the value range is `[0, 3.63199]`
    ///
    /// Slightly cheaper then [`ln_default`]
    #[inline]
    pub fn ln_unscaled_default(x: f32) -> f32 {
        (x / (0.01_f32 * std::f32::consts::E)).ln_1p()
    }

    /// Ln based function, with `a = 0.01`, the value range is `[0, 3.63199]`, it is mirrored by `x = 1.815995`
    ///
    /// Slightly cheaper then [`ln`]
    #[inline]
    pub fn ln_reversed_unscaled_default(x: f32) -> f32 {
        3.63199_f32 - (x / (0.01_f32 * std::f32::consts::E)).ln_1p()
    }
}
