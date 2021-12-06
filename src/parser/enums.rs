use std::fmt;


#[derive(Debug)]
pub enum Statement {

}


impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}