# numas
numas is Rust library implementing n-dimensional array for generic types.

## Content

Library contains:
- Multidimensional array support
- Multidimensional view support
- Random factories, fillers and other factories
- Hyperbolic functions for arrays
- Trigonometric functions for arrays
- Arithmetic functions for arrays


## Usage

### Initializing array
Array can be initialized in many ways, for example factories, or calling `new` method.

```rust
let array = Array::new(vec![1,2,3,4,5,6], vec![2, 3]);
// Creates array with elements from 1 to 6 and shape of two dimensions with 2 and 3 length
```

### Reshaping
Sometimes is needed to change shape of an array. Actually there are two ways of doing that.
First one is via method `reshape` which returns array that it's called on enabling fluent/builder pattern interface.
Second one is method `set_shape` which just sets shape and doesnt return anything.

### Macros
Currently there is one macro `s` for convenient indexing of array.

```rust
let view = array.get(s![0, 1 => 2]);
// Array view now contains first row of array columns from index 1 to 2 and its shape is onedimensional of length 2
```

