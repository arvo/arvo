use super::*;

/// Create a `FunctionDecl` that represents a binary operator from the prelude
/// library.
///
/// # Arguments
/// * operator - The name of the binary operator.
/// * ty - The formal and return type of the binary operator.
/// * type_name - The name of the formal and return type.
///
/// # Return
/// An extern `FunctionDecl` with a profile that matches the respective binary
/// operator from the prelude library. The formals are immutable and owned.
#[macro_export]
macro_rules! decl_binary_operator {
    ($operator: expr, $ty: expr,  $type_expr: expr) => (
        FunctionDecl::new(
            true,
            vec![
                FunctionFormal::new("x", false, false, $ty),
                FunctionFormal::new("y", false, false, $ty),
            ],
            format!("__libprelude__{}_{}", $operator, $type_name),
            $ty,
            None,
        )
    )
}

#[macro_export]
macro_rules! decl_add_operator {
    ($ty: expr, $type_name: expr) => (decl_binary_operator!("add", $ty, $type_name))
}

#[macro_export]
macro_rules! decl_div_operator {
    ($ty: expr, $type_name: expr => (decl_binary_operator!("div", $ty, $type_name))
}

#[macro_export]
macro_rules! decl_mul_operator {
    ($ty: expr, $type_name: expr => (decl_binary_operator!("mul", $ty, $type_name))
}

#[macro_export]
macro_rules! decl_sub_operator {
    ($ty: expr, $type_name: expr => (decl_binary_operator!("sub", $ty, $type_name))
}

/// Create a `FunctionDecl` that represents the string concat operator from
/// the prelude library.
///
/// # Arguments
/// * ty - The second formal type of the stringconcat operator.
/// * type_name - The name of the second formal type.
///
/// # Return
/// An extern `Function` with a profile that matches the respective string 
/// concat operator from the prelude library.
#[macro_export]
macro_rules! concat_string_fn {
    ($type_name: expr,  $type_expr: expr) => (
        FunctionDecl::new(
            true,
            vec![
                FunctionFormal::new("x", false, false, UnresolvedType::string()),
                FunctionFormal::new("y", false, false, $ty),
            ],
            format!("__libprelude__concat_string_{}", $type_name),
            UnresolvedType::string(),
            None,
        )
    )
}

/// Create a `Function` that represents an extern write function.
///
/// # Arguments
/// * type_name - The name of the type that is used by the write function.
/// * type_expr - A path to a `Type`.
///
/// # Return
/// An extern `Function` with a profile that matches the write function in the
/// prelude library.
#[macro_export]
macro_rules! write_fn {
    ($type_name: expr,  $type_expr: expr) => (
        Function::new(
            // symbol of the function
            Symbol::new(format!("__libprelude__write_{}", $type_name)),
            // formals of the function
            vec![Variable::new(Symbol::new("x"), $type_expr)],
            // return type of the function
            Type::Primitive(PrimitiveType::Void),
            // definition of the function
            None
        )
    )
}

/// Create a `Function` that represents an extern writeln function.
///
/// # Arguments
/// * type_name - The name of the type that is used by the writeln function.
/// * type_expr - A path to a `Type`.
///
/// # Return
/// An extern `Function` with a profile that matches the writeln function in
/// the prelude library.
#[macro_export]
macro_rules! writeln_fn {
    ($type_name: expr,  $type_expr: expr) => (
        Function::new(
            // symbol of the function
            Symbol::new(format!("__libprelude__writeln_{}", $type_name)),
            // formals of the function
            vec![Variable::new(Symbol::new("x"), $type_expr)],
            // return type of the function
            Type::Primitive(PrimitiveType::Void),
            // definition of the function
            None
        )
    )
}