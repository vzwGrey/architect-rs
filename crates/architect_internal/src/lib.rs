#[macro_export]
macro_rules! rtl {
    ($($tt:tt)*) => {
        $crate::rtl_internal!(start => $($tt)*)
    };
}

#[macro_export]
macro_rules! rtl_internal {
    (start => $($tt:tt)*) => {{
        let mut architecture = Rtl::default();
        $crate::rtl_internal!(&mut architecture => $($tt)*);
        return architecture;
    }};

    ($architecture:expr => $self:ident . $signal_name:ident () = $value:expr ; $($rest:tt)*) => {
        // $architecture
        $architecture.assign($self.$signal_name(), $value);
        // $crate::rtl_internal!($($rest:tt)*);
    };
}

pub enum RtlExpression {
    LiteralValue { value: LogicValue },
}

pub enum RtlStatement {
    Assignment {
        signal_name: &'static str,
        value: RtlExpression,
    },
}

#[derive(Default)]
pub struct Rtl {
    statements: Vec<RtlStatement>,
}

impl Rtl {
    pub fn assign<T: LogicType>(
        &mut self,
        signal: Signal<T, OutputSignal>,
        value: impl Into<LogicValue>,
    ) {
        self.statements.push(RtlStatement::Assignment {
            signal_name: signal.name,
            value: RtlExpression::LiteralValue {
                value: value.into(),
            },
        })
    }
}

pub enum LogicValue {
    Low,
    High,
}

impl From<bool> for LogicValue {
    fn from(val: bool) -> Self {
        match val {
            false => LogicValue::Low,
            true => LogicValue::High,
        }
    }
}

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
    fn create() -> Self;
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

pub trait Architecture {
    fn elaborate(&self) -> Rtl;
}

pub fn translate_entity<M>() -> std::io::Result<()>
where
    M: Entity + Architecture,
{
    use std::io::Write;

    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    let entity = M::create();
    let (inputs, outputs) = (entity.inputs(), entity.outputs());

    // --- Entity ---
    writeln!(out, "library ieee;")?;
    writeln!(out, "use ieee.std_logic_1164.all;")?;
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

    let rtl = entity.elaborate();

    // --- Architecture ---
    writeln!(out, "architecture rtl of {} is", entity.name())?;
    writeln!(out, "begin")?;
    for statement in &rtl.statements {
        match statement {
            RtlStatement::Assignment { signal_name, value } => {
                let value = match value {
                    RtlExpression::LiteralValue {
                        value: LogicValue::Low,
                    } => "'0'",
                    RtlExpression::LiteralValue {
                        value: LogicValue::High,
                    } => "'1'",
                };
                writeln!(out, "\t{signal_name} <= {value};")?;
            }
        }
    }
    writeln!(out, "end rtl;")?;

    Ok(())
}
