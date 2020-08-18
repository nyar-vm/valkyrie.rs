{% for i in b %}

{% end for %}

| Trait          | Formatter |
|:---------------|-----------|
| Show           | `{}`      |
| ShowDetail     | `:?`      |
| ShowPointer    | `:p`      |
| ShowBinary     | `:b`      |
| ShowOctal      | `:o`      |
| ShowHexLower   | `:x`      |
| ShowHexUpper   | `:X`      |
| ShowExpLower   | `:e`      |
| ShowExpUpper   | `:E`      |
| ShowPretty     | `:P`      |
| ShowMathInline | `:m`      |
| ShowMathBlock  | `:M`      |

```
# std.show
triat Show {
    show(self, f: Formatter): Unit;
}

# std.show
trait ShowDetail {
    show_detail(self, f: Formatter): Unit;
}

# std.show, std.show.math
trait ShowMath {
    show_math(self, f: MathFormatter): MathDisplay;
}

class MathDisplay {
    attributes: Dict<String>,
}

class MathRoot(MathDisplay) {
    children: List<MathDisplay>,
}






# std.show, std.show.pretty
trait ShowPretty {
    show_pretty(self) -> Result<Pretty, PrettyError>;
}
# std.show, std.show.html
trait ShowHtml {
    show_html(self) -> Result<Html, HtmlError>;
}
```







```
vouch T {
}
class A(U<T>) {

}

effect async {
    await<T>(k: T -> IO()): T
}
vow {
    return Unit;
    effect [async];
}
fun sleep(d: duration): () / async {
    await {
        
    }
}

```