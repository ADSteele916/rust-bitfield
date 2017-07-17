#[macro_use]
extern crate bitfield;

// We use a constant to make sure bits positions don't need to be literals but
// can also be constants or expressions.
const THREE: usize = 3;

bitfield! {
    #[derive(Copy, Clone)]
    /// documentation comments also work!
    struct FooBar(u32);
    foo1, set_foo1: 0, 0;
    u8;
    foo2, set_foo2: 31, 31;
    foo3, set_foo3: THREE, 0;
    // We make sure attributes are applied to fields. If attributes were not
    // applied, the compilation would fail with a `duplicate definition`
    // error.
    #[cfg(not(test))]
    foo3, set_foo3: 3, 0;
    u16, foo4, set_foo4: 31, 28;
    foo5, set_foo5: 0, 0, 32;
    u32;
    foo6, set_foo6: 5, THREE, THREE;
    getter_only, _: 3, 1;
    _, setter_only: 2*2, 2;
    getter_only_array, _: 5, 3, 3;
    _, setter_only_array: 2*THREE, 4, 3;
    all_bits, set_all_bits: 31, 0;
    single_bit, set_single_bit: 3;
}


impl FooBar {
    bitfield_fields!{
        // Boolean field don't need a type
        foo7, _: 1;
    }


    bitfield_fields!{
        // If all fields have a type, we don't need to specify a default type
        u8, foo8,_: 1, 0;
        u32, foo9, _: 2, 0;
    }

    bitfield_fields! {
        // We can still set a default type
        u16;
        foo10, _: 2, 0;
        u32, foo11, _: 2, 0;
        foo12, _: 2, 0;
    }

    // Check if an empty bitfield_fields compiles without errors.
    bitfield_fields!{}
}

#[test]
fn test_single_bit() {
    let mut fb = FooBar(0);

    fb.set_foo1(1);
    assert_eq!(0x1, fb.0);
    assert_eq!(0x1, fb.foo1());
    assert_eq!(0x0, fb.foo2());
    assert_eq!(false, fb.single_bit());

    fb.set_foo2(1);
    assert_eq!(0x80000001, fb.0);
    assert_eq!(0x1, fb.foo1());
    assert_eq!(0x1, fb.foo2());
    assert_eq!(false, fb.single_bit());

    fb.set_foo1(0);
    assert_eq!(0x80000000, fb.0);
    assert_eq!(0x0, fb.foo1());
    assert_eq!(0x1, fb.foo2());
    assert_eq!(false, fb.single_bit());

    fb.set_single_bit(true);
    assert_eq!(0x80000008, fb.0);
    assert_eq!(0x0, fb.foo1());
    assert_eq!(0x1, fb.foo2());
    assert_eq!(true, fb.single_bit());
}

#[test]
fn test_single_bit_plus_garbage() {
    let mut fb = FooBar(0);

    fb.set_foo1(0b10);
    assert_eq!(0x0, fb.0);
    assert_eq!(0x0, fb.foo1());
    assert_eq!(0x0, fb.foo2());

    fb.set_foo1(0b11);
    assert_eq!(0x1, fb.0);
    assert_eq!(0x1, fb.foo1());
    assert_eq!(0x0, fb.foo2());

}

#[test]
fn test_multiple_bit() {
    let mut fb = FooBar(0);

    fb.set_foo3(0x0F);
    assert_eq!(0xF, fb.0);
    assert_eq!(0xF, fb.foo3());
    assert_eq!(0x0, fb.foo4());

    fb.set_foo4(0x0F);
    assert_eq!(0xF000000F, fb.0);
    assert_eq!(0xF, fb.foo3());
    assert_eq!(0xF, fb.foo4());

    fb.set_foo3(0);
    assert_eq!(0xF0000000, fb.0);
    assert_eq!(0x0, fb.foo3());
    assert_eq!(0xF, fb.foo4());

    fb.set_foo3(0xA);
    assert_eq!(0xF000000A, fb.0);
    assert_eq!(0xA, fb.foo3());
    assert_eq!(0xF, fb.foo4());
}

