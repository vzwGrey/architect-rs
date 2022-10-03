use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LogicTypeId(usize);

pub trait LogicType {
    fn get_type(&self, context: &mut Context) -> LogicTypeId;
}

#[derive(Default)]
pub struct Logic {}

impl LogicType for Logic {
    fn get_type(&self, context: &mut Context) -> LogicTypeId {
        context.get_type_id("ieee.std_logic_1164.std_logic", None)
    }
}

#[derive(Default)]
pub struct LogicVector<const HI: usize, const LO: usize> {}

impl<const HI: usize, const LO: usize> LogicType for LogicVector<HI, LO> {
    fn get_type(&self, context: &mut Context) -> LogicTypeId {
        context.get_type_id("ieee.std_logic_1164.std_logic_vector", Some((HI, LO)))
    }
}

#[derive(Default)]
pub struct Context {
    type_db: HashMap<(&'static str, Option<(usize, usize)>), LogicTypeId>,
}

impl Context {
    fn get_type_id(
        &mut self,
        type_name: &'static str,
        range: Option<(usize, usize)>,
    ) -> LogicTypeId {
        let next_type_id = self.type_db.len();
        *self
            .type_db
            .entry((type_name, range))
            .or_insert(LogicTypeId(next_type_id))
    }

    fn get_type_from_id(&self, type_id: LogicTypeId) -> (&'static str, Option<(usize, usize)>) {
        *self
            .type_db
            .iter()
            .find(|(_, id)| **id == type_id)
            .expect("invalid type id")
            .0
    }
}

pub struct ModuleInterface {
    pub name: &'static str,
    pub inputs: Vec<(&'static str, LogicTypeId)>,
    pub outputs: Vec<(&'static str, LogicTypeId)>,
}

pub trait Module {
    fn interface(&self, context: &mut Context) -> ModuleInterface;
}

pub struct Rtl;

pub trait Architecture {
    fn rtl(&self) -> Rtl;
}

pub fn translate_module<M>(context: &mut Context) -> std::io::Result<()>
where
    M: Default + Module + Architecture,
{
    use std::io::Write;

    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    let module = M::default();
    let module_interface = module.interface(context);

    // --- Entity ---
    writeln!(out, "library ieee;")?;
    writeln!(out)?;
    writeln!(out, "entity {} is", module_interface.name)?;
    writeln!(out, "\tport (")?;
    for (i, (name, type_id)) in module_interface.inputs.iter().enumerate() {
        let (type_name, range) = context.get_type_from_id(*type_id);
        write!(out, "\t\t{} : in {}", name, type_name)?;
        if let Some((hi, lo)) = range {
            write!(out, "({hi} downto {lo})")?;
        }

        if (i != module_interface.inputs.len() - 1) || !module_interface.outputs.is_empty() {
            writeln!(out, ";")?;
        } else {
            writeln!(out)?;
        }
    }
    for (i, (name, type_id)) in module_interface.outputs.iter().enumerate() {
        let (type_name, range) = context.get_type_from_id(*type_id);
        write!(out, "\t\t{} : out {}", name, type_name)?;
        if let Some((hi, lo)) = range {
            write!(out, "({hi} downto {lo})")?;
        }

        if i != module_interface.inputs.len() - 1 {
            writeln!(out, ";")?;
        } else {
            writeln!(out)?;
        }
    }
    writeln!(out, "\t);")?;
    writeln!(out, "end {};", module_interface.name)?;
    writeln!(out)?;

    // --- Architecture ---
    writeln!(out, "architecture rtl of {} is", module_interface.name)?;
    writeln!(out, "begin")?;
    writeln!(out, "end rtl;")?;

    Ok(())
}
