use bit_by_bit::bit_by_bit;
use quickcheck_macros::quickcheck;

trait Primitive: Eq {}

macro_rules! gen_test {
    ($f:ident, $t:ty, $width:literal) => {
        #[quickcheck]
        fn $f(x: $t) {
            #[bit_by_bit]
            #[derive(Default)]
            struct T {
                #[bit($width)]
                x: $t,
            }

            let mut t = T::default();
            t.set_x(x);
            assert_eq!(t.x(), x);
        }
    };
}

gen_test!(field_max_width_u8, u8, 8);
gen_test!(field_max_width_u16, u16, 16);
gen_test!(field_max_width_u32, u32, 32);
gen_test!(field_max_width_u64, u64, 64);
gen_test!(field_max_width_u128, u128, 128);