#[test]
fn test_getter_setter_only() {
    let mut fb = FooBar(0);
    fb.setter_only(0x7);
    assert_eq!(0x1C, fb.0);
    assert_eq!(0x6, fb.getter_only());
}

#[test]
fn test_array_field1() {
    let mut fb = FooBar(0);

    fb.set_foo5(0, 1);
    assert_eq!(0x1, fb.0);
    assert_eq!(1, fb.foo5(0));

    fb.set_foo5(0, 0);
    assert_eq!(0x0, fb.0);
    assert_eq!(0, fb.foo5(0));

    fb.set_foo5(0, 1);
    fb.set_foo5(6, 1);
    fb.set_foo5(31, 1);
    assert_eq!(0x80000041, fb.0);
    assert_eq!(1, fb.foo5(0));
    assert_eq!(1, fb.foo5(6));
    assert_eq!(1, fb.foo5(31));
    assert_eq!(0, fb.foo5(1));
    assert_eq!(0, fb.foo5(5));
    assert_eq!(0, fb.foo5(7));
    assert_eq!(0, fb.foo5(30));
}

#[test]
fn test_array_field2() {
    let mut fb = FooBar(0);

    fb.set_foo6(0, 1);
    assert_eq!(0x8, fb.0);
    assert_eq!(1, fb.foo6(0));
    assert_eq!(0, fb.foo6(1));
    assert_eq!(0, fb.foo6(2));

    fb.set_foo6(0, 7);
    assert_eq!(0x38, fb.0);
    assert_eq!(7, fb.foo6(0));
    assert_eq!(0, fb.foo6(1));
    assert_eq!(0, fb.foo6(2));

    fb.set_foo6(2, 7);
    assert_eq!(0xE38, fb.0);
    assert_eq!(7, fb.foo6(0));
    assert_eq!(0, fb.foo6(1));
    assert_eq!(7, fb.foo6(2));

    fb.set_foo6(0, 0);
    assert_eq!(0xE00, fb.0);
    assert_eq!(0, fb.foo6(0));
    assert_eq!(0, fb.foo6(1));
    assert_eq!(7, fb.foo6(2));
}

#[allow(unknown_lints)]
#[allow(identity_op)]
#[test]
fn test_setter_only_array() {
    let mut fb = FooBar(0);

    fb.setter_only_array(0, 0);
    assert_eq!(0x0, fb.0);

    fb.setter_only_array(0, 0b111);
    assert_eq!(0b111 << (4 + 0 * 2), fb.0);

    fb.setter_only_array(0, 0);
    fb.setter_only_array(1, 0b111);
    assert_eq!(0b111 << (4 + 1 * 3), fb.0);

    fb.setter_only_array(1, 0);
    fb.setter_only_array(2, 0b111);
    assert_eq!(0b111 << (4 + 2 * 3), fb.0);
}

#[test]
fn test_getter_only_array() {
    let mut fb = FooBar(0);

    assert_eq!(0, fb.getter_only_array(0));
    assert_eq!(0, fb.getter_only_array(1));
    assert_eq!(0, fb.getter_only_array(2));

    fb.0 = !(0x1FF << 3);
    assert_eq!(0, fb.getter_only_array(0));
    assert_eq!(0, fb.getter_only_array(1));
    assert_eq!(0, fb.getter_only_array(2));

    fb.0 = 0xF << 3;
    assert_eq!(0b111, fb.getter_only_array(0));
    assert_eq!(0b001, fb.getter_only_array(1));
    assert_eq!(0, fb.getter_only_array(2));

    fb.0 = 0xF << 6;
    assert_eq!(0, fb.getter_only_array(0));
    assert_eq!(0b111, fb.getter_only_array(1));
    assert_eq!(0b001, fb.getter_only_array(2));

    fb.0 = 0xF << 8;
    assert_eq!(0, fb.getter_only_array(0));
    assert_eq!(0b100, fb.getter_only_array(1));
    assert_eq!(0b111, fb.getter_only_array(2));

    fb.0 = 0b101_010_110 << 3;
    assert_eq!(0b110, fb.getter_only_array(0));
    assert_eq!(0b010, fb.getter_only_array(1));
    assert_eq!(0b101, fb.getter_only_array(2));
}

