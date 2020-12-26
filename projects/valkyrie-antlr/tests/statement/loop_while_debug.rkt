(while while)
while
(call/lambda
  true
  ((call/apply print (apply Loop!))
    while
    (>
      n
      (call/lambda
        0
        ((call/apply print (apply Deep loop!))
          while
          (≠ k (call/lambda n ((call/apply print (apply Deeper loop!)))))
          (call/lambda else ((call/apply print (apply Empty deeper loop!)))))))))
