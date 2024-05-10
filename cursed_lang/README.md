# Cursed language

Imagine the paren-stacking of lisp. With the evaluation order of forth. And the type safety of php. That's about what this is.

## Syntax

The syntax is tiny. Special characters are ()' and space/newline. Parentheses define a string and pushes it to the stack. You have to close as many as you open for the string to end. An apostrophe quotes the next token (including itself or par-open or par-close) and pushes it to the stack. space/newline delimits tokens. Both parens and apostrophe are self-delimiting, so that 'foo'bar is equivalent to ' foo ' bar.

Non-special tokens are first checked against user defined words, then builtins, and if it is neither it's interpreted as a string and pushed on the stack.

## Functions

In this language everything is a string and that includes functions. To define a function, just bind the body of it to a name with the ! word. Typing the name of a variable will load the string and evaluate it.

If evaluation is not desired, then the string itself can be pushed to the stack with the @ word.

## Words

For a full list of words, refer to the source itself.

## Examples

Hello world
```
(Hello world!) print
```

Factorial
```
(dup 1 > (dup 1 - factorial *) if) factorial !
10 factorial print
```

Fizzbuzz
```
0 a ! 0 b ! 0 c !

((()) 'd !) resetd !
(a 3 = (d fizz + 'd ! 0 'a !) if) addfizz !
(b 5 = (d buzz + 'd ! 0 'b !) if) addbuzz !
(d () = (d c + 'd !) if) fallback !

(resetd addfizz addbuzz fallback d print) printone !

(c 100 <) (a 1 + 'a ! b 1 + 'b ! c 1 + 'c ! printone) loop
```
