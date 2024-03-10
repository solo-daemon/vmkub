use super::KEY_LEN;
extern crate sha1;

use sha1::{Sha1, Digest};
use serde::{Deserialize, Serialize};

use std::fmt::{Binary, Debug, Error, Formatter};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub struct Key(pub [u8; KEY_LEN]);

impl Key {
    pub fn new(input: String) -> Self {
        let mut sha = Sha1::new();
        sha.update(input.as_bytes());

        // we know that the hash output is going to be 160 bits = 20 bytes
        let result = sha.finalize();
        let mut hash = [0; KEY_LEN];

        for i in 0..result.len() {
            hash[i] = result[i];
        }
        Self(hash)
    }

    pub fn to_string(&self) -> String {
        // Convert each byte to its hexadecimal representation
        let hex_string: String = self.0.iter().map(|byte| format!("{:02X}", byte)).collect();
        hex_string
    }
}


impl From<&Key> for String {
    fn from(key: &Key) -> Self {
        // Convert your Key to a String here
        // For example, assuming you have a method `to_string` on Key:
        key.to_string()
    }
}
impl Debug for Key {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for x in &self.0 {
            write!(f, "{:X}", x).expect("[FAILED] Key::Debug --> Failed to format contents of Key");
        }
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize, Hash, Ord, PartialOrd, Eq, PartialEq, Copy)]
pub struct Distance(pub [u8; KEY_LEN]);

impl Distance {
    pub fn new(k1: &Key, k2: &Key) -> Distance {
        let mut ret = [0; KEY_LEN];
        for i in 0..KEY_LEN {
            ret[i] = k1.0[i] ^ k2.0[i];
        }

        Self(ret)
    }
}

impl Debug for Distance {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for x in &self.0 {
            write!(f, "{:X}", x)
                .expect("[FAILED] Distance::Debug --> Failed to format contents of Key");
        }
        Ok(())
    }
}

impl Binary for Distance {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for x in &self.0 {
            write!(f, "{:b}", x)
                .expect("[FAILED] Key::Binary --> Failed to format contents of Distance");
        }
        Ok(())
    }
}
