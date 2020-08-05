use std::fmt;

use enumflags2::BitFlags;

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum RenderingRequirements {
    Texture = 0b0000_0001
}

impl fmt::Display for RenderingRequirements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "{}", 
            BitFlags::from(*self)
                     .iter()
                     .map(|requirement| {
                         match requirement {
                             RenderingRequirements::Texture => "Texture"
                         }
                     })
                     .collect::<Vec<&str>>()
                     .join(", ")
        )
    }
}

