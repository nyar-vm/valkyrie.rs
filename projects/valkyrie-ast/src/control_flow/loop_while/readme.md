`while cond {...} otherwise {...}`


```vk
outer: while cond1 {
    foo();
    while cond2  {
        bar();
        if cond3 {
            break outer;
        }
        baz();
    }
    qux();
}
```