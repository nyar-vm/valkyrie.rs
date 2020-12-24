

```scala
class C(B1, B2) { }

structure C(B1, B2) {
    _b1: B1,
    _b2: B2,
}
imply C: Cast(B1) {
    cast(self): B1 {
        self._b1;
    }
}
imply C: Cast(B2) {
    cast(self): B2 {
        self._b2;
    }
}
```



```vk
class A {
    @get
    _field_a: i32;
    
    set field_a() {
        self._field_a = value;
    }
    
    set field_b(mut self, value: i32) {
        self._field_b = value;
    }
}

effect AskName(Integer) -> String;
 
def ask_name(user): Unit / AskName {
    var name: String? = user.name;
    if name == null {
        name = raise AskName(user.id);
    }
    print("你的名字是：{name}")
}

try {
    ask_name({ id: 24, name: "是学生" });
    ask_name({ id: 10 });
}
catch AskName(id) {
    if id > 10 {
        return 0
    }
    else {
        resume "无名氏";
    }
}
return -1



class Point {
    x: int,
    y: int,
}

refine interface Positive {
    get positive(self): bool;
}

refine interface Negative {
    get negative(self): bool;
}

refine interfae Mutable {
    get mutable(self): bool;
    set mutable(self, value: bool);
}


refine interface Scalable {
    get scalable(self): bool;
    set scalable(self, value: bool);
}

refine interface Modifiable: Scalable {
    final get modifiable(self): bool {
        if self.scalable {
            return true;
        }
        my_modifiable
    }
    final set modifiable(self, value: bool) {
        my_modifiable = value;
    }
    
    private my_modifiable: bool
}
```



```vk
let a = 1;

fn test() {
    let a = 2;
    let b = 1;
    {
        b = 2;
        a = 3;
        // do things with effect
    }
    let a = 4;
    // do things with effect
}




```


```vk
effect DivideZeroError(numerator: Integer): Integer;

def lower(a: Integer, b:Integer): Integer / DivideZeroError {
    if b != 0 {
        a / b
    }
    else {
        raise DivideZeroError(a)
    }
}

def middle(): Integer / DivideZeroError {
    lower(1, 0)
}

def higher(): Integer {
    try {
        middle()
    }
    catch DivideZeroError(e) {
        if a >= 0 {
            print(e)
            resume 0
        }
        else {
            return -1
        }
    }
}
```


```logger
final singleton class Logger {
    event logging(
        **messages: UTF8Text, 
        <, >, 
        eos: UTF8Text
    );
    trace(messages: UTF8Text, <, >) {
        logging(messages, separator: separator, eos: eos, level: LogLevel::Trace);
    }
    debug(**messages: UTF8Text, <, >, separator: UTF8Text = '', eos: UTF8Text = '\n') {
        logging(messages, separator: separator, eos: eos, level: LogLevel::Debug);
    }
    print(**messages: UTF8Text, <, >, separator: UTF8Text = '', eos: UTF8Text = '\n') {
        let mut first = true;
        let mut std_out = std::io::out().lock();
        for s in messages {
            if first {
                first = false;
                write(io, s);
            }
            else {
                write(io, separator);
                write(io, s);
            }
        }
        write(io, eos);
    }
    alert(**messages: UTF8Text, <, >, separator: UTF8Text = '', eos: UTF8Text = '\n') {
        logging(messages, separator: separator, eos: eos, level: LogLevel::Error);
    }
    error(**messages: UTF8Text, <, >, separator: UTF8Text = '', eos: UTF8Text = '\n') {
        logging(messages, separator: separator, eos: eos, level: LogLevel::Error);
    }
    fatal(**messages: UTF8Text, <, >, separator: UTF8Text = '', eos: UTF8Text = '\n') {
        logging(messages, separator: separator, eos: eos, level: LogLevel::Fatal);
    }
    initialize() {
        self.logging += {
            let mut first = true;
            let mut std_out = std::io::out().lock();
            for s in messages {
                if first {
                    first = false;
                    write(io, s);
                }
                else {
                    write(io, separator);
                    write(io, s);
                }
            }
            write(io, eos);
        }
    }
    reinitialize() {
        self.print.clear()
    }
    deinitialize() {
    
    }
}
```


```vk
micro DivideZeroError(numerator: Integer): Integer {
    print(numerator)
    return 0
}

def lower(a: Integer, b:Integer, e: DivideZeroError): Integer {
    if b != 0 {
        a / b
    }
    else {
        if a >= 0 {
            print(e)
            return 0
        }
        else {
            return(higher)
        }
    }
}

def middle(e: DivideZeroError): Integer {
    lower(1, 0)
}

def higher(return): Integer {
    let mut error = null;
    middle(error)
    if a >= 0 {
        print(e)
        resume 0
    }
    else {
        return()
    }
}
```