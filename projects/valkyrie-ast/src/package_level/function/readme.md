


- explicit: 必须精确匹配类型, 但不会拒绝子类型
  - `if type is T`
- implicit(default): 尝试使用隐式转换匹配类型
  - `@cast.implicit(T, U)?`
- into: 尝试使用隐式转换匹配类型，如果失败则尝试使用显式转换
  - `@cast(T, U)?`


- take: 从上下文中强制传入名称为 args 的参数


```scala
@cps
name(args) := sequence {
    val name = "Valkyrie"
    @[yield] (tuple)
    @yield
    name
}
𝒦 = name(args)

// forbidden add vow to shortcut function
@𝒦
vow T {
    explicit return T {
        T: A
    }
}
name(args) := sequence {
    val name = "Valkyrie"
    @[yield] (tuple)
    @yield
    name
}



𝒦 micro name(exact duty args) {
    sequence {
        val name = "Valkyrie"
        @[yield] (tuple)
        @yield
        name
    }
}
```