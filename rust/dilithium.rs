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

/* Dilithium API high-level functions.  Constant time where it matters. Slow (spends nearly all of its time running SHA3) but small.

Note that the Matrix A is calculated on-the-fly to keep memory requirement minimal
But this makes all stages much slower
Note that 
1. Matrix A can just be generated randomly for Key generation (without using SHA3 which is very slow)
2. A precalculated A can be included in the public key, for use by signature and verification (which blows up public key size)
3. Precalculating A for signature calculation means that the A does not have to re-calculated for each attempt to find a good signature

Might be simpler to wait for hardware support for SHA3!

   M.Scott 30/09/2021
*/

use crate::rand::RAND;
use crate::sha3;
use crate::sha3::SHA3;

//use std::process;

//q= 8380417
const LGN: usize = 8;
const DEGREE: usize = 1<<LGN;
const PRIME: i32 = 0x7fe001;
const D: usize = 13;
const TD: usize = 23-D;
const WC: usize = 49;
const LG: usize = 19;
const GAMMA1: i32 = 1<<LG;
const RR: i32 = 32;
const GAMMA2: i32 = (PRIME-1)/RR;
const K: usize = 6;
const L: usize = 5;
const ETA: usize = 4;
const LG2ETA1: usize = 4;  // lg(2*ETA+1) rounded up
const BETA: i32 = 196;
const OMEGA: usize = 55;
const YBYTES: usize = ((LG+1)*DEGREE)/8;
const W1B: usize = 4;
const ONE: i32 = 0x3FFE00;    // R mod Q
const COMBO: i32 = 0xA3FA;    // ONE*inv mod Q
const ND: u32 = 0xFC7FDFFF; // 1/(R-Q) mod R

pub const SK_SIZE: usize=32*3+DEGREE*(K*D+L*LG2ETA1+K*LG2ETA1)/8;
pub const PK_SIZE: usize=(K*DEGREE*TD)/8+32;
pub const SIG_SIZE: usize=(DEGREE*L*(LG+1))/8+OMEGA+K+32;

