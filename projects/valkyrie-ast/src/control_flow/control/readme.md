

```
type Continuation<..T> = Function<[..T], ()>

type Action<..T> = Continuation<..T>

@resource
let mut file = File::open("test.txt");
```


```
yield file.next();
fallthough(unchecked)
```


```
let `.ret` = file.next();
file.dispose();
return `.ret`
```