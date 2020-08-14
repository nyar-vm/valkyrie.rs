## Tests

```bash
wee test
```


# nullable

```vk
# A | throw
a?
a.match {
    case Some(a) => a
    case None => raise NPE
}
a.match {
    case Success(s) => a
    case Failure(e) => raise e
}


# B?
a?.b
a.and_then(a => a.c)


# B | throw
a?.b?



# C | throw
a?.b?.c

a.and_then(a => ErrorB::from(a.b))
// or explicit throw
a!.b?.c


```


````vk
a ?? b ?? c

a ?= b ?= c
````