## 泛型语法


class A<G=()>() {}
  
```js
A::<int>(0)
A(0, G: int)
A⦇int⦈(0)
A⦅int⦆(0)
A⦓int⦔(0)
A⟅ing⟆(30)
```



array -> Vec
ArrayView -> []
List -> VecDeque


String -> String
StringView -> str

Avoiding The Fencepost Problem

```
a[1, 1:-1]
```