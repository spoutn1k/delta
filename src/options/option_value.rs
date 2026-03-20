#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("Error converting OptionValue to bool.")]
    Bool,
    #[error("Error converting OptionValue to f64.")]
    Float,
    #[error("Error converting OptionValue to Option<String>.")]
    OptionString,
    #[error("Error converting OptionValue to String.")]
    String,
    #[error("Error converting OptionValue to usize.")]
    Int,
}

/// A value associated with a Delta command-line option name.
pub enum OptionValue {
    Boolean(bool),
    Float(f64),
    OptionString(Option<String>),
    String(String),
    Int(usize),
}

/// An OptionValue, tagged according to its provenance/semantics.
pub enum ProvenancedOptionValue {
    GitConfigValue(OptionValue),
    DefaultValue(OptionValue),
}

impl From<bool> for OptionValue {
    fn from(value: bool) -> Self {
        OptionValue::Boolean(value)
    }
}

impl TryFrom<OptionValue> for bool {
    type Error = ConversionError;

    fn try_from(value: OptionValue) -> Result<Self, Self::Error> {
        match value {
            OptionValue::Boolean(value) => Ok(value),
            _ => Err(ConversionError::Bool),
        }
    }
}

impl From<f64> for OptionValue {
    fn from(value: f64) -> Self {
        OptionValue::Float(value)
    }
}

impl TryFrom<OptionValue> for f64 {
    type Error = ConversionError;
    fn try_from(value: OptionValue) -> Result<Self, Self::Error> {
        match value {
            OptionValue::Float(value) => Ok(value),
            _ => Err(ConversionError::Float),
        }
    }
}

impl From<Option<String>> for OptionValue {
    fn from(value: Option<String>) -> Self {
        OptionValue::OptionString(value)
    }
}

impl TryFrom<OptionValue> for Option<String> {
    type Error = ConversionError;

    fn try_from(value: OptionValue) -> Result<Self, Self::Error> {
        match value {
            OptionValue::OptionString(value) => Ok(value),
            _ => Err(ConversionError::OptionString),
        }
    }
}

impl From<String> for OptionValue {
    fn from(value: String) -> Self {
        OptionValue::String(value)
    }
}

impl From<&str> for OptionValue {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl TryFrom<OptionValue> for String {
    type Error = ConversionError;

    fn try_from(value: OptionValue) -> Result<Self, Self::Error> {
        match value {
            OptionValue::String(value) => Ok(value),
            _ => Err(ConversionError::OptionString),
        }
    }
}

impl From<usize> for OptionValue {
    fn from(value: usize) -> Self {
        OptionValue::Int(value)
    }
}

impl TryFrom<OptionValue> for usize {
    type Error = ConversionError;

    fn try_from(value: OptionValue) -> Result<Self, Self::Error> {
        match value {
            OptionValue::Int(value) => Ok(value),
            _ => Err(ConversionError::OptionString),
        }
    }
}
