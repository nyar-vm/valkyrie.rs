(call/apply f (apply))
(call/apply f (apply 1))
(call/apply f (apply (k 1)))
(call/apply f (apply null (a 1) (⁑ args) (⁂ kwargs)))
(call/apply (call/apply (call/apply (call/apply f (apply)) (apply 1)) (apply 1 2)) (apply 1 2 3))
(call/apply a (apply-dot placeholder b))
(call/apply a (apply-dot placeholder b))
(call/apply (call/apply a (apply-dot placeholder b)) (apply-dot placeholder c))
(call/apply
  (call/apply
    (call/apply (call/apply (call/apply (call/apply a (apply 1)) (apply-dot placeholder b 1)) (apply 2)) (apply-dot placeholder b 1))
    (apply 2))
  (apply 3))
