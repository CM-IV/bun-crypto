#![deny(clippy::all)]

use std::{str::FromStr, io::{Write, Read}, iter};

use age::secrecy::ExposeSecret;
use napi::bindgen_prelude::Uint8Array;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn gen_secret_key() -> String {
  let secret_str = age::x25519::Identity::generate().to_string();

  secret_str.expose_secret().to_owned()
}

#[napi]
pub fn derive_pub_key(secret: String) -> String {
  let identity = age::x25519::Identity::from_str(&secret).unwrap();

  let pub_key = identity.to_public().to_string();

  pub_key
}

#[napi]
pub fn encrypt_file(file_data: Uint8Array, pub_str: String) -> Uint8Array {

  let pub_key = age::x25519::Recipient::from_str(&pub_str).unwrap();

  let encryptor = age::Encryptor::with_recipients(vec![Box::new(pub_key)])
  .expect("we provided a recipient");
  
  let mut encrypted = vec![];
  let mut writer = encryptor.wrap_output(&mut encrypted).unwrap();
  writer.write_all(&file_data).unwrap();
  writer.finish().unwrap();

  encrypted.as_slice().into()

}

#[napi]
pub fn decrypt_file(encrypted_data: Uint8Array, secret_key_str: String) -> Uint8Array {

  let secret_key = age::x25519::Identity::from_str(&secret_key_str).unwrap();

  let decryptor = match age::Decryptor::new(&encrypted_data[..]).unwrap() {
      age::Decryptor::Recipients(d) => d,
      _ => unreachable!(),
  };

  let mut decrypted = vec![];
  let mut reader = decryptor.decrypt(iter::once(&secret_key as &dyn age::Identity)).unwrap();
  reader.read_to_end(&mut decrypted).unwrap();

  decrypted.as_slice().into()
  
}

#[napi]
pub fn gen_file_hash(input_file_data: Uint8Array) -> String {

  let hash_bytes = hmac_sha512::Hash::hash(input_file_data);

  let hex_chars: Vec<String> = hash_bytes.iter().map(|byte| format!("{:02x}", byte)).collect();
  let hash_str = hex_chars.join("");

  hash_str
}