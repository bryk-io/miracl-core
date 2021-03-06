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

extern crate mcore;
use mcore::gcm;

fn char2int(inp: u8) -> u8 {
    if inp>='0' as u8 && inp <='9' as u8 {
        return inp-'0' as u8;
    }
    if inp>='A' as u8 && inp <='F' as u8 {
        return inp-('A' as u8) +10;
    }
    if inp>='a' as u8 && inp <='f' as u8 {
        return inp-('a' as u8) +10;
    }
    return 0;
}

// s better have even number of characters!
fn decode_hex(x: &mut[u8],s: &str) -> usize {
    let c=s.as_bytes();
    let mut i=0;
    let mut j=0;

    while j<c.len() {
        x[i]=char2int(c[j])*16+char2int(c[j+1]);
        i+=1;
        j+=2;
    }
    return i;
}

fn printbinary(array: &[u8]) {
    for i in 0..array.len() {
        print!("{:02X}", array[i])
    }
    println!("")
}

#[allow(non_snake_case)]
fn hpke_c25519() {
    use mcore::c25519::hpke;
    use mcore::c25519::ecdh;
    use mcore::c25519::ecp;

    const EGS: usize = ecdh::EGS;
    const EFS: usize = ecdh::EFS;
    const EAS: usize = ecp::AESKEY;
    const EHS: usize = ecp::HASH_TYPE;

	let config_id=0x520;

    let mut skE: [u8; EGS] = [0; EGS];
    let mut skR: [u8; EGS] = [0; EGS];
    let mut skS: [u8; EGS] = [0; EGS];
    let mut pkE: [u8; EFS] = [0; EFS];
    let mut pkR: [u8; EFS] = [0; EFS];
    let mut pkS: [u8; EFS] = [0; EFS];

    let mut z: [u8;EHS] = [0;EHS];

    let mut key:[u8;EAS]=[0;EAS];
    let mut nonce:[u8;12]=[0;12];
    let mut exp_secret:[u8;EHS]=[0;EHS];

    const INFO:&str="4f6465206f6e2061204772656369616e2055726e";  let mut info: [u8; INFO.len()/2]=[0;INFO.len()/2];
    const PSK:&str="0247fd33b913760fa1fa51e1892d9f307fbe65eb171e8132c2af18555a738b82"; let mut psk: [u8;PSK.len()/2]=[0;PSK.len()/2];
    const PSKID:&str="456e6e796e20447572696e206172616e204d6f726961"; let mut pskID: [u8; PSKID.len()/2]=[0;PSKID.len()/2];
    const PLAIN:&str="4265617574792069732074727574682c20747275746820626561757479"; let mut plain: [u8; PLAIN.len()/2]=[0;PLAIN.len()/2];
    const AAD:&str="436f756e742d30"; let mut aad: [u8; AAD.len()/2]=[0;AAD.len()/2];
    let mut seedE:[u8;32]=[0;32];
    let mut seedR:[u8;32]=[0;32];
    let mut seedS:[u8;32]=[0;32];

    decode_hex(&mut info,&INFO);
    decode_hex(&mut psk,&PSK);
    decode_hex(&mut pskID,&PSKID);
    decode_hex(&mut plain,&PLAIN);
    decode_hex(&mut aad,&AAD);

    let mut cipher:[u8;64]=[0;64];
    let mut tag:[u8;16]=[0;16];
    println!("\nTesting HPKE code for curve c25519");
// Mode 0
    let mut mode=0;
    println!("\nMode 0");
    decode_hex(&mut seedE,"1dacee520c81ade608f4fa3e5ccae0ecedcc7880e3fc6f3e5afd2e4af8396571");
    decode_hex(&mut seedR,"0a3367dadc97e200074936b5adedcd5680f30672d1ec7158fdfcb795040ec909");
    hpke::deriveKeyPair(config_id,&mut skE,&mut pkE,&seedE);
    hpke::deriveKeyPair(config_id,&mut skR,&mut pkR,&seedR);

	hpke::encap(config_id,&skE,&mut z,&pkE,&pkR);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z);

	hpke::decap(config_id,&skR,&mut z,&pkE,&pkR);
	print!("Decapsulated z= "); printbinary(&z);

	hpke::keyschedule(config_id,&mut key,&mut nonce,&mut exp_secret,mode,&mut z,&info,None,None);
	print!("key= "); printbinary(&key);
	print!("nonce= "); printbinary(&nonce);
	print!("exporter secret= "); printbinary(&exp_secret);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key,&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

