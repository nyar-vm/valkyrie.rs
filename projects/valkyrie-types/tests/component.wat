(component $root
    (core module $MockMemory
        (func $realloc (export "realloc") (param i32 i32 i32 i32) (result i32)
            (i32.const 0)
        )
        (memory $memory (export "memory") 15)
    )
    (core instance $memory (instantiate $MockMemory))
    (import "wasi:io/streams" (instance $wasi:io/streams
        (export $package::io::InputStream "input-stream" (type (sub resource)))
        (export $package::io::OutputStream "output-stream" (type (sub resource)))
    ))
    (alias export $wasi:io/streams "input-stream" (type $package::io::InputStream))
    (alias export $wasi:io/streams "output-stream" (type $package::io::OutputStream))
    (import "wasi:io/error" (instance $wasi:io/error
        (export $package::io::IoError "error" (type (sub resource)))
    ))
    (alias export $wasi:io/error "error" (type $package::io::IoError))
    (core module $Main
        
        
        
    )
    (core instance $main (instantiate $Main
        (with "wasi:io/streams" (instance
        ))
        (with "wasi:io/error" (instance
        ))
    ))
)