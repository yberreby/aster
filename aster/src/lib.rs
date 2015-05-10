#[cfg(feature = "aster_syntax")]
pub mod syntax {
    extern crate aster_syntax;

    pub use self::aster_syntax::*;
}

#[cfg(feature = "aster_syntex")]
pub mod syntex {
    extern crate aster_syntex;

    pub use self::aster_syntex::*;
}
