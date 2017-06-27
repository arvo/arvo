
/// Send a message to a `Decl`. An immutable message cannot result in the
/// modification of the `Decl`.
#[macro_export]
macro_rules! decl_proxy {
    ($decl: expr => $($method: ident ( $($arg: expr ),* )).+) => (match $decl {
        Decl::Function(ref decl) => decl.$($method($($arg),*)).+,
        Decl::Module(ref decl) => decl.$($method($($arg),*)).+,
        Decl::Type(ref decl) => decl.$($method($($arg),*)).+,
    });
}

/// Send a message to an `Expr`. An immutable message cannot result in the
/// modification of the `Expr`.
#[macro_export]
macro_rules! expr_proxy {
    ($expr: expr => $($method: ident ( $($arg: expr ),* )).+) => (match $expr {
        Expr::Block(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Call(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Def(ref expr) => expr.$($method($($arg),*)).+,
        Expr::For(ref expr) => expr.$($method($($arg),*)).+,
        Expr::If(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Item(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Literal(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Ref(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Struct(ref expr) => expr.$($method($($arg),*)).+,
        Expr::StructElement(ref expr) => expr.$($method($($arg),*)).+,
        Expr::Void(ref expr) => expr.$($method($($arg),*)).+,
    });
}

/// Send a message to an `Item`. An immutable message cannot result in the
/// modification of the `Item`.
#[macro_export]
macro_rules! item_proxy {
    ($item: expr => $($method: ident ( $($arg: expr ),* )).+) => (match $item {
        Item::Function(ref item) => item.$($method($($arg),*)).+,
        Item::Unresolved(ref item) => item.$($method($($arg),*)).+,
        Item::Variable(ref item) => item.$($method($($arg),*)).+,
    });
}

/// Send a message to a `Type`. An immutable message cannot result in the
/// modification of the `Type`.
#[macro_export]
macro_rules! type_proxy {
    ($ty: expr => $($method: ident ( $($arg: expr ),* )).+) => (match $ty {
        Type::Lambda(ref ty) => ty.$($method($($arg),*)).+,
        Type::Primitive(ref ty) => ty.$($method($($arg),*)).+,
        Type::Ref(ref ty) => ty.$($method($($arg),*)).+,
        Type::Struct(ref ty) => ty.$($method($($arg),*)).+,
        Type::Unresolved(ref ty) => ty.$($method($($arg),*)).+,
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
#[macro_export]
macro_rules! binary_fn {
    ($op_name: expr, $type_name: expr,  $type_expr: expr) => (
        Function::new(
            // symbol of the function
            Symbol::new(format!("__libprelude__{}_{}", $op_name, $type_name)),
            // formals of the function
            vec![Variable::new(Symbol::new("x"), $type_expr), Variable::new(Symbol::new("y"), $type_expr)],
            // return type of the function
            $type_expr,
            // definition of the function
            None
        )
    )
}

#[macro_export]
macro_rules! add_fn {
    ($type_name: expr, $type_expr: expr) => (binary_fn!("add", $type_name, $type_expr))
}

#[macro_export]
macro_rules! div_fn {
    ($type_name: expr, $type_expr: expr) => (binary_fn!("div", $type_name, $type_expr))
}

#[macro_export]
macro_rules! mul_fn {
    ($type_name: expr, $type_expr: expr) => (binary_fn!("mul", $type_name, $type_expr))
}

#[macro_export]
macro_rules! sub_fn {
    ($type_name: expr, $type_expr: expr) => (binary_fn!("sub", $type_name, $type_expr))
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
#[macro_export]
macro_rules! concat_string_fn {
    ($type_name: expr,  $type_expr: expr) => (
        Function::new(
            // symbol of the function
            Symbol::new(format!("__libprelude__concat_string_{}", $type_name)),
            // formals of the function
            vec![Variable::new(Symbol::new("x"), $type_expr)],
            // return type of the function
            Type::Primitive(PrimitiveType::Str),
            // definition of the function
            None
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