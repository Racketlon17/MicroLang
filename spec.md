# MicroLang Language Specification
**Version:** 1.0

---

## 1. Lexical Rules

| Element | Pattern |
|---|---|
| Identifiers | `[a-zA-Z_][a-zA-Z0-9_]` |
| Numbers | `[0-9]+` |
| Keywords | `printf` `var` `const` `int` `str` `float` `bool` `toStr` `toInt` |
| Operators | `+` `-` `*` `/` `=` `or` `and` `xor` `nand` `==` `!=` `<=` `>=` |
| Whitespace | Ignored except as separator |
| String literals | Characters enclosed in double quotes `"..."` |

---

## 2. Syntax

- Identifiers are letters or underscores followed by letters, digits, or underscores
- Numbers are integers only
- Statements are separated by newlines or semicolons

---

### Variable Declaration

```
var [variable_name] :: [data_type]
```

```
var num1 :: int
num1 = 1
```

Multiple variables of the same type can be declared on one line:

```
var [variable_name_1], [variable_name_2] :: [data_type]
```

---

### Constant Declaration

```
const [constant_name] :: [data_type]
```

---

### Assignment

```
[variable_name] = [x]
```

---

### Expressions

```
[x] [symbol] [y]
```

```
a = b + 2
a = a * 3
```

---

### Strings

Strings are either in quotations or converted using `toStr`.

```
a = "Hi"
```

---

### `toStr(non_string_value)`

Converts a value to its string representation.

```
a = toStr(3)  →  "3"
```

---

### `toInt(non_integer_value)`

Converts a value to an integer. Floats are truncated.

```
a = toInt("3")     →  3
a = toInt(3.4264)  →  3
```

---

### `printf(statement)`

Prints the given statement.

```
printf("Hello, World!")  →  Hello, World!
```

---

### Booleans

Boolean values can be represented as `true`, `false`, `1`, or `0`.

```
var inpt1, inpt2 :: bool
inpt1 = true
input2 = false
printf(inpt1 and input2)  →  false
```
