//!This crate contains two elements: trait <i>Strip</i> and function <i>mutual_strip</i> allowing strippig
//! a sequence from the both sides of string simultaneously

///Trait implementing a mutual_strip method for String and &str structures.
/// This method strips argument sequense from the both sides of the string
///```
/// use mutual_strip::MutualStrip;
/// let s = "AnnnnnA";
/// assert_eq!("nnnnn", s.mutual_strip("A"));
/// let s = String::from(s);
/// assert_eq!("nnnnn", s.mutual_strip("A"));
/// ```

pub trait MutualStrip {
    fn mutual_strip<'a>(self: &'a Self, p: &'a str) -> &'a str;
}

impl MutualStrip for String {
    fn mutual_strip<'a>(self: &'a Self, p: &'a str) -> &'a str {
        let s = self.strip_prefix(p).unwrap_or(self);
        s.strip_suffix(p).unwrap_or(s)
    }
}

impl MutualStrip for str {
    fn mutual_strip<'a>(self: &'a Self, p: &'a str) -> &'a str {
        let s = self.strip_prefix(p).unwrap_or(self);
        s.strip_suffix(p).unwrap_or(s)
    }
}

///Function striping the second argument sequence from the both sides of the string in the first argument
/// ```
/// use mutual_strip::mutual_strip;
/// let s = "AnnnA";
/// assert_eq!("nnn", mutual_strip(s, "A"));
/// let s = String::from(s);
/// assert_eq!("nnn", mutual_strip(&s, "A"));
/// ```
pub fn mutual_strip<'a>(s: &'a str, p: &'a str) -> &'a str {
    let s = s.strip_prefix(p).unwrap_or(s);
    s.strip_suffix(p).unwrap_or(s)
}
