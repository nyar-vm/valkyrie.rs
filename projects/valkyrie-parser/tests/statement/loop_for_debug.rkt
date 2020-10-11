(loop/for (iterator list) (pattern ((i))) (looping) otherwise ())
(loop/for (iterator (call/subscript1 ℤ (subscript1 (range (- 1) 1 null)))) (pattern ((i))) () otherwise ())
(loop/for (iterator (call/subscript0 ℤ (subscript0 (range (- 1) 1 2)))) (pattern ((i))) condition (if (> i 0)) () otherwise ())
(loop/for
  (iterator dict)
  (pattern ((k) (v)))
  ((loop/for (iterator list) (pattern ((i) (j))) (looping) otherwise ()))
  otherwise
  ())
(loop/for (iterator dict) (pattern ((k) (mut v))) condition (if (> k 0)) (looping) otherwise ())
(loop/for (iterator dict) (pattern ((k) (mut v))) condition (if (> k 0)) (looping) otherwise ())
(call/lambda else (nothing))
