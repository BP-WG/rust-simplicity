// SPDX-License-Identifier: CC0-1.0

use crate::impl_midstate_wrapper;
use hashes::sha256::Midstate;

/// Type Merkle root
///
/// A Merkle root that commits to a type's primitive (unit, sum, product)
/// and recursively its sub-types.
///
/// Uniquely identifies a type.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tmr(pub(crate) Midstate);

impl_midstate_wrapper!(Tmr);

impl Tmr {
    /// The IV for a TMR of a unit type
    #[rustfmt::skip]
    pub const UNIT_IV: Tmr = Tmr(Midstate([
        0x12, 0xb4, 0xc4, 0xa9, 0xa4, 0xb0, 0xed, 0xf6,
        0x5a, 0x44, 0xf3, 0x0e, 0xa7, 0x62, 0x57, 0x8f,
        0xdd, 0x59, 0xf1, 0x05, 0xf0, 0xe4, 0xd8, 0xf3,
        0x88, 0xcb, 0x9b, 0x6b, 0xd2, 0xc1, 0x3a, 0xdf,
    ]));

    /// The IV for a TMR of a sum type
    #[rustfmt::skip]
    pub const SUM_IV: Tmr = Tmr(Midstate([
        0x05, 0xcc, 0x9d, 0xdd, 0x0e, 0x50, 0xb0, 0xec,
        0x99, 0xfd, 0x5f, 0xad, 0xdc, 0x4d, 0x95, 0x06,
        0xcd, 0x3e, 0x7b, 0xb8, 0xed, 0xeb, 0x40, 0xca,
        0x98, 0x33, 0x86, 0x6e, 0x3a, 0x0a, 0xbc, 0x33,
    ]));

    /// The IV for a TMR of a product type
    #[rustfmt::skip]
    pub const PRODUCT_IV: Tmr = Tmr(Midstate([
        0xc1, 0x71, 0x96, 0x87, 0x4b, 0x51, 0x21, 0xfd,
        0x5d, 0xbe, 0x2f, 0xef, 0x5b, 0xa0, 0xd2, 0xed,
        0xce, 0x23, 0x92, 0xe3, 0x55, 0x15, 0xa2, 0xf2,
        0x06, 0xb2, 0x2b, 0xbe, 0x08, 0x8b, 0xb1, 0xaf,
    ]));

