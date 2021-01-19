


```shell
java -jar ./antlr4-4.8-2-SNAPSHOT-complete.jar -o ./src/antlr -encoding utf8 -listener -visitor -Dlanguage=Rust ValkyrieAntlrLexer.g4; 
java -jar ./antlr4-4.8-2-SNAPSHOT-complete.jar -o ./src/antlr -encoding utf8 -listener -visitor -Dlanguage=Rust ValkyrieAntlr.g4; 
cargo fmt;
```