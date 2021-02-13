(annotation/list space)
(annotation/list decorator)
(define/class Empty (modifiers))
(annotation/list decorator)
(union Empty)
(define/class
  ClassA
  (modifiers)
  (annotation/list decorator)
  (class/field field (modifiers))
  (class/field field (modifiers public static) : A)
  (class/method method (modifiers) () (return/type Unit) (return/type Pure))
  (class/method
    `/`
    (modifiers infix)
    (((self) Any null) ((rhs) Self null))
    (return/type Self)
    (return/effect DivideZero)
    (return (/ self rhs)))
  (annotation/list inline)
  (class/method
    join
    (modifiers)
    (define/generic (T Any null))
    (((mut self) T null) ((other) T null))
    (return/type T)
    (return/type Pure)
    (return (call/apply-dot self (append other)))))
(union UnionA)