    /// The TMRs of the types TWO^(2^n) for small values of n
    #[rustfmt::skip]
    pub const POWERS_OF_TWO: [Tmr; 32] = [
        Tmr(Midstate([
            0x07, 0x18, 0x2b, 0xe8, 0x37, 0xcf, 0xdd, 0x22,
            0xce, 0x79, 0x44, 0x2e, 0x00, 0xb6, 0xaf, 0x5a,
            0xd9, 0xd3, 0xa3, 0xd8, 0x7d, 0xf2, 0x81, 0x94,
            0x43, 0xbe, 0x3f, 0xc5, 0xfe, 0x53, 0xb9, 0x88,
        ])),
        Tmr(Midstate([
            0x56, 0xe6, 0x13, 0x52, 0x92, 0xca, 0xed, 0xde,
            0xf1, 0x0a, 0x68, 0x30, 0x40, 0x8e, 0x14, 0xd8,
            0x86, 0x47, 0x4d, 0x75, 0xd3, 0xa5, 0x66, 0xf0,
            0x55, 0x68, 0xae, 0x8b, 0x98, 0x58, 0x4f, 0x29,
        ])),
        Tmr(Midstate([
            0xd5, 0x74, 0x8f, 0xd3, 0xc0, 0x61, 0xe3, 0xf1,
            0x33, 0xf5, 0xe9, 0xa3, 0xa2, 0xe4, 0xcb, 0xcb,
            0x7c, 0x58, 0xcf, 0x62, 0x15, 0xed, 0x2d, 0x9f,
            0x5e, 0x81, 0xa5, 0x3d, 0xd4, 0x0a, 0xd3, 0x09,
        ])),
        Tmr(Midstate([
            0xd7, 0x2f, 0x47, 0x29, 0x99, 0x92, 0xc4, 0x27,
            0x0a, 0xa6, 0x0f, 0xdc, 0x4c, 0x47, 0x03, 0xd1,
            0x80, 0x55, 0xf5, 0x78, 0xd0, 0x3e, 0x7a, 0x24,
            0x48, 0xd9, 0x64, 0x06, 0xe2, 0xbb, 0x8d, 0x6f,
        ])),
        Tmr(Midstate([
            0xfd, 0x2c, 0x7e, 0x26, 0xeb, 0xf1, 0x31, 0xb4,
            0x16, 0x0a, 0x5d, 0x95, 0xd4, 0x07, 0x0b, 0x7d,
            0xeb, 0x67, 0xfc, 0xcb, 0xdf, 0x1f, 0x96, 0xe1,
            0x7c, 0x87, 0xf1, 0x21, 0xc7, 0x62, 0xd1, 0x6f,
        ])),
        Tmr(Midstate([
            0xdc, 0x18, 0x65, 0x3a, 0xff, 0xf1, 0xba, 0x26,
            0xbf, 0x0e, 0xfd, 0xc3, 0x26, 0xc8, 0x71, 0x38,
            0xe1, 0x68, 0x99, 0x41, 0xc3, 0xe9, 0x0f, 0x94,
            0xdc, 0xcd, 0xf1, 0xc2, 0xa4, 0xc3, 0x81, 0x29,
        ])),
        Tmr(Midstate([
            0xdf, 0xf3, 0x89, 0x07, 0x1e, 0x1a, 0xad, 0xe0,
            0xf4, 0x84, 0x5c, 0x2f, 0x32, 0x23, 0xf6, 0xc4,
            0x9d, 0x15, 0xa1, 0x62, 0x65, 0x09, 0x80, 0xc9,
            0x7f, 0x3c, 0x11, 0x48, 0x8e, 0x3b, 0x62, 0xb6,
        ])),
        Tmr(Midstate([
            0x35, 0x5c, 0x21, 0x15, 0xb1, 0x43, 0xeb, 0x6a,
            0xe5, 0x44, 0x86, 0x6f, 0x79, 0xd0, 0x26, 0xf2,
            0xcd, 0x9b, 0xbc, 0x70, 0xbe, 0x92, 0xdd, 0x5d,
            0x16, 0x0e, 0x03, 0x62, 0x34, 0x9a, 0xdb, 0x2b,
        ])),
        Tmr(Midstate([
            0x10, 0xf1, 0xfc, 0x6f, 0x17, 0xc4, 0x0d, 0x52,
            0xa8, 0x01, 0xce, 0x38, 0xd7, 0x91, 0xdc, 0xc5,
            0xba, 0x7a, 0x58, 0x82, 0xbd, 0x28, 0x65, 0x95,
            0xc5, 0x88, 0xaf, 0x6b, 0x68, 0x53, 0xfa, 0xcb,
        ])),
        Tmr(Midstate([
            0x46, 0x14, 0xfa, 0x1a, 0x08, 0x82, 0x02, 0x45,
            0x04, 0x2f, 0xa5, 0x4b, 0x7e, 0xaf, 0x83, 0xec,
            0xaf, 0x23, 0x5f, 0x35, 0x3d, 0x5a, 0x6e, 0x2f,
            0x0b, 0x2b, 0xc0, 0x58, 0xae, 0x56, 0xad, 0xa5,
        ])),
        Tmr(Midstate([
            0x59, 0x0e, 0xd9, 0x0e, 0x09, 0x6e, 0xfc, 0x99,
            0x6a, 0x26, 0x40, 0x56, 0xb7, 0x89, 0xf8, 0xbc,
            0x28, 0x0b, 0x88, 0xf8, 0x20, 0x35, 0x42, 0x78,
            0x71, 0x8e, 0xfa, 0xd0, 0xb9, 0x0f, 0x34, 0xd2,
        ])),
        Tmr(Midstate([
            0xe4, 0x08, 0x26, 0xb1, 0x17, 0x3e, 0x0e, 0x25,
            0x11, 0x8f, 0x0b, 0x3a, 0x74, 0x55, 0x35, 0x80,
            0x67, 0xc6, 0x24, 0x1c, 0x47, 0xdb, 0xeb, 0x87,
            0xd8, 0xf8, 0xf3, 0xf7, 0xba, 0x30, 0xb6, 0xd0,
        ])),
        Tmr(Midstate([
            0x65, 0x4e, 0xed, 0xfe, 0x51, 0x48, 0x7d, 0xe7,
            0x5c, 0x70, 0xec, 0xd6, 0x09, 0xa9, 0x4f, 0x47,
            0x92, 0xee, 0x4e, 0x8c, 0x1f, 0x19, 0xcf, 0x86,
            0x54, 0x7f, 0x67, 0xc8, 0xdf, 0x29, 0x47, 0x4c,
        ])),
        Tmr(Midstate([
            0xcc, 0x70, 0x5c, 0x34, 0x1d, 0xc4, 0xda, 0x07,
            0xfd, 0xe2, 0x8d, 0x3c, 0x47, 0xa3, 0x29, 0x0f,
            0x69, 0x2d, 0xee, 0x07, 0x0f, 0x1e, 0xef, 0x2d,
            0x2e, 0x8c, 0xd8, 0x2f, 0xd2, 0x0e, 0x31, 0x0c,
        ])),
        Tmr(Midstate([
            0xea, 0x08, 0xc8, 0x53, 0xe8, 0xe8, 0x98, 0x28,
            0xc5, 0xbe, 0xbf, 0xce, 0xb8, 0xe2, 0x4e, 0xa0,
            0x12, 0xfb, 0xd6, 0x29, 0x3c, 0xd9, 0x41, 0xf3,
            0xc0, 0xdb, 0x24, 0x4e, 0x50, 0x1a, 0x3a, 0x86,
        ])),
        Tmr(Midstate([
            0x71, 0xcb, 0xe7, 0x99, 0x84, 0x20, 0x3e, 0x59,
            0x41, 0x80, 0xee, 0x67, 0x55, 0xb8, 0x4b, 0x44,
            0xab, 0x61, 0x30, 0x12, 0xfa, 0xd3, 0x73, 0x95,
            0x46, 0xda, 0x40, 0x8b, 0xa0, 0x5d, 0x4d, 0x9a,
        ])),
        Tmr(Midstate([
            0x3e, 0x82, 0x89, 0x31, 0xf3, 0x6f, 0x6a, 0xe6,
            0x6a, 0xb1, 0x3e, 0x31, 0x77, 0x03, 0x08, 0x0d,
            0x89, 0xac, 0xb5, 0x30, 0xed, 0xf7, 0x46, 0x14,
            0x5f, 0x32, 0x04, 0x5c, 0x3a, 0xe1, 0x79, 0xa3,
        ])),
        Tmr(Midstate([
            0x38, 0x93, 0x0a, 0x55, 0x77, 0x59, 0xdb, 0x60,
            0x40, 0x29, 0x2b, 0x95, 0x2a, 0x04, 0x5d, 0x65,
            0xbd, 0x20, 0x91, 0xea, 0xe2, 0xb9, 0xf5, 0x25,
            0xaa, 0x84, 0xc7, 0xb8, 0xa0, 0x5e, 0xd7, 0xd7,
        ])),
        Tmr(Midstate([
            0xf8, 0xfb, 0xbe, 0x3d, 0xdb, 0x9d, 0xf5, 0x46,
            0xfb, 0x71, 0xff, 0x79, 0x8c, 0x49, 0x28, 0xfb,
            0x41, 0xb4, 0xfa, 0xe0, 0xb4, 0xac, 0x10, 0x37,
            0x32, 0x87, 0x7e, 0x12, 0x51, 0x61, 0x2e, 0xd1,
        ])),
        Tmr(Midstate([
            0x5f, 0x8f, 0xea, 0xb9, 0x31, 0xbe, 0x75, 0xf2,
            0x82, 0x05, 0xb2, 0xee, 0x1e, 0xd9, 0x58, 0x2e,
            0xf3, 0xf3, 0x98, 0x40, 0xfc, 0x4a, 0x06, 0xcf,
            0x6f, 0x6e, 0xf7, 0x2c, 0x7a, 0x58, 0x24, 0x8b,
        ])),
        Tmr(Midstate([
            0x5c, 0x12, 0x19, 0x1a, 0x1d, 0xa9, 0x42, 0x22,
            0x8f, 0x94, 0x75, 0x0c, 0x83, 0xce, 0x1a, 0x6e,
            0x55, 0xd8, 0x8e, 0xcb, 0x41, 0x0c, 0x2e, 0x8e,
            0x8f, 0xb0, 0xe7, 0xbe, 0x67, 0xcb, 0x4f, 0x45,
        ])),
        Tmr(Midstate([
            0x92, 0x79, 0x7f, 0xfc, 0x16, 0xc2, 0xb0, 0x33,
            0x62, 0x72, 0x3b, 0xd5, 0x76, 0x00, 0x50, 0xb4,
            0x02, 0x23, 0x80, 0x52, 0x53, 0xbd, 0x8f, 0x15,
            0x0f, 0x65, 0xba, 0xbb, 0x40, 0xea, 0x25, 0x3f,
        ])),
        Tmr(Midstate([
            0xe6, 0x50, 0x9f, 0x99, 0xc2, 0x99, 0x8f, 0x33,
            0xa1, 0x2d, 0x64, 0x98, 0x2b, 0x7c, 0x78, 0x1f,
            0xa1, 0xf5, 0x55, 0x45, 0x82, 0xdb, 0xc3, 0x72,
            0x7d, 0x4c, 0x3b, 0x4d, 0x39, 0x93, 0x70, 0x6b,
        ])),
        Tmr(Midstate([
            0xad, 0x38, 0x65, 0x42, 0x68, 0x1d, 0xcf, 0x7e,
            0x45, 0xab, 0x7b, 0x3e, 0x6d, 0xa6, 0x96, 0x76,
            0xbf, 0xd4, 0xf7, 0x28, 0xaa, 0x8e, 0x1c, 0x2b,
            0x62, 0xa6, 0x95, 0xe4, 0x9b, 0x09, 0x47, 0x1d,
        ])),
        Tmr(Midstate([
            0xc1, 0xd4, 0x96, 0xa9, 0x82, 0x23, 0x94, 0xde,
            0x42, 0x33, 0x1e, 0x75, 0x16, 0xf2, 0x7f, 0x5e,
            0xf6, 0x2f, 0x58, 0xf6, 0x4c, 0x8e, 0xf9, 0x1a,
            0x12, 0x1e, 0x3b, 0xdc, 0x44, 0x9a, 0x6b, 0x99,
        ])),
        Tmr(Midstate([
            0xa1, 0x60, 0xa3, 0x27, 0x73, 0xcf, 0x35, 0x08,
            0xdf, 0x39, 0x95, 0x38, 0x5a, 0xa5, 0xf3, 0x91,
            0xcf, 0x3d, 0xba, 0xb0, 0x6b, 0x81, 0xbf, 0x38,
            0x73, 0x6e, 0x2d, 0x81, 0xa6, 0x15, 0x69, 0x1d,
        ])),
        Tmr(Midstate([
            0x28, 0x53, 0xc9, 0x86, 0xa6, 0x21, 0x5a, 0xbe,
            0x31, 0x24, 0x87, 0xc9, 0x11, 0x3d, 0x77, 0x2b,
            0xbe, 0x52, 0x8e, 0x86, 0xbf, 0x5c, 0x06, 0x94,
            0xeb, 0xdc, 0x7f, 0xe7, 0x29, 0x78, 0x2b, 0x27,
        ])),
        Tmr(Midstate([
            0x7a, 0x13, 0x2f, 0x33, 0xbd, 0x6e, 0xcc, 0xa3,
            0xff, 0x79, 0x16, 0xb1, 0x35, 0x27, 0x19, 0x40,
            0x15, 0x9e, 0x31, 0x42, 0x13, 0x11, 0x11, 0x01,
            0x3a, 0x68, 0xe0, 0xb2, 0xc7, 0xad, 0xda, 0xc5,
        ])),
        Tmr(Midstate([
            0x97, 0x9e, 0x55, 0x28, 0x99, 0x10, 0x39, 0x68,
            0x9f, 0xc4, 0x9d, 0x38, 0xfc, 0xea, 0xc8, 0xc0,
            0x34, 0x7f, 0xde, 0x1d, 0xee, 0xe2, 0xf8, 0x12,
            0xe1, 0xe4, 0x71, 0x8f, 0xdd, 0xcd, 0x0e, 0xaa,
        ])),
        Tmr(Midstate([
            0x76, 0x49, 0xc4, 0xf5, 0x3b, 0x70, 0xd8, 0xbe,
            0x27, 0x8c, 0xdf, 0xd9, 0x2d, 0x0e, 0x13, 0x78,
            0x7b, 0xf6, 0x2a, 0x9c, 0x5b, 0x78, 0x79, 0x57,
            0xd7, 0x1d, 0xdf, 0xae, 0xc6, 0xe0, 0x64, 0x58,
        ])),
        Tmr(Midstate([
            0xf3, 0x3d, 0xc3, 0x9b, 0x53, 0xd6, 0xcd, 0x1a,
            0x3f, 0xdf, 0xf9, 0x5d, 0x42, 0x6e, 0xe0, 0x1b,
            0x8d, 0xd3, 0xfd, 0x32, 0x1b, 0xeb, 0xc7, 0xec,
            0xb6, 0x7e, 0xf3, 0xd2, 0x36, 0xe6, 0xb8, 0xf1,
        ])),
        Tmr(Midstate([
            0x54, 0x0c, 0x28, 0x6c, 0xbc, 0x6f, 0xf0, 0xab,
            0x27, 0xe1, 0x61, 0x3d, 0x17, 0x78, 0x87, 0xda,
            0x48, 0x8e, 0x7f, 0x5b, 0xc1, 0xae, 0x44, 0x16,
            0x42, 0x4d, 0x14, 0xc5, 0x6a, 0xdf, 0xc2, 0xd4,
        ])),
    ];