const ROOTS: [i32; 256] = [
    0x3ffe00,0x64f7,0x581103,0x77f504,0x39e44,0x740119,0x728129,0x71e24,0x1bde2b,0x23e92b,0x7a64ae,
    0x5ff480,0x2f9a75,0x53db0a,0x2f7a49,0x28e527,0x299658,0xfa070,0x6f65a5,0x36b788,0x777d91,0x6ecaa1,
    0x27f968,0x5fb37c,0x5f8dd7,0x44fae8,0x6a84f8,0x4ddc99,0x1ad035,0x7f9423,0x3d3201,0x445c5,0x294a67,
    0x17620,0x2ef4cd,0x35dec5,0x668504,0x49102d,0x5927d5,0x3bbeaf,0x44f586,0x516e7d,0x368a96,0x541e42,
    0x360400,0x7b4a4e,0x23d69c,0x77a55e,0x65f23e,0x66cad7,0x357e1e,0x458f5a,0x35843f,0x5f3618,0x67745d,
    0x38738c,0xc63a8,0x81b9a,0xe8f76,0x3b3853,0x3b8534,0x58dc31,0x1f9d54,0x552f2e,0x43e6e6,0x688c82,
    0x47c1d0,0x51781a,0x69b65e,0x3509ee,0x2135c7,0x67afbc,0x6caf76,0x1d9772,0x419073,0x709cf7,0x4f3281,
    0x4fb2af,0x4870e1,0x1efca,0x3410f2,0x70de86,0x20c638,0x296e9f,0x5297a4,0x47844c,0x799a6e,0x5a140a,
    0x75a283,0x6d2114,0x7f863c,0x6be9f8,0x7a0bde,0x1495d4,0x1c4563,0x6a0c63,0x4cdbea,0x40af0,0x7c417,
    0x2f4588,0xad00,0x6f16bf,0xdcd44,0x3c675a,0x470bcb,0x7fbe7f,0x193948,0x4e49c1,0x24756c,0x7ca7e0,
    0xb98a1,0x6bc809,0x2e46c,0x49a809,0x3036c2,0x639ff7,0x5b1c94,0x7d2ae1,0x141305,0x147792,0x139e25,
    0x67b0e1,0x737945,0x69e803,0x51cea3,0x44a79d,0x488058,0x3a97d9,0x1fea93,0x33ff5a,0x2358d4,0x3a41f8,
    0x4cdf73,0x223dfb,0x5a8ba0,0x498423,0x412f5,0x252587,0x6d04f1,0x359b5d,0x4a28a1,0x4682fd,0x6d9b57,
    0x4f25df,0xdbe5e,0x1c5e1a,0xde0e6,0xc7f5a,0x78f83,0x67428b,0x7f3705,0x77e6fd,0x75e022,0x503af7,
    0x1f0084,0x30ef86,0x49997e,0x77dcd7,0x742593,0x4901c3,0x53919,0x4610c,0x5aad42,0x3eb01b,0x3472e7,
    0x4ce03c,0x1a7cc7,0x31924,0x2b5ee5,0x291199,0x585a3b,0x134d71,0x3de11c,0x130984,0x25f051,0x185a46,
    0x466519,0x1314be,0x283891,0x49bb91,0x52308a,0x1c853f,0x1d0b4b,0x6fd6a7,0x6b88bf,0x12e11b,0x4d3e3f,
    0x6a0d30,0x78fde5,0x1406c7,0x327283,0x61ed6f,0x6c5954,0x1d4099,0x590579,0x6ae5ae,0x16e405,0xbdbe7,
    0x221de8,0x33f8cf,0x779935,0x54aa0d,0x665ff9,0x63b158,0x58711c,0x470c13,0x910d8,0x463e20,0x612659,
    0x251d8b,0x2573b7,0x7d5c90,0x1ddd98,0x336898,0x2d4bb,0x6d73a8,0x4f4cbf,0x27c1c,0x18aa08,0x2dfd71,
    0xc5ca5,0x19379a,0x478168,0x646c3e,0x51813d,0x35c539,0x3b0115,0x41dc0,0x21c4f7,0x70fbf5,0x1a35e7,
    0x7340e,0x795d46,0x1a4cd0,0x645caf,0x1d2668,0x666e99,0x6f0634,0x7be5db,0x455fdc,0x530765,0x5dc1b0,
    0x7973de,0x5cfd0a,0x2cc93,0x70f806,0x189c2a,0x49c5aa,0x776a51,0x3bcf2c,0x7f234f,0x6b16e0,0x3c15ca,
    0x155e68,0x72f6b7,0x1e29ce,

];
const IROOTS: [i32; 256] = [
    0x3ffe00,0x7f7b0a,0x7eafd,0x27cefe,0x78c1dd,0xd5ed8,0xbdee8,0x7c41bd,0x56fada,0x5065b8,0x2c04f7,
    0x50458c,0x1feb81,0x57b53,0x5bf6d6,0x6401d6,0x7b9a3c,0x42ae00,0x4bde,0x650fcc,0x320368,0x155b09,
    0x3ae519,0x20522a,0x202c85,0x57e699,0x111560,0x86270,0x492879,0x107a5c,0x703f91,0x5649a9,0x2ab0d3,
    0x6042ad,0x2703d0,0x445acd,0x44a7ae,0x71508b,0x77c467,0x737c59,0x476c75,0x186ba4,0x20a9e9,0x4a5bc2,
    0x3a50a7,0x4a61e3,0x19152a,0x19edc3,0x83aa3,0x5c0965,0x495b3,0x49dc01,0x2bc1bf,0x49556b,0x2e7184,
    0x3aea7b,0x442152,0x26b82c,0x36cfd4,0x195afd,0x4a013c,0x50eb34,0x7e69e1,0x56959a,0x454828,0x375fa9,
    0x3b3864,0x2e115e,0x15f7fe,0xc66bc,0x182f20,0x6c41dc,0x6b686f,0x6bccfc,0x2b520,0x24c36d,0x1c400a,
    0x4fa93f,0x3637f8,0x7cfb95,0x1417f8,0x744760,0x33821,0x5b6a95,0x319640,0x66a6b9,0x2182,0x38d436,
    0x4378a7,0x7212bd,0x10c942,0x7f3301,0x509a79,0x781bea,0x7bd511,0x330417,0x15d39e,0x639a9e,0x6b4a2d,
    0x5d423,0x13f609,0x59c5,0x12beed,0xa3d7e,0x25cbf7,0x64593,0x385bb5,0x2d485d,0x567162,0x5f19c9,0xf017b,
    0x4bcf0f,0x7df037,0x376f20,0x302d52,0x30ad80,0xf430a,0x3e4f8e,0x62488f,0x13308b,0x183045,0x5eaa3a,
    0x4ad613,0x1629a3,0x2e67e7,0x381e31,0x17537f,0x3bf91b,0x61b633,0xce94a,0x6a8199,0x43ca37,0x14c921,
    0xbcb2,0x4410d5,0x875b0,0x361a57,0x6743d7,0xee7fb,0x7d136e,0x22e2f7,0x66c23,0x221e51,0x2cd89c,
    0x3a8025,0x3fa26,0x10d9cd,0x197168,0x62b999,0x1b8352,0x659331,0x682bb,0x78abf3,0x65aa1a,0xee40c,
    0x5e1b0a,0x7bc241,0x44deec,0x4a1ac8,0x2e5ec4,0x1b73c3,0x385e99,0x66a867,0x73835c,0x51e290,0x6735f9,
    0x7d63e5,0x309342,0x126c59,0x7d0b46,0x4c7769,0x620269,0x28371,0x5a6c4a,0x5ac276,0x1eb9a8,0x39a1e1,
    0x76cf29,0x38d3ee,0x276ee5,0x1c2ea9,0x198008,0x2b35f4,0x846cc,0x4be732,0x5dc219,0x74041a,0x68fbfc,
    0x14fa53,0x26da88,0x629f68,0x1386ad,0x1df292,0x4d6d7e,0x6bd93a,0x6e21c,0x15d2d1,0x32a1c2,0x6cfee6,
    0x145742,0x10095a,0x62d4b6,0x635ac2,0x2daf77,0x362470,0x57a770,0x6ccb43,0x397ae8,0x6785bb,0x59efb0,
    0x6cd67d,0x41fee5,0x6c9290,0x2785c6,0x56ce68,0x54811c,0x7cc6dd,0x65633a,0x32ffc5,0x4b6d1a,0x412fe6,
    0x2532bf,0x7b7ef5,0x7aa6e8,0x36de3e,0xbba6e,0x8032a,0x364683,0x4ef07b,0x60df7d,0x2fa50a,0x9ffdf,
    0x7f904,0xa8fc,0x189d76,0x78507e,0x7360a7,0x71ff1b,0x6381e7,0x7221a3,0x30ba22,0x1244aa,0x395d04,
    0x35b760,0x4a44a4,0x12db10,0x5aba7a,0x7bcd0c,0x365bde,0x255461,0x5da206,0x33008e,0x459e09,0x5c872d,
    0x4be0a7,0x5ff56e,
];

