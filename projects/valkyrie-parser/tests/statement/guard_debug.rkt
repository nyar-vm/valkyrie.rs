(guard/positive
  condition1
  (guard/positive condition2 (return success))
  (guard/negative condition3 (return failure))
  (return success))
guard
let
(= (call/apply Some (apply a)) (call/lambda b ((return a))))
