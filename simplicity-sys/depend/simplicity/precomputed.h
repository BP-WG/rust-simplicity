/* This file has been automatically generated by GenPecomputed.hs */
#ifndef SIMPLICITY_PRECOMPUTED_H
#define SIMPLICITY_PRECOMPUTED_H

#include "sha256.h"

/* This array contains the cmr of all canonical expressions of type X |- 2 that output distinct values. */
static const sha256_midstate bit_cmr[] =
  { {{0xbd0cce93u, 0xe713a2aeu, 0x961bf91cu, 0x7d113edbu, 0x0671c786u, 0x9c722513u, 0x64682ac8u, 0x977eade7u}}
  , {{0x79a70c6au, 0xe11897acu, 0xc1428c38u, 0x568a4522u, 0x2e7c3ea6u, 0x4c66ab4au, 0x104324eeu, 0x391bff9du}} };

/* word_type_root[0] contains the type root of the ONE type.
 * word_type_root[1] contains the type root of the TWO type.
 * word_type_root[2] contains the type root of the TWO^2 type.
 * word_type_root[3] contains the type root of the TWO^4 type.
 * ...
 * word_type_root[32] contains the type root of the TWO^(2^31) type.
 */
static const sha256_midstate word_type_root[] =
  { {{0x12b4c4a9u, 0xa4b0edf6u, 0x5a44f30eu, 0xa762578fu, 0xdd59f105u, 0xf0e4d8f3u, 0x88cb9b6bu, 0xd2c13adfu}}
  , {{0x07182be8u, 0x37cfdd22u, 0xce79442eu, 0x00b6af5au, 0xd9d3a3d8u, 0x7df28194u, 0x43be3fc5u, 0xfe53b988u}}
  , {{0x56e61352u, 0x92caeddeu, 0xf10a6830u, 0x408e14d8u, 0x86474d75u, 0xd3a566f0u, 0x5568ae8bu, 0x98584f29u}}
  , {{0xd5748fd3u, 0xc061e3f1u, 0x33f5e9a3u, 0xa2e4cbcbu, 0x7c58cf62u, 0x15ed2d9fu, 0x5e81a53du, 0xd40ad309u}}
  , {{0xd72f4729u, 0x9992c427u, 0x0aa60fdcu, 0x4c4703d1u, 0x8055f578u, 0xd03e7a24u, 0x48d96406u, 0xe2bb8d6fu}}
  , {{0xfd2c7e26u, 0xebf131b4u, 0x160a5d95u, 0xd4070b7du, 0xeb67fccbu, 0xdf1f96e1u, 0x7c87f121u, 0xc762d16fu}}
  , {{0xdc18653au, 0xfff1ba26u, 0xbf0efdc3u, 0x26c87138u, 0xe1689941u, 0xc3e90f94u, 0xdccdf1c2u, 0xa4c38129u}}
  , {{0xdff38907u, 0x1e1aade0u, 0xf4845c2fu, 0x3223f6c4u, 0x9d15a162u, 0x650980c9u, 0x7f3c1148u, 0x8e3b62b6u}}
  , {{0x355c2115u, 0xb143eb6au, 0xe544866fu, 0x79d026f2u, 0xcd9bbc70u, 0xbe92dd5du, 0x160e0362u, 0x349adb2bu}}
  , {{0x10f1fc6fu, 0x17c40d52u, 0xa801ce38u, 0xd791dcc5u, 0xba7a5882u, 0xbd286595u, 0xc588af6bu, 0x6853facbu}}
  , {{0x4614fa1au, 0x08820245u, 0x042fa54bu, 0x7eaf83ecu, 0xaf235f35u, 0x3d5a6e2fu, 0x0b2bc058u, 0xae56ada5u}}
  , {{0x590ed90eu, 0x096efc99u, 0x6a264056u, 0xb789f8bcu, 0x280b88f8u, 0x20354278u, 0x718efad0u, 0xb90f34d2u}}
  , {{0xe40826b1u, 0x173e0e25u, 0x118f0b3au, 0x74553580u, 0x67c6241cu, 0x47dbeb87u, 0xd8f8f3f7u, 0xba30b6d0u}}
  , {{0x654eedfeu, 0x51487de7u, 0x5c70ecd6u, 0x09a94f47u, 0x92ee4e8cu, 0x1f19cf86u, 0x547f67c8u, 0xdf29474cu}}
  , {{0xcc705c34u, 0x1dc4da07u, 0xfde28d3cu, 0x47a3290fu, 0x692dee07u, 0x0f1eef2du, 0x2e8cd82fu, 0xd20e310cu}}
  , {{0xea08c853u, 0xe8e89828u, 0xc5bebfceu, 0xb8e24ea0u, 0x12fbd629u, 0x3cd941f3u, 0xc0db244eu, 0x501a3a86u}}
  , {{0x71cbe799u, 0x84203e59u, 0x4180ee67u, 0x55b84b44u, 0xab613012u, 0xfad37395u, 0x46da408bu, 0xa05d4d9au}}
  , {{0x3e828931u, 0xf36f6ae6u, 0x6ab13e31u, 0x7703080du, 0x89acb530u, 0xedf74614u, 0x5f32045cu, 0x3ae179a3u}}
  , {{0x38930a55u, 0x7759db60u, 0x40292b95u, 0x2a045d65u, 0xbd2091eau, 0xe2b9f525u, 0xaa84c7b8u, 0xa05ed7d7u}}
  , {{0xf8fbbe3du, 0xdb9df546u, 0xfb71ff79u, 0x8c4928fbu, 0x41b4fae0u, 0xb4ac1037u, 0x32877e12u, 0x51612ed1u}}
  , {{0x5f8feab9u, 0x31be75f2u, 0x8205b2eeu, 0x1ed9582eu, 0xf3f39840u, 0xfc4a06cfu, 0x6f6ef72cu, 0x7a58248bu}}
  , {{0x5c12191au, 0x1da94222u, 0x8f94750cu, 0x83ce1a6eu, 0x55d88ecbu, 0x410c2e8eu, 0x8fb0e7beu, 0x67cb4f45u}}
  , {{0x92797ffcu, 0x16c2b033u, 0x62723bd5u, 0x760050b4u, 0x02238052u, 0x53bd8f15u, 0x0f65babbu, 0x40ea253fu}}
  , {{0xe6509f99u, 0xc2998f33u, 0xa12d6498u, 0x2b7c781fu, 0xa1f55545u, 0x82dbc372u, 0x7d4c3b4du, 0x3993706bu}}
  , {{0xad386542u, 0x681dcf7eu, 0x45ab7b3eu, 0x6da69676u, 0xbfd4f728u, 0xaa8e1c2bu, 0x62a695e4u, 0x9b09471du}}
  , {{0xc1d496a9u, 0x822394deu, 0x42331e75u, 0x16f27f5eu, 0xf62f58f6u, 0x4c8ef91au, 0x121e3bdcu, 0x449a6b99u}}
  , {{0xa160a327u, 0x73cf3508u, 0xdf399538u, 0x5aa5f391u, 0xcf3dbab0u, 0x6b81bf38u, 0x736e2d81u, 0xa615691du}}
  , {{0x2853c986u, 0xa6215abeu, 0x312487c9u, 0x113d772bu, 0xbe528e86u, 0xbf5c0694u, 0xebdc7fe7u, 0x29782b27u}}
  , {{0x7a132f33u, 0xbd6ecca3u, 0xff7916b1u, 0x35271940u, 0x159e3142u, 0x13111101u, 0x3a68e0b2u, 0xc7addac5u}}
  , {{0x979e5528u, 0x99103968u, 0x9fc49d38u, 0xfceac8c0u, 0x347fde1du, 0xeee2f812u, 0xe1e4718fu, 0xddcd0eaau}}
  , {{0x7649c4f5u, 0x3b70d8beu, 0x278cdfd9u, 0x2d0e1378u, 0x7bf62a9cu, 0x5b787957u, 0xd71ddfaeu, 0xc6e06458u}}
  , {{0xf33dc39bu, 0x53d6cd1au, 0x3fdff95du, 0x426ee01bu, 0x8dd3fd32u, 0x1bebc7ecu, 0xb67ef3d2u, 0x36e6b8f1u}}
  , {{0x540c286cu, 0xbc6ff0abu, 0x27e1613du, 0x177887dau, 0x488e7f5bu, 0xc1ae4416u, 0x424d14c5u, 0x6adfc2d4u}} };

#endif
