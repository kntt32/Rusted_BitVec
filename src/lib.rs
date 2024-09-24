use std::convert::From;
use std::iter::Iterator;
use std::ops::*;
use std::fmt::{Display, Formatter, Error};

#[derive(Clone, Debug)]
pub struct BitVec {
    len: usize,
    vec: Vec<u64>,
}

pub struct BitVecIter {
    index: usize,
    inf_bit: BitVec,
}

impl BitVec {
    pub fn new() -> BitVec {
        BitVec {
            len: 0,
            vec: Vec::new(),
        }
    }

    pub fn resize(&mut self, len: usize) -> &mut Self {
        let max_index = self.vec.len() -1;
        self.vec[max_index] &= !(0xffff_ffff_ffff_ffff >> (self.len & 0x7f));
        self.vec.resize((len + 63) / 64, 0);
        self.len = len;

        self
    }

    pub const fn len(&self) -> usize {
        self.len
    }
}

impl BitVec {
    pub fn push(&mut self, value: bool) -> &mut Self {
        if self.len & 0x3f == 0 {
            self.vec.push(0);
        }

        self.len += 1;
        self.set(self.len - 1, value);

        self
    }

    pub fn pop(&mut self) -> Option<bool> {
        if self.len == 0 {
            None
        } else {
            let value = self.get(self.len - 1);
            self.len -= 1;

            if self.len & 0x7f == 0 {
                self.vec.pop();
            }

            Some(value)
        }
    }

    pub fn get(&self, index: usize) -> bool {
        if self.len <= index {
            panic!("out of range");
        }

        let global_index = index / 64;
        let local_index = index - global_index * 64;

        if self.vec[global_index] & (1 << local_index) != 0 {
            true
        } else {
            false
        }
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if self.len <= index {
            panic!("out of range");
        }

        let global_index = index / 64;
        let local_index = index - global_index * 64;

        if value {
            self.vec[global_index] |= 1 << local_index;
        } else {
            self.vec[global_index] &= !(1 << local_index);
        }
    }
}

impl BitVec {
    pub fn count_true(&self) -> usize {
        fn count_true_u64(mut value: u64) -> usize {
            for i in [(0x0101_0101_0101_0101, 0), (0x0000_0000_0000_00ff, 3)] {
                let mut sum: u64 = 0;
                for k in 0..8 {
                    sum += (value >> (k << i.1)) & i.0;
                }
                value = sum;
            }
            value as usize
        }

        let mut count: usize = 0;

        for i in 0..self.vec.len() - 1 {
            count += count_true_u64(self.vec[i]);
        }
        count += count_true_u64(shl_nopanic(
            self.vec[self.vec.len() - 1],
            self.vec.len() * 64 - self.len,
        ));

        count
    }
}

impl Iterator for BitVecIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inf_bit.len == self.index {
            None
        } else {
            self.index += 1;
            Some(self.inf_bit.get(self.index - 1))
        }
    }
}

impl IntoIterator for BitVec {
    type Item = bool;
    type IntoIter = BitVecIter;

    fn into_iter(self) -> Self::IntoIter {
        BitVecIter {
            inf_bit: self,
            index: 0,
        }
    }
}

impl BitAnd<&BitVec> for BitVec {
    type Output = BitVec;

    fn bitand(mut self, rhs: &BitVec) -> Self::Output {
        self &= rhs;
        self
    }
}

impl BitAndAssign<&BitVec> for BitVec {
    fn bitand_assign(&mut self, rhs: &BitVec) {
        if rhs.len != self.len {
            panic!("Invalid Input");
        }

        for i in 0..self.vec.len() {
            self.vec[i] &= rhs.vec[i];
        }
    }
}

impl BitOr<&BitVec> for BitVec {
    type Output = BitVec;

    fn bitor(mut self, rhs: &BitVec) -> Self::Output {
        self |= rhs;
        self
    }
}

impl BitOrAssign<&BitVec> for BitVec {
    fn bitor_assign(&mut self, rhs: &BitVec) {
        if rhs.len != self.len {
            panic!("Invalid Input");
        }

        for i in 0..self.vec.len() {
            self.vec[i] |= rhs.vec[i];
        }
    }
}

impl BitXor<&BitVec> for BitVec {
    type Output = BitVec;

    fn bitxor(mut self, rhs: &BitVec) -> Self::Output {
        self ^= rhs;
        self
    }
}

impl BitXorAssign<&BitVec> for BitVec {
    fn bitxor_assign(&mut self, rhs: &BitVec) {
        if rhs.len != self.len {
            panic!("Invalid Input");
        }

        for i in 0..self.vec.len() {
            self.vec[i] ^= rhs.vec[i];
        }
    }
}

impl Not for BitVec {
    type Output = BitVec;

    fn not(mut self) -> Self::Output {
        for i in 0..self.vec.len() {
            self.vec[i] = !self.vec[i];
        }
        self
    }
}

impl From<&[bool]> for BitVec {
    fn from(value: &[bool]) -> Self {
        let mut inf_bit = BitVec::new();

        for i in value {
            inf_bit.push(*i);
        }

        inf_bit
    }
}

impl<const N: usize> From<&[bool; N]> for BitVec {
    fn from(value: &[bool; N]) -> Self {
        BitVec::from(&value[..])
    }
}

impl Display for BitVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[")?;
        
        for i in 0 .. self.len {
            if i != 0 {
                write!(f, ", ")?;
            }
            if self.get(i) {
                write!(f, "{}", 1)?;
            }else {
                write!(f, "{}", 0)?;
            }
            
        }

        write!(f, "]")?;

        Ok(())
    }
}

const fn shl_nopanic(value: u64, rhs: usize) -> u64 {
    let mask = 0xffff_ffff_ffff_ffff >> rhs;
    (value & mask) << rhs
}

