(guard/positive
  condition1
  (guard/positive condition2 (return success))
  (guard/negative condition3 (return failure))
  (return success))
(guard/cases
  ((x) (y))
  point
  guard
  let
  (= (call/apply Failure (apply error)) x)
  (call/lambda then ((return error)))
  (return some {x}, {y}))
