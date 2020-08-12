(micro function (arguments) (body))
(macro module∷function (arguments) (body))
(micro function (arguments ((a) Any null) ((k) T null) ((p) U 1)) (body (call/apply print (apply Hello, world!))))
(micro function (arguments ((g) G null)) (body (+ 1 1) (∈ a b) (call/apply print (apply Hello, world!))))
(macro
  outer
  (arguments ((lhs) L null))
  (body (micro inner (arguments ((rhs) R null)) (body (loop (< count 10) (+= count 1) (call/apply print (apply {lhs} {rhs}))))) {count}))
(micro main (arguments ((mut args) (table String) null)) (body (call/apply (call/apply outer (apply Hello)) (apply world)) (return 0)))
