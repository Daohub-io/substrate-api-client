/*
    Copyright 2019 Supercomputing Systems AG
    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

use codec::{Decode, Encode};
use keyring::AccountKeyring;

use primitives::crypto::Pair;
use primitives::sr25519;

use substrate_api_client::{
    Api,
    compose_extrinsic,
    extrinsic,
    extrinsic::xt_primitives::UncheckedExtrinsicV3,
    utils::{hexstr_to_u64, hexstr_to_vec}
};

#[derive(Encode, Decode, Debug)]
struct Kitty {
    id: [u8; 32],
    price: u128,
}

fn main() {
    let url = "127.0.0.1:9944";

    let signer = AccountKeyring::Alice.pair();

    let api = Api::new(format!("ws://{}", url))
        .set_signer(signer.clone());

    let xt: UncheckedExtrinsicV3<_, sr25519::Pair> = compose_extrinsic!(
        api.clone(),
        "KittyModule",
        "create_kitty",
        10 as u128
    );

    println!("[+] Extrinsic: {:?}\n", xt);

    let tx_hash = api.send_extrinsic(xt.hex_encode()).unwrap();
    println!("[+] Transaction got finalized. Hash: {:?}\n", tx_hash);

    // get the index at which Alice's Kitty resides. Alternatively, we could listen to the StoredKitty
    // event similar to what we do in the example_contract.
    let res_str = api.get_storage("Kitty",
                                  "KittyIndex",
                                  Some(signer.public().encode())).unwrap();

    let index = hexstr_to_u64(res_str).unwrap();
    println!("[+] Alice's Kitty is at index : {}\n", index);

    // get the Kitty
    let res_str = api.get_storage("Kitty",
                                  "Kitties",
                                  Some(index.encode())).unwrap();

    let res_vec = hexstr_to_vec(res_str).unwrap();

    // type annotations are needed here to know that to decode into.
    let kitty: Kitty = Decode::decode(&mut res_vec.as_slice()).unwrap();
    println!("[+] Cute decoded Kitty: {:?}\n", kitty);
}
