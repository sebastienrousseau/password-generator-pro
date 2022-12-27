//! # QRCode generation
//!
//! QRCode provides a simple way to generate a new QRCode.
//!

use qrcode::{render::svg, QrCode};
use std::fs;

/// QRCode Utility
///
/// By default, a new QRCode is generated.
#[non_exhaustive]
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QRCode;


impl QRCode {
    /// Generate a new QRCode from the text.
    ///
    /// # Arguments
    ///
    /// * `data` - The text to generate the QRCode from.
    ///
    /// # Example
    ///
    /// ```
    /// use util::qrcode::QRCode;
    ///
    /// let qrcode = QRCode::new("Hello World!");
    /// ```
    pub fn qrcode(content: &str) -> String {
        let code = QrCode::new(content).unwrap();
        let svg_data = code
            .render()
            .min_dimensions(300, 300)
            .max_dimensions(300, 300)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();
        return svg_data.to_string();




        // Ok(())

        // let code = QrCode::new(content).unwrap();
        // let svg_data = code
        //     .render()
        //     .min_dimensions(300, 300)
        //     .max_dimensions(300, 300)
        //     .dark_color(svg::Color("#000000"))
        //     .light_color(svg::Color("#ffffff"))
        //     .build();



        // return svg_data.to_string();
        // println!("{}", image);
            // println!("{}", image);
    }


    pub fn export(content: &str, name: &str) {
        let code = QrCode::new(content).unwrap();
        let svg_data = code
            .render()
            .min_dimensions(300, 300)
            .max_dimensions(300, 300)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();

        fs::write(name, svg_data).expect("Unable to write file");

        println!("QRCode generated!");


    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        let qrcode = QRCode::default();
        assert_eq!(format!("{:?}", qrcode), "QRCode");
    }

    #[test]
    fn test_clone() {
        let qrcode = QRCode::qrcode("Hello World!");
        let qrcode_clone = qrcode.clone();
        assert_eq!(qrcode, qrcode_clone);
    }

    #[test]
    fn test_default() {
        let qrcode = QRCode::default();
        assert_eq!(qrcode, QRCode);
    }

    #[test]
    fn test_partial_eq() {
        let qrcode = QRCode::qrcode("Hello World!");
        let qrcode_clone = qrcode.clone();
        assert_eq!(qrcode, qrcode_clone);
    }
}
