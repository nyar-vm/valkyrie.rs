(call/lambda a ())
(call/lambda (call/apply s (apply args)) ((call/apply print (apply hello))))
(call/lambda
  (call/apply-dot
    (call/lambda
      (call/apply-dot (call/lambda (call/apply-dot (call/lambda-dot chain (body)) (map)) ((call/apply print (apply hello)))) (filter))
      ((> x 0)))
    (filter_collect))
  ((> x 5)))