#[test]
fn test_field_type() {
    let fb = FooBar(0);
    let _: u32 = fb.foo1();
    let _: u8 = fb.foo2();
    let _: u8 = fb.foo3();
    let _: u16 = fb.foo4();
    let _: u8 = fb.foo5(0);
    let _: u32 = fb.foo6(0);

    let _: bool = fb.foo7();
    let _: u8 = fb.foo8();
    let _: u32 = fb.foo9();
    let _: u16 = fb.foo10();
    let _: u32 = fb.foo11();
    let _: u16 = fb.foo12();
}

#[test]
fn test_all_bits() {
    let mut fb = FooBar(0);

    assert_eq!(0, fb.all_bits());

    fb.set_all_bits(!0u32);
    assert_eq!(!0u32, fb.0);
    assert_eq!(!0u32, fb.all_bits());

    fb.0 = 0x80000001;
    assert_eq!(0x80000001, fb.all_bits());
}

#[test]
fn test_is_copy() {
    let a = FooBar(0);
    let _b = a;
    let _c = a;
}

bitfield! {
    struct ArrayBitfield([u8]);
    u32;
    foo1, set_foo1: 0, 0;
    foo2, set_foo2: 7, 0;
    foo3, set_foo3: 8, 1;
    foo4, set_foo4: 19, 4;
}

#[test]
fn test_arraybitfield() {
    let mut ab = ArrayBitfield([0; 3]);

    assert_eq!(0, ab.foo1());
    assert_eq!(0, ab.foo2());
    assert_eq!(0, ab.foo3());
    assert_eq!(0, ab.foo4());

    ab.set_foo1(1);
    assert_eq!([1, 0, 0], ab.0);
    assert_eq!(1, ab.foo1());
    assert_eq!(1, ab.foo2());
    assert_eq!(0, ab.foo3());
    assert_eq!(0, ab.foo4());

    ab.set_foo1(0);
    ab.set_foo2(0xFF);
    assert_eq!([0xFF, 0, 0], ab.0);
    assert_eq!(1, ab.foo1());
    assert_eq!(0xFF, ab.foo2());
    assert_eq!(0x7F, ab.foo3());
    assert_eq!(0x0F, ab.foo4());

    ab.set_foo2(0);
    ab.set_foo3(0xFF);
    assert_eq!([0xFE, 0x1, 0], ab.0);
    assert_eq!(0, ab.foo1());
    assert_eq!(0xFE, ab.foo2());
    assert_eq!(0xFF, ab.foo3());
    assert_eq!(0x1F, ab.foo4());

    ab.set_foo3(0);
    ab.set_foo4(0xFFFF);
    assert_eq!([0xF0, 0xFF, 0x0F], ab.0);
    assert_eq!(0, ab.foo1());
    assert_eq!(0xF0, ab.foo2());
    assert_eq!(0xF8, ab.foo3());
    assert_eq!(0xFFFF, ab.foo4());
}


#[test]
fn test_arraybitfield2() {
    // Check that the macro can be called from a function.
    bitfield! {
        struct ArrayBitfield2([u16]);
        u32;
        foo1, set_foo1: 0, 0;
        foo2, set_foo2: 7, 0;
        foo3, set_foo3: 8, 1;
        foo4, set_foo4: 20, 4;
    }
    let mut ab = ArrayBitfield2([0; 2]);

    assert_eq!(0, ab.foo1());
    assert_eq!(0, ab.foo2());
    assert_eq!(0, ab.foo3());
    assert_eq!(0, ab.foo4());

    ab.set_foo1(1);
    assert_eq!([1, 0], ab.0);
    assert_eq!(1, ab.foo1());
    assert_eq!(1, ab.foo2());
    assert_eq!(0, ab.foo3());
    assert_eq!(0, ab.foo4());

    ab.set_foo1(0);
    ab.set_foo2(0xFF);
    assert_eq!([0xFF, 0], ab.0);
    assert_eq!(1, ab.foo1());
    assert_eq!(0xFF, ab.foo2());
    assert_eq!(0x7F, ab.foo3());
    assert_eq!(0x0F, ab.foo4());

    ab.set_foo2(0);
    ab.set_foo3(0xFF);
    assert_eq!([0x1FE, 0x0], ab.0);
    assert_eq!(0, ab.foo1());
    assert_eq!(0xFE, ab.foo2());
    assert_eq!(0xFF, ab.foo3());
    assert_eq!(0x1F, ab.foo4());

    ab.set_foo3(0);
    ab.set_foo4(0xFFFF);
    assert_eq!([0xFFF0, 0xF], ab.0);
    assert_eq!(0, ab.foo1());
    assert_eq!(0xF0, ab.foo2());
    assert_eq!(0xF8, ab.foo3());
    assert_eq!(0xFFFF, ab.foo4());
}

