(component $root
    (core module $MockMemory
        (func $realloc (export "realloc") (param i32 i32 i32 i32) (result i32)
            (i32.const 0)
        )
        (memory $memory (export "memory") 15)
    )
    (core instance $memory (instantiate $MockMemory))
    (import "wasi:io/streams" (instance $wasi:io/streams
        (export $std::io::InputStream "input-stream" (type (sub resource)))
        (export $std::io::OutputStream "output-stream" (type (sub resource)))
    ))
    (alias export $wasi:io/streams "input-stream" (type $std::io::InputStream))
    (alias export $wasi:io/streams "output-stream" (type $std::io::OutputStream))
    (import "wasi:io/error" (instance $wasi:io/error
        (export $std::io::IoError "error" (type (sub resource)))
    ))
    (alias export $wasi:io/error "error" (type $std::io::IoError))
    (import "wasi:cli/stderr" (instance $wasi:cli/stderr
        (export "get-stderr" (func
        ))
    ))
    (alias export $wasi:cli/stderr "get-stderr" (func $std::io::standard_error))
    (import "wasi:cli/stdin" (instance $wasi:cli/stdin
        (export "get-stdin" (func
        ))
    ))
    (alias export $wasi:cli/stdin "get-stdin" (func $std::io::standard_input))
    (import "wasi:cli/stdout" (instance $wasi:cli/stdout
        (export "get-stdout" (func
        ))
    ))
    (alias export $wasi:cli/stdout "get-stdout" (func $std::io::standard_output))
    (core func $std::io::standard_error (canon lower
        (func $wasi:cli/stderr "get-stderr")
        (memory $memory "memory")(realloc (func $memory "realloc"))
        string-encoding=utf8
    ))
    (core func $std::io::standard_input (canon lower
        (func $wasi:cli/stdin "get-stdin")
        (memory $memory "memory")(realloc (func $memory "realloc"))
        string-encoding=utf8
    ))
    (core func $std::io::standard_output (canon lower
        (func $wasi:cli/stdout "get-stdout")
        (memory $memory "memory")(realloc (func $memory "realloc"))
        string-encoding=utf8
    ))
    (core module $Main
        
        
        
        
        (import "wasi:cli/stderr" "get-stderr" (func $std::io::standard_error))
        
        (import "wasi:cli/stdin" "get-stdin" (func $std::io::standard_input))
        
        (import "wasi:cli/stdout" "get-stdout" (func $std::io::standard_output))
    )
    (core instance $main (instantiate $Main
        (with "wasi:io/streams" (instance
        ))
        (with "wasi:io/error" (instance
        ))
        (with "wasi:cli/stderr" (instance
            (export "get-stderr" (func $std::io::standard_error))
        ))
        (with "wasi:cli/stdin" (instance
            (export "get-stdin" (func $std::io::standard_input))
        ))
        (with "wasi:cli/stdout" (instance
            (export "get-stdout" (func $std::io::standard_output))
        ))
    ))
)