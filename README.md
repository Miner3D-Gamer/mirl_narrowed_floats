# Mirl Narrowed Floats

### Minaf - Limit float capabilities to avoid having to handle edge cases (NoNaN, NoInf, NoZero, PositiveOnly, NegativeOnly)

<details>
<summary>Flags</summary>

### Default:

**Core**

- `std` (Default)

</details>

### Purpose

This crate allows you to restrict the value ranges of numbers dynamically.

It uses [`mirl_extensions_core`](https://github.com/Miner3D-Gamer/mirl_extensions_core) and [`mirl_extensions_math`](https://github.com/Miner3D-Gamer/mirl_extensions_math) to dynamically support any type, given they implement the right traits.

| Type                      | `-∞` | `]-∞; -0.0[` | `-0.0` | `+0.0` | `]+0.0; +∞[` | `+∞` | `NaN` |
| ------------------------- | ---- | ------------ | ------ | ------ | ------------ | ---- | ----- |
| [`NonNaNPositive`]        | ❌   | ❌           | ❌     | ✔️     | ✔️           | ✔️   | ✔️    |
| [`NonNaNNegative`]        | ✔️   | ✔️           | ✔️     | ❌     | ❌           | ❌   | ✔️    |
| [`NonNaN`]                | ✔️   | ✔️           | ✔️     | ✔️     | ✔️           | ✔️   | ❌    |
| [`NonNaNFinite`]          | ❌   | ✔️           | ✔️     | ✔️     | ✔️           | ❌   | ❌    |
| [`NonZeroNonNaN`]         | ✔️   | ✔️           | ❌     | ❌     | ✔️           | ✔️   | ❌    |
| [`NonZeroNonNaNFinite`]   | ❌   | ✔️           | ❌     | ❌     | ✔️           | ❌   | ❌    |
| [`Positive`]              | ❌   | ❌           | ❌     | ✔️     | ✔️           | ✔️   | ❌    |
| [`PositiveFinite`]        | ❌   | ❌           | ❌     | ✔️     | ✔️           | ❌   | ❌    |
| [`NonZeroPositive`]       | ❌   | ❌           | ❌     | ❌     | ✔️           | ✔️   | ❌    |
| [`NonZeroPositiveFinite`] | ❌   | ❌           | ❌     | ❌     | ✔️           | ❌   | ❌    |
| [`Negative`]              | ✔️   | ✔️           | ✔️     | ❌     | ❌           | ❌   | ❌    |
| [`NegativeFinite`]        | ❌   | ✔️           | ✔️     | ❌     | ❌           | ❌   | ❌    |
| [`NonZeroNegative`]       | ✔️   | ✔️           | ❌     | ❌     | ❌           | ❌   | ❌    |
| [`NonZeroNegativeFinite`] | ❌   | ✔️           | ❌     | ❌     | ❌           | ❌   | ❌    |

Special types:
- `Normalized`: Supports `0.0; 1.0`
- `SymmetricallyNormalized`: Supports `-1.0; 1.0`