bitfield! {
    struct ArrayBitfieldMsb0(MSB0 [u8]);
    u32;
    foo1, set_foo1: 0, 0;
    foo2, set_foo2: 7, 0;
    foo3, set_foo3: 8, 1;
    foo4, set_foo4: 19, 4;
}

#[test]
fn test_arraybitfield_msb0() {
    let mut ab = ArrayBitfieldMsb0([0; 3]);

    assert_eq!(0, ab.foo1());
    assert_eq!(0, ab.foo2());
    assert_eq!(0, ab.foo3());
    assert_eq!(0, ab.foo4());

    ab.set_foo1(1);
    assert_eq!([0b1000_0000, 0, 0], ab.0);
    assert_eq!(1, ab.foo1());
    assert_eq!(0b1000_0000, ab.foo2());
    assert_eq!(0, ab.foo3());
    assert_eq!(0, ab.foo4());

    ab.set_foo1(0);
    ab.set_foo2(0xFF);
    assert_eq!([0b1111_1111, 0, 0], ab.0);
    assert_eq!(1, ab.foo1());
    assert_eq!(0b1111_1111, ab.foo2());
    assert_eq!(0b1111_1110, ab.foo3());
    assert_eq!(0b1111_0000_0000_0000, ab.foo4());

    ab.set_foo2(0);
    ab.set_foo3(0xFF);
    assert_eq!([0b01111111, 0b10000000, 0], ab.0);
    assert_eq!(0, ab.foo1());
    assert_eq!(0b01111111, ab.foo2());
    assert_eq!(0xFF, ab.foo3());
    assert_eq!(0b1111_1000_0000_0000, ab.foo4());

    ab.set_foo3(0);
    ab.set_foo4(0xFFFF);
    assert_eq!([0x0F, 0xFF, 0xF0], ab.0);
    assert_eq!(0, ab.foo1());
    assert_eq!(0x0F, ab.foo2());
    assert_eq!(0b0001_1111, ab.foo3());
    assert_eq!(0xFFFF, ab.foo4());
}

mod some_module {
    bitfield! {
        pub struct PubBitFieldInAModule(u32);
        /// Attribute works on pub fields
        pub field1, set_field1: 1;
        pub field2, _: 1;
        pub _, set_field3: 1;
        pub u16, field4, set_field4: 1;
        /// Check if multiple attributes are applied
        #[cfg(not(test))]
        pub u16, field4, set_field4: 1;
        pub u16, _, set_field5: 1;
        pub u16, field6, _: 1;
        pub field7, set_field7: 1;
        pub field8, set_field8: 1, 1;
        #[cfg(not(test))]
        /// And make sure not only the last attributes is applied
        pub field8, set_field8: 1, 1;
        pub field9, set_field9: 1, 1, 1;
        pub u32, field10, set_field10: 1;
        pub u32, field11, set_field11: 1, 1;
        pub u32, field12, set_field12: 1, 1, 1;
    }

}

#[test]
fn struct_can_be_public() {
    let _ = some_module::PubBitFieldInAModule(0);
}
#[test]
fn field_can_be_public() {
    let mut a = some_module::PubBitFieldInAModule(0);
    let _ = a.field1();
    a.set_field1(true);
    let _ = a.field2();
    a.set_field3(true);
    let _ = a.field4();
    a.set_field4(true);
    a.set_field5(true);
    let _ = a.field6();
    let _ = a.field7();
    a.set_field7(true);
    let _ = a.field8();
    a.set_field8(0);
    let _ = a.field9(0);
    a.set_field9(0, 0);
    let _ = a.field10();
    a.set_field10(true);
    let _ = a.field11();
    a.set_field11(0);
    let _ = a.field12(0);
    a.set_field12(0, 0);
}

