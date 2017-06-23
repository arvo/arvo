use super::*;

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
            // definition of the function
            None,
            // type profile of the function
            LambdaType::new(vec![$type_expr, $type_expr], $type_expr)))
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
            // definition of the function
            None,
            // type profile of the function
            LambdaType::new(vec![$type_expr], Type::Primitive(PrimitiveType::Str))))
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
            // definition of the function
            None,
            // type profile of the function
            LambdaType::new(vec![$type_expr], Type::Primitive(PrimitiveType::Void))))
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
            // definition of the function
            None,
            // type profile of the function
            LambdaType::new(vec![$type_expr], Type::Primitive(PrimitiveType::Void))))
}

pub struct Prelude {
    add_f32_fn: Function,
    add_f64_fn: Function,
    add_i8_fn: Function,
    add_i16_fn: Function,
    add_i32_fn: Function,
    add_i64_fn: Function,
    add_u8_fn: Function,
    add_u16_fn: Function,
    add_u32_fn: Function,
    add_u64_fn: Function,
    add_usize_fn: Function,

    div_f32_fn: Function,
    div_f64_fn: Function,
    div_i8_fn: Function,
    div_i16_fn: Function,
    div_i32_fn: Function,
    div_i64_fn: Function,
    div_u8_fn: Function,
    div_u16_fn: Function,
    div_u32_fn: Function,
    div_u64_fn: Function,
    div_usize_fn: Function,

    mul_f32_fn: Function,
    mul_f64_fn: Function,
    mul_i8_fn: Function,
    mul_i16_fn: Function,
    mul_i32_fn: Function,
    mul_i64_fn: Function,
    mul_u8_fn: Function,
    mul_u16_fn: Function,
    mul_u32_fn: Function,
    mul_u64_fn: Function,
    mul_usize_fn: Function,

    sub_f32_fn: Function,
    sub_f64_fn: Function,
    sub_i8_fn: Function,
    sub_i16_fn: Function,
    sub_i32_fn: Function,
    sub_i64_fn: Function,
    sub_u8_fn: Function,
    sub_u16_fn: Function,
    sub_u32_fn: Function,
    sub_u64_fn: Function,
    sub_usize_fn: Function,

    concat_string_bool_fn: Function,
    concat_string_char_fn: Function,
    concat_string_f32_fn: Function,
    concat_string_f64_fn: Function,
    concat_string_i8_fn: Function,
    concat_string_i16_fn: Function,
    concat_string_i32_fn: Function,
    concat_string_i64_fn: Function,
    concat_string_string_fn: Function,
    concat_string_u8_fn: Function,
    concat_string_u16_fn: Function,
    concat_string_u32_fn: Function,
    concat_string_u64_fn: Function,
    concat_string_usize_fn: Function,

    write_bool_fn: Function,
    write_char_fn: Function,
    write_f32_fn: Function,
    write_f64_fn: Function,
    write_i8_fn: Function,
    write_i16_fn: Function,
    write_i32_fn: Function,
    write_i64_fn: Function,
    write_string_fn: Function,
    write_u8_fn: Function,
    write_u16_fn: Function,
    write_u32_fn: Function,
    write_u64_fn: Function,
    write_usize_fn: Function,

    writeln_bool_fn: Function,
    writeln_char_fn: Function,
    writeln_f32_fn: Function,
    writeln_f64_fn: Function,
    writeln_i8_fn: Function,
    writeln_i16_fn: Function,
    writeln_i32_fn: Function,
    writeln_i64_fn: Function,
    writeln_string_fn: Function,
    writeln_u8_fn: Function,
    writeln_u16_fn: Function,
    writeln_u32_fn: Function,
    writeln_u64_fn: Function,
    writeln_usize_fn: Function,
}

