use crate::AddVariableError;
use crate::PropertyType;
use crate::PropertyTypes;
use crate::RemoveVariableError;
use crate::UpdateVariableError;

pub type Variable = PropertyType;
pub type Variables = PropertyTypes;

pub type VariableNames = Vec<String>;

/// A type which contains variables.
pub trait VariablesContainer {
    /// Returns true, if the type contains an variables with the given variable_name.
    fn has_variable<S: Into<String>>(&self, variable_name: S) -> bool;

    /// Returns true, if the type contains any of the given variables.
    fn has_any_variables(&self, variable_names: &VariableNames) -> bool {
        variable_names.iter().any(|variable_name| self.has_variable(variable_name))
    }

    /// Returns true, if the type contains all of the given variables.
    fn has_all_variables(&self, variable_names: &VariableNames) -> bool {
        variable_names.iter().all(|variable_name| self.has_variable(variable_name))
    }

    /// Returns the variable with the given variable_name.
    fn get_variable<N: Into<String>>(&self, variable_name: N) -> Option<Variable>;

    /// Adds the given variable.
    fn add_variable<V: Into<Variable>>(&self, variable: V) -> Result<Variable, AddVariableError>;

    /// Updates the variable with the given variable_name.
    /// It's possible to rename the variable by using another name in the new variable than the provided variable_name.
    fn update_variable<N: Into<String>, V: Into<Variable>>(&self, variable_name: N, variable: V) -> Result<Variable, UpdateVariableError>;

    /// Removes the variable with the given variable_name.
    fn remove_variable<S: Into<String>>(&self, variable_name: S) -> Result<Variable, RemoveVariableError>;

    /// Merges the given variables with the existing variables.
    fn merge_variables<V: Into<Variables>>(&mut self, variables_to_merge: V);
}

/// Collection of a type which contains variables.
pub trait NamespacedTypeVariablesContainer<T, AddVariableError, UpdateVariableError, RemoveVariableError, MergeVariablesError> {
    /// Adds a variable to the given type.
    fn add_variable<V: Into<Variable>>(&self, ty: &T, variable: V) -> Result<Variable, AddVariableError>;

    /// Updates the variable with the given name of the given type.
    /// It's possible to rename the variable by using another name in the new variable than the provided variable_name.
    fn update_variable<N: Into<String>, V: Into<Variable>>(&self, ty: &T, variable_name: N, variable: V) -> Result<Variable, UpdateVariableError>;

    /// Remove the variable with the given name from the given type.
    fn remove_variable<N: Into<String>>(&self, ty: &T, variable_name: N) -> Result<Variable, RemoveVariableError>;

    /// Merges the given variables with the variables of the given type.
    fn merge_variables<V: Into<Variables>>(&mut self, ty: &T, variables_to_merge: V) -> Result<(), MergeVariablesError>;
}