/* Montgomery stuff */

fn redc(t: u64) -> i32 {
    let m = (t as u32).wrapping_mul(ND);
    (((m as u64) * (PRIME as u64) + t) >> 32) as i32
}

//fn nres(x: i32) -> i32 {
//    redc((x as u64) * R2MODP)
//}

fn modmul(a: i32, b: i32) -> i32 {
    redc((a as u64) * (b as u64))
}

// NTT code

// Important!
// nres(x); ntt(x)
// nres(y); ntt(y)
// z=x*y
// intt(z);
// redc(z);

// is equivalent to (note that nres() and redc() cancel out)

// ntt(x);
// nres(y); ntt(y);
// z=x*y
// intt(z)

// is equivalent to

// ntt(x)
// ntt(y)
// z=x*y
// intt(z)
// nres(z)

// In all cases z ends up in normal (non-Montgomery) form!
// So the conversion to Montgomery form can be "pushed" through the calculation.

// Here intt(z) <- intt(z);nres(z); 
// Combining is more efficient
// note that ntt() and intt() are not mutually inverse

/* Cooley-Tukey NTT */
/* Excess of 2 allowed on input - coefficients must be < 2*PRIME */
fn ntt(x: &mut [i32]) {
    let mut t = DEGREE / 2;
    let q = PRIME;

    /* Make positive */
    for j in 0..DEGREE {
        x[j] += (x[j]>>31)&PRIME;
    }
    let mut m = 1;
    while m < DEGREE {
        let mut k = 0;
        for i in 0..m {
            let s = ROOTS[m + i];
            for j in k..k + t {
                let u = x[j];
                let v = modmul(x[j + t], s);
                x[j] = u + v;
                x[j + t] = u + 2 * q - v;
            }
            k += 2 * t;
        }
        t /= 2;
        m *= 2;
    }
}

/* Gentleman-Sande INTT */
/* Excess of 2 allowed on input - coefficients must be < 2*PRIME */
/* Output fully reduced */

const NTTL: usize = 2; // maybe could be 1?

fn intt(x: &mut [i32]) {
    let mut t = 1;
    let q = PRIME;
    let mut m = DEGREE / 2;
    let mut n=LGN;
    while m >= 1 {
        let lim=NTTL>>n;
        n-=1;
        let mut k = 0;
        for i in 0..m {
            let s = IROOTS[m + i];
            for j in k..k + t {
                let u:i32;
                let v:i32;
                if m<NTTL && j<k+lim {
                    u=modmul(x[j],ONE);
                    v=modmul(x[j+t],ONE);
                } else {
                    u = x[j];
                    v = x[j + t];
                }
                x[j] = u + v;
                let w = u + ((DEGREE/NTTL) as i32) * q - v;
                x[j + t] = modmul(w, s);
            }
            k += 2 * t;
        }
        t *= 2;
        m /= 2;
    }

    // fully reduce, nres combined with 1/DEGREE
    for j in 0..DEGREE {
        x[j] = modmul(x[j],COMBO);
        x[j] -= q;
        x[j] += (x[j] >> 31) & q;
    }
}

fn redc_it(p: &mut [i32]) {
    for i in 0..DEGREE {
        p[i] = redc(p[i] as u64);
    }
}

fn poly_copy(p1: &mut [i32], p3: &[i32]) {
    for i in 0..DEGREE {
        p1[i] = p3[i];
    }    
}

fn poly_scopy(p1: &mut [i32], p3: &[i8]) {
    for i in 0..DEGREE {
        p1[i] = p3[i] as i32;
    }    
}

fn poly_mcopy(p1: &mut [i32], p3: &[i16]) {
    for i in 0..DEGREE {
        p1[i] = p3[i] as i32;
    }    
}

fn poly_zero(p1: &mut [i32]) {
    for i in 0..DEGREE {
        p1[i] = 0;
    }    
}

fn poly_negate(p1: &mut [i32], p3: &[i32]) {
    for i in 0..DEGREE {
        p1[i] = PRIME-p3[i];
    }    
}

fn poly_mul(p1: &mut [i32], p3: &[i32]) {
    for i in 0..DEGREE {
        p1[i] = modmul(p1[i], p3[i]);
    }
}

