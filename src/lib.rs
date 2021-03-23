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

impl FromStr for AnimalName {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digest = hex_digest(s);
        Ok(Self {
            adjective: words::ADJECTIVES[digest[0]],
            color: words::COLORS[digest[1]],
            animal: words::ANIMALS[digest[2]],
        })
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

mod test {
    #[test]
    fn basic() {
        use super::AnimalName;
        let known = "112CuoXo7WCcp6GGwDNBo6H5nKXGH45UNJ39iEefdv2mwmnwdFt8";
        let animal_name = known.parse::<AnimalName>().expect("animal name");
        assert_eq!(animal_name, "feisty-glass-dalmatian")
    }
}
