(call/subscript1 a (subscript1 placeholder 1))
(call/subscript1 a (subscript1 placeholder (table 1)))
(call/subscript1 a (subscript1 placeholder 2 (+ 1 1)))
(call/subscript1 a (subscript1 placeholder (range 1 1 null)))
(call/subscript1 a (subscript1 placeholder (range 2 2 null) (range (+ 1 1) (+ 1 1) null)))
(call/subscript1 e (subscript1 placeholder (range 1 2 3) (table 1 2 3)))
(call/subscript1 a (subscript1 placeholder (range null null (- 1))))
(call/subscript1 a (subscript1 placeholder (range null (- 2) null)))
(call/subscript1 a (subscript1 placeholder (range 2 null null)))
(call/subscript1 a (subscript1 placeholder (+ 1 1)))
(+ (call/subscript1 b (subscript1 placeholder (+ 1 1))) 1)
(call/subscript1 c (subscript1 placeholder 1 2 3))
(call/subscript1 d (subscript1 placeholder (table 1 2 3)))
(call/subscript1 e (subscript1 placeholder (range 1 2 3) (table 1 2 3)))
(call/subscript1
  (call/subscript1 (call/subscript1 array (subscript1 placeholder)) (subscript1 placeholder (range null null null)))
  (subscript1 placeholder (range null null null)))
(call/subscript1 array (subscript1 placeholder (range null null null)))
(call/subscript1
  array
  (subscript1
    placeholder
    (range null null null)
    (range null null null)
    (range null null null)
    1
    (range null index0 null)
    (range null null (- 1))
    i∷j
    (range i null j)))
(call/subscript0
  (call/subscript0 (call/subscript0 array (subscript0 placeholder)) (subscript0 placeholder (range null null null)))
  (subscript0 placeholder (range null null null)))
(call/subscript0 array (subscript0 placeholder (range null null null)))
(call/subscript0
  array
  (subscript0
    placeholder
    (range null null null)
    (range null null null)
    (range null null null)
    1
    (range null index0 null)
    (range null null (- 1))
    i∷j
    (range i null j)))
