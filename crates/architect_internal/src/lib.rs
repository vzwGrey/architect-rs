use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LogicTypeId(usize);

pub trait LogicType {
    fn name() -> String;
}

#[derive(Default)]
pub struct Logic {}

impl LogicType for Logic {
    fn name() -> String {
        "ieee.std_logic_1164.std_logic".to_owned()
    }
}

#[derive(Default)]
pub struct LogicVector<const HI: usize, const LO: usize> {}

impl<const HI: usize, const LO: usize> LogicType for LogicVector<HI, LO> {
    fn name() -> String {
        format!("ieee.std_logic_1164.std_logic_vector({HI} downto {LO})")
    }
}

#[derive(Default)]
pub struct Context {}

pub trait Entity {
    fn create(context: Rc<Context>) -> Self;
    fn name(&self) -> &'static str;
    fn inputs(&self) -> Vec<&'static str>;
    fn outputs(&self) -> Vec<&'static str>;
    fn get_type_name_for_signal(&self, name: &'static str) -> String;
}

pub struct InputSignal;
pub struct OutputSignal;

pub struct Signal<SignalType, InOut> {
    name: &'static str,
    _signal_type: std::marker::PhantomData<*const SignalType>,
    _in_out: std::marker::PhantomData<*const InOut>,
}

impl<SignalType, InOut> Signal<SignalType, InOut> {
    pub fn with_name(name: &'static str) -> Self {
        Self {
            name,
            _signal_type: ::std::marker::PhantomData,
            _in_out: ::std::marker::PhantomData,
        }
    }
}

pub struct Rtl;

pub trait Architecture {
    fn rtl(&self) -> Rtl;
}

pub fn translate_entity<M>(context: Rc<Context>) -> std::io::Result<()>
where
    M: Entity + Architecture,
{
    use std::io::Write;

    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    let entity = M::create(Rc::clone(&context));
    let (inputs, outputs) = (entity.inputs(), entity.outputs());

    // --- Entity ---
    writeln!(out, "library ieee;")?;
    writeln!(out)?;
    writeln!(out, "entity {} is", entity.name())?;
    writeln!(out, "\tport (")?;
    for (i, signal_name) in inputs.iter().enumerate() {
        let type_name = entity.get_type_name_for_signal(signal_name);
        write!(out, "\t\t{signal_name} : in {type_name}")?;

        if (i != inputs.len() - 1) || !outputs.is_empty() {
            writeln!(out, ";")?;
        } else {
            writeln!(out)?;
        }
    }
    for (i, signal_name) in outputs.iter().enumerate() {
        let type_name = entity.get_type_name_for_signal(signal_name);
        write!(out, "\t\t{signal_name} : out {type_name}")?;

        if i != outputs.len() - 1 {
            writeln!(out, ";")?;
        } else {
            writeln!(out)?;
        }
    }
    writeln!(out, "\t);")?;
    writeln!(out, "end {};", entity.name())?;
    writeln!(out)?;

    // --- Architecture ---
    writeln!(out, "architecture rtl of {} is", entity.name())?;
    writeln!(out, "begin")?;
    writeln!(out, "end rtl;")?;

    Ok(())
}
