(loop)
(loop (call/apply print (apply Loop!)) (loop (call/apply print (apply Deep loop!)) (loop (call/apply print (apply Deeper loop!)))))
