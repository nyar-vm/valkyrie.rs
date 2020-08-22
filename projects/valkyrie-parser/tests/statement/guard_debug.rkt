guard
true
(call/lambda else ())
(return 0)
guard
false
(call/lambda else ((return (- 1))))
