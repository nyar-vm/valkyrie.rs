(table 1)
(table (+ 1 2))
(table (+ 1 2) 3)
(table (+ 1 2) 3 (table))
(table (+ 1 2) 3 (table 4 5))
(table (+ 1 2) 3 (table 4 5 (table)))
(table (table) (table (table)) (table (table) (table (table))))
(table v (table) (table (+ v 1)) (pair a (table)) (pair b (table (+ v 2))) (⁑ args) (⁂ kwargs))