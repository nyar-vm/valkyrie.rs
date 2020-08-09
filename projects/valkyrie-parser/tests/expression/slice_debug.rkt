(call/subscript1 a (subscript1 1))
(call/subscript1 a (subscript1 (table 1)))
(call/subscript1 a (subscript1 2 (+ 1 1)))
(call/subscript1 a (subscript1 (range 1 1 null)))
(call/subscript1 a (subscript1 (range 2 2 null) (range (+ 1 1) (+ 1 1) null)))
(call/subscript1 e (subscript1 (range 1 2 3) (table 1 2 3)))
(call/subscript1 a (subscript1 (range null null (- 1))))
(call/subscript1 a (subscript1 (range null (- 2) null)))
(call/subscript1 a (subscript1 (range 2 null null)))
(call/subscript1 a (subscript1 (+ 1 1)))
(+ (call/subscript1 b (subscript1 (+ 1 1))) 1)
(call/subscript1 c (subscript1 1 2 3))
(call/subscript1 d (subscript1 (table 1 2 3)))
(call/subscript1 e (subscript1 (range 1 2 3) (table 1 2 3)))
(call/subscript1 (call/subscript1 (call/subscript1 array (subscript1)) (subscript1 (range null null null))) (subscript1 (range null null null)))
(call/subscript1 array (subscript1 (range null null null)))
(call/subscript1
  array
  (subscript1
    (range null null null)
    (range null null null)
    (range null null null)
    1
    (range null index0 null)
    (range null null (- 1))
    i∷j
    (range i null j)))
(call/subscript0 (call/subscript0 (call/subscript0 array (subscript0)) (subscript0 (range null null null))) (subscript0 (range null null null)))
(call/subscript0 array (subscript0 (range null null null)))
(call/subscript0
  array
  (subscript0
    (range null null null)
    (range null null null)
    (range null null null)
    1
    (range null index0 null)
    (range null null (- 1))
    i∷j
    (range i null j)))
