# numas
[![Build Status](https://travis-ci.com/numaslibrary/numas.svg?branch=master)](https://travis-ci.com/numaslibrary/numas)
[![](http://meritbadge.herokuapp.com/numas)](https://crates.io/crates/numas)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

numas is Rust library implementing n-dimensional array for generic types.

## Content

- Multidimensional array support
- Multidimensional view support
- Random factories, fillers and other factories
- Hyperbolic, trigonometric, arithmetic, logarithmic, exponential and rounding functions for arrays


## Usage

### Initializing array
Array can be initialized in many ways, for example factories, or calling `new` method.

```rust
let array = Array::new(vec![1,2,3,4,5,6], vec![2, 3]);
// Creates array with elements from 1 to 6 and shape of two dimensions with 2 and 3 length
```

### Reshaping
Sometimes is needed to change shape of an array. Actually there are two ways of doing that.

- `reshape`
Using `reshape` returns array which method is called on. This behavior enables fluent/builder pattern interface resulting in method chaining posibility.
- `set_shape`
Method `set_shape` only sets shape and has no return value.

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
let mut view = array.get(s![3 => 7]);

// Multiply view by 10, also possible to pass whole array with same shape as view
view *= u![10];

// Now elements in view are 40, 50, 60, 70
// Modifying elements in view also affects origin array
// origin array now contains elements 1, 2, 3, 40, 50, 60, 70, 8, 9
```

### Macros
Currently there are following macros:

#### s
Macro `s!` is for convenient indexing of an array. Example of usage is following

```rust
let view = array.get(s![0; 1 => 3]);
// Array view now contains first row of array columns from index 1 to 3 (excluded) and its shape is onedimensional of length 2
```
#### u
Macro `u!` is for creating "unit arrays". Unit arrays are arrays of exactly one element with shape of one dimension of length one.
```rust
let array = u![7];
// Creates array of element 7
// Equivalent would be Array::new(vec![7], vec![1]);
```

#### tuple
Macro `tuple!` is an internal macro used inside of `u!`. It is not supposed to be used by user. It pushes value with `0` to provided
vector if only one value is provided. If two values are provided, it pushes both of them.
```rust
let mut vec: Vec<i32> = Vec::new();
tuple![vec; 1, 5];
// vec now contains [1, 5]

tuple![vec; 6]
// vec now contains [1, 5, 6, 0]
```

