# Akuru Specifications

## 1. **Variables**
- declared with `let`, and by default immutable
- add `mut` keyword to make it mutable
- constants are defined with `const` keyword
- type inferred from context or can specified
- variables are shadowed

```rust
let immutable = 1;
let mut mutable = 2;
const constant = 3;

let immutable: u8 = 1;
let mut mutable: u8 = 2;
const constant: u8 = 3;
  
```

## 2. **Functions**
- functions are defined with `fn` keyword
- plan to support variadic funtion parameters
- optional parameters are also supported as well as return values
- expression based functions or closures can be written using `|`

```rust
fn add(x: u8, y: u8): u8 { x + y }
fn sub(x: u8, y: u8): u8 { return x + y; }
fn print(value: u8?) { /** compiler internal print **/ }

let mul = |x, y| x + y; 
let div = |x: u8, y: u8|: u8 { return x + y; };

```

## 3. planned but coming later
 
