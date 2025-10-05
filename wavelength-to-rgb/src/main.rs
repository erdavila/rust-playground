use wavelength_to_rgb::{Color, Converter};

const STEP: f64 = 10.0;

const PADDING: f64 = 10.0;
const MIN_WAVELENGTH: f64 = wavelength_to_rgb::MIN_WAVELENGTH - PADDING;
const MAX_WAVELENGTH: f64 = wavelength_to_rgb::MAX_WAVELENGTH + PADDING;

fn main() {
    let converter = Converter::new();
    let raw_converter = Converter::new().with_fading(None).with_gamma(None);

    let mut wavelength = MIN_WAVELENGTH;
    loop {
        print_color(wavelength, converter.wavelength_to_rgb(wavelength));
        print!("  ");
        print_color(wavelength, raw_converter.wavelength_to_rgb(wavelength));
        println!();

        wavelength += STEP;
        if wavelength > MAX_WAVELENGTH {
            break;
        }
    }
}

fn print_color(wavelength: f64, color: Color) {
    print!(
        "\x1b[1;48;2;{r};{g};{b}m {wavelength:.1} nm = #{r:02x}{g:02x}{b:02x} \x1b[0m",
        r = color.r,
        g = color.g,
        b = color.b
    );
}
