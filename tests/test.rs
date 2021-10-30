use bit_by_bit::bit_by_bit;

#[bit_by_bit]
pub struct T<'a, Y: Into<u32>, S: std::fmt::Display>
where
    Y: Clone,
{
    #[bit(10)]
    x1: u32,
    #[bit(21)]
    x2: u32,
    #[bit(10)]
    x3: i32,
    _y: &'a (Y, S),
}

const VAL: (u8, &'static str) = (5, "test");

impl T<'static, u8, &'static str> {
    fn new() -> Self {
        Self {
            __base_field_0: 0,
            __base_field_1: 0,
            _y: &VAL,
        }
    }
}

#[test]
fn field_set() {
    let mut t = T::new();

    assert_eq!(t.x1(), 0);
    assert_eq!(t.x2(), 0);
    assert_eq!(t.x3(), 0);

    t.set_x1(0x12);
    t.set_x2(0x34);
    t.set_x3(0x56);

    assert_eq!(t.x1(), 0x12);
    assert_eq!(t.x2(), 0x34);
    assert_eq!(t.x3(), 0x56);
}

#[test]
fn field_truncate() {
    let mut t = T::new();

    t.set_x1(0b10111010111);
    t.set_x2(0x12345);
    t.set_x3(0x678);

    assert_eq!(t.x1(), 0b111010111);
    assert_eq!(t.x2(), 0x12345);
    assert_eq!(t.x3(), 0x278);
}

#[test]
fn field_max_value() {
    #[bit_by_bit]
    #[derive(Default)]
    struct T {
        #[bit(8)]
        a: u8,
        #[bit(16)]
        b: u16,
        #[bit(32)]
        c: u32,
        #[bit(64)]
        d: u64,
        #[bit(128)]
        e: u128,
    }
    let mut t = T::default();

    t.set_a(u8::max_value());
    assert_eq!(t.a(), u8::max_value());

    t.set_b(u16::max_value());
    assert_eq!(t.b(), u16::max_value());

    t.set_c(u32::max_value());
    assert_eq!(t.c(), u32::max_value());

    t.set_d(u64::max_value());
    assert_eq!(t.d(), u64::max_value());

    t.set_e(u128::max_value());
    assert_eq!(t.e(), u128::max_value());
}
