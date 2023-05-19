# About

Example in **Rust** of (de)serializing **XML** to/from an enumeration with variants of structures.

Notable aspects of this implementation are:
* the enumeration variant is given as an attribute of the **XML** element
* the fields of the variant's structure are *flattened* as children of the **XML** element

# Example

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

let choice = Choice::TypeA(TypeA { field_a: ... });
```

the desired **XML** is

```XML
<element xsi:type="TypeA" xlmns:xsi="http://www.w3.org/2001/XMLSchema-instance"><field_a>...</field_a></element>
```

# Motivation

Commonly used [`serde`](https://crates.io/crates/serde) **XML** implementations such as [`quick-xml`](https://crates.io/crates/quick-xml) and [`serde-xml-rs`](https://crates.io/crates/serde-xml-rs) do not readily support this (de)serialization due to a few implementation quirks.
This is largely due to **XML** being a data format that can (de)serialize in many ways.

# Technical Notes

Deserialization uses the [`serde-value`](https://crates.io/crates/serde-value) crate as an intermediary to provide access the variant name.

# References

* **serde**: [Conditionally deserialize sub-struct based on visited values](https://github.com/serde-rs/serde/issues/1470)
* **Stack Overflow**: [serde: deserialize a field based on the value of another field](https://stackoverflow.com/questions/69767906)
* **Stack Overflow**: [Deserialization of json with serde by a numerical value as type identifier](https://stackoverflow.com/questions/65575385)
