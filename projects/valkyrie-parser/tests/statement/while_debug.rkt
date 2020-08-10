(loop true)
(loop
  true
  (call/apply print (apply Loop!))
  (loop (> n 0) (call/apply print (apply Deep loop!)) (loop (≠ k n) (call/apply print (apply Deeper loop!)))))