fn poly_add(p1: &mut [i32], p3: &[i32]) {
    for i in 0..DEGREE {
        p1[i] += p3[i];
    }
}

fn poly_sub(p1: &mut [i32], p3: &[i32]) {
    for i in 0..DEGREE {
        p1[i] += PRIME - p3[i];
    }
}

/* reduces inputs < 2q */
fn poly_soft_reduce(poly: &mut [i32]) {
    for i in 0..DEGREE {
        let e = poly[i] - PRIME;
        poly[i] = e + ((e >> 31) & PRIME);
    }
}

/* fully reduces modulo q */
fn poly_hard_reduce(poly: &mut [i32]) {
    for i in 0..DEGREE {
        let mut e = modmul(poly[i], ONE);
        e -= PRIME;
        poly[i] = e + ((e >> 31) & PRIME);
    }
}

// Generate a[i][j] from rho
fn expandaij(rho: &[u8],aij: &mut [i32],i:usize,j:usize) {
    let mut buff: [u8; 4*DEGREE] = [0; 4*DEGREE];
    let mut sh = SHA3::new(sha3::SHAKE128);
    for m in 0..32 {
        sh.process(rho[m])
    }
    sh.process(j as u8);
    sh.process(i as u8);
    sh.shake(&mut buff, 4*DEGREE);
    let mut m=0;
    let mut n=0;
    while m<DEGREE {
        let b0=buff[n] as u32;
        let b1=buff[n+1] as u32;
        let b2=buff[n+2] as u32;
        let cf=(((b2&0x7f)<<16) + (b1<<8) + b0) as i32;
        n+=3;
        if cf>=PRIME {
            continue;
        }
        aij[m]=cf;
        m+=1; 
    }
}

// array t has ab active bits per word
// extract bytes from array of words
// if mx!=0 then -mx<=t[i]<=+mx
fn nextbyte32(ab: usize,mx: usize,t: &[i32],ptr:&mut usize,bts: &mut usize) -> u8 {
    let mut left=ab-*bts;
    let mut w=t[*ptr];
    let mxm=mx as i32;
    if mxm!=0 {
        w=mxm-w;
    }
    let mut r=w>>*bts;
    let mut i=0;
    while left<8 {
        i+=1;
        w=t[(*ptr)+i];
        if mxm!=0 {
            w=mxm-w;
        }
        r |= w<<left;
        left += ab;
    }
    *bts+=8;
    while *bts>=ab {
        *bts -= ab;
        *ptr+=1;
    }
    return r as u8;
}

fn nextbyte16(ab: usize,mx: usize,t: &[i16],ptr:&mut usize,bts: &mut usize) -> u8 {
    let mut left=ab-*bts;
    let mut w=t[*ptr];
    let mxm=mx as i16;
    if mxm!=0 {
        w=mxm-w;
    }
    let mut r=w>>*bts;
    let mut i=0;
    while left<8 {
        i+=1;
        w=t[(*ptr)+i];
        if mxm!=0 {
            w=mxm-w;
        }
        r |= w<<left;
        left += ab;
    }
    *bts+=8;
    while *bts>=ab {
        *bts -= ab;
        *ptr += 1;
    }
    return r as u8;
}

fn nextbyte8(ab: usize,mx: usize,t: &[i8],ptr:&mut usize,bts: &mut usize) -> u8 {
    let mut left=ab-*bts;
    let mut w=t[*ptr];
    let mxm=mx as i8;
    if mxm!=0 {
        w=mxm-w;
    }
    let mut r=w>>*bts;
    let mut i=0;
    while left<8 {
        i+=1;
        w=t[(*ptr)+i];
        if mxm!=0 {
            w=mxm-w;
        }
        r |= w<<left;
        left += ab;
    }
    *bts+=8;
    while *bts>=ab {
        *bts -= ab;
        *ptr+=1;
    }
    return r as u8;
}

fn nextword(ab: usize,mx: usize,t: &[u8],ptr:&mut usize,bts: &mut usize) -> i32 {
    let mut r=(t[*ptr]>>*bts) as i32;
    let mxm=mx as i32;
    let mask=(1<<ab)-1;
    let mut w:i32;
    let mut i=0;
    let mut gotbits=8-*bts;
    while gotbits<ab {
        i+=1;
        w=t[(*ptr)+i] as i32;
        r |= w<<gotbits;
        gotbits += 8;
    }
    *bts += ab;
    while *bts>=8 {
        *bts -= 8;
        *ptr+=1;
    }
    w=r&mask;
    if mxm!=0 {
        w=mxm-w;
    }
    return w;
}

fn pack_pk(pk: &mut [u8],rho: &[u8],t1: &[i16]) {
    for i in 0..32 {
        pk[i]=rho[i];
    }
    let mut ptr=0 as usize;
    let mut bts=0 as usize;
    let mut n=32;
    for _ in 0..(K*DEGREE*TD)/8 {
        pk[n]=nextbyte16(TD,0,t1,&mut ptr,&mut bts);
        n += 1;
    }
}

