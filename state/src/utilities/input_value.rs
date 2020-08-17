use crate::InputValueError;
use leo_typed::{InputValue, Parameter};

use std::collections::HashMap;

pub fn find_input(
    name: String,
    parameters: &HashMap<Parameter, Option<InputValue>>,
) -> Result<InputValue, InputValueError> {
    let matched_parameter = parameters
        .iter()
        .find(|(parameter, _value)| parameter.variable.name == name);

    match matched_parameter {
        Some((_parameter, value_option)) => match value_option {
            Some(value) => Ok(value.clone()),
            None => Err(InputValueError::MissingParameter(name)),
        },
        None => Err(InputValueError::MissingParameter(name)),
    }
}

pub fn input_to_integer_string(input: InputValue) -> Result<String, InputValueError> {
    match input {
        InputValue::Integer(_type, string) => Ok(string),
        value => Err(InputValueError::ExpectedInteger(value.to_string())),
    }
}

pub fn input_to_u8_vec(input: InputValue) -> Result<Vec<u8>, InputValueError> {
    let input_array = match input {
        InputValue::Array(values) => values,
        value => return Err(InputValueError::ExpectedBytes(value.to_string())),
    };

    let mut result_vec = vec![];

    for input in input_array {
        let integer_string = input_to_integer_string(input)?;
        let byte = integer_string.parse::<u8>()?;

        result_vec.push(byte);
    }

    Ok(result_vec)
}

pub fn input_to_nested_u8_vec(input: InputValue) -> Result<Vec<Vec<u8>>, InputValueError> {
    let inner_arrays = match input {
        InputValue::Array(arrays) => arrays,
        value => return Err(InputValueError::ExpectedBytes(value.to_string())),
    };

    let mut result_vec = vec![];

    for input_array in inner_arrays {
        let array = input_to_u8_vec(input_array)?;

        result_vec.push(array);
    }

    Ok(result_vec)
}