use std::collections::HashMap;

use super::band::{Band, BandDirection};

pub struct Machine<'a> {
    state: i32,
    step: i32,
    ended: bool,

    band: Box<Band<'static>>,

    actions: HashMap<(i32, &'a str), fn(state: i32, symbol: &str) -> (&str, i32, BandDirection)>,
}

impl<'a> Machine<'a> {
    pub fn new(state: i32) -> Box<Self> {
        let instance = Box::new(Self {
            state: state,
            step: 0,
            ended: false,

            band: Band::new(),

            actions: HashMap::new(),
        });

        return instance;
    }

    //

    pub fn state(&self) -> i32 {
        return self.state;
    }

    // 
    
    pub fn band(&self) -> Box<Band> { self.band.to_owned() }

    pub fn setBand(&mut self, band: Box<Band<'static>>) { self.band = band; }

    //

    pub fn ended(&self) -> bool {
        return self.ended;
    }

    //

    pub fn getAction(
        &self,
        state: i32,
        symbol: &'a str
    ) -> Option<fn(state: i32, symbol: &str) -> (&str, i32, BandDirection)> {
        match self.actions.get(&mut (state, symbol)) {
            Some(&x) => Some(x),
            None => None,
        }
    }

    pub fn removeAction(&mut self, state: i32, symbol: &'a str) {
        let &mut actionkey = &mut (state, symbol);

        if self.actions.contains_key(&actionkey) {
            self.actions.remove(&actionkey);
        }
    }

    pub fn setAction(
        &mut self,
        state: i32,
        symbol: &'a str,
        action: fn(state: i32, symbol: &str) -> (&str, i32, BandDirection),
    ) {
        self.removeAction(state, symbol);
        self.actions.insert((state, symbol), action);
    }

    //

    pub fn init(&mut self, state: i32, cursor: isize) {
        self.state = state;
        self.step = 0;
        self.band.setCursor(cursor);
        self.ended = false;
    }

    pub fn next(&mut self) -> Result<(), &'static str> {
        let _band_symbol = self.band.readCursor();

        match _band_symbol {
            Some(band_symbol) => {
                let _action = self.getAction(self.state, band_symbol);

                match _action {
                    Some(action) => {
                        let result = action(self.state, band_symbol);
        
                        self.band.writeCursor(result.0)?;
                        self.band.moveCursor(result.2)?;
        
                        self.state = result.1;
        
                        self.ended = false;
                    }
                    None => self.ended = true,
                }
            },
            None => ()
        }

        return Ok( () );
    }

    pub fn run(&mut self, step_callback: fn(instance: &Self, step: i32) -> ()) -> Result<(), &'static str> {
        step_callback(self, self.step);

        while !self.ended() {
            self.next()?;
            self.step += 1;
            step_callback(self, self.step);
        }

        return Ok( () );
    }
}
