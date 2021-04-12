# Block

Basic syntaxes in Vyasa are mostly inline expressions, which can be expanded to multiple expressions by a block.
The result value of a block is decided by the last line.

## Multiline Block

At anywhere expects an expression, new lines with aligned extra indention are regarded as a multiline block:

```vyasa
foo =
    expression1
    expression2
    ...
```

## Multiline Block With Brackets

Multiline blocks could be attached with additional brackets:

```vyasa
foo = {
    expression1
    expression2
    ...
}
```

## Inline Block

Inline blocks are expressions inside a pair of bracket divided by a semicolon, where the trailing semicolon of the last expression is optional:

```vyasa
foo = { expression1; expression2; ... }
```
