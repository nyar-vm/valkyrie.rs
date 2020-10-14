(guard/positive
  condition1
  (guard/positive condition2 (return success))
  (guard/negative condition3 (return failure))
  (return success))
(guard/cases
  (pattern/tuple (x) (y))
  point
  (then (guard (let (Failure)) (= error x) (call/lambda then ((return error))) (return some {x}, {y}))))
