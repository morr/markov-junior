macro_rules! use_modules {
    ( $( $x:ident ),* ) => {
        $(
            pub mod $x;
            pub use crate::$x::*;
        )*
    };
}

// macro_rules! expose_submodules {
//     ( $( $x:ident ),* ) => {
//         $(
//             mod $x;
//             pub use self::$x::*;
//         )*
//     };
// }

use_modules!(
    pattern,
    rule,
    algo,
    utils,
    models
);
