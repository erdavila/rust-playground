use std::ops::Range;

const VIOLET_WAVELENGTH: f64 = 380.0;
const BLUE_WAVELENGTH: f64 = 440.0;
const CYAN_WAVELENGTH: f64 = 490.0;
const GREEN_WAVELENGTH: f64 = 510.0;
const YELLOW_WAVELENGTH: f64 = 580.0;
const RED_MIN_WAVELENGTH: f64 = 645.0;
const RED_MAX_WAVELENGTH: f64 = 780.0;

const BLACK: Color = Color { r: 0, g: 0, b: 0 };

pub const MIN_WAVELENGTH: f64 = VIOLET_WAVELENGTH;
pub const MAX_WAVELENGTH: f64 = RED_MAX_WAVELENGTH;

#[derive(Debug, Clone, Copy)]
pub struct UnfadedRange {
    pub unfaded_lower_bound: f64,
    pub unfaded_upper_bound: f64,
}

pub const DEFAULT_UNFADED_RANGE: UnfadedRange = UnfadedRange {
    unfaded_lower_bound: 420.0,
    unfaded_upper_bound: 700.0,
};

pub const DEFAULT_GAMMA: f64 = 0.8;

#[derive(Debug, Clone)]
pub struct Converter {
    pub fading: Option<UnfadedRange>,
    pub gamma: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Converter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            fading: Some(DEFAULT_UNFADED_RANGE),
            gamma: Some(DEFAULT_GAMMA),
        }
    }

    #[must_use]
    pub fn with_fading(mut self, fading: Option<UnfadedRange>) -> Self {
        self.fading = fading;
        self
    }

    #[must_use]
    pub fn with_gamma(mut self, gamma: Option<f64>) -> Self {
        self.gamma = gamma;
        self
    }

    #[must_use]
    pub fn wavelength_to_rgb(&self, wavelength: f64) -> Color {
        // https://gemini.google.com/app/8b49a5e4e4d67a2e

        let mut red: f64;
        let mut green: f64;
        let mut blue: f64;

        if wavelength < VIOLET_WAVELENGTH {
            return BLACK;
        } else if wavelength < BLUE_WAVELENGTH {
            red = remap(wavelength, VIOLET_WAVELENGTH..BLUE_WAVELENGTH, 1.0..0.0);
            green = 0.0;
            blue = 1.0;
        } else if wavelength < CYAN_WAVELENGTH {
            red = 0.0;
            green = remap(wavelength, BLUE_WAVELENGTH..CYAN_WAVELENGTH, 0.0..1.0);
            blue = 1.0;
        } else if wavelength < GREEN_WAVELENGTH {
            red = 0.0;
            green = 1.0;
            blue = remap(wavelength, CYAN_WAVELENGTH..GREEN_WAVELENGTH, 1.0..0.0);
        } else if wavelength < YELLOW_WAVELENGTH {
            red = remap(wavelength, GREEN_WAVELENGTH..YELLOW_WAVELENGTH, 0.0..1.0);
            green = 1.0;
            blue = 0.0;
        } else if wavelength < RED_MIN_WAVELENGTH {
            red = 1.0;
            green = remap(wavelength, YELLOW_WAVELENGTH..RED_MIN_WAVELENGTH, 1.0..0.0);
            blue = 0.0;
        } else if wavelength <= RED_MAX_WAVELENGTH {
            red = 1.0;
            green = 0.0;
            blue = 0.0;
        } else {
            return BLACK;
        }

        if let Some(fading) = self.fading {
            let factor: f64 = if wavelength < fading.unfaded_lower_bound {
                remap(
                    wavelength,
                    MIN_WAVELENGTH..fading.unfaded_lower_bound,
                    0.3..1.0,
                )
            } else if wavelength > fading.unfaded_upper_bound {
                remap(
                    wavelength,
                    fading.unfaded_upper_bound..MAX_WAVELENGTH,
                    1.0..0.3,
                )
            } else {
                1.0
            };

            red *= factor;
            green *= factor;
            blue *= factor;
        }

        if let Some(gamma) = self.gamma {
            red = red.powf(gamma);
            green = green.powf(gamma);
            blue = blue.powf(gamma);
        }

        let r = adjust(red);
        let g = adjust(green);
        let b = adjust(blue);

        Color { r, g, b }
    }
}

impl Default for Converter {
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
fn remap(value: f64, from: Range<f64>, to: Range<f64>) -> f64 {
    to.start + (to.end - to.start) * (value - from.start) / (from.end - from.start)
}

#[inline]
fn adjust(value: f64) -> u8 {
    let scaled = 255.0 * value;
    let rounded = scaled.round();

    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let cast = rounded as u8;

    cast.clamp(0, 255)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wavelength_to_rgb() {
        let raw_converter = Converter::new().with_fading(None).with_gamma(None);

        assert_eq!(
            raw_converter.wavelength_to_rgb(VIOLET_WAVELENGTH),
            Color::new(255, 0, 255)
        );
        assert_eq!(
            raw_converter.wavelength_to_rgb(BLUE_WAVELENGTH),
            Color::new(0, 0, 255)
        );
        assert_eq!(
            raw_converter.wavelength_to_rgb(CYAN_WAVELENGTH),
            Color::new(0, 255, 255)
        );
        assert_eq!(
            raw_converter.wavelength_to_rgb(GREEN_WAVELENGTH),
            Color::new(0, 255, 0)
        );
        assert_eq!(
            raw_converter.wavelength_to_rgb(YELLOW_WAVELENGTH),
            Color::new(255, 255, 0)
        );
        assert_eq!(
            raw_converter.wavelength_to_rgb(RED_MIN_WAVELENGTH),
            Color::new(255, 0, 0)
        );
        assert_eq!(
            raw_converter.wavelength_to_rgb(RED_MAX_WAVELENGTH),
            Color::new(255, 0, 0)
        );

        // Limits of the visible range
        assert_eq!(
            raw_converter.wavelength_to_rgb(MIN_WAVELENGTH.next_down()),
            BLACK
        );
        assert_eq!(
            raw_converter.wavelength_to_rgb(MIN_WAVELENGTH),
            Color::new(255, 0, 255)
        );
        assert_eq!(
            raw_converter.wavelength_to_rgb(MAX_WAVELENGTH),
            Color::new(255, 0, 0)
        );
        assert_eq!(
            raw_converter.wavelength_to_rgb(MAX_WAVELENGTH.next_up()),
            BLACK
        );
    }

    #[test]
    fn test_map() {
        assert_eq!(remap(0.0, 0.0..1.0, 0.0..1.0), 0.0);
        assert_eq!(remap(0.5, 0.0..1.0, 0.0..1.0), 0.5);
        assert_eq!(remap(1.0, 0.0..1.0, 0.0..1.0), 1.0);

        assert_eq!(remap(0.0, 0.0..10.0, 0.0..100.0), 0.0);
        assert_eq!(remap(5.0, 0.0..10.0, 0.0..100.0), 50.0);
        assert_eq!(remap(10.0, 0.0..10.0, 0.0..100.0), 100.0);

        assert_eq!(remap(10.0, 10.0..20.0, 0.0..100.0), 0.0);
        assert_eq!(remap(15.0, 10.0..20.0, 0.0..100.0), 50.0);
        assert_eq!(remap(20.0, 10.0..20.0, 0.0..100.0), 100.0);
    }
}