fn unpack_pk(rho: &mut [u8],t1: &mut [i16],pk: &[u8]) {
    for i in 0..32 {
        rho[i]=pk[i];
    }
    let mut ptr=0 as usize;
    let mut bts=0 as usize;
    for i in 0..K*DEGREE {
        t1[i]=nextword(TD,0,&pk[32..],&mut ptr,&mut bts) as i16;
    }
}

fn pack_sk(sk: &mut [u8],rho: &[u8],bk: &[u8],tr: &[u8],s1: &[i8],s2: &[i8],t0: &[i16]) {
    for i in 0..32 {
        sk[i]=rho[i];
    } 
    let mut n=32;
    for i in 0..32 {
        sk[n]=bk[i]; n+=1;
    }
    for i in 0..32 {
        sk[n]=tr[i]; n+=1;
    }
    let mut ptr=0 as usize;
    let mut bts=0 as usize;

    for _ in 0..(L*DEGREE*LG2ETA1)/8 {
        sk[n]=nextbyte8(LG2ETA1,ETA,s1,&mut ptr,&mut bts);
        n += 1;
    }
    ptr=0; bts=0;
    for _ in 0..(K*DEGREE*LG2ETA1)/8 {
        sk[n]=nextbyte8(LG2ETA1,ETA,s2,&mut ptr,&mut bts);
        n += 1;
    }
    ptr=0; bts=0;
    for _ in 0..(K*DEGREE*D)/8 {
        sk[n]=nextbyte16(D,1<<(D-1),t0,&mut ptr,&mut bts);
        n += 1;
    }
}

fn unpack_sk(rho: &mut [u8],bk: &mut [u8],tr: &mut [u8],s1: &mut [i8],s2: &mut [i8],t0: &mut [i16],sk: &[u8]) {
    for i in 0..32 {
        rho[i]=sk[i];
    }
    let mut n=32;
    for i in 0..32 {
        bk[i]=sk[n]; n+=1;
    }
    for i in 0..32 {
        tr[i]=sk[n]; n+=1;
    }
    let mut ptr=0 as usize;
    let mut bts=0 as usize;

    for i in 0..L*DEGREE {
        s1[i]=nextword(LG2ETA1,ETA,&sk[n..],&mut ptr,&mut bts) as i8;
    }  
    n += ptr;
    ptr=0; bts=0;
    for i in 0..K*DEGREE {
        s2[i]=nextword(LG2ETA1,ETA,&sk[n..],&mut ptr,&mut bts) as i8;
    }  
    n += ptr;
    ptr=0; bts=0;
    for i in 0..K*DEGREE {
        t0[i]=nextword(D,1<<(D-1),&sk[n..],&mut ptr,&mut bts) as i16;
    } 
}

// pack signature - changes z
fn pack_sig(sig: &mut [u8],z: &mut [i32],ct: &[u8],h: &[u8]) {
    for i in 0..32 {
        sig[i]=ct[i];
    }
    let mut n=32;
    let mut ptr=0 as usize;
    let mut bts=0 as usize;

    for i in 0..L {
        let row=DEGREE*i;
        for m in 0..DEGREE {
            let mut t=z[row+m];
            if t>PRIME/2 {
                t -= PRIME;
            }
            t=GAMMA1-t;
            z[row+m]=t;
        }
    }
    for _ in 0..(L*DEGREE*(LG+1))/8 {
        sig[n]=nextbyte32(LG+1,0,z,&mut ptr,&mut bts);
        n+=1;
    }
    for i in 0..OMEGA+K {
        sig[n]=h[i];
        n+=1;
    }
}

fn unpack_sig(z: &mut [i32],ct: &mut [u8],h: &mut [u8],sig: &[u8]) {
    for i in 0..32 {
        ct[i]=sig[i];
    }

    let mut ptr=0 as usize;
    let mut bts=0 as usize;

    for i in 0..L*DEGREE {
        let mut t=nextword(LG+1,0,&sig[32..],&mut ptr,&mut bts);
        t=GAMMA1-t;
        if t<0 {
            t += PRIME;
        }
        z[i]=t;
    }
    let mut m=32+(L*DEGREE*(LG+1))/8;
    for i in 0..OMEGA+K {
        h[i]=sig[m];
        m+=1;
    }
}

fn sample_sn(rhod: &[u8],s: &mut [i8],n: usize) {
    let mut buff: [u8; 272] = [0; 272];
    let mut sh = SHA3::new(sha3::SHAKE256);
    for m in 0..64 {
        sh.process(rhod[m]);
    }
    sh.process((n&0xff) as u8);
    sh.process(((n>>8)&0xff) as u8);
    sh.shake(&mut buff, 272);    

    let mut ptr=0 as usize;
    let mut bts=0 as usize;
    for m in 0..DEGREE {
        loop {
            s[m]=nextword(LG2ETA1,0,&buff,&mut ptr,&mut bts) as i8;
            if s[m]<=2*(ETA as i8) {
                break;
            }
        }
        s[m]=(ETA as i8)-s[m];
    }
}

