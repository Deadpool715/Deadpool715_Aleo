use crate::State;
use leo_input::{
    sections::{Header, Section},
    InputParserError,
};

#[derive(Clone, PartialEq, Eq)]
pub struct PublicState {
    state: State,
}

impl PublicState {
    pub fn new() -> Self {
        Self { state: State::new() }
    }

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private input variables.
    pub fn empty(&self) -> Self {
        let state = self.state.empty();

        Self { state }
    }

    pub fn len(&self) -> usize {
        if self.state.is_present() { 1usize } else { 0usize }
    }

    /// Parse all input variables included in a file and store them in `self`.
    pub fn parse(&mut self, sections: Vec<Section>) -> Result<(), InputParserError> {
        for section in sections {
            match section.header {
                Header::State(_state) => self.state.parse(section.definitions)?,
                header => return Err(InputParserError::public_section(header)),
            }
        }

        Ok(())
    }

    /// Returns the runtime state input values
    pub fn get_state(&self) -> &State {
        &self.state
    }
}