if
(call/lambda a (1))
(branches (a (body (1))) (else (2)))
if
(call/lambda a (1))
else
if
(call/lambda b (2))
(branches (a (body (1))) (b (body (2))) (else (3)))
if
(call/lambda a (1))
else
if
(call/lambda b (2))
else
if
(call/lambda c (3))
(branches (a (body (1))) (b (body (2))) (c (body (3))) (else (4)))