impl Prelude {
    pub fn new() -> Prelude {
        Prelude {
            add_f32_fn: add_fn!("f32", Type::Primitive(PrimitiveType::F32)),
            add_f64_fn: add_fn!("f64", Type::Primitive(PrimitiveType::F64)),
            add_i8_fn: add_fn!("i8", Type::Primitive(PrimitiveType::I8)),
            add_i16_fn: add_fn!("i16", Type::Primitive(PrimitiveType::I16)),
            add_i32_fn: add_fn!("i32", Type::Primitive(PrimitiveType::I32)),
            add_i64_fn: add_fn!("i64", Type::Primitive(PrimitiveType::I64)),
            add_u8_fn: add_fn!("u8", Type::Primitive(PrimitiveType::U8)),
            add_u16_fn: add_fn!("u16", Type::Primitive(PrimitiveType::U16)),
            add_u32_fn: add_fn!("u32", Type::Primitive(PrimitiveType::U32)),
            add_u64_fn: add_fn!("u64", Type::Primitive(PrimitiveType::U64)),
            add_usize_fn: add_fn!("usize", Type::Primitive(PrimitiveType::USize)),

            div_f32_fn: div_fn!("f32", Type::Primitive(PrimitiveType::F32)),
            div_f64_fn: div_fn!("f64", Type::Primitive(PrimitiveType::F64)),
            div_i8_fn: div_fn!("i8", Type::Primitive(PrimitiveType::I8)),
            div_i16_fn: div_fn!("i16", Type::Primitive(PrimitiveType::I16)),
            div_i32_fn: div_fn!("i32", Type::Primitive(PrimitiveType::I32)),
            div_i64_fn: div_fn!("i64", Type::Primitive(PrimitiveType::I64)),
            div_u8_fn: div_fn!("u8", Type::Primitive(PrimitiveType::U8)),
            div_u16_fn: div_fn!("u16", Type::Primitive(PrimitiveType::U16)),
            div_u32_fn: div_fn!("u32", Type::Primitive(PrimitiveType::U32)),
            div_u64_fn: div_fn!("u64", Type::Primitive(PrimitiveType::U64)),
            div_usize_fn: div_fn!("usize", Type::Primitive(PrimitiveType::USize)),

            mul_f32_fn: mul_fn!("f32", Type::Primitive(PrimitiveType::F32)),
            mul_f64_fn: mul_fn!("f64", Type::Primitive(PrimitiveType::F64)),
            mul_i8_fn: mul_fn!("i8", Type::Primitive(PrimitiveType::I8)),
            mul_i16_fn: mul_fn!("i16", Type::Primitive(PrimitiveType::I16)),
            mul_i32_fn: mul_fn!("i32", Type::Primitive(PrimitiveType::I32)),
            mul_i64_fn: mul_fn!("i64", Type::Primitive(PrimitiveType::I64)),
            mul_u8_fn: mul_fn!("u8", Type::Primitive(PrimitiveType::U8)),
            mul_u16_fn: mul_fn!("u16", Type::Primitive(PrimitiveType::U16)),
            mul_u32_fn: mul_fn!("u32", Type::Primitive(PrimitiveType::U32)),
            mul_u64_fn: mul_fn!("u64", Type::Primitive(PrimitiveType::U64)),
            mul_usize_fn: mul_fn!("usize", Type::Primitive(PrimitiveType::USize)),

            sub_f32_fn: sub_fn!("f32", Type::Primitive(PrimitiveType::F32)),
            sub_f64_fn: sub_fn!("f64", Type::Primitive(PrimitiveType::F64)),
            sub_i8_fn: sub_fn!("i8", Type::Primitive(PrimitiveType::I8)),
            sub_i16_fn: sub_fn!("i16", Type::Primitive(PrimitiveType::I16)),
            sub_i32_fn: sub_fn!("i32", Type::Primitive(PrimitiveType::I32)),
            sub_i64_fn: sub_fn!("i64", Type::Primitive(PrimitiveType::I64)),
            sub_u8_fn: sub_fn!("u8", Type::Primitive(PrimitiveType::U8)),
            sub_u16_fn: sub_fn!("u16", Type::Primitive(PrimitiveType::U16)),
            sub_u32_fn: sub_fn!("u32", Type::Primitive(PrimitiveType::U32)),
            sub_u64_fn: sub_fn!("u64", Type::Primitive(PrimitiveType::U64)),
            sub_usize_fn: sub_fn!("usize", Type::Primitive(PrimitiveType::USize)),

            concat_string_bool_fn: concat_string_fn!("bool", Type::Primitive(PrimitiveType::Bool)),
            concat_string_char_fn: concat_string_fn!("char", Type::Primitive(PrimitiveType::Char)),
            concat_string_f32_fn: concat_string_fn!("f32", Type::Primitive(PrimitiveType::F32)),
            concat_string_f64_fn: concat_string_fn!("f64", Type::Primitive(PrimitiveType::F64)),
            concat_string_i8_fn: concat_string_fn!("i8", Type::Primitive(PrimitiveType::I8)),
            concat_string_i16_fn: concat_string_fn!("i16", Type::Primitive(PrimitiveType::I16)),
            concat_string_i32_fn: concat_string_fn!("i32", Type::Primitive(PrimitiveType::I32)),
            concat_string_i64_fn: concat_string_fn!("i64", Type::Primitive(PrimitiveType::I64)),
            concat_string_string_fn: concat_string_fn!("string", Type::Primitive(PrimitiveType::Str)),
            concat_string_u8_fn: concat_string_fn!("u8", Type::Primitive(PrimitiveType::U8)),
            concat_string_u16_fn: concat_string_fn!("u16", Type::Primitive(PrimitiveType::U16)),
            concat_string_u32_fn: concat_string_fn!("u32", Type::Primitive(PrimitiveType::U32)),
            concat_string_u64_fn: concat_string_fn!("u64", Type::Primitive(PrimitiveType::U64)),
            concat_string_usize_fn: concat_string_fn!("usize", Type::Primitive(PrimitiveType::USize)),

            write_bool_fn: write_fn!("bool", Type::Primitive(PrimitiveType::Bool)),
            write_char_fn: write_fn!("char", Type::Primitive(PrimitiveType::Char)),
            write_f32_fn: write_fn!("f32", Type::Primitive(PrimitiveType::F32)),
            write_f64_fn: write_fn!("f64", Type::Primitive(PrimitiveType::F64)),
            write_i8_fn: write_fn!("i8", Type::Primitive(PrimitiveType::I8)),
            write_i16_fn: write_fn!("i16", Type::Primitive(PrimitiveType::I16)),
            write_i32_fn: write_fn!("i32", Type::Primitive(PrimitiveType::I32)),
            write_i64_fn: write_fn!("i64", Type::Primitive(PrimitiveType::I64)),
            write_string_fn: write_fn!("string", Type::Primitive(PrimitiveType::Str)),
            write_u8_fn: write_fn!("u8", Type::Primitive(PrimitiveType::U8)),
            write_u16_fn: write_fn!("u16", Type::Primitive(PrimitiveType::U16)),
            write_u32_fn: write_fn!("u32", Type::Primitive(PrimitiveType::U32)),
            write_u64_fn: write_fn!("u64", Type::Primitive(PrimitiveType::U64)),
            write_usize_fn: write_fn!("usize", Type::Primitive(PrimitiveType::USize)),

            writeln_bool_fn: writeln_fn!("bool", Type::Primitive(PrimitiveType::Bool)),
            writeln_char_fn: writeln_fn!("char", Type::Primitive(PrimitiveType::Char)),
            writeln_f32_fn: writeln_fn!("f32", Type::Primitive(PrimitiveType::F32)),
            writeln_f64_fn: writeln_fn!("f64", Type::Primitive(PrimitiveType::F64)),
            writeln_i8_fn: writeln_fn!("i8", Type::Primitive(PrimitiveType::I8)),
            writeln_i16_fn: writeln_fn!("i16", Type::Primitive(PrimitiveType::I16)),
            writeln_i32_fn: writeln_fn!("i32", Type::Primitive(PrimitiveType::I32)),
            writeln_i64_fn: writeln_fn!("i64", Type::Primitive(PrimitiveType::I64)),
            writeln_string_fn: writeln_fn!("string", Type::Primitive(PrimitiveType::Str)),
            writeln_u8_fn: writeln_fn!("u8", Type::Primitive(PrimitiveType::U8)),
            writeln_u16_fn: writeln_fn!("u16", Type::Primitive(PrimitiveType::U16)),
            writeln_u32_fn: writeln_fn!("u32", Type::Primitive(PrimitiveType::U32)),
            writeln_u64_fn: writeln_fn!("u64", Type::Primitive(PrimitiveType::U64)),
            writeln_usize_fn: writeln_fn!("usize", Type::Primitive(PrimitiveType::USize)),
        }
    }

