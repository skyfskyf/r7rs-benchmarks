;;; Override hide to use values-proc since grift treats values as a special form
;;; (values cannot be stored in vectors or passed as a first-class value)
(define (hide r x)
  (call-with-values
   (lambda ()
     (values (vector values-proc (lambda (x) x))
             (if (< r 100) 0 1)))
   (lambda (v i)
     ((vector-ref v i) x))))
