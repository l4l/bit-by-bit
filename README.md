# bit-by-bit

This crates allow you to define structs bitfields and safely work with them.

Current limitations:

- Only structs with named fields are supported;
- Supported only primitive integer types, i.e. {i,u}{8,16,32,64,128};
- Type aliases are not supported;
- References to bitfields are not supported.

## Example

```rust
use bit_by_bit::bit_by_bit;

#[bit_by_bit]
#[derive(Default)]
struct EthernetHeader {
    #[bit(7)]
    preamble: u8,
    #[bit(1)]
    sd: u8,
    #[bit(6)]
    dest: u8,
    #[bit(6)]
    src: u8,
    #[bit(2)]
    length: u8,
}

// Will expand to something like that:

#[derive(Default)]
struct EthernetHeader {
    __base_field_0: u8,
    __base_field_1: u8,
    __base_field_2: u8,
    __base_field_3: u8,
}

impl EthernetHeader {
    fn preamble(&self) -> u8 {
        self.__base_field_0 & (1 << 7) - 1
    }

    fn set_preamble(&mut self, val: u8) {
        self.__base_field_0 ^= self.__base_field_0 & (1 << 7) - 1;
        self.__base_field_0 |= val & (1 << 7) - 1;
    }

    fn sd(&self) -> u8 { /*impl*/ }
    fn set_sd(&mut self, val: u8) { /*impl*/ }
    fn dest(&self) -> u8 { /*impl*/ }
    fn set_dest(&mut self, val: u8) { /*impl*/ }
    fn src(&self) -> u8 { /*impl*/ }
    fn set_src(&mut self, val: u8) { /*impl*/ }
    fn length(&self) -> u8 { /*impl*/ }
    fn set_length(&mut self, val: u8) { /*impl*/ }
}
```