    pub fn add_f32_fn(&self) -> Function {
        self.add_f32_fn.clone()
    }

    pub fn add_f64_fn(&self) -> Function {
        self.add_f64_fn.clone()
    }

    pub fn add_i8_fn(&self) -> Function {
        self.add_i8_fn.clone()
    }

    pub fn add_i16_fn(&self) -> Function {
        self.add_i16_fn.clone()
    }

    pub fn add_i32_fn(&self) -> Function {
        self.add_i32_fn.clone()
    }

    pub fn add_i64_fn(&self) -> Function {
        self.add_i64_fn.clone()
    }

    pub fn add_u8_fn(&self) -> Function {
        self.add_u8_fn.clone()
    }

    pub fn add_u16_fn(&self) -> Function {
        self.add_u16_fn.clone()
    }

    pub fn add_u32_fn(&self) -> Function {
        self.add_u32_fn.clone()
    }

    pub fn add_u64_fn(&self) -> Function {
        self.add_u64_fn.clone()
    }

    pub fn add_usize_fn(&self) -> Function {
        self.add_usize_fn.clone()
    }

    pub fn div_f32_fn(&self) -> Function {
        self.div_f32_fn.clone()
    }

    pub fn div_f64_fn(&self) -> Function {
        self.div_f64_fn.clone()
    }