    /// The TMR for the unit type
    pub const fn unit() -> Tmr {
        Self::UNIT_IV
    }

    /// The TMR for the sum of two types, whose TMRs are given
    pub fn sum(tmr1: Tmr, tmr2: Tmr) -> Tmr {
        Self::SUM_IV.update(tmr1, tmr2)
    }

    /// The TMR for the product of two types, whose TMRs are given
    pub fn product(tmr1: Tmr, tmr2: Tmr) -> Tmr {
        Self::PRODUCT_IV.update(tmr1, tmr2)
    }
}

#[cfg(test)]
mod tests {
    use super::super::bip340_iv;
    use super::*;

    #[test]
    fn const_ivs() {
        assert_eq!(
            Tmr(bip340_iv(b"Simplicity-Draft\x1fType\x1funit")),
            Tmr::UNIT_IV,
        );
        assert_eq!(
            Tmr(bip340_iv(b"Simplicity-Draft\x1fType\x1fsum")),
            Tmr::SUM_IV,
        );
        assert_eq!(
            Tmr(bip340_iv(b"Simplicity-Draft\x1fType\x1fprod")),
            Tmr::PRODUCT_IV,
        );
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn const_powers_of_2() {
        let n = Tmr::POWERS_OF_TWO.len();
        let types = crate::types::Type::powers_of_two(n);
        for i in 0..n {
            assert_eq!(Some(Tmr::POWERS_OF_TWO[i]), types[i].tmr());
        }
    }
}
