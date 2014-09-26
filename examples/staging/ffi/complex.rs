//extern crate std;
use std::fmt;

#[repr(C)]
pub struct Complex32 {
    pub re: f32,
    pub im: f32
}

/* string conversions */
impl fmt::Show for Complex32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0f32 {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
