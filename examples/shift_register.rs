use architect::{entity, rtl, translate_entity, Architecture, Logic, LogicVector, Rtl};

#[entity]
struct ShiftRegister {
    #[input]
    _clk: Logic,
    #[input]
    _input: Logic,
    #[output]
    _state: LogicVector<7, 0>,
    #[output]
    output: Logic,
}

impl Architecture for ShiftRegister {
    fn elaborate(&self) -> architect::Rtl {
        rtl! {
            self.output() = true;
        }
    }
}

fn main() -> std::io::Result<()> {
    translate_entity::<ShiftRegister>()
}
