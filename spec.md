# MicroLang Language Specification
**Version:** 1.0

---

---

## 1. Lexical Rules
- **Identifiers:** `[a-zA-Z_][a-zA-Z0-9_]`
- **Numbers:** `[0-9]+`
- **Keywords:** `printf, var, const, int, str, float, bool, toStr, toInt`
- **Operators:** `+ - * / = or and xor nand == != <= >=`
- **Whitespace:** Ignored except as separator
- **String literals:** Characters enclosed in double quotes "..."
---

## 2. Syntax
- Identifiers are letters or underscores followed by letters, digits, or underscores
- Numbers are integers only
- Statements are separated by newlines or semicolons
- Variables are declared using **var [variable_name] :: [data_type]**
- Multiple variables can be declared on the same line separated by a comma if of same data type 
- **var [variable_name_1], [variable_name_2] :: [data_type]**
- Constants are declared using **const [constant_name] :: [data_type]**
- Assigning a variable or constant is as follows: **[variable_name] = [x]**
# var num1 :: int
# num1 = 1
- Expressions are as follows: **[x] [symbol] [y]**
# a = b + 2
# a = a * 3
- Strings are either in quotations or covnerted to string using the toStr function
# a = "Hi"
- The toStr function is used as followed: **toStr([non_string_value])**
# a = toStr(3) --> "3"

- The toInt function is used as followed: **toItr([non_integer_value])**
# a = toInt("3") --> 3
# a = toInt(3.4264) --> 3
# *The toInt function returns an integer value stripped of its decimal places if it is a float*

- Print statements are as follows: **printf([statement])**
# printf("Hello, World!") --> "Hello, World!"

- Boolean statements can be represented as true, false, 1 or 0
# var inpt1, inpt2 :: bool
# inpt1 = true
# input2 = false
# printf(inpt1 and input2) --> false