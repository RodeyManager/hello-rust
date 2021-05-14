// //! # My Crate
// //!
// //! `my-crate` is a collection of utilities to make performing certain
// //! calculations more convenient.

// /// Adds one to the number given.
// /// # Examples
// /// ```rust
// /// let arg = 5;
// /// let answer = add_one(arg);
// ///
// /// assert_eq!(answer, 6);
// /// ```
// pub fn add_one(x: i32) -> i32 {
//     x + 1
// }

//! Art
//!
//! A library for modeling artistic concepts.

// 对于有很多嵌套模块的情况，使用 pub use 将类型重导出到顶级结构对于使用 crate 的人来说将会是大为不同的体验。

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {


    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Green,
        Blue,
    }


    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
        Cyan,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amount to create
    /// a secondary color
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        SecondaryColor::Purple
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
