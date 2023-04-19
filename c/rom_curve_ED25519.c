/*
 * Copyright (c) 2012-2020 MIRACL UK Ltd.
 *
 * This file is part of MIRACL Core
 * (see https://github.com/miracl/core).
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#include "arch.h"
#include "ecp_Ed25519.h"

/* Ed25519 */

#if CHUNK==16
/* Ed25519 Curve */

const int CURVE_Cof_I_Ed25519= 8;
const BIG_256_13 CURVE_Cof_Ed25519= {0x8,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0};
const int CURVE_B_I_Ed25519= 0;
const BIG_256_13 CURVE_B_Ed25519= {0x18A3,0x1ACB,0x1284,0x169B,0x175E,0xC55,0x507,0x9A8,0x100A,0x3,0x1A26,0xEF3,0x797,0x3A0,0xE33,0x1FCE,0xB6F,0x771,0xDB,0xA4};
const BIG_256_13 CURVE_Order_Ed25519= {0x13ED,0x7AE,0x697,0x4C6,0x581,0xE6B,0xBDE,0x1BD4,0x1EF9,0xA6,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x20};
const BIG_256_13 CURVE_Gx_Ed25519= {0x151A,0x192E,0x1823,0xC5A,0xC95,0x13D9,0x1496,0xC12,0xCC7,0x349,0x1717,0x1BAD,0x31F,0x1271,0x1B02,0xA7F,0xD6E,0x169E,0x1A4D,0x42};
const BIG_256_13 CURVE_Gy_Ed25519= {0x658,0x1333,0x1999,0xCCC,0x666,0x1333,0x1999,0xCCC,0x666,0x1333,0x1999,0xCCC,0x666,0x1333,0x1999,0xCCC,0x666,0x1333,0x1999,0xCC};
const BIG_256_13 CURVE_HTPC_Ed25519= {0x104F,0x83A,0x4E9,0xE1B,0x627,0x191E,0x1A5F,0x6A,0x18C,0x1CBD,0x1AF4,0x1DFB,0x4C9,0x1340,0x1656,0x1DF0,0x7E0,0x1201,0x1064,0xAB};
#endif

#if CHUNK==32

const int CURVE_Cof_I_Ed25519= 8;
const BIG_256_29 CURVE_Cof_Ed25519= {0x8,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0};
const int CURVE_B_I_Ed25519= 0;
const BIG_256_29 CURVE_B_Ed25519= {0x135978A3,0xF5A6E50,0x10762ADD,0x149A82,0x1E898007,0x3CBBBC,0x19CE331D,0x1DC56DFF,0x52036C};
const BIG_256_29 CURVE_Order_Ed25519= {0x1CF5D3ED,0x9318D2,0x1DE73596,0x1DF3BD45,0x14D,0x0,0x0,0x0,0x100000};
const BIG_256_29 CURVE_Gx_Ed25519= {0xF25D51A,0xAB16B04,0x969ECB2,0x198EC12A,0xDC5C692,0x1118FEEB,0xFFB0293,0x1A79ADCA,0x216936};
const BIG_256_29 CURVE_Gy_Ed25519= {0x6666658,0x13333333,0x19999999,0xCCCCCCC,0x6666666,0x13333333,0x19999999,0xCCCCCCC,0x666666};
const BIG_256_29 CURVE_HTPC_Ed25519= {0x507504F,0x13B86C9D,0x5FC8F18,0x31806AD,0x1EBD397A,0x264F7E,0x1E16569A,0x804FC1D,0x55C192};

#endif

#if CHUNK==64

const int CURVE_Cof_I_Ed25519= 8;
const BIG_256_56 CURVE_Cof_Ed25519= {0x8L,0x0L,0x0L,0x0L,0x0L};
const int CURVE_B_I_Ed25519= 0;
const BIG_256_56 CURVE_B_Ed25519= {0xEB4DCA135978A3L,0xA4D4141D8AB75L,0x797779E8980070L,0x2B6FFE738CC740L,0x52036CEEL};
const BIG_256_56 CURVE_Order_Ed25519= {0x12631A5CF5D3EDL,0xF9DEA2F79CD658L,0x14DEL,0x0L,0x10000000L};
const BIG_256_56 CURVE_Gx_Ed25519= {0x562D608F25D51AL,0xC7609525A7B2C9L,0x31FDD6DC5C692CL,0xCD6E53FEC0A4E2L,0x216936D3L};
const BIG_256_56 CURVE_Gy_Ed25519= {0x66666666666658L,0x66666666666666L,0x66666666666666L,0x66666666666666L,0x66666666L};
const BIG_256_56 CURVE_HTPC_Ed25519= {0x770D93A507504FL,0x8C035697F23C62L,0x4C9EFDEBD397A1L,0x27E0EF8595A680L,0x55C19240L};
#endif

