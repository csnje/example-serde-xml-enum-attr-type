# About

Example implementation of serializing to and deserializing from **XML** an enumeration of structures in **Rust**.
The notable aspects of this method are:
* the type of the enumeration is a namespaced attribute of the **XML** element
* the data of the enumerated structure is *flattened* as children of the **XML** element

## Example

For **Rust** code like

```Rust
struct TypeA {
    field_a: ...,
}

struct TypeB {
    field_b: ...,
}

enum Choice {
    TypeA(TypeA),
    TypeB(TypeB),
}
```

the desired **XML** is

```XML
<element xsi:type="TypeA" xlmns:xsi="http://www.w3.org/2001/XMLSchema-instance"><field_a>...</field_a></element>
```

## Motivation

The popular [`serde`](https://crates.io/crates/serde) **XML** implementations such as [`quick-xml`](https://crates.io/crates/quick-xml) and [`serde-xml-rs`](https://crates.io/crates/serde-xml-rs) do not readily support such (de)serialize out-of-the-box.

## Technical Notes

Uses the [`serde-value`](https://crates.io/crates/serde-value) crate as an intermediary during deserialization to extract the type name.
See tests in the `serde-value` implementation for example usage.

## References

* **serde**: [Conditionally deserialize sub-struct based on visited values](https://github.com/serde-rs/serde/issues/1470)
* **Stack Overflow**: [serde: deserialize a field based on the value of another field](https://stackoverflow.com/questions/69767906)
* **Stack Overflow**: [Deserialization of json with serde by a numerical value as type identifier](https://stackoverflow.com/questions/65575385)
