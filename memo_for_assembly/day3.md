(gdb) break main
Breakpoint 1 at 0xd2b0
(gdb) where
No stack.
(gdb) cont
The program is not being run.
(gdb) run hello
Starting program: /root/src_rust/hacking/com_line_args hello
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, 0x00005555555612b0 in main ()
(gdb) where
#0  0x00005555555612b0 in main ()
#1  0x00007ffff7de209b in __libc_start_main (main=0x5555555612b0 <main>, argc=2, argv=0x7fffffffeb78, init=<optimized out>, 
    fini=<optimized out>, rtld_fini=<optimized out>, stack_end=0x7fffffffeb68) at ../csu/libc-start.c:308
#2  0x000055555555c4ca in _start ()
(gdb) cont
Continuing.
文法が違います
Usage: /root/src_rust/hacking/com_line_args <メッセージ> <繰り返し回数>
[Inferior 1 (process 87) exited with code 01]
(gdb) where
No stack.

- where 
breakなどで止まっている時にスタックのバックトレースを表示
- cont 
breakで止まってるのを進める


(gdb) print &args
$2 = (alloc::vec::Vec<alloc::string::String, alloc::alloc::Global> *) 0x7fffffffe8c0
0x7fffffffe8c0: 0x555aaa40      0x00005555      0x00000002
(gdb) x/4xw 0x7fffffffe8c0