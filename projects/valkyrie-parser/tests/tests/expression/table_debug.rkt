(table 1)
(table (+ 1 2))
(table (+ 1 2) 3)
(table (+ 1 2) 3 (table))
(table (+ 1 2) 3 (table 4 5))
(table (+ 1 2) 3 (table 4 5 (table)))
(table (table) (table (table)) (table (table) (table (table))))
(call/subscript1 array (subscript1))
(call/subscript0 array (subscript0))
(call/subscript1 array (subscript1 0))
(call/subscript0 array (subscript0 1))
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
(table v (table) (table (+ v 1)) (a (table)) (b (table (+ v 2))) (⁑ args) (⁂ kwargs))