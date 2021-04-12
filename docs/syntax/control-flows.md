# Control Flows

## Conditional Expression

### Basic syntax

```vyasa
condition ? if_expression

condition ? if_expression !? else_expression
```

### With Blocks

Inline expressions could be expanded by a block

```vyasa
condition ?
    if_expression1
    if_expression2
    ...

condition ?
    if_expression1
    if_expression2
    ...
!?
    else_expression1
    else_expression2
    ...
```

### Multiple Else-If

Multiple else-if logics could be chained simply by nested condition expressions:

```vyasa
condition1 ? expression1 !? condition2 ? expression2 !? else_expression

condition1 ?
    expression1
    ...
!? condition2 ?
    expression2
    ...
!?
    else_expression
    ...
```

### Resulting Value

The resulting value of a conditional expression is a union type of both branches, that missing else branch is regarded as an empty type `()`

```vyasa
// r: i32 | srting
r = c ?
    1i32
!?
    "a"

// r: i32 | ()
r = c ?
    1i32

// r: i32 | string | bool
r = c1 ?
    1i32
!? c2 ?
    "a"
!?
    true
```

## Loop Expression

### Syntax

Loop expressions are simply like conditional expressions which the `?` is replaced with `^`:

```vyasa
condition ^ body_expression

condition ^
    body_expressions
    ...
```

### Resulting Value

Loop expression generates an array of each results:

```vyasa
// r = { 0, 1, 2, 3, 4}
i = 0
r = 1 < 5 ^
    v = i
    i += 1
    v
```

### Generator Loop Expression

Generator loop expressions are deferred loops that results in a iterator for each result

#### Syntax

```vyasa
condition ^^ body_expression
```

#### Example

```vyasa
// r is an iterator that emits 0, 1, 2, 3, 4
i = 0
r = 1 < 5 ^^
    v = i
    i += 1
    v
```
