mod turing;

use turing::machine::{Machine};

use crate::turing::band::{Band, BandDirection};

fn main() -> Result<(), &'static str>{
    let mut turing_machine = Machine::new(0);

    // 

    // Cette table d'action correspond à un programme qui double les 1 dans une écriture binaire. (exemple: 111 -> 1110111, on a doublé le nombre de 1 en séparant le duplicatat par un 0)

    turing_machine.setAction(0, "1", |state: i32, symbol: &str| { return ("0", 1, BandDirection::RIGHT); } );
    
    turing_machine.setAction(1, "1", |state: i32, symbol: &str| { return ("1", 1, BandDirection::RIGHT); } );
    turing_machine.setAction(1, "0", |state: i32, symbol: &str| { return ("0", 2, BandDirection::RIGHT); } );
    
    turing_machine.setAction(2, "1", |state: i32, symbol: &str| { return ("1", 2, BandDirection::RIGHT); } );
    turing_machine.setAction(2, "0", |state: i32, symbol: &str| { return ("1", 3, BandDirection::LEFT); } );
    
    turing_machine.setAction(3, "1", |state: i32, symbol: &str| { return ("1", 3, BandDirection::LEFT); } );
    turing_machine.setAction(3, "0", |state: i32, symbol: &str| { return ("0", 4, BandDirection::LEFT); } );
    
    turing_machine.setAction(4, "1", |state: i32, symbol: &str| { return ("1", 4, BandDirection::LEFT); } );
    turing_machine.setAction(4, "0", |state: i32, symbol: &str| { return ("1", 0, BandDirection::RIGHT); } );
    
    // 

    turing_machine.setBand(Band::from(vec!["1", "1"]));

    turing_machine.run(|instance, step: i32| {
        let band = instance.band();
        let _band_state = band.state();
    
        match _band_state {
            Some(band_state) => {
                println!("[{}] Result : {}", step+1, band_state.to_printable_string());
                println!()
            },
            None => ()
        }
    })?;

    return Ok( () );
}
