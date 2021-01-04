(define/class A (modifiers))
(define/class B (modifiers value))
(define/class Positive (modifiers refine))
(define/class A (modifiers) (class/field Base (modifiers inherit)) (class/field Base2 (modifiers inherit)))
(define/class B (modifiers) (class/field item (modifiers)))
(define/class A (modifiers) (class/field _a (modifiers)))
(define/class A (modifiers))
(define/class Empty1 (modifiers))
(define/class Empty2 (modifiers))
(define/class Empty3 (modifiers public static dynamic_transaction_safe volatile final))
(define/class Inherit1 (modifiers open impart))
(define/class
  Inherit2
  (modifiers open impart)
  (class/field x_base (modifiers public virtual inherit))
  (class/field _y_base (modifiers private inline inherit)))
(define/class 原神 (modifiers))
(define/class
  ClassA
  (modifiers)
  (class/field field (modifiers))
  (class/field field (modifiers public static))
  (class/field item (modifiers)))
(define/class TestClass (modifiers))
(define/class
  Class
  (modifiers)
  (class/field field1 (modifiers readonly))
  (class/field field 2 (modifiers readonly))
  (class/field main (modifiers class static void))
  (class/field fields (modifiers mut)))
