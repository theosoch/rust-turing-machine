use std::{fmt::{Write}};

// 

pub enum BandDirection {
    RIGHT,
    LEFT
}

// 

pub struct BandState<'a> {
    cursor: isize,
    data: Vec<&'a str>
}

impl<'a> BandState<'a> {

    pub fn new(cursor: isize, data: Vec<&'a str>) -> Box<Self> {
        let instance = Box::new(Self {
            cursor,
            data
        });

        return instance;
    }

    // 

    pub fn cursor(&self) -> isize { self.cursor }
        
    pub fn data(&self) -> Vec<&'a str> { self.data.to_owned() }

    // 

    fn _to_string(&self, toprint: bool) -> &str {
        let mut data_string = String::new();
        let mut i = 0;
        for d in self.data() {
            if i == self.cursor() && toprint { data_string.write_str("\x1b[38;2;255;0;0m"); }
            data_string.write_str(d);
            if i == self.cursor() && toprint { data_string.write_str("\x1b[0m"); }

            i+=1;
        }

        let brackets_cc = "\x1b[38;2;64;64;64m";
        let key_cc = "\x1b[38;2;196;196;148m";
        let value_cc = "\x1b[38;2;196;148;196m";
        let reset_cc = "\x1b[0m";

        let result = format!("{brackets_cc}{{{reset_cc} {key_cc}cursor{reset_cc}: {value_cc}{}{reset_cc}, {key_cc}len{reset_cc}: {value_cc}{}{reset_cc}, {key_cc}data{reset_cc}: \"{}\" {brackets_cc}}}{reset_cc}", self.cursor(), self.data().len(), data_string);
        return Box::leak(result.into_boxed_str());
    }

    // 

    pub fn to_string(&self) -> &str {
        return self._to_string(false);
    }

    pub fn to_printable_string(&self) -> &str {
        return self._to_string(true);
    }

}

// 

#[derive(Clone)]
pub struct Band<'a> {
    cursor: isize,
    data: Vec<&'a str>
}

impl<'a> Band<'a> {

    pub const BLANK_SYMBOL: &'a str = "0";

    pub fn from(data: Vec<&'a str>) -> Box<Self> {
        let mut instance = Self::new();
        
        let mut i = 0;
        
        for symbol in data {
            instance.data.push(symbol);
            i+=1;
        }

        return instance;
    }

    // 

    pub fn new() -> Box<Self> {
        let instance = Box::new(Self {
            cursor: 0,
            data: Vec::new()
        });

        return instance;
    }

    // 

    pub fn getCursor(&self) -> isize { self.cursor }

    pub fn setCursor(&mut self, pos: isize) { self.cursor = pos; }
    
    // 

    pub fn moveCursor(&mut self, direction: BandDirection) -> Result<isize, &'static str> {
        match direction {
            BandDirection::LEFT => {
                if self.getCursor() < 1 {
                    return Err( "Cursor can't be moved out of the data." );
                }
                
                self.setCursor(self.getCursor() - 1);
            },
            BandDirection::RIGHT => {
                self.setCursor(self.getCursor() + 1);
                
                match self.readCursor() {
                    Some(symbol) => self.writeCursor(symbol)?,
                    None => return Err( "An error happened while moving cursor to the right." )
                }
            }
        }

        return Ok( self.getCursor() );
    }

    // 

    pub fn readAt(&self, pos: isize) -> Option<&'a str> {
        if pos > -1 {
            return Some( match self.data.get(pos as usize) {
                Some(&x) => x,
                None => Band::BLANK_SYMBOL
            } );
        }
        else {
            return None
        }
    }

    pub fn readCursor(&self) -> Option<&'a str> {
        return self.readAt(self.getCursor());
    }

    // 

    pub fn writeAt(&mut self, pos: isize, symbol: &'a str) -> Result<(), &'static str>{
        if pos < 0 {
            return Err( "Invalid position in data : must be a positive position." );
        }

        let mut i;
        let count_from_end = pos - ((self.data.len() as isize) - 1);
        if count_from_end >= 0 {
            if symbol != Band::BLANK_SYMBOL || self.cursor == pos {
                i = 0;
                while i < count_from_end {
                    self.data.push(Band::BLANK_SYMBOL);
                    i += 1;
                }

                self.data[pos as usize] = symbol;
            }
        }
        else {
            self.data[pos as usize] = symbol;

            i = (self.data.len() as isize )-1;
            while i > -1 && self.data[i as usize] == Band::BLANK_SYMBOL && self.cursor != i {
                self.data.pop();
                i += -1;
            }
        }

        return Ok( () );
    }

    pub fn writeCursor(&mut self, symbol: &'a str) -> Result<(), &'static str> {
        return self.writeAt(self.getCursor() + 0, symbol);
    }

    //

    pub fn state(&self) -> Option<Box<BandState>> {
        return Some(BandState::new(self.getCursor(), self.data.to_owned()));
    }

}