


```valkyrie
await? Awaitable -> !
let data = await! Awaitable // block on
let data = await Awaitable // await
```


## 批量异步任务

- 错误写法:

```valkyrie
let data1 = await Awaitable
let data2 = await Awaitable
return (data1, data2)
```



- 正确写法:

```valkyrie
let async data1 = Awaitable
let async data2 = Awaitable
return await (data1, data2)
```



## `async` 传染


- 中断 `async` 传染

保证后续不再使用 `awaitable` 值, 此时 `async` 不再传染

await* Awaitable<T> -> !

- 强制同步

令调度器强制同步, 此时 `async` 不再传染

await! Awaitable<T> -> T

