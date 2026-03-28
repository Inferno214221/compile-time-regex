use std::fmt::Debug;

pub trait HaystackItem: Debug + Default + Copy + Eq + Ord {
    fn vec_from_str(value: &str) -> Vec<Self>;
}

pub trait HaystackSlice<'a>: Debug + Copy + Sized {
    type Item: HaystackItem;
}

impl HaystackItem for char {
    fn vec_from_str(value: &str) -> Vec<Self> {
        value.chars().collect()
    }
}

impl<'a> HaystackSlice<'a> for &'a str {
    type Item = char;
}

impl HaystackItem for u8 {
    fn vec_from_str(s: &str) -> Vec<Self> {
        s.as_bytes().to_vec()
    }
}

impl<'a> HaystackSlice<'a> for &'a [u8] {
    type Item = u8;
}