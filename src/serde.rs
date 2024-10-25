use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use super::Hash;

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(self.to_string().as_str())
        } else {
            self.as_bytes().serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;
            s.parse().map_err(de::Error::custom)
        } else {
            let data: [u8; 32] = Deserialize::deserialize(deserializer)?;
            Ok(Hash::from(data))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hash;

    #[test]
    fn test_hash_postcard() {
        let hash = hash(b"hello");
        let ser = postcard::to_stdvec(&hash).unwrap();
        let de: Hash = postcard::from_bytes(&ser).unwrap();
        assert_eq!(hash, de);

        assert_eq!(ser.len(), 32);
    }

    #[test]
    fn test_hash_json() {
        let hash = hash(b"hello");
        let ser = serde_json::to_string(&hash).unwrap();
        let de: Hash = serde_json::from_str(&ser).unwrap();
        assert_eq!(hash, de);
        // 64 bytes of hex + 2 quotes
        assert_eq!(ser.len(), 66);
    }
}
