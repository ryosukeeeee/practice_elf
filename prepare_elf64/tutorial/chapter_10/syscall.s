// w0: system call number
// w1: int fd
// x2: char *s
// w3: int n 

 .global syscall
 syscall:
       mov x8, x0 
       mov x0, x1 // x0: output
       mov x1, x2 // x1: string address
       mov x2, x3 // x2: length
       svc     #0
       mov     x0,  xzr
       ret
