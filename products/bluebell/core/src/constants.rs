/// Enums used in various routines
pub enum TreeTraversalMode {
    /// Used when emit_... is invoked before children are visited
    Enter,
    /// Used when emit_... is invoked before children are visited  
    Exit,
}

#[derive(PartialEq, Eq)]
pub enum TraversalResult {
    /// Returned when the visitor should continue tree traversal
    Continue,
    /// Returned when the visitor should skip the children and exit traversal
    SkipChildren,
}

// Symbol constant: These are the constants we use when representing types and instances
// internally.

/// prefix for global variables and functions
pub const GLOBAL_VAR_PREFIX: &str = "@";

/// prefix for global variables and functions
pub const NAMESPACE_SEPARATOR: &str = "::";

/// marks the start of template parameters
pub const TEMPLATE_PARAMETERS_START: &str = "<";

/// marks the end of template parameters
pub const TEMPLATE_PARAMETERS_END: &str = ">";

/// separator of template parameters
pub const TEMPLATE_PARAMETERS_SEPARATOR: &str = ",";

/// marks the start of function arguments
pub const FUNCTION_ARGUMENTS_START: &str = "(";

/// marks the end of function arguments
pub const FUNCTION_ARGUMENTS_END: &str = ")";

/// separator of function arguments
pub const FUNCTION_ARGUMENTS_SEPARATOR: &str = ",";

/// separates function arguments and return type
pub const FUNCTION_RETURN_TYPE_SEPARATOR: &str = "->";

/// prefix for local variables
pub const LOCAL_VAR_PREFIX: &str = "%";