// Mode 1
    mode=1;
    println!("\nMode 1");
    decode_hex(&mut seedE,"7f5b697a87fb11d733e31265410c80b72525637e1b872f9ede00d507c0c534d7");
    decode_hex(&mut seedR,"89a6ce4b7b8eb12eebff3864615a2539e3e64fa2d4ab525c18c7bc451a2b5e40");
    hpke::deriveKeyPair(config_id,&mut skE,&mut pkE,&seedE);
    hpke::deriveKeyPair(config_id,&mut skR,&mut pkR,&seedR);
   
	hpke::encap(config_id,&skE,&mut z,&pkE,&pkR);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z);

	hpke::decap(config_id,&skR,&mut z,&pkE,&pkR);
	print!("Decapsulated z= "); printbinary(&z);

	hpke::keyschedule(config_id,&mut key,&mut nonce,&mut exp_secret,mode,&mut z,&info,Some(&psk),Some(&pskID));
	print!("key= "); printbinary(&key);
	print!("nonce= "); printbinary(&nonce);
	print!("exporter secret= "); printbinary(&exp_secret);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key,&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

// Mode 2
    mode=2;
    println!("\nMode 2");
    decode_hex(&mut seedE,"f7faa81395a22af1071b5273f9f94db0609ab9a7f4eca54834ea408d97294cc2");
    decode_hex(&mut seedR,"a10d107753f85bf7128768693e53129fb44a281ea4849a86231632db48dbb28c");
    decode_hex(&mut seedS,"e1257ce84dd98bca7847d75661b658cd2cc6c4efbf9e989a40f390720b57d409");
    hpke::deriveKeyPair(config_id,&mut skE,&mut pkE,&seedE);
    hpke::deriveKeyPair(config_id,&mut skR,&mut pkR,&seedR);
    hpke::deriveKeyPair(config_id,&mut skS,&mut pkS,&seedS);
   
	hpke::authencap(config_id,&skE,&skS,&mut z,&pkE,&pkR,&pkS);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z);

	hpke::authdecap(config_id,&skR,&mut z,&pkE,&pkR,&pkS);
	print!("Decapsulated z= "); printbinary(&z);

	hpke::keyschedule(config_id,&mut key,&mut nonce,&mut exp_secret,mode,&mut z,&info,None,None);
	print!("key= "); printbinary(&key);
	print!("nonce= "); printbinary(&nonce);
	print!("exporter secret= "); printbinary(&exp_secret);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key,&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

// Mode 3
    mode=3;
    println!("\nMode 3");
    decode_hex(&mut seedE,"e98713f7e24e7025fb7abddfff317d865ad5f4c3b6be22cda48dbcdde6151a68");
    decode_hex(&mut seedR,"b281654cdb2cd6a9670c3035eaa702736a951585a938381198f7d29085ca7f3a");
    decode_hex(&mut seedS,"ea95c4d3c5e7868af29fc0cb0841485dead8d54b57ee1b8ed31a153295454835");
    hpke::deriveKeyPair(config_id,&mut skE,&mut pkE,&seedE);
    hpke::deriveKeyPair(config_id,&mut skR,&mut pkR,&seedR);
    hpke::deriveKeyPair(config_id,&mut skS,&mut pkS,&seedS);

	hpke::authencap(config_id,&skE,&skS,&mut z,&pkE,&pkR,&pkS);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z);

	hpke::authdecap(config_id,&skR,&mut z,&pkE,&pkR,&pkS);
	print!("Decapsulated z= "); printbinary(&z);

	hpke::keyschedule(config_id,&mut key,&mut nonce,&mut exp_secret,mode,&mut z,&info,Some(&psk),Some(&pskID));
	print!("key= "); printbinary(&key);
	print!("nonce= "); printbinary(&nonce);
	print!("exporter secret= "); printbinary(&exp_secret);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key,&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

}

