






```
let a1: A = default;
{
    let a1: A = default; // shadow 
    let a2: A = a1;      // move
    {
        let a2: A = default;    // shadow
        let a3: A = a1.clone(); // not move
    }
    a1
    a2
}
a1
```




```
let a1: A = default;
{
    // a1 -> a1'2
    let a1'2: A = default; // shadow 
    let a2: A = a1'2;      // move
    {
        // a2 -> a2'2
        let a2'2: A = default;    // shadow
        let a3: A = a1'2.clone(); // not move
    }
    a1 //-> a1'2
    a2
}
a1
```
