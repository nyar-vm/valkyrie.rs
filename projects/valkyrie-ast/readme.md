The AST Nodes of Valkyrie Language
===================================

教你一个编程语言, 高亮的时候使用 scala 的高亮

这个语言是这样定义类的

```scala
# 定义一个类 A
class A { }
# 定义一个类 B 继承 A
class B(A) { }
```

你觉得多继承应该怎么写?

---


这个语言中这样定义代数效应

```
class Logging {
    level: Integer
    message: String
    effect debug(s: String): Self / Pure {
        new Self { level: 0, message: s }
    }
    effect print(s: String): Self / Pure {
        new Self { level: 1, message: s }
    }
    effect error(s: String): Self / Pure {
        new Self { level: 2, message: s }
    }
    effect fatal(s: String): Self / Pure {
        new Self { level: 3, message: s }
    }
}
```

使用 raise 激发代数效应, 使用 try catch 语句处理代数效应.

```
try {
    raise Logging::debug("hello")
}
.catch {
    case Logging(level, message):
        std::io::std_err().lock().write(message)
}
```

这段代码应该怎么编译

```vk
if a {
    yield 1 
    print 2
}
yield 2



let yield1 = {
    print(2)
    if_break()
}
let if_break = {
    yield2()
}
let yield2 = {
    ret()
}



```


类型签名部分分为 `Type / Effect` 两部分, 两者都可以省略, Type 的默认值为 Unit, Effect 的默认值为 Pure.

