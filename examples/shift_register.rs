use architect::{
    translate_module, Architecture, Context, Logic, LogicVector, Module, ModuleInterface, Rtl,
};

#[derive(Default, Module)]
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
    let mut context = Context::default();
    translate_module::<ShiftRegister>(&mut context)
}
