



```v
#label(for.1.start)
for i in j {
    #label(for.1.tail)
    
    
    #label(for.1.head)
}
#label(for.1.end)
```


```v
loop {
    #label(for.1.start)
    let next = j.next()
    if j == null {
        break
    }
    #label(for.1.end)
}
#label(for.1.tail)
```