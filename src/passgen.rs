use anyhow::{Result, ensure};
use rand::Rng;
use rand::seq::{IndexedRandom, SliceRandom};

#[derive(Debug)]
pub struct PassGen<R>
where
    R: Rng,
{
    pub digits: u8,
    pub lowercase: u8,
    pub uppercase: u8,
    pub special: u8,
    pub rng: R,
}

impl<R> PassGen<R>
where
    R: Rng,
{
    const DIGITS: &[u8] = b"0123456789";
    const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const SPECIAL: &[u8] = br##"!"#$%&'()*+,-./:;<=>?@[\]^_`{|}~"##;

    pub fn generate(&mut self) -> String {
        let len = self.digits as usize
            + self.lowercase as usize
            + self.uppercase as usize
            + self.special as usize;
        let mut password = Vec::with_capacity(len);

        for _ in 0..self.digits {
            let range = 0..Self::DIGITS.len();
            let index = self.rng.random_range(range);
            let char = Self::DIGITS[index];
            password.push(char);
        }

        for _ in 0..self.lowercase {
            let range = 0..Self::LOWERCASE.len();
            let index = self.rng.random_range(range);
            let char = Self::LOWERCASE[index];
            password.push(char);
        }

        for _ in 0..self.uppercase {
            let range = 0..Self::UPPERCASE.len();
            let index = self.rng.random_range(range);
            let char = Self::UPPERCASE[index];
            password.push(char);
        }

        for _ in 0..self.special {
            let range = 0..Self::SPECIAL.len();
            let index = self.rng.random_range(range);
            let char = Self::SPECIAL[index];
            password.push(char);
        }

        password.shuffle(&mut self.rng);

        String::from_utf8(password).unwrap_or_else(|_| unreachable!())
    }

    pub fn generate_unique(&mut self) -> Result<String>
    where
        R: Rng,
    {
        let digits = self.digits as usize;
        let lowercase = self.lowercase as usize;
        let uppercase = self.uppercase as usize;
        let special = self.special as usize;

        ensure!(
            digits <= Self::DIGITS.len(),
            "number of unique digits exceeds available"
        );
        ensure!(
            lowercase <= Self::LOWERCASE.len(),
            "number of unique lowercase letters exceeds available"
        );
        ensure!(
            uppercase <= Self::UPPERCASE.len(),
            "number of unique uppercase letters exceeds available"
        );
        ensure!(
            special <= Self::SPECIAL.len(),
            "number of unique special symbols exceeds available"
        );

        let mut password: Vec<u8> = Self::DIGITS
            .choose_multiple(&mut self.rng, digits)
            .chain(Self::LOWERCASE.choose_multiple(&mut self.rng, lowercase))
            .chain(Self::UPPERCASE.choose_multiple(&mut self.rng, uppercase))
            .chain(Self::SPECIAL.choose_multiple(&mut self.rng, special))
            .copied()
            .collect();

        password.shuffle(&mut self.rng);

        String::from_utf8(password).map_err(|_| unreachable!())
    }
}
