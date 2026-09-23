# openpbr

The [OpenPBR Surface](https://academysoftwarefoundation.github.io/OpenPBR/)
parameter set as plain, `no_std` Rust data.

- `Parameters<CS>`: every constant parameter with the specification's
  defaults, range validation, dynamic access by `Param`, and conversion
  between linear color spaces.
- `Param` / `ParamInfo`: all 41 parameters of the specification's parameter
  reference, including the `vector3` geometry inputs, with identifier, label,
  group, type, allowed and typical ranges, default, unit, and whether the
  value is a color.

Colors are [`color`](https://crates.io/crates/color) `OpaqueColor` values
typed by their color space. OpenPBR assumes ACEScg unless a material says
otherwise, so `Parameters` defaults to `Parameters<AcesCg>`; renderers working
in linear Rec. 709 use `Parameters<LinearSrgb>` and `Parameters::convert`.

```rust
use openpbr::color::{AcesCg, LinearSrgb, OpaqueColor};
use openpbr::{Param, Parameters};

let gold = Parameters::<AcesCg> {
    base_metalness: 1.0,
    base_color: OpaqueColor::new([0.94, 0.78, 0.37]),
    specular_roughness: 0.2,
    ..Parameters::DEFAULT
};
assert_eq!(gold.validate(), Ok(()));
let for_renderer: Parameters<LinearSrgb> = gold.convert();

for param in Param::ALL {
    let info = param.info();
    println!("{} ({}) default {:?}", info.identifier, info.kind.name(), info.default);
}
```

This crate implements OpenPBR 1.1.1. It evaluates no BSDF and reads or writes
no file format; glTF, MaterialX, and renderer bindings belong in adapters.

## Features

- `std` (default) or `libm`: one is required; they select `color`'s float math.
- `serde`: identifiers as field names, colors as `[r, g, b]`, missing fields
  filled with defaults, and unknown fields rejected.

## Minimum supported Rust Version (MSRV)

This version of `openpbr` has been verified to compile with **Rust 1.88** and
later.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

OpenPBR is a specification of the Academy Software Foundation; this crate is
an independent implementation of its parameter set.