    pub fn div_i8_fn(&self) -> Function {
        self.div_i8_fn.clone()
    }

    pub fn div_i16_fn(&self) -> Function {
        self.div_i16_fn.clone()
    }

    pub fn div_i32_fn(&self) -> Function {
        self.div_i32_fn.clone()
    }

    pub fn div_i64_fn(&self) -> Function {
        self.div_i64_fn.clone()
    }

    pub fn div_u8_fn(&self) -> Function {
        self.div_u8_fn.clone()
    }

    pub fn div_u16_fn(&self) -> Function {
        self.div_u16_fn.clone()
    }

    pub fn div_u32_fn(&self) -> Function {
        self.div_u32_fn.clone()
    }

    pub fn div_u64_fn(&self) -> Function {
        self.div_u64_fn.clone()
    }

    pub fn div_usize_fn(&self) -> Function {
        self.div_usize_fn.clone()
    }

    pub fn mul_f32_fn(&self) -> Function {
        self.mul_f32_fn.clone()
    }

    pub fn mul_f64_fn(&self) -> Function {
        self.mul_f64_fn.clone()
    }

    pub fn mul_i8_fn(&self) -> Function {
        self.mul_i8_fn.clone()
    }

    pub fn mul_i16_fn(&self) -> Function {
        self.mul_i16_fn.clone()
    }

    pub fn mul_i32_fn(&self) -> Function {
        self.mul_i32_fn.clone()
    }

    pub fn mul_i64_fn(&self) -> Function {
        self.mul_i64_fn.clone()
    }

    pub fn mul_u8_fn(&self) -> Function {
        self.mul_u8_fn.clone()
    }

    pub fn mul_u16_fn(&self) -> Function {
        self.mul_u16_fn.clone()
    }

    pub fn mul_u32_fn(&self) -> Function {
        self.mul_u32_fn.clone()
    }

    pub fn mul_u64_fn(&self) -> Function {
        self.mul_u64_fn.clone()
    }

    pub fn mul_usize_fn(&self) -> Function {
        self.mul_usize_fn.clone()
    }

    pub fn sub_f32_fn(&self) -> Function {
        self.sub_f32_fn.clone()
    }

    pub fn sub_f64_fn(&self) -> Function {
        self.sub_f64_fn.clone()
    }

    pub fn sub_i8_fn(&self) -> Function {
        self.sub_i8_fn.clone()
    }

    pub fn sub_i16_fn(&self) -> Function {
        self.sub_i16_fn.clone()
    }

    pub fn sub_i32_fn(&self) -> Function {
        self.sub_i32_fn.clone()
    }

    pub fn sub_i64_fn(&self) -> Function {
        self.sub_i64_fn.clone()
    }

    pub fn sub_u8_fn(&self) -> Function {
        self.sub_u8_fn.clone()
    }

    pub fn sub_u16_fn(&self) -> Function {
        self.sub_u16_fn.clone()
    }

    pub fn sub_u32_fn(&self) -> Function {
        self.sub_u32_fn.clone()
    }

    pub fn sub_u64_fn(&self) -> Function {
        self.sub_u64_fn.clone()
    }

    pub fn sub_usize_fn(&self) -> Function {
        self.sub_usize_fn.clone()
    }

    pub fn concat_string_bool_fn(&self) -> Function {
        self.concat_string_bool_fn.clone()
    }

    pub fn concat_string_char_fn(&self) -> Function {
        self.concat_string_char_fn.clone()
    }

