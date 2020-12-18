(guard/positive
  condition1
  (guard/positive condition2 (return success))
  (guard/negative condition3 (return failure))
  (return success))
(guard/cases
  (pattern/tuple (x) (y))
  point
  (then ((guard/cases (pattern/tuple Failure (error)) x (then ((return error)))) (return some {x}, {y}))))
