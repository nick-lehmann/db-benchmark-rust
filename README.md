# DB Benchmark



## Auto-vectorisation


Given the following example:

```rust
pub fn f(a: &[i32; 4], b: &[i32; 4], c: &mut [i32; 4]) {
  c[0] = a[0] + b[0]
  c[1] = a[1] + b[1]
  c[2] = a[2] + b[2]
  c[3] = a[3] + b[3]
}
```

The Rust compiler `rustc` provides two direct flags that control auto-vectorisation. However, `rustc` does not do any auto-vectorization on its own but uses LLVM for that. However, `rustc` provides the information to LLVM that allows it to decide which instructions get vectorized.

- [no-vectorize-loops](https://doc.rust-lang.org/rustc/codegen-options/index.html#no-vectorize-loops): If enabled, LLVM tries to unroll loops. [llvm-docs](https://llvm.org/docs/Vectorizers.html#the-loop-vectorizer)
- [no-vectorize-slp](https://doc.rust-lang.org/rustc/codegen-options/index.html#no-vectorize-slp): If enabled, LLVM tries to combine similar independent instructions into SIMD instructions. [llvm-docs](https://llvm.org/docs/Vectorizers.html#the-slp-vectorizer)


[enabled](https://godbolt.org/#g:!((g:!((g:!((h:codeEditor,i:(filename:'1',fontScale:14,fontUsePx:'0',j:1,lang:rust,selection:(endColumn:22,endLineNumber:5,positionColumn:22,positionLineNumber:5,selectionStartColumn:22,selectionStartLineNumber:5,startColumn:22,startLineNumber:5),source:'pub+fn+f(a:+%26%5Bi32%3B+4%5D,+b:+%26%5Bi32%3B+4%5D,+c:+%26mut+%5Bi32%3B+4%5D)+%7B%0A++c%5B0%5D+%3D+a%5B0%5D+%2B+b%5B0%5D%3B%0A++c%5B1%5D+%3D+a%5B1%5D+%2B+b%5B1%5D%3B%0A++c%5B2%5D+%3D+a%5B2%5D+%2B+b%5B2%5D%3B%0A++c%5B3%5D+%3D+a%5B3%5D+%2B+b%5B3%5D%3B%0A%7D%0A'),l:'5',n:'0',o:'Rust+source+%231',t:'0')),k:49.55903684363215,l:'4',n:'0',o:'',s:0,t:'0'),(g:!((h:compiler,i:(compiler:r1620,filters:(b:'0',binary:'1',commentOnly:'0',demangle:'0',directives:'0',execute:'1',intel:'0',libraryCode:'0',trim:'1'),flagsViewOpen:'1',fontScale:14,fontUsePx:'0',j:1,lang:rust,libs:!(),options:'-C+opt-level%3D3',selection:(endColumn:1,endLineNumber:1,positionColumn:1,positionLineNumber:1,selectionStartColumn:1,selectionStartLineNumber:1,startColumn:1,startLineNumber:1),source:1,tree:'1'),l:'5',n:'0',o:'rustc+1.62.0+(Rust,+Editor+%231,+Compiler+%231)',t:'0')),k:50.440963156367864,l:'4',n:'0',o:'',s:0,t:'0')),l:'2',n:'0',o:'',t:'0')),version:4)

```asm
example::f:
        movdqu  xmm0, xmmword ptr [rdi]
        movdqu  xmm1, xmmword ptr [rsi]
        paddd   xmm1, xmm0
        movdqu  xmmword ptr [rdx], xmm1
        ret
```

[-no-vectorize-slp](https://godbolt.org/#g:!((g:!((g:!((h:codeEditor,i:(filename:'1',fontScale:14,fontUsePx:'0',j:1,lang:rust,selection:(endColumn:22,endLineNumber:5,positionColumn:22,positionLineNumber:5,selectionStartColumn:22,selectionStartLineNumber:5,startColumn:22,startLineNumber:5),source:'pub+fn+f(a:+%26%5Bi32%3B+4%5D,+b:+%26%5Bi32%3B+4%5D,+c:+%26mut+%5Bi32%3B+4%5D)+%7B%0A++c%5B0%5D+%3D+a%5B0%5D+%2B+b%5B0%5D%3B%0A++c%5B1%5D+%3D+a%5B1%5D+%2B+b%5B1%5D%3B%0A++c%5B2%5D+%3D+a%5B2%5D+%2B+b%5B2%5D%3B%0A++c%5B3%5D+%3D+a%5B3%5D+%2B+b%5B3%5D%3B%0A%7D%0A'),l:'5',n:'0',o:'Rust+source+%231',t:'0')),k:49.55903684363215,l:'4',n:'0',o:'',s:0,t:'0'),(g:!((h:compiler,i:(compiler:r1620,filters:(b:'0',binary:'1',commentOnly:'0',demangle:'0',directives:'0',execute:'1',intel:'0',libraryCode:'0',trim:'1'),flagsViewOpen:'1',fontScale:14,fontUsePx:'0',j:1,lang:rust,libs:!(),options:'-C+opt-level%3D3+-C+no-vectorize-slp',selection:(endColumn:1,endLineNumber:1,positionColumn:1,positionLineNumber:1,selectionStartColumn:1,selectionStartLineNumber:1,startColumn:1,startLineNumber:1),source:1,tree:'1'),l:'5',n:'0',o:'rustc+1.62.0+(Rust,+Editor+%231,+Compiler+%231)',t:'0')),k:50.440963156367864,l:'4',n:'0',o:'',s:0,t:'0')),l:'2',n:'0',o:'',t:'0')),version:4)

```asm
example::f:
        mov     eax, dword ptr [rsi]
        add     eax, dword ptr [rdi]
        mov     dword ptr [rdx], eax
        mov     eax, dword ptr [rsi + 4]
        add     eax, dword ptr [rdi + 4]
        mov     dword ptr [rdx + 4], eax
        mov     eax, dword ptr [rsi + 8]
        add     eax, dword ptr [rdi + 8]
        mov     dword ptr [rdx + 8], eax
        mov     eax, dword ptr [rsi + 12]
        add     eax, dword ptr [rdi + 12]
        mov     dword ptr [rdx + 12], eax
        ret
```

Since the function does not include a loop, setting `no-vectorize-loops` has no impact.

To make the comparisons fair, we set both `no-vectorize-loops` and `no-vectorize-slp` in our `Cargo.toml`.