use bit_by_bit::bit_by_bit;

trait A {}
trait B {}
trait C {}

#[bit_by_bit]
struct T1 {
    #[bit(128)]
    x: i128,
}

static_assertions::assert_eq_size!(T1, i128);

#[bit_by_bit]
struct T2 {
    #[bit(1)]
    x1: u8,
    #[bit(1)]
    x2: u8,
    #[bit(1)]
    x3: u8,
    #[bit(1)]
    x4: u8,
    #[bit(1)]
    x5: u8,
    #[bit(1)]
    x6: u8,
    #[bit(1)]
    x7: u8,
    #[bit(1)]
    x8: u8,
}

static_assertions::assert_eq_size!(T2, u8);

#[bit_by_bit]
struct T3<'a, Y: A>
where
    Y: B + C,
{
    x: &'a Y,
    #[bit(5)]
    y: u8,
}

#[bit_by_bit]
struct T4 {
    #[bit(6)]
    x1: u8,
    #[bit(6)]
    x2: u8,
}

static_assertions::assert_eq_size!(T4, (u8, u8));

#[bit_by_bit]
struct T5 {
    #[bit(5)]
    x1: u16,
    #[bit(6)]
    x2: u16,
    #[bit(4)]
    x3: u16,
}

static_assertions::assert_eq_size!(T5, u16);

#[bit_by_bit]
struct T6 {
    #[bit(5)]
    x1: u16,
    #[bit(6)]
    x2: u16,
    #[bit(5)]
    x3: u16,
}

static_assertions::assert_eq_size!(T6, u16);

#[bit_by_bit]
struct T7 {
    #[bit(5)]
    x1: u16,
    #[bit(6)]
    x2: u8,
    #[bit(5)]
    x3: u16,
}

static_assertions::assert_eq_size!(T7, (u16, u8, u16));

#[bit_by_bit]
#[repr(C, packed)]
struct T8 {
    #[bit(5)]
    x1: u16,
    #[bit(6)]
    x2: u8,
    #[bit(5)]
    x3: u16,
}

static_assertions::const_assert_eq!(core::mem::size_of::<T8>(), (16 + 8 + 16) / 8);

#[bit_by_bit]
struct T9 {
    #[bit(7)]
    x: u32,
    s: str,
}

#[test]
fn it_compiles() {}