#[allow(non_snake_case)]
fn hpke_nist521() {
    use mcore::nist521::hpke;
    use mcore::nist521::ecdh;
    use mcore::nist521::ecp;

    const EGS: usize = ecdh::EGS;
    const EFS: usize = ecdh::EFS;
    const EAS: usize = ecp::AESKEY;
    const EHS: usize = ecp::HASH_TYPE;

	let config_id=0xB12;
    let mut skE: [u8; EGS] = [0; EGS];
    let mut skR: [u8; EGS] = [0; EGS];
    let mut skS: [u8; EGS] = [0; EGS];
    let mut pkE: [u8; 2*EFS+1] = [0; 2*EFS+1];
    let mut pkR: [u8; 2*EFS+1] = [0; 2*EFS+1];
    let mut pkS: [u8; 2*EFS+1] = [0; 2*EFS+1];

    let mut z: [u8;EHS] = [0; EHS];

    let mut key:[u8;EAS]=[0;EAS];
    let mut nonce:[u8;12]=[0;12];
    let mut exp_secret:[u8;EHS]=[0;EHS];


    const INFO:&str="4f6465206f6e2061204772656369616e2055726e";  let mut info: [u8; INFO.len()/2]=[0;INFO.len()/2];
    const PSK:&str="0247fd33b913760fa1fa51e1892d9f307fbe65eb171e8132c2af18555a738b82"; let mut psk: [u8;PSK.len()/2]=[0;PSK.len()/2];
    const PSKID:&str="456e6e796e20447572696e206172616e204d6f726961"; let mut pskID: [u8; PSKID.len()/2]=[0;PSKID.len()/2];
    const PLAIN:&str="4265617574792069732074727574682c20747275746820626561757479"; let mut plain: [u8; PLAIN.len()/2]=[0;PLAIN.len()/2];
    const AAD:&str="436f756e742d30"; let mut aad: [u8; AAD.len()/2]=[0;AAD.len()/2];
    let mut seedE:[u8;66]=[0;66];
    let mut seedR:[u8;66]=[0;66];
    let mut seedS:[u8;66]=[0;66];

    decode_hex(&mut info,&INFO);
    decode_hex(&mut psk,&PSK);
    decode_hex(&mut pskID,&PSKID);
    decode_hex(&mut plain,&PLAIN);
    decode_hex(&mut aad,&AAD);

    let mut cipher:[u8;EAS]=[0;EAS];
    let mut tag:[u8;16]=[0;16];
    println!("\nTesting HPKE code for curve nist521");
// Mode 0
    let mut mode=0;
    println!("\nMode 0");
    decode_hex(&mut seedE,"bb8f0c6c0d81a0a54d6084e3d7d8001875ce2b4831400075246ddeabc3f62d065d91fa8f2ca0cfee3a94f1fa6f43f53a2e39241ccd5ad9f0d889aba103e78286ca17");
    decode_hex(&mut seedR,"28b1403adcf3d2bba7d7c733df4ef8ab9f4c2cf92c840beb7de3fd139611a95be850589a49825450c033b2e1f6cb66127c83827d5464fca1e427c2bb31bf18e857e2");
    hpke::deriveKeyPair(config_id,&mut skE,&mut pkE,&seedE);
    hpke::deriveKeyPair(config_id,&mut skR,&mut pkR,&seedR);   

	hpke::encap(config_id,&skE,&mut z,&pkE,&pkR);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z);

	hpke::decap(config_id,&skR,&mut z,&pkE,&pkR);
	print!("Decapsulated z= "); printbinary(&z);

	hpke::keyschedule(config_id,&mut key,&mut nonce,&mut exp_secret,mode,&mut z,&info,None,None);
	print!("key= "); printbinary(&key);
	print!("nonce= "); printbinary(&nonce);
	print!("exporter secret= "); printbinary(&exp_secret);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key,&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

// Mode 1
    mode=1;
    println!("\nMode 1");
    decode_hex(&mut seedE,"bd12d21f5a36063ec0242aa26f22144aa7d24c3cc2e6a1805868250d8e9ce6fb2a04f8852cd6ec3241249fcf262b8f245d179636fcfbf275a75539cfc756a4dd3f53");
    decode_hex(&mut seedR,"e9a016952866c33da399e64ecd629cf32826285698726f2c22debd4799b8fac112ac1a546ce39452406396a389fb10f51036c7022dc1cdde177b54d005731fb79ce4");
    hpke::deriveKeyPair(config_id,&mut skE,&mut pkE,&seedE);
    hpke::deriveKeyPair(config_id,&mut skR,&mut pkR,&seedR);   
   
	hpke::encap(config_id,&skE,&mut z,&pkE,&pkR);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z);

	hpke::decap(config_id,&skR,&mut z,&pkE,&pkR);
	print!("Decapsulated z= "); printbinary(&z);

	hpke::keyschedule(config_id,&mut key,&mut nonce,&mut exp_secret,mode,&mut z,&info,Some(&psk),Some(&pskID));
	print!("key= "); printbinary(&key);
	print!("nonce= "); printbinary(&nonce);
	print!("exporter secret= "); printbinary(&exp_secret);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key,&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

