
/// Send a message to an `Item`. An immutable message cannot result in the
/// modification of the `Item`.
macro_rules! expr_proxy {
    ($expr: expr => $($method: ident ( $($arg: expr ),* )).+) => (match $expr {
        Expr::Block(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Call(ref expr) => expr.$($method($($arg),*)).+,
        _ => unimplemented!()
    });
}

/// Send a message to an `Item`. An immutable message cannot result in the
/// modification of the `Item`.
macro_rules! item_proxy {
    ($item: expr => $($method: ident ( $($arg: expr ),* )).+) => (match $item {
        Item::Function(ref item) => item.$($method($($arg),*)).+,
        Item::Variable(ref item) => item.$($method($($arg),*)).+,
    });
}

/// Create a `Function` that represents an extern binary operator.
///
/// # Arguments
/// * op_name - The name of the binary operator.
/// * type_name - The name of the type that is used by the binary operator.
/// * type_expr - A path to a `Type`.
///
/// # Return
/// An extern `Function` with a profile that matches the binary operator in
/// the prelude library.
macro_rules! bin_fn {
    ($op_name: expr, $type_name: expr,  $type_expr: expr) => (
        Function::new(
            // symbol of the function
            Symbol::new(format!("__libprelude__{}_{}", $op_name, $type_name)),
            // formals of the function
            vec![Variable::new(Symbol::new("x"), $type_expr), Variable::new(Symbol::new("y"), $type_expr)],
            // type profile of the function
            $type_expr,
            // definition of the function
            None,
        )
    )
}

macro_rules! add_fn {
    ($type_name: expr, $type_expr: expr) => (bin_fn!("add", $type_name, $type_expr))
}

macro_rules! div_fn {
    ($type_name: expr, $type_expr: expr) => (bin_fn!("div", $type_name, $type_expr))
}

macro_rules! mul_fn {
    ($type_name: expr, $type_expr: expr) => (bin_fn!("mul", $type_name, $type_expr))
}

macro_rules! sub_fn {
    ($type_name: expr, $type_expr: expr) => (bin_fn!("sub", $type_name, $type_expr))
}

/// Create a `Function` that represents an extern concat operator for strings.
///
/// # Arguments
/// * type_name - The name of the type that is used by the concat operator.
/// * type_expr - A path to a `Type`.
///
/// # Return
/// An extern `Function` with a profile that matches the concat operator for
/// strings in  the prelude library.
macro_rules! concat_string_fn {
    ($type_name: expr,  $type_expr: expr) => (
        Function::new(
            // symbol of the function
            Symbol::new(format!("__libprelude__concat_string_{}", $type_name)),
            // formals of the function
            vec![Variable::new(Symbol::new("x"), $type_expr)],
            // type profile of the function
            PrimitiveType::Str.into(),
            // definition of the function
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
macro_rules! write_fn {
    ($type_name: expr,  $type_expr: expr) => (
        Function::new(
            // symbol of the function
            Symbol::new(format!("__libprelude__write_{}", $type_name)),
            // formals of the function
            vec![Variable::new(Symbol::new("x"), $type_expr)],
            // type profile of the function
            PrimitiveType::Void.into(),
            // definition of the function
            None,
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
macro_rules! writeln_fn {
    ($type_name: expr,  $type_expr: expr) => (
        Function::new(
            // symbol of the function
            Symbol::new(format!("__libprelude__writeln_{}", $type_name)),
            // formals of the function
            vec![Variable::new(Symbol::new("x"), $type_expr)],
            // type profile of the function
            PrimitiveType::Void.into(),
            // definition of the function
            None,
        )
    )
}