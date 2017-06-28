use super::*;

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
            add_f32_fn: add_fn!("f32", PrimitiveType::F32.into()),
            add_f64_fn: add_fn!("f64", PrimitiveType::F64.into()),
            add_i8_fn: add_fn!("i8", PrimitiveType::I8.into()),
            add_i16_fn: add_fn!("i16", PrimitiveType::I16.into()),
            add_i32_fn: add_fn!("i32", PrimitiveType::I32.into()),
            add_i64_fn: add_fn!("i64", PrimitiveType::I64.into()),
            add_u8_fn: add_fn!("u8", PrimitiveType::U8.into()),
            add_u16_fn: add_fn!("u16", PrimitiveType::U16.into()),
            add_u32_fn: add_fn!("u32", PrimitiveType::U32.into()),
            add_u64_fn: add_fn!("u64", PrimitiveType::U64.into()),
            add_usize_fn: add_fn!("usize", PrimitiveType::USize.into()),

            div_f32_fn: div_fn!("f32", PrimitiveType::F32.into()),
            div_f64_fn: div_fn!("f64", PrimitiveType::F64.into()),
            div_i8_fn: div_fn!("i8", PrimitiveType::I8.into()),
            div_i16_fn: div_fn!("i16", PrimitiveType::I16.into()),
            div_i32_fn: div_fn!("i32", PrimitiveType::I32.into()),
            div_i64_fn: div_fn!("i64", PrimitiveType::I64.into()),
            div_u8_fn: div_fn!("u8", PrimitiveType::U8.into()),
            div_u16_fn: div_fn!("u16", PrimitiveType::U16.into()),
            div_u32_fn: div_fn!("u32", PrimitiveType::U32.into()),
            div_u64_fn: div_fn!("u64", PrimitiveType::U64.into()),
            div_usize_fn: div_fn!("usize", PrimitiveType::USize.into()),

            mul_f32_fn: mul_fn!("f32", PrimitiveType::F32.into()),
            mul_f64_fn: mul_fn!("f64", PrimitiveType::F64.into()),
            mul_i8_fn: mul_fn!("i8", PrimitiveType::I8.into()),
            mul_i16_fn: mul_fn!("i16", PrimitiveType::I16.into()),
            mul_i32_fn: mul_fn!("i32", PrimitiveType::I32.into()),
            mul_i64_fn: mul_fn!("i64", PrimitiveType::I64.into()),
            mul_u8_fn: mul_fn!("u8", PrimitiveType::U8.into()),
            mul_u16_fn: mul_fn!("u16", PrimitiveType::U16.into()),
            mul_u32_fn: mul_fn!("u32", PrimitiveType::U32.into()),
            mul_u64_fn: mul_fn!("u64", PrimitiveType::U64.into()),
            mul_usize_fn: mul_fn!("usize", PrimitiveType::USize.into()),

            sub_f32_fn: sub_fn!("f32", PrimitiveType::F32.into()),
            sub_f64_fn: sub_fn!("f64", PrimitiveType::F64.into()),
            sub_i8_fn: sub_fn!("i8", PrimitiveType::I8.into()),
            sub_i16_fn: sub_fn!("i16", PrimitiveType::I16.into()),
            sub_i32_fn: sub_fn!("i32", PrimitiveType::I32.into()),
            sub_i64_fn: sub_fn!("i64", PrimitiveType::I64.into()),
            sub_u8_fn: sub_fn!("u8", PrimitiveType::U8.into()),
            sub_u16_fn: sub_fn!("u16", PrimitiveType::U16.into()),
            sub_u32_fn: sub_fn!("u32", PrimitiveType::U32.into()),
            sub_u64_fn: sub_fn!("u64", PrimitiveType::U64.into()),
            sub_usize_fn: sub_fn!("usize", PrimitiveType::USize.into()),

            concat_string_bool_fn: concat_string_fn!("bool", PrimitiveType::Bool.into()),
            concat_string_char_fn: concat_string_fn!("char", PrimitiveType::Char.into()),
            concat_string_f32_fn: concat_string_fn!("f32", PrimitiveType::F32.into()),
            concat_string_f64_fn: concat_string_fn!("f64", PrimitiveType::F64.into()),
            concat_string_i8_fn: concat_string_fn!("i8", PrimitiveType::I8.into()),
            concat_string_i16_fn: concat_string_fn!("i16", PrimitiveType::I16.into()),
            concat_string_i32_fn: concat_string_fn!("i32", PrimitiveType::I32.into()),
            concat_string_i64_fn: concat_string_fn!("i64", PrimitiveType::I64.into()),
            concat_string_string_fn: concat_string_fn!("string", PrimitiveType::Str.into()),
            concat_string_u8_fn: concat_string_fn!("u8", PrimitiveType::U8.into()),
            concat_string_u16_fn: concat_string_fn!("u16", PrimitiveType::U16.into()),
            concat_string_u32_fn: concat_string_fn!("u32", PrimitiveType::U32.into()),
            concat_string_u64_fn: concat_string_fn!("u64", PrimitiveType::U64.into()),
            concat_string_usize_fn: concat_string_fn!("usize", PrimitiveType::USize.into()),

            write_bool_fn: write_fn!("bool", PrimitiveType::Bool.into()),
            write_char_fn: write_fn!("char", PrimitiveType::Char.into()),
            write_f32_fn: write_fn!("f32", PrimitiveType::F32.into()),
            write_f64_fn: write_fn!("f64", PrimitiveType::F64.into()),
            write_i8_fn: write_fn!("i8", PrimitiveType::I8.into()),
            write_i16_fn: write_fn!("i16", PrimitiveType::I16.into()),
            write_i32_fn: write_fn!("i32", PrimitiveType::I32.into()),
            write_i64_fn: write_fn!("i64", PrimitiveType::I64.into()),
            write_string_fn: write_fn!("string", PrimitiveType::Str.into()),
            write_u8_fn: write_fn!("u8", PrimitiveType::U8.into()),
            write_u16_fn: write_fn!("u16", PrimitiveType::U16.into()),
            write_u32_fn: write_fn!("u32", PrimitiveType::U32.into()),
            write_u64_fn: write_fn!("u64", PrimitiveType::U64.into()),
            write_usize_fn: write_fn!("usize", PrimitiveType::USize.into()),

            writeln_bool_fn: writeln_fn!("bool", PrimitiveType::Bool.into()),
            writeln_char_fn: writeln_fn!("char", PrimitiveType::Char.into()),
            writeln_f32_fn: writeln_fn!("f32", PrimitiveType::F32.into()),
            writeln_f64_fn: writeln_fn!("f64", PrimitiveType::F64.into()),
            writeln_i8_fn: writeln_fn!("i8", PrimitiveType::I8.into()),
            writeln_i16_fn: writeln_fn!("i16", PrimitiveType::I16.into()),
            writeln_i32_fn: writeln_fn!("i32", PrimitiveType::I32.into()),
            writeln_i64_fn: writeln_fn!("i64", PrimitiveType::I64.into()),
            writeln_string_fn: writeln_fn!("string", PrimitiveType::Str.into()),
            writeln_u8_fn: writeln_fn!("u8", PrimitiveType::U8.into()),
            writeln_u16_fn: writeln_fn!("u16", PrimitiveType::U16.into()),
            writeln_u32_fn: writeln_fn!("u32", PrimitiveType::U32.into()),
            writeln_u64_fn: writeln_fn!("u64", PrimitiveType::U64.into()),
            writeln_usize_fn: writeln_fn!("usize", PrimitiveType::USize.into()),
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