fn sample_y(k: usize,rhod: &[u8],y: &mut [i32]) {
    let mut buff: [u8; YBYTES] = [0; YBYTES];
    for i in 0..L {
        let row=DEGREE*i;
        let mut sh = SHA3::new(sha3::SHAKE256);
        for j in 0..64 {
            sh.process(rhod[j]);
        }
        let ki=k+i;
        sh.process((ki&0xff) as u8);
        sh.process((ki>>8) as u8);
        sh.shake(&mut buff, YBYTES); 

        let mut ptr=0 as usize;
        let mut bts=0 as usize;

        for m in 0..DEGREE {
            let mut w=nextword(LG+1,0,&buff,&mut ptr,&mut bts);
            w=GAMMA1-w;
            let t=w>>31;
            y[row+m]=w+(PRIME&t);
        }
    }
}

fn crh1(h: &mut [u8],rho: &[u8],t1: &[i16]) {
    let mut sh = SHA3::new(sha3::SHAKE256);
    for j in 0..32 {
        sh.process(rho[j]);
    }

    let mut ptr=0 as usize;
    let mut bts=0 as usize;

    for _ in 0..(K*DEGREE*TD)/8 {
        sh.process(nextbyte16(TD,0,t1,&mut ptr,&mut bts));
    }
    sh.shake(h, 32); 
}

fn crh2(h: &mut [u8],tr: &[u8],mess: &[u8]) {
    let mut sh = SHA3::new(sha3::SHAKE256);
    for j in 0..32 {
        sh.process(tr[j]);
    }
    for j in 0..mess.len() {
        sh.process(mess[j]);
    }
    sh.shake(h, 64); 
}

fn crh3(h: &mut [u8],bk: &[u8],mu: &[u8]) {
    let mut sh = SHA3::new(sha3::SHAKE256);
    for j in 0..32 {
        sh.process(bk[j]);
    }
    for j in 0..64 {
        sh.process(mu[j]);
    }
    sh.shake(h, 64); 
}

fn h4(ct: &mut [u8], mu: &[u8],w1: &[i8]) {
    let mut sh = SHA3::new(sha3::SHAKE256);
    for j in 0..64 {
        sh.process(mu[j]);
    }
    
    let mut ptr=0 as usize;
    let mut bts=0 as usize;

    for _ in 0..(K*DEGREE*W1B)/8 {
        sh.process(nextbyte8(W1B,0,w1,&mut ptr,&mut bts));
    }
    sh.shake(ct, 32); 
}

fn sampleinball(ct: &[u8],c: &mut [i32]) {
    let mut buff: [u8; 136] = [0; 136]; 
    let mut signs: [u8; 8] = [0; 8]; 
    let mut sh = SHA3::new(sha3::SHAKE256);
    for j in 0..32 {
        sh.process(ct[j]);
    }  
    sh.shake(&mut buff,136);
    for i in 0..8 {
        signs[i]=buff[i];
    }
    let mut k=8;
    let mut b=0;
    poly_zero(c);
    let mut j:usize;
    let mut n=1;
    let mut sn=signs[0];
    for i in DEGREE-WC..DEGREE {
        loop {
            j=buff[k] as usize; k+=1;
            if j<=i {
                break;
            }
        }
        c[i]=c[j];
        c[j]=1-2*((sn as i32)&1);
        sn >>= 1; b += 1;
        if b==8 {
            sn=signs[n]; n += 1; b=0;
        }
    }
}

fn p2r(r0: &mut i32) -> i16 {
    let d=(1<<D) as i32;
    let r1=(*r0+d/2-1)>>D;
    *r0-=r1*d;
    return r1 as i16;
}

fn power2round(t: &[i32],t0: &mut [i16],t1: &mut [i16]) {
    for m in 0..DEGREE {
        let mut w=t[m];
        t1[m]=p2r(&mut w);
        t0[m]=w as i16;
    }
}

fn decompose_lo(a: i32) -> i32 {
    let mut a1=(a+127) >> 7;
    a1 = (a1*1025+(1<<21))>>22;
    a1 &= 15;

    let mut a0=a-a1*(PRIME-1)/16;
    a0 -= (((PRIME-1)/2-a0)>>31)&PRIME;
    a0 += (a0>>31)&PRIME;
    return a0;
}

fn decompose_hi(a: i32) -> i8 {
    let mut a1=(a+127) >> 7;
    a1 = (a1*1025+(1<<21))>>22;
    a1 &= 15;
    return a1 as i8;
}

fn lobits(r0: &mut [i32],r: &[i32]) {
    for m in 0..DEGREE {
        r0[m]=decompose_lo(r[m]);
    }
}

fn hibits(r1: &mut [i8],r: &[i32]) {
    for m in 0..DEGREE {
        r1[m]=decompose_hi(r[m]);
    }
}

fn makepartialhint(h: &mut [u8],hptr: usize,z: &[i32],r: &[i32]) -> usize {
    let mut ptr=hptr;
    for m in 0..DEGREE {
        let a0=decompose_hi(r[m]);
        let mut rz=r[m]+z[m];
        let t=rz-PRIME;
        rz=rz+((t>>31)&PRIME);
        let a1=decompose_hi(rz);
        if a0!=a1 {
            if ptr>=OMEGA {
                return OMEGA+1;
            }
            h[ptr]=(m&0xff) as u8; ptr += 1;
        }
    }
    return ptr;
}