// Everything in this module is to make sure that its possible to specify types
// in most of the possible ways.
#[allow(dead_code)]
mod test_types {
    use bitfield::BitRange;
    use std;
    use std::sync::atomic::{self, AtomicUsize};

    struct Foo;

    impl Foo {
        bitfield_fields!{
            std::sync::atomic::AtomicUsize, field1, set_field1: 0, 0;
            std::sync::atomic::AtomicUsize;
            field2, set_field2: 0, 0;
            ::std::sync::atomic::AtomicUsize, field3, set_field3: 0, 0;
            ::std::sync::atomic::AtomicUsize;
            field4, set_field4: 0, 0;
            atomic::AtomicUsize, field5, set_field5: 0, 0;
            atomic::AtomicUsize;
            field6, set_field6: 0, 0;
            AtomicUsize, field7, set_field7: 0, 0;
            AtomicUsize;
            field8, set_field8: 0, 0;
            Vec<std::sync::atomic::AtomicUsize>, field9, set_field9: 0, 0;
            Vec<std::sync::atomic::AtomicUsize>;
            field10, set_field10: 0, 0;
            Vec<::std::sync::atomic::AtomicUsize>, field11, set_field11: 0, 0;
            Vec<::std::sync::atomic::AtomicUsize>;
            field12, set_field12: 0, 0;
            Vec<atomic::AtomicUsize>, field13, set_field13: 0, 0;
            Vec<atomic::AtomicUsize>;
            field14, set_field14: 0, 0;
            Vec<AtomicUsize>, field15, set_field15: 0, 0;
            Vec<AtomicUsize>;
            field16, set_field16: 0, 0;
            &str, field17, set_field17: 0, 0;
            &str;
            field18, set_field18: 0, 0;
            &'static str, field19, set_field19: 0, 0;
            &'static str;
            field20, set_field20: 0, 0;
        }
    }

    impl BitRange<AtomicUsize> for Foo {
        fn bit_range(&self, _msb: usize, _lsb: usize) -> AtomicUsize {
            AtomicUsize::new(0)
        }
        fn set_bit_range(&mut self, _msb: usize, _lsb: usize, _value: AtomicUsize) {}
    }


    impl BitRange<Vec<AtomicUsize>> for Foo {
        fn bit_range(&self, _msb: usize, _lsb: usize) -> Vec<AtomicUsize> {
            vec![AtomicUsize::new(0)]
        }
        fn set_bit_range(&mut self, _msb: usize, _lsb: usize, _value: Vec<AtomicUsize>) {}
    }


    impl<'a> BitRange<&'a str> for Foo {
        fn bit_range(&self, _msb: usize, _lsb: usize) -> &'a str {
            ""
        }
        fn set_bit_range(&mut self, _msb: usize, _lsb: usize, _value: &'a str) {}
    }


    #[test]
    fn test_field_type() {
        let test = Foo;
        let _: AtomicUsize = test.field1();
        let _: AtomicUsize = test.field2();
        let _: AtomicUsize = test.field3();
        let _: AtomicUsize = test.field4();
        let _: AtomicUsize = test.field5();
        let _: AtomicUsize = test.field6();
        let _: AtomicUsize = test.field7();
        let _: AtomicUsize = test.field8();
        let _: Vec<AtomicUsize> = test.field9();
        let _: Vec<AtomicUsize> = test.field10();
        let _: Vec<AtomicUsize> = test.field11();
        let _: Vec<AtomicUsize> = test.field12();
        let _: Vec<AtomicUsize> = test.field13();
        let _: Vec<AtomicUsize> = test.field14();
        let _: Vec<AtomicUsize> = test.field15();
        let _: Vec<AtomicUsize> = test.field16();
        let _: &str = test.field17();
        let _: &str = test.field18();
        let _: &'static str = test.field19();
        let _: &'static str = test.field20();
    }
}
