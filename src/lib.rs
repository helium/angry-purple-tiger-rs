use std::{fmt, str::FromStr};
mod words;

#[derive(Debug, PartialEq)]
pub struct AnimalName {
    adjective: &'static str,
    color: &'static str,
    animal: &'static str,
}

impl fmt::Display for AnimalName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}-{}-{}",
            self.adjective, self.color, self.animal
        ))
    }
}

impl PartialEq<&str> for AnimalName {
    fn eq(&self, other: &&str) -> bool {
        let mut split = (*other).split('-');
        for entry in &[self.adjective, self.color, self.animal] {
            // if we don't get a split fragment or the fragment isn't the entry
            // we're not equal
            if split.next().map_or_else(|| true, |s| &s != entry) {
                return false;
            }
        }
        true
    }
}

fn str_to_animal_name(s: &str) -> AnimalName {
    let digest = hex_digest(s);
    AnimalName {
        adjective: words::ADJECTIVES[digest[0]],
        color: words::COLORS[digest[1]],
        animal: words::ANIMALS[digest[2]],
    }
}

impl FromStr for AnimalName {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(str_to_animal_name(s))
    }
}
#[cfg(feature = "helium_crypto")]
impl From<helium_crypto::PublicKey> for AnimalName {
    fn from(pubkey: helium_crypto::PublicKey) -> Self {
        str_to_animal_name(&pubkey.to_string())
    }
}

fn hex_digest(s: &str) -> [usize; 3] {
    let digest = md5::compute(s);
    let mut result = [0usize; 3];
    compress(digest.0.len() / 3, &digest.0, &mut result);
    result
}

fn compress(size: usize, bytes: &[u8], dest: &mut [usize]) {
    if bytes.len() >= (2 * size) {
        dest[0] = bytes[0..size].iter().fold(0u8, |acc, b| acc ^ b) as usize;
        compress(size, &bytes[size..], &mut dest[1..])
    } else {
        dest[0] = bytes.iter().fold(0u8, |acc, b| acc ^ b) as usize;
    }
}

#[cfg(test)]
mod test {
    use super::AnimalName;

    #[test]
    fn basic() {
        let known = "112CuoXo7WCcp6GGwDNBo6H5nKXGH45UNJ39iEefdv2mwmnwdFt8";
        let animal_name = known.parse::<AnimalName>().expect("animal name");
        assert_eq!(animal_name, "feisty-glass-dalmatian")
    }

    #[test]
    #[cfg(feature = "helium-crypto")]
    fn from_public_key() {
        use std::str::FromStr;
        let known = helium_crypto::PublicKey::from_str(
            "112CuoXo7WCcp6GGwDNBo6H5nKXGH45UNJ39iEefdv2mwmnwdFt8",
        )
        .expect("public key");
        let animal_name: AnimalName = known.into();
        assert_eq!(animal_name, "feisty-glass-dalmatian")
    }
}