fn usepartialhint(r: &mut [i8],h: &[u8], hptr: usize,i: usize,w: &[i32]) -> usize{
    let mut ptr=hptr;
    let md=(RR/2) as i8;
    for m in 0..DEGREE {
        let mut a1=decompose_hi(w[m]);
        if m==h[ptr] as usize && ptr<h[OMEGA+i] as usize {
            ptr += 1;
            let a0=decompose_lo(w[m]);
            if a0 <= PRIME/2 {
                a1 += 1;
                if a1 >= md {
                    a1 -= md;
                }
            } else {
                a1 -= 1;
                if a1<0 {
                    a1 += md;
                }
            }
        }
        r[m]=a1;
    }
    return ptr;
}

fn infinity_norm(w: &[i32]) -> i32 {
    let mut n=0 as i32;
    for m in 0..DEGREE {
        let mut az=w[m];
        if az>PRIME/2 {
            az=PRIME-az;
        }
        if az>n {
            n=az;
        }   
    }
    return n;
}

pub fn keypair(rng: &mut impl RAND,sk: &mut [u8],pk: &mut [u8]) {
    let mut sh = SHA3::new(sha3::SHAKE256);
    let mut tau: [u8; 32] = [0; 32]; 
    let mut buff: [u8; 128] = [0; 128]; 
    let mut rho: [u8; 32] = [0; 32]; 
    let mut rhod: [u8; 64] = [0; 64];
    let mut bk: [u8; 32] = [0; 32];
    let mut tr: [u8; 32] = [0; 32]; 
    let mut aij: [i32; DEGREE] = [0; DEGREE];
    let mut s1: [i8; L*DEGREE] = [0; L*DEGREE];
    let mut s2: [i8; K*DEGREE] = [0; K*DEGREE];
    let mut t0: [i16; K*DEGREE] = [0; K*DEGREE];
    let mut t1: [i16; K*DEGREE] = [0; K*DEGREE];
    let mut w: [i32; DEGREE] = [0; DEGREE];
    let mut r: [i32; DEGREE] = [0; DEGREE];

    for i in 0..32 {
        tau[i] = rng.getbyte();
    }
    for i in 0..32 {
        sh.process(tau[i]);
    }
    sh.shake(&mut buff,128);
    for i in 0..32 {
        rho[i] = buff[i];
        bk[i] = buff[i+96];
    }    
    for i in 0..64 {
        rhod[i] = buff[i+32];
    }

    for i in 0..L {
        let row=DEGREE*i;
        sample_sn(&rhod,&mut s1[row..],i);
    }

    for i in 0..K {
        let row=DEGREE*i;
        sample_sn(&rhod,&mut s2[row..],L+i);
        poly_zero(&mut r);
        for j in 0..L {
            poly_scopy(&mut w,&s1[j*DEGREE..]);
            ntt(&mut w);
            expandaij(&rho,&mut aij,i,j);
            poly_mul(&mut w,&aij);
            poly_add(&mut r,&w);
        }
        poly_hard_reduce(&mut r);
        intt(&mut r);
        poly_scopy(&mut w,&s2[row..]);
        poly_add(&mut r,&w);
        poly_soft_reduce(&mut r);
        power2round(&r,&mut t0[row..],&mut t1[row..]);
    }
    crh1(&mut tr,&rho,&t1);
    pack_pk(pk,&rho,&t1);
    pack_sk(sk,&rho,&bk,&tr,&s1,&s2,&t0);
}

