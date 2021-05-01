

### `f(T)(x)`

歧义, 你不纯在

-  歧义, 函数返回函数咋办
- `f[T](x)` 不错, 但是你的 Slice 咋办, 不用 


那 




```vk
class A {
    a() {
        print("A.a")
    }
}
```



## 委派类

```vk
delegate class Monad<T>: A [

]
```

## 自主类

```vk
autonomy class B(A, B, C) {
    
}
```

function a() {
    print("B.a")
}

def instantly b() {
    print("B.b")
}


