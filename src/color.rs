use std::fmt;
pub type Color = crate::Vec3;

impl fmt::Display for Color {
    // Write the translated [0,255] value of each color component.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.999 * self.x()) as u8,
            (255.999 * self.y()) as u8,
            (255.999 * self.z()) as u8,
        )
    }
}
