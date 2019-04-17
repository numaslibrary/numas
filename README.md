# numas
numas is Rust library implementing n-dimensional array for generic types.

#### New in numas 0.1.9
- Rounding functions
- Logarithm functions
- Bug fixes

## Content

Library contains:
- Multidimensional array support
- Multidimensional view support
- Random factories, fillers and other factories
- Hyperbolic, trigonometric, arithmetic, logarithmic and rounding functions for arrays


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

### Functions
The majority of functions are accessible via an array instance.

```rust

let array = Array::new(vec![1,2,3,4,5], vec![5]);
array.sin(); // Returns new array with elements equal to sin(1), sin(2)...
array.cos();
array.sqrt();
//etc.
```

### Operators
Currently there are supported basic mathematical operators on array - add, sub, mul, div and negation, all element wise.
Also theirs assign equivalents are implemented (+=, -= ...)

### View
numas implements views the same way as array, so views act as arrays and it's possible to perform all operations as on array, but
modifying elements affects the origin array as well.

```rust
// Create new linear array with elements from 1 to 9
let array = Array::new(vec![1,2,3,4,5,6,7,8,9], vec![9]);

// Create view into array of index from 3 to 6 (elements 4,5,6,7)
let mut view = array.get(s![3 => 6]);

// Multiply view by 10, also possible to pass whole array with same shape as view
view *= u![10];

// Now elements in view are 40, 50, 60, 70
// Modifying elements in view also affects origin array
// origin array now contains elements 1, 2, 3, 40, 50, 60, 70, 8, 9
```

### Macros
Currently there are two macros `s` for convenient indexing of array and `u` for creating 'unit arrays'.

#### s
```rust
let view = array.get(s![0, 1 => 2]);
// Array view now contains first row of array columns from index 1 to 2 and its shape is onedimensional of length 2
```
#### u
Unit arrays are arrays of exactly one element with shape of one dimension of length one.
```rust
let array = u![7];
// Creates array of element 7
// Equivalent would be Array::new(vec![7], vec![1]);
```
