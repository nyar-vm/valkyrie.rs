I regard it as syntax sugar, and then generate code to turn it into a statement block with only `let` + `if-else`

Here I would like to introduce my ideas and five basic modes. Further you can read the full document: [virtual-kontinuation#tutorial](https://valkyrie-language.netlify.app/en/#tutorial)

## Dispatch according to type

First, I decide the generated code based on the different types of return values

I defined the following type
- **Atomic**
    - number, character, string, boolean
- **List**
    - *tuple*: list of heterogeneous elements
    - *array*: list of homogeneous elements
- **Dict**: key-value pair structure, the key must be an identifier
- **Encode**: numbers encoding special information
    - *flags*: These attributes can coexist
    - *enumerate*: These properties are mutually exclusive
- **Extractor**: Custom pattern matching

Consider the following expression, statically dispatched based on type:

```valkyrie
match f(progress) {
    case 1: pass
    case "c": pass
    case (a, b): pass
    case [a, b, ..c]: pass
    case [..a, _]: pass
    case { mut a, ref b: c, d: _, .. }: pass
    case Sign::Positive: pass
    case Sign::Negative: pass
    case Person(name, age): pass
    case _: pass
}
```

## Atomic Pattern

There is no way for the user to define the Atomic Pattern, it is hard-coded in the compiler.

Let us start with `$` to represent a compiler variable, the user cannot enter such a variable.

```valkyrie
let $bind = f(progress);
if $bind == 1 {
    // do action
}
else {
    // try other compatible types
}
```

## Tuple pattern

A type can implement `TuplePattern` trait, introduce itself to return `Tuple<...>?`

```valkyrie
let (a, b) = f(progress)
```

Tuple can use `.` to extract elements

```valkyrie
let $bind = f(progress);
let a = $bind.1
let b = $bind.2
```

## Array pattern

A type can implement `ArrayPattern` trait, introduce itself to return `Array<T>?`

```valkyrie
let [a, b, ..c] = f(progress)
```

Tuple can use `[n]` to extract elements

```valkyrie
let $bind = f(progress);
let a = $bind[1]
let b = $bind[2]
let c = $bind[3, $bind.length]
```

## Array pattern reverse

It is also possible to match from back to front

```valkyrie
let [..a, _] = f(progress)
```

```valkyrie
let $bind = f(progress);
let a = $bind[1, $bind.length - 1]
let _ = $bind[$bind.length]
```

The value represented by `_` will be subjected to dead code elimination.

## Record Pattern

A type can implement `RecordPattern` trait, introduce itself to return `{ k: K, ... }?`

```valkyrie
let { mut a, ref b: c, d: _, .. } = f(progress)
```

```valkyrie
let $bind = f(progress);
let mut a = $bind.a
let ref c = $bind.b
let _ = $bind.d // dead code elimination
```

## Enumeration Pattern

The user cannot define the `EncodePattern`, the compiler derives it based on the `enumerate` and `flags` keyword.

```valkyrie
let $bind = f(progress);
if $bind == (East as u8) {
    // do action
}
else {
    // try other compatible types
}
```

## Extractor Pattern

This is the most complicated part.

Users need to implement such an trait and specify the proxy pattern.

```valkyrie
trait ExtractorPattern {
    type Pattern    
    extract(input: Any): Self::Pattern?
}
```

```valkyrie
let Persion(a) = f(progress)
```

```valkyrie
let $bind = f(progress);
let a: Persion? = Person::extract($bind)
if a.is_some() {
    let (name, age) = a.unwrap()
    // Expand according to tuple pattern
}
else {
    // Try other compatibility modes
}
```

## Nested Pattern

The compiler checks the generated bind pattern.

```valkyrie
let (a, (b, c)) = f(progress)
```

If the left side is still a pattern, it recursively expands the pattern.

```valkyrie
let $bind = f(progress);
let a = $bind.1
let (b, c) = $bind.2
```

Finally generate code like this:

```
let $bind = f(progress);
let a = $bind.1
let $bind = $bind.2
let b = $bind.1
let c = $bind.2
```