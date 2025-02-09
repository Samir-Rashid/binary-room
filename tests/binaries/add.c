int main(void) {
	int x = 3;
	int y = 4;

	return x + y;
}
//////////// riscv
// https://godbolt.org/#g:!((g:!((g:!((h:codeEditor,i:(filename:'1',fontScale:14,fontUsePx:'0',j:1,lang:c%2B%2B,selection:(endColumn:1,endLineNumber:8,positionColumn:1,positionLineNumber:8,selectionStartColumn:1,selectionStartLineNumber:8,startColumn:1,startLineNumber:8),source:'int+main(void)+%7B%0A%09int+x+%3D+3%3B%0A%09int+y+%3D+4%3B%0A%0A%09return+x+%2B+y%3B%0A%7D%0A%0A'),l:'5',n:'0',o:'C%2B%2B+source+%231',t:'0')),k:50,l:'4',n:'0',o:'',s:0,t:'0'),(g:!((h:compiler,i:(compiler:rv64-gcc1420,filters:(b:'0',binary:'1',binaryObject:'1',commentOnly:'0',debugCalls:'1',demangle:'0',directives:'0',execute:'1',intel:'0',libraryCode:'0',trim:'1',verboseDemangling:'0'),flagsViewOpen:'1',fontScale:14,fontUsePx:'0',j:1,lang:c%2B%2B,libs:!(),options:'',overrides:!(),selection:(endColumn:19,endLineNumber:19,positionColumn:19,positionLineNumber:19,selectionStartColumn:1,selectionStartLineNumber:1,startColumn:1,startLineNumber:1),source:1),l:'5',n:'0',o:'+RISC-V+(64-bits)+gcc+14.2.0+(Editor+%231)',t:'0')),k:50,l:'4',n:'0',o:'',s:0,t:'0')),l:'2',n:'0',o:'',t:'0')),version:4
// main:
//         addi    sp,sp,-32
//         sd      ra,24(sp)
//         sd      s0,16(sp)
//         addi    s0,sp,32
//         li      a5,3
//         sw      a5,-20(s0)
//         li      a5,4
//         sw      a5,-24(s0)
//         lw      a5,-20(s0)
//         mv      a4,a5
//         lw      a5,-24(s0)
//         addw    a5,a4,a5
//         sext.w  a5,a5
//         mv      a0,a5
//         ld      ra,24(sp)
//         ld    ß  s0,16(sp)
//         addi    sp,sp,32
//         jr      ra

//////////////// arm
// main:
//         sub     sp, sp, #16
//         mov     w0, 3
//         str     w0, [sp, 12]
//         mov     w0, 4
//         str     w0, [sp, 8]
//         ldr     w1, [sp, 12]
//         ldr     w0, [sp, 8]
//         add     w0, w1, w0
//         add     sp, sp, 16
//         ret

