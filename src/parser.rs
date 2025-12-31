

pub struct Parser<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src, 
            pos: 0
        }
    }
    
    pub fn parse(mut self) -> Result<(), String> {
        loop {
            
        }
        
        Ok(())
    }
}


pub fn parse(src: &str) -> Result<(), String> {
    Parser::new(&src).parse()
}