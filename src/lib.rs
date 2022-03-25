// 2022 Reusable Taproot Address
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Reusable taproot addresses
//! This library implements the scheme suggested in the `Reusable taproot address` document
//! https://gist.github.com/Kixunil/0ddb3a9cdec33342b97431e438252c0a

pub mod constants;
pub mod receiver;
pub mod sender;

#[test]
fn test_receiver_sender_same_destinations() {
    #![allow(non_snake_case)]
    use std::cmp::Ordering;

    use secp256k1::{Secp256k1, KeyPair, SecretKey, PublicKey};

    let secp = Secp256k1::new();

    let sender_key_pair = KeyPair::new(&secp, &mut secp256k1::rand::thread_rng());
    let sender_secret_key = SecretKey::from_keypair(&sender_key_pair);
    let sender_pub_key = PublicKey::from_keypair(&sender_key_pair);

    let receiver_key_pair = KeyPair::new(&secp, &mut secp256k1::rand::thread_rng());
    let receiver_secret_key = SecretKey::from_keypair(&receiver_key_pair);
    let receiver_pub_key = PublicKey::from_keypair(&receiver_key_pair);

    let (P_change_sender, relationship_seed) = sender::generate_change_output_script(
        &sender_secret_key,
        &receiver_pub_key);

    let master_xpub = sender::generate_master_extended_public_key(
        &relationship_seed,
        &receiver_pub_key);

    let master_xpriv = receiver::generate_master_extended_private_key(
        &P_change_sender,
        &sender_pub_key,
        &receiver_secret_key);

    let receiver_master_xpub = PublicKey::from_secret_key(&secp, &master_xpriv);
    assert_eq!(receiver_master_xpub.cmp(&master_xpub), Ordering::Equal);
}
