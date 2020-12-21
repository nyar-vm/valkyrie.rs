## Template

```awsl
<script>
	let name = 'world';
</script>

<h1>Hello world!</h1>
```

Equivalent to

```vk
let name = "world"
h1 {
    bind(name)
    style("text-sm-400")
    "Hello world!"
}
```

## raw

```awsl
<h1>Hello {name}!</h1>
<raw>Hello {name}!</raw>
```

Equivalent to

```vk
h1 {
   "Hello {name}!"
   r"Hello {name}!"
}
```

## If Statement

```awsl
<if cond>
    a is bigger than b
</if>
```

Equivalent to

```vk
if cond {
    "a is bigger than b"
}
```

## If Else Statement

```awsl
<if cond>
    a is bigger than b
<else/>
    b is bigger than a
</if>
```

Equivalent to

```vk
if cond {
    "a is bigger than b"
} else {
    "b is bigger than a"
}
```

## Switch Statement

```awsl
<switch>
<case cond/>
    b is bigger than a
<case cond/>
    b is bigger than a
</switch>
```

Equivalent to

```vk
switch {
    case cond: 
        "b is bigger than a"
    case cond:
        "b is bigger than a"
}
```

## Try Statement

```awsl
<try T?>


</try>
```

Equivalent to

```vk
try {
    T?
}
```

## For Statement

```awsl
<for i, j in range(0, 10)>
    <if i == 5>
        1
    </if>
    <if i == 3>
        1
    </if>
    <if i == 7>
        1
    </if>
    <if i == 9>
        1
    </if>
<otherwise/>
    nothing to do
</for>
```

Equivalent to

```vk
for i, j in range(0, 10) {
    if i == 5 {
        "1"
    }
    if i == 3 {
        "1"
    }
    if i == 7 {
        "1"
    }
    if i == 9 {
        "1"
    }
}
otherwise {
    "nothing to do"
}
```

```awsl
<button on:click={handleClick}>
	Clicked {count} {count === 1 ? 'time' : 'times'}
</button>
```

```vk
button {
    event(click, handleClick)
    "Clicked {count} {count === 1 ? 'time' : 'times'}"
}
```