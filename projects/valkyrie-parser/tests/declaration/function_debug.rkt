(micro function (arguments) (body))
(macro module∷function (arguments) (body))
(micro function (arguments ((a) Any null) ((k) T null) ((p) U 1)) (body (apply print Hello, world!)))
(micro function (arguments ((g) G null)) (body (+ 1 1) (∈ a b) (apply print Hello, world!)))
(macro
  outer
  (arguments ((lhs) L null))
  (body (micro inner (arguments ((rhs) R null)) (body (loop (+= count 1) (apply print {lhs} {rhs})))) {count}))
(micro main (arguments ((mut args) (table String) null)) (body (apply (apply outer Hello) world) return 0))