pub fn signature(sk: &[u8],m: &[u8],sig: &mut[u8]) -> usize {
    let mut rho: [u8; 32] = [0; 32]; 
    let mut bk: [u8; 32] = [0; 32];
    let mut ct: [u8; 32] = [0; 32];
    let mut tr: [u8; 32] = [0; 32]; 
    let mut mu: [u8; 64] = [0; 64];
    let mut rhod: [u8; 64] = [0; 64];
    let mut hint: [u8; OMEGA+K] = [0; OMEGA+K];

    //let mut aij: [i32; DEGREE] = [0; DEGREE];
    let mut s1: [i8; L*DEGREE] = [0; L*DEGREE];
    let mut s2: [i8; K*DEGREE] = [0; K*DEGREE];
    let mut t0: [i16; K*DEGREE] = [0; K*DEGREE];
    
    let mut y: [i32; L*DEGREE] = [0; L*DEGREE];
    let mut ay: [i32; K*DEGREE] = [0; K*DEGREE];

    let mut w1: [i8; K*DEGREE] = [0; K*DEGREE];
    let mut c: [i32; DEGREE] = [0; DEGREE];
    let mut w: [i32; DEGREE] = [0; DEGREE];
    let mut r: [i32; DEGREE] = [0; DEGREE];

    unpack_sk(&mut rho,&mut bk,&mut tr,&mut s1,&mut s2,&mut t0,&sk);

// signature
    crh2(&mut mu,&tr,m);
    crh3(&mut rhod,&bk,&mu);
    let mut k=0;

    loop {
        let fk=k*L; k+=1;
        sample_y(fk,&rhod,&mut y);
// NTT y
        for i in 0..L {
            let row=DEGREE*i;
            ntt(&mut y[row..]);
        }
// Calculate ay 
        for i in 0..K {
            let row=DEGREE*i;
            poly_zero(&mut r);
            for j in 0..L {
                poly_copy(&mut w,&y[j*DEGREE..]);
                expandaij(&rho,&mut c,i,j);
                poly_mul(&mut w,&c);
                poly_add(&mut r,&w);
            }
            poly_hard_reduce(&mut r);
            intt(&mut r);
            poly_copy(&mut ay[row..],&r);
// Calculate w1
            hibits(&mut w1[row..],&ay[row..]);
        }
// Calculate c
        h4(&mut ct,&mu,&w1);
        sampleinball(&ct,&mut c);
        let mut badone=false;
// Calculate z=y+c.s1
        ntt(&mut c);
        for i in 0..L {
            let row=DEGREE*i;
            poly_scopy(&mut w,&s1[row..]);
            ntt(&mut w);
            poly_mul(&mut w,&c);
            intt(&mut w);
// unNTT y
            redc_it(&mut y[row..]);
            intt(&mut y[row..]);

            poly_add(&mut y[row..],&w);
            poly_soft_reduce(&mut y[row..]);
            if infinity_norm(&y[row..])>=GAMMA1-BETA {
                badone=true;
                break;
            }
        }
        if badone {
            continue;
        }
// Calculate ay=w-c.s2 and r0=lobits(w-c.s2)
        let mut nh=0;
        for i in 0..OMEGA+K {
            hint[i]=0;
        }
        for i in 0..K {
            let row=DEGREE*i;
            poly_scopy(&mut w,&s2[row..]);
            ntt(&mut w);
            poly_mul(&mut w,&c);
            intt(&mut w);
            poly_sub(&mut ay[row..],&w);
            poly_soft_reduce(&mut ay[row..]);
            lobits(&mut w,&ay[row..]);
            if infinity_norm(&w) >= GAMMA2-BETA {
                badone=true;
                break;
            }
            poly_mcopy(&mut w,&t0[row..]);
            ntt(&mut w);
            poly_mul(&mut w,&c);
            intt(&mut w);
            poly_negate(&mut r,&w);
            if infinity_norm(&r) >= GAMMA2 {
                badone=true;
                break;
            }  
            poly_sub(&mut ay[row..],&r);
            poly_soft_reduce(&mut ay[row..]);
            nh=makepartialhint(&mut hint,nh,&r,&ay[row..]);
            if nh>OMEGA {
                badone=true;
                break;
            }
            hint[OMEGA+i]=nh as u8;
        }
        if badone {
            continue;
        }
        break;
    }

    pack_sig(sig,&mut y,&ct,&hint);
    return k;
}

pub fn verify(pk: &[u8],m: &[u8],sig: &[u8]) -> bool {
    let mut rho: [u8; 32] = [0; 32]; 
    let mut ct: [u8; 32] = [0; 32];
    let mut cct: [u8; 32] = [0; 32];
    let mut tr: [u8; 32] = [0; 32]; 
    let mut mu: [u8; 64] = [0; 64];
    let mut hint: [u8; OMEGA+K] = [0; OMEGA+K];

    let mut z: [i32; L*DEGREE] = [0; L*DEGREE];
    let mut t1: [i16; K*DEGREE] = [0; K*DEGREE];
    let mut w1d: [i8; K*DEGREE] = [0; K*DEGREE];

    let mut aij: [i32; DEGREE] = [0; DEGREE];
    let mut c: [i32; DEGREE] = [0; DEGREE];
    let mut w: [i32; DEGREE] = [0; DEGREE];
    let mut r: [i32; DEGREE] = [0; DEGREE];    

    unpack_pk(&mut rho,&mut t1,pk);
    unpack_sig(&mut z,&mut ct,&mut hint,sig);

    for i in 0..L {
        let row=DEGREE*i;
        if infinity_norm(&z[row..]) >= GAMMA1-BETA {
            return false;
        }
        ntt(&mut z[row..]);
    }
    crh1(&mut tr,&rho,&t1);
    crh2(&mut mu,&tr,m);

    sampleinball(&ct,&mut c);
    ntt(&mut c);

// Calculate az
    let mut hints=0;
    for i in 0..K {
        let row=DEGREE*i;
        poly_zero(&mut r);
        for j in 0..L {
            poly_copy(&mut w,&z[j*DEGREE..]);
            expandaij(&rho,&mut aij,i,j);
            poly_mul(&mut w,&aij);
            poly_add(&mut r,&w);
        }
        poly_hard_reduce(&mut r);

// Calculate Az-ct1.2^d
        for m in 0..DEGREE {
            w[m]=((t1[row+m]) as i32) << D;
        }
        ntt(&mut w);
        poly_mul(&mut w,&c);
        poly_sub(&mut r,&w);
        intt(&mut r);

        hints=usepartialhint(&mut w1d[row..],&mut hint,hints,i,&r);
        if hints>OMEGA {
            return false;
        }
    }

    h4(&mut cct,&mu,&w1d);

    for i in 0..32 {
        if ct[i]!=cct[i] {
            return false;
        }
    }
    return true;
}