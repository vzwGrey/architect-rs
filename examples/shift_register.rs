use architect::{entity, translate_entity, Architecture, Context, Logic, LogicVector, Rtl};

#[entity]
struct ShiftRegister {
    #[input]
    clk: Logic,
    #[input]
    input: Logic,
    #[output]
    state: LogicVector<7, 0>,
    #[output]
    output: Logic,
}

impl Architecture for ShiftRegister {
    fn rtl(&self) -> architect::Rtl {
        Rtl
    }
}

fn main() -> std::io::Result<()> {
    let context = std::rc::Rc::new(Context::default());
    translate_entity::<ShiftRegister>(context)
}