// Mode 2
    mode=2;
    println!("\nMode 2");
    decode_hex(&mut seedE,"0e5ec115c172744e793ef333abfd5d9838fd31eef595f0b77a2f5623404aa42f0e77198c2ac78defeae75cc79cf5dc5c7587b6eda733e48f3b17d5e3496482f19cdb");
    decode_hex(&mut seedR,"3f3c12492eee6f1f8029a89002b6539ea9754b48412c14d2fe94368cf1fa473f0171cdb1117aaa447ad6914bfb794ead14a9cd22cd83caa22bd905bd7d614d401ddc");
    decode_hex(&mut seedS,"02d1581b120d0415bda1fb2cbe6a3c81edf28209f0fc1460745ad9018747c31e6d42a36ed1734214312311db80bb3f88c2da6ea7624cf2915d32f2993ebeb92c2934");

    hpke::deriveKeyPair(config_id,&mut skE,&mut pkE,&seedE);
    hpke::deriveKeyPair(config_id,&mut skR,&mut pkR,&seedR);   
    hpke::deriveKeyPair(config_id,&mut skS,&mut pkS,&seedS);   
    
	hpke::authencap(config_id,&skE,&skS,&mut z,&pkE,&pkR,&pkS);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z);

	hpke::authdecap(config_id,&skR,&mut z,&pkE,&pkR,&pkS);
	print!("Decapsulated z= "); printbinary(&z);

	hpke::keyschedule(config_id,&mut key,&mut nonce,&mut exp_secret,mode,&mut z,&info,None,None);
	print!("key= "); printbinary(&key);
	print!("nonce= "); printbinary(&nonce);
	print!("exporter secret= "); printbinary(&exp_secret);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key,&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

// Mode 3
    mode=3;
    println!("\nMode 3");
    decode_hex(&mut seedE,"c393a0a0cab3313f0526d72b8bef55b26a156493263fa1a653069ed5f2f3637f371b15704778c89a6e01c3fd958dfee5f85613e2d4b2eb17a152810690d8252df44b");
    decode_hex(&mut seedR,"e7fa07c9938a3d9d883d256abd7fb5777ed7c4195a8d244c0e682fd1a08fdcce58ab1aa7ba31b1fa120f03d9df438509a1e31992506dbc196ac4489079b3b64bea9b");
    decode_hex(&mut seedS,"403e8d5ff0155c71232a887f542df221c385956eaca3acceffdc4681b99c7e9613f90ba03b0c03c0c57a90b2dd31effc6278ab6dc059f3e5389dff6495bda12b4da4");

    hpke::deriveKeyPair(config_id,&mut skE,&mut pkE,&seedE);
    hpke::deriveKeyPair(config_id,&mut skR,&mut pkR,&seedR);   
    hpke::deriveKeyPair(config_id,&mut skS,&mut pkS,&seedS);   

	hpke::authencap(config_id,&skE,&skS,&mut z,&pkE,&pkR,&pkS);
	print!("pkE= "); printbinary(&pkE);
	print!("Encapsulated z= "); printbinary(&z);

	hpke::authdecap(config_id,&skR,&mut z,&pkE,&pkR,&pkS);
	print!("Decapsulated z= "); printbinary(&z);

	hpke::keyschedule(config_id,&mut key,&mut nonce,&mut exp_secret,mode,&mut z,&info,Some(&psk),Some(&pskID));
	print!("key= "); printbinary(&key);
	print!("nonce= "); printbinary(&nonce);
	print!("exporter secret= "); printbinary(&exp_secret);

	gcm::encrypt(&mut cipher[0..plain.len()],&mut tag,&key,&nonce,&aad,&plain);
	print!("Cipher= ");  printbinary(&cipher[0..plain.len()]);
	print!("Tag= ");  printbinary(&tag);

}

fn main() {

    hpke_c25519();
    hpke_nist521();
}