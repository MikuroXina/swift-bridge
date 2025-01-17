#[swift_bridge::bridge]
mod ffi {
    enum EnumWithNoData {
        Variant1,
        Variant2,
    }

    extern "Rust" {
        fn reflect_enum_with_no_data(arg: EnumWithNoData) -> EnumWithNoData;
    }

    enum EnumWithUnnamedData {
        Variant1(String, u32),
        Variant2(i32, u8),
        Variant3,
    }

    extern "Rust" {
        fn reflect_enum_with_unnamed_data(arg: EnumWithUnnamedData) -> EnumWithUnnamedData;
    }

    enum EnumWithNamedData {
        Variant1 { hello: String, data_u8: u8 },
        Variant2 { data_i32: i32 },
        Variant3,
    }

    extern "Rust" {
        fn reflect_enum_with_named_data(arg: EnumWithNamedData) -> EnumWithNamedData;
    }
}

fn reflect_enum_with_no_data(arg: ffi::EnumWithNoData) -> ffi::EnumWithNoData {
    arg
}

fn reflect_enum_with_unnamed_data(arg: ffi::EnumWithUnnamedData) -> ffi::EnumWithUnnamedData {
    arg
}

fn reflect_enum_with_named_data(arg: ffi::EnumWithNamedData) -> ffi::EnumWithNamedData {
    arg
}
