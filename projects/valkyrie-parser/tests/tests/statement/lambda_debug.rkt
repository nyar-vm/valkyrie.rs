(call/lambda a ())
(call/lambda (call/apply s (apply args)) ((call/apply print (apply hello))))
(call/lambda
  (call/apply-dot
    (call/lambda
      (call/apply-dot
        (call/lambda (call/apply-dot (call/lambda-dot chain (body)) (map)) ((call/apply print (apply hello))))
        (select))
      ((> x 0)))
    (reject))
  ((> x 5)))
