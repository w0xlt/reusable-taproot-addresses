# Reusable Taproot Addresses

## Documentation

This work is still in progress.

Currently it implements the steps described in [Inital Transaction](https://gist.github.com/Kixunil/0ddb3a9cdec33342b97431e438252c0a?permalink_comment_id=4081736#initial-transaction) section of "Reusable taproot addresses" document.

1) The first step is to calculate the `change address` of the sender and the `relationship_seed` that the sender will share with the receiver.

```rust
pub fn sender::generate_change_output_script(sender_secret_key: &SecretKey, receiver_pub_key: &PublicKey) -> (PublicKey, [u8; 32]);
```

The `sender_secret_key` must be the private key associated with the input with the lowest index, belonging to the sender.

The `receiver_pub_key` must be the public key of the Taproot address used by the receiver.

This function returns a tuple. The first element is the `PublicKey` of the change address and the second is the 32-byte relationship seed (`[u8; 32]`).


2) The sender can now calculate the addresses that can be used to send funds to the recipient. At the moment, the function generates the first 10 addresses, but it will be parameterized or adapted to the BIP 32 scheme.

```rust
pub fn sender::generate_public_keys_to_send(relationship_seed: &[u8; 32], receiver_pub_key: &PublicKey) -> Vec<PublicKey>;
```

The `relationship_seed` was calculated in the first step and must be used to derive the public keys.

The `receiver_pub_key` must be the public key of the Taproot address used by the receiver.

The function returns a list of public keys (`Vec<PublicKey>`) that will be used to send coins.

3) Likewise, the receiver can calculate the same addresses, watches them and react to the received transaction.

```rust
pub fn receiver::generate_public_keys_to_watch(P_change_sender: &PublicKey, sender_pub_key: &PublicKey, receiver_secret_key: &SecretKey) -> Vec<PublicKey>;
```

The `P_change_sender` must be the public key of the `change address` of the sender calculated in the fist step (the first element of the tuple returned by the `sender::generate_change_output_script` function).

The `sender_pub_key` must be the public key associated with the input with the lowest index, belonging to the sender.

The `receiver_secret_key` must be the private key of the Taproot address used by the receiver.

The function returns a list of public keys (`Vec<PublicKey>`) that will be used to receive coins.

| :exclamation:  The addresses calculated in `sender::generate_public_keys_to_send` and `receiver::generate_public_keys_to_watch` must  be the same  |
|-----------------------------------------------------------------------------------------------------------------------------------------------------|

Next steps:

* [ ] Implement the [Recovery](https://gist.github.com/Kixunil/0ddb3a9cdec33342b97431e438252c0a?permalink_comment_id=4081736#recovery) section
* [ ] Or implement a way to make this descriptor compatible so that it can be used out of box with the current version of Bitcoin Core, as suggested in [this comment](https://gist.github.com/Kixunil/0ddb3a9cdec33342b97431e438252c0a?permalink_comment_id=4013170#gistcomment-4013170).


## Example App

Create a new app `cargo new <app-name>`.

Add the code below to the corresponding files and run `cargo run`.

`Cargo.toml`
```yaml
[package]
name = "<app-name>"
version = "0.1.0"
edition = "2021"

[dependencies]
reusable_taproot_addresses = { git = "https://github.com/w0xlt/reusable-taproot-addresses.git" }
secp256k1 = { git = "https://github.com/rust-bitcoin/rust-secp256k1.git", rev = "39e47fb64", features = [ "rand-std", "bitcoin_hashes", "std" ] }
bech32 = "0.8.1"
```

`main.rs`
```rust
use bech32::{ToBase32, Variant};
use reusable_taproot_addresses::{sender, receiver};
use secp256k1::{Secp256k1, KeyPair, SecretKey, PublicKey};


fn main() {
    #![allow(non_snake_case)]

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

    let sender_change_address = bech32::encode("bc", P_change_sender.serialize().to_base32(), Variant::Bech32m).unwrap();
    println!("Change address that will be used by the sender:\n{}\n---", sender_change_address);

    let sender_addresses = sender::generate_public_keys_to_send(
        &relationship_seed,
        &receiver_pub_key);

    let receive_addresses = receiver::generate_public_keys_to_watch(
        &P_change_sender,
        &sender_pub_key,
        &receiver_secret_key);

    println!("Addresses calculated by the sender. Must match those calculated by the receiver.");

    for sa in sender_addresses {
        let encoded = bech32::encode("bc", sa.serialize().to_base32(), Variant::Bech32m).unwrap();
        println!("{}: {}", sa, encoded);
    }

    println!("---");

    println!("Addresses calculated by the receiver.");

    for ra in receive_addresses {
        let encoded = bech32::encode("bc", ra.serialize().to_base32(), Variant::Bech32m).unwrap();
        println!("{}: {}", ra, encoded);
    }

    println!("---");
}
```