    pub fn concat_string_f32_fn(&self) -> Function {
        self.concat_string_f32_fn.clone()
    }

    pub fn concat_string_f64_fn(&self) -> Function {
        self.concat_string_f64_fn.clone()
    }

    pub fn concat_string_i8_fn(&self) -> Function {
        self.concat_string_i8_fn.clone()
    }

    pub fn concat_string_i16_fn(&self) -> Function {
        self.concat_string_i16_fn.clone()
    }

    pub fn concat_string_i32_fn(&self) -> Function {
        self.concat_string_i32_fn.clone()
    }

    pub fn concat_string_i64_fn(&self) -> Function {
        self.concat_string_i64_fn.clone()
    }

    pub fn concat_string_string_fn(&self) -> Function {
        self.concat_string_string_fn.clone()
    }

    pub fn concat_string_u8_fn(&self) -> Function {
        self.concat_string_u8_fn.clone()
    }

    pub fn concat_string_u16_fn(&self) -> Function {
        self.concat_string_u16_fn.clone()
    }

    pub fn concat_string_u32_fn(&self) -> Function {
        self.concat_string_u32_fn.clone()
    }

    pub fn concat_string_u64_fn(&self) -> Function {
        self.concat_string_u64_fn.clone()
    }

    pub fn concat_string_usize_fn(&self) -> Function {
        self.concat_string_usize_fn.clone()
    }

    pub fn write_bool_fn(&self) -> Function {
        self.write_bool_fn.clone()
    }

    pub fn write_char_fn(&self) -> Function {
        self.write_char_fn.clone()
    }

    pub fn write_f32_fn(&self) -> Function {
        self.write_f32_fn.clone()
    }

    pub fn write_f64_fn(&self) -> Function {
        self.write_f64_fn.clone()
    }

    pub fn write_i8_fn(&self) -> Function {
        self.write_i8_fn.clone()
    }

    pub fn write_i16_fn(&self) -> Function {
        self.write_i16_fn.clone()
    }

    pub fn write_i32_fn(&self) -> Function {
        self.write_i32_fn.clone()
    }

    pub fn write_i64_fn(&self) -> Function {
        self.write_i64_fn.clone()
    }

    pub fn write_string_fn(&self) -> Function {
        self.write_string_fn.clone()
    }

    pub fn write_u8_fn(&self) -> Function {
        self.write_u8_fn.clone()
    }

    pub fn write_u16_fn(&self) -> Function {
        self.write_u16_fn.clone()
    }

    pub fn write_u32_fn(&self) -> Function {
        self.write_u32_fn.clone()
    }

    pub fn write_u64_fn(&self) -> Function {
        self.write_u64_fn.clone()
    }

    pub fn write_usize_fn(&self) -> Function {
        self.write_usize_fn.clone()
    }

    pub fn writeln_bool_fn(&self) -> Function {
        self.writeln_bool_fn.clone()
    }

    pub fn writeln_char_fn(&self) -> Function {
        self.writeln_char_fn.clone()
    }

    pub fn writeln_f32_fn(&self) -> Function {
        self.writeln_f32_fn.clone()
    }

    pub fn writeln_f64_fn(&self) -> Function {
        self.writeln_f64_fn.clone()
    }

    pub fn writeln_i8_fn(&self) -> Function {
        self.writeln_i8_fn.clone()
    }

    pub fn writeln_i16_fn(&self) -> Function {
        self.writeln_i16_fn.clone()
    }

    pub fn writeln_i32_fn(&self) -> Function {
        self.writeln_i32_fn.clone()
    }

    pub fn writeln_i64_fn(&self) -> Function {
        self.writeln_i64_fn.clone()
    }

    pub fn writeln_string_fn(&self) -> Function {
        self.writeln_string_fn.clone()
    }

    pub fn writeln_u8_fn(&self) -> Function {
        self.writeln_u8_fn.clone()
    }

    pub fn writeln_u16_fn(&self) -> Function {
        self.writeln_u16_fn.clone()
    }

    pub fn writeln_u32_fn(&self) -> Function {
        self.writeln_u32_fn.clone()
    }

    pub fn writeln_u64_fn(&self) -> Function {
        self.writeln_u64_fn.clone()
    }

    pub fn writeln_usize_fn(&self) -> Function {
        self.writeln_usize_fn.clone()
    }
}