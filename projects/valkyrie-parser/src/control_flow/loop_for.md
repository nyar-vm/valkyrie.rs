


## Simple For Loop

```v
for i in j if c1 {
    if a1 {
        continue
    }
    else if a2 {
        break
    }
    else {
        return
    }
}
```


```v
j = j.into_iterator()
loop {
    let next = j.next()
    match j.next() {
        case Some(i) if c1:
            if a1 {
                continue
            }
            else if a2 {
                break
            }
            else {
                return
            }
        case _:
            break
    }
}
```


```v
j = j.into_iterator()
loop {
    ᳀label(for.1.start)
    let next = j.next()
    match j.next() {
        case Some(i) if c1:
            if a1 {
                ⤮for.1.start
            }
            else if a2 {
                ⤮for.1.tail
            }
            else {
                ⤮function.return
            }
        case _:
            ⤮for.1.tail
    }
    ᳀for.1.end
}
᳀for.1.tail

᳀function.1.return
```

## Nested For Loop