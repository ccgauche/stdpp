/*
Implement StringIndex
*/

pub trait StringIndex {
    fn get_byte(&self, index: usize) -> Option<&u8>;
    fn get_char(&self, index: usize) -> Option<char>;
}

impl StringIndex for String {

    fn get_byte(&self, index: usize) -> Option<&u8> {
        self.as_bytes().get(index)
    }
    fn get_char(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
}

impl StringIndex for &str {

    fn get_byte(&self, index: usize) -> Option<&u8> {
        self.as_bytes().get(index)
    }
    fn get_char(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }
}

/*
As char array
*/
pub trait AsCharArray {

    fn as_chars(&self) -> Vec<char>;

}

impl AsCharArray for &str {

    fn as_chars(&self) -> Vec<char> {
        self.chars().collect()
    }
}

impl AsCharArray for String {

    fn as_chars(&self) -> Vec<char> {
        self.chars().collect()
    }
}

/*
StringCut
*/

pub trait StringCut {

    fn before(&self, index: usize) -> Option<&str>;
    fn after(&self, index: usize) -> Option<&str>;
    fn between(&self, start: usize, end: usize) -> Option<&str>;
    
    fn before_str(&self, index: &str) -> Option<&str>;
    fn after_str(&self, index: &str) -> Option<&str>;
    fn between_str(&self, start: &str, end: &str) -> Option<&str>;
}

impl StringCut for String{

    fn before_str(&self, index: &str) -> Option<&str> {
        Some(&self[0..self.find(index)?])
    }

    fn after_str(&self, index: &str) -> Option<&str> {
        let start = self.find(index)? + index.len();
        let end = self.len();
        if start < end {
            Some(&self[start..end])
        } else {
            None
        }
    }

    fn between_str(&self, start_str: &str, end_str: &str) -> Option<&str> {
        let start = self.find(start_str)? + start_str.len();
        let len = self.len();
        if len <= start {
            return None;
        }
        let after = &self[(start + start_str.len())..len];
        let end = after.find(end_str)?;
        Some(&self[start..end])
    }

    fn before(&self, index: usize) -> Option<&str> {
        if self.len() > index {
            Some(&self[0..index])
        } else {
            None
        }
    }
    fn after(&self, index: usize) -> Option<&str> {
        let len = self.len();
        if len > index {
            Some(&self[index..len])
        } else {
            None
        }
    }
    fn between(&self, start: usize, end: usize) -> Option<&str> {
        let len = self.len();
        if len > end && end > start  {
            Some(&self[start..end])
        } else {
            None
        }
    }
}

impl StringCut for &str{

    fn before_str(&self, index: &str) -> Option<&str> {
        Some(&self[0..self.find(index)?])
    }

    fn after_str(&self, index: &str) -> Option<&str> {
        let start = self.find(index)? + index.len();
        let end = self.len();
        if start < end {
            Some(&self[start..end])
        } else {
            None
        }
    }

    fn between_str(&self, start_str: &str, end_str: &str) -> Option<&str> {
        let start = self.find(start_str)? + start_str.len();
        let len = self.len();
        if len <= start {
            return None;
        }
        let after = &self[(start + start_str.len())..len];
        let end = after.find(end_str)?;
        Some(&self[start..end])
    }

    fn before(&self, index: usize) -> Option<&str> {
        if self.len() > index {
            Some(&self[0..index])
        } else {
            None
        }
    }
    fn after(&self, index: usize) -> Option<&str> {
        let len = self.len();
        if len > index {
            Some(&self[index..len])
        } else {
            None
        }
    }
    fn between(&self, start: usize, end: usize) -> Option<&str> {
        let len = self.len();
        if len > end && end > start  {
            Some(&self[start..end])
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_char() {
        assert_eq!("rollerisation".get_char(2), Some('l'));
        assert_eq!("rollerisation".get_char(20), None);
        assert_eq!("rollerisation".get_char(0), Some('r'));
    }

    #[test]
    fn test_get_byte() {
        assert_eq!("rollerisation".get_byte(2), Some(&('l' as u8)));
        assert_eq!("rollerisation".get_byte(20), None);
        assert_eq!("rollerisation".get_byte(0), Some(&('r' as u8)));
    }

    #[test]
    fn test_before() {
        assert_eq!("rollerisation".before(20), None);
        assert_eq!("rollerisation".before(6), Some("roller"));
    }

    #[test]
    fn test_after() {
        assert_eq!("rollerisation".after(20), None);
        assert_eq!("rollerisation".after(6), Some("isation"));
    }

    #[test]
    fn test_between() {
        assert_eq!("rollerisation".between(0,20), None);
        assert_eq!("rollerisation".between(0,6), Some("roller"));
        assert_eq!("rollerisation".between(7,6), None);
        assert_eq!("rollerisation".between(6,6), None);
        assert_eq!("rollerisation".between(2,6), Some("ller"));
    }

    #[test]
    fn test_before_str() {
        assert_eq!("rollerisation".before_str("Roll"), None);
        assert_eq!("rollerisation".before_str("risa"), Some("rolle"));
    }

    #[test]
    fn test_after_str() {
        assert_eq!("rollerisation".after_str("Roll"), None);
        assert_eq!("rollerisation".after_str("risa"), Some("tion"));
    }
}