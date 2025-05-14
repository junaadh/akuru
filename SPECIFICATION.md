# Akuru Specifications

## 1. **Variables**
- declared with `let`, and by default immutable
- add `mut` keyword to make it mutable
- constants are defined with `const` keyword
- type inferred from context or can specified
- shadowing is allowed

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

## 3. **Section**
- section is a translational unit like a module
- sections help group related items
- they act as namespaces and can be nested
- the root section is considered as the script root 
 
```rust
section math {
  fn add(x: u8, y: u8): u8 { x + y }
  fn sub(x: u8, y: u8): u8 { x - y }

  section internal {
    const identity = 0;
  }
}
```

## 4. **Statments and Expressions**
- most constructs in akuru are expressions
- statements include
  - let declerations
  - expression stmts
  - semi-expression (expr;)

```rust
let value = if condition 42 else 0;

if condition1 == condition2 {
  print("true")
} else {
  print("false")
}

```

### 5. **Types (WIP)**
- algebraic types (tuples structs)
- type inference
- type aliasing
- generics

