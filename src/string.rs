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
        Some(&self[self.find(index)?..self.len()])
    }

    fn between_str(&self, start: &str, end: &str) -> Option<&str> {
        let start = self.find(start)?;
        let after = &self[start..self.len()];
        let end = after.find(end)?;
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
        Some(&self[self.find(index)?..self.len()])
    }

    fn between_str(&self, start: &str, end: &str) -> Option<&str> {
        let start = self.find(start)?;
        let after = &self[start..self.len()];
        let end = after.find(end)?;
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