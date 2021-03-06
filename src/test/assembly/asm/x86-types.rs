// no-system-llvm
// revisions: x86_64 i686
// assembly-output: emit-asm
//[x86_64] compile-flags: --target x86_64-unknown-linux-gnu
//[i686] compile-flags: --target i686-unknown-linux-gnu
// compile-flags: -C llvm-args=--x86-asm-syntax=intel
// compile-flags: -C target-feature=+avx512bw

#![feature(no_core, lang_items, rustc_attrs, repr_simd)]
#![crate_type = "rlib"]
#![no_core]
#![allow(asm_sub_register, non_camel_case_types)]

#[rustc_builtin_macro]
macro_rules! asm {
    () => {};
}
#[rustc_builtin_macro]
macro_rules! concat {
    () => {};
}
#[rustc_builtin_macro]
macro_rules! stringify {
    () => {};
}

#[lang = "sized"]
trait Sized {}
#[lang = "copy"]
trait Copy {}

type ptr = *mut u8;

#[repr(simd)]
pub struct i8x16(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8);
#[repr(simd)]
pub struct i16x8(i16, i16, i16, i16, i16, i16, i16, i16);
#[repr(simd)]
pub struct i32x4(i32, i32, i32, i32);
#[repr(simd)]
pub struct i64x2(i64, i64);
#[repr(simd)]
pub struct f32x4(f32, f32, f32, f32);
#[repr(simd)]
pub struct f64x2(f64, f64);

#[repr(simd)]
pub struct i8x32(
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
);
#[repr(simd)]
pub struct i16x16(i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16, i16);
#[repr(simd)]
pub struct i32x8(i32, i32, i32, i32, i32, i32, i32, i32);
#[repr(simd)]
pub struct i64x4(i64, i64, i64, i64);
#[repr(simd)]
pub struct f32x8(f32, f32, f32, f32, f32, f32, f32, f32);
#[repr(simd)]
pub struct f64x4(f64, f64, f64, f64);

#[repr(simd)]
pub struct i8x64(
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
    i8,
);
#[repr(simd)]
pub struct i16x32(
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
    i16,
);
#[repr(simd)]
pub struct i32x16(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32);
#[repr(simd)]
pub struct i64x8(i64, i64, i64, i64, i64, i64, i64, i64);
#[repr(simd)]
pub struct f32x16(f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32);
#[repr(simd)]
pub struct f64x8(f64, f64, f64, f64, f64, f64, f64, f64);

impl Copy for i8 {}
impl Copy for i16 {}
impl Copy for i32 {}
impl Copy for f32 {}
impl Copy for i64 {}
impl Copy for f64 {}
impl Copy for ptr {}
impl Copy for i8x16 {}
impl Copy for i16x8 {}
impl Copy for i32x4 {}
impl Copy for i64x2 {}
impl Copy for f32x4 {}
impl Copy for f64x2 {}
impl Copy for i8x32 {}
impl Copy for i16x16 {}
impl Copy for i32x8 {}
impl Copy for i64x4 {}
impl Copy for f32x8 {}
impl Copy for f64x4 {}
impl Copy for i8x64 {}
impl Copy for i16x32 {}
impl Copy for i32x16 {}
impl Copy for i64x8 {}
impl Copy for f32x16 {}
impl Copy for f64x8 {}

extern "C" {
    fn extern_func();
    static extern_static: u8;
}

// CHECK-LABEL: sym_fn:
// CHECK: #APP
// CHECK: call extern_func
// CHECK: #NO_APP
#[no_mangle]
pub unsafe fn sym_fn() {
    asm!("call {}", sym extern_func);
}

// CHECK-LABEL: sym_static:
// CHECK: #APP
// CHECK: mov al, byte ptr [extern_static]
// CHECK: #NO_APP
#[no_mangle]
pub unsafe fn sym_static() {
    asm!("mov al, byte ptr [{}]", sym extern_static);
}

macro_rules! check {
    ($func:ident $ty:ident $class:ident $mov:literal) => {
        #[no_mangle]
        pub unsafe fn $func(x: $ty) -> $ty {
            // Hack to avoid function merging
            extern "Rust" {
                fn dont_merge(s: &str);
            }
            dont_merge(stringify!($func));

            let y;
            asm!(concat!($mov, " {}, {}"), out($class) y, in($class) x);
            y
        }
    };
}

// CHECK-LABEL: reg_i16:
// CHECK: #APP
// x86_64: mov r{{[a-z0-9]+}}, r{{[a-z0-9]+}}
// i686: mov e{{[a-z0-9]+}}, e{{[a-z0-9]+}}
// CHECK: #NO_APP
check!(reg_i16 i16 reg "mov");

// CHECK-LABEL: reg_i32:
// CHECK: #APP
// x86_64: mov r{{[a-z0-9]+}}, r{{[a-z0-9]+}}
// i686: mov e{{[a-z0-9]+}}, e{{[a-z0-9]+}}
// CHECK: #NO_APP
check!(reg_i32 i32 reg "mov");

// CHECK-LABEL: reg_f32:
// CHECK: #APP
// x86_64: mov r{{[a-z0-9]+}}, r{{[a-z0-9]+}}
// i686: mov e{{[a-z0-9]+}}, e{{[a-z0-9]+}}
// CHECK: #NO_APP
check!(reg_f32 f32 reg "mov");

// x86_64-LABEL: reg_i64:
// x86_64: #APP
// x86_64: mov r{{[a-z0-9]+}}, r{{[a-z0-9]+}}
// x86_64: #NO_APP
#[cfg(x86_64)]
check!(reg_i64 i64 reg "mov");

// x86_64-LABEL: reg_f64:
// x86_64: #APP
// x86_64: mov r{{[a-z0-9]+}}, r{{[a-z0-9]+}}
// x86_64: #NO_APP
#[cfg(x86_64)]
check!(reg_f64 f64 reg "mov");

// CHECK-LABEL: reg_ptr:
// CHECK: #APP
// x86_64: mov r{{[a-z0-9]+}}, r{{[a-z0-9]+}}
// i686: mov e{{[a-z0-9]+}}, e{{[a-z0-9]+}}
// CHECK: #NO_APP
check!(reg_ptr ptr reg "mov");

// CHECK-LABEL: reg_abcd_i16:
// CHECK: #APP
// x86_64: mov r{{[a-z0-9]+}}, r{{[a-z0-9]+}}
// i686: mov e{{[a-z0-9]+}}, e{{[a-z0-9]+}}
// CHECK: #NO_APP
check!(reg_abcd_i16 i16 reg_abcd "mov");

// CHECK-LABEL: reg_abcd_i32:
// CHECK: #APP
// x86_64: mov r{{[a-z0-9]+}}, r{{[a-z0-9]+}}
// i686: mov e{{[a-z0-9]+}}, e{{[a-z0-9]+}}
// CHECK: #NO_APP
check!(reg_abcd_i32 i32 reg_abcd "mov");

// CHECK-LABEL: reg_abcd_f32:
// CHECK: #APP
// x86_64: mov r{{[a-z0-9]+}}, r{{[a-z0-9]+}}
// i686: mov e{{[a-z0-9]+}}, e{{[a-z0-9]+}}
// CHECK: #NO_APP
check!(reg_abcd_f32 f32 reg_abcd "mov");

// x86_64-LABEL: reg_abcd_i64:
// x86_64: #APP
// x86_64: mov r{{[a-z0-9]+}}, r{{[a-z0-9]+}}
// x86_64: #NO_APP
#[cfg(x86_64)]
check!(reg_abcd_i64 i64 reg_abcd "mov");

// x86_64-LABEL: reg_abcd_f64:
// x86_64: #APP
// x86_64: mov r{{[a-z0-9]+}}, r{{[a-z0-9]+}}
// x86_64: #NO_APP
#[cfg(x86_64)]
check!(reg_abcd_f64 f64 reg_abcd "mov");

// CHECK-LABEL: reg_abcd_ptr:
// CHECK: #APP
// x86_64: mov r{{[a-z0-9]+}}, r{{[a-z0-9]+}}
// i686: mov e{{[a-z0-9]+}}, e{{[a-z0-9]+}}
// CHECK: #NO_APP
check!(reg_abcd_ptr ptr reg_abcd "mov");

// CHECK-LABEL: reg_byte:
// CHECK: #APP
// CHECK: mov {{[a-z0-9]+}}, {{[a-z0-9]+}}
// CHECK: #NO_APP
check!(reg_byte i8 reg_byte "mov");

// CHECK-LABEL: xmm_reg_i32:
// CHECK: #APP
// CHECK: movaps xmm{{[0-9]+}}, xmm{{[0-9]+}}
// CHECK: #NO_APP
check!(xmm_reg_i32 i32 xmm_reg "movaps");

// CHECK-LABEL: xmm_reg_f32:
// CHECK: #APP
// CHECK: movaps xmm{{[0-9]+}}, xmm{{[0-9]+}}
// CHECK: #NO_APP
check!(xmm_reg_f32 f32 xmm_reg "movaps");

// CHECK-LABEL: xmm_reg_i64:
// CHECK: #APP
// CHECK: movaps xmm{{[0-9]+}}, xmm{{[0-9]+}}
// CHECK: #NO_APP
check!(xmm_reg_i64 i64 xmm_reg "movaps");

// CHECK-LABEL: xmm_reg_f64:
// CHECK: #APP
// CHECK: movaps xmm{{[0-9]+}}, xmm{{[0-9]+}}
// CHECK: #NO_APP
check!(xmm_reg_f64 f64 xmm_reg "movaps");

// CHECK-LABEL: xmm_reg_ptr:
// CHECK: #APP
// CHECK: movaps xmm{{[0-9]+}}, xmm{{[0-9]+}}
// CHECK: #NO_APP
check!(xmm_reg_ptr ptr xmm_reg "movaps");

// CHECK-LABEL: xmm_reg_i8x16:
// CHECK: #APP
// CHECK: movaps xmm{{[0-9]+}}, xmm{{[0-9]+}}
// CHECK: #NO_APP
check!(xmm_reg_i8x16 i8x16 xmm_reg "movaps");

// CHECK-LABEL: xmm_reg_i16x8:
// CHECK: #APP
// CHECK: movaps xmm{{[0-9]+}}, xmm{{[0-9]+}}
// CHECK: #NO_APP
check!(xmm_reg_i16x8 i16x8 xmm_reg "movaps");

// CHECK-LABEL: xmm_reg_i32x4:
// CHECK: #APP
// CHECK: movaps xmm{{[0-9]+}}, xmm{{[0-9]+}}
// CHECK: #NO_APP
check!(xmm_reg_i32x4 i32x4 xmm_reg "movaps");

// CHECK-LABEL: xmm_reg_i64x2:
// CHECK: #APP
// CHECK: movaps xmm{{[0-9]+}}, xmm{{[0-9]+}}
// CHECK: #NO_APP
check!(xmm_reg_i64x2 i64x2 xmm_reg "movaps");

// CHECK-LABEL: xmm_reg_f32x4:
// CHECK: #APP
// CHECK: movaps xmm{{[0-9]+}}, xmm{{[0-9]+}}
// CHECK: #NO_APP
check!(xmm_reg_f32x4 f32x4 xmm_reg "movaps");

// CHECK-LABEL: xmm_reg_f64x2:
// CHECK: #APP
// CHECK: movaps xmm{{[0-9]+}}, xmm{{[0-9]+}}
// CHECK: #NO_APP
check!(xmm_reg_f64x2 f64x2 xmm_reg "movaps");

// CHECK-LABEL: ymm_reg_i32:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_i32 i32 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_f32:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_f32 f32 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_i64:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_i64 i64 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_f64:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_f64 f64 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_ptr:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_ptr ptr ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_i8x16:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_i8x16 i8x16 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_i16x8:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_i16x8 i16x8 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_i32x4:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_i32x4 i32x4 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_i64x2:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_i64x2 i64x2 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_f32x4:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_f32x4 f32x4 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_f64x2:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_f64x2 f64x2 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_i8x32:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_i8x32 i8x32 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_i16x16:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_i16x16 i16x16 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_i32x8:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_i32x8 i32x8 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_i64x4:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_i64x4 i64x4 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_f32x8:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_f32x8 f32x8 ymm_reg "vmovaps");

// CHECK-LABEL: ymm_reg_f64x4:
// CHECK: #APP
// CHECK: vmovaps ymm{{[0-9]+}}, ymm{{[0-9]+}}
// CHECK: #NO_APP
check!(ymm_reg_f64x4 f64x4 ymm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_i32:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_i32 i32 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_f32:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_f32 f32 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_i64:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_i64 i64 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_f64:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_f64 f64 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_ptr:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_ptr ptr zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_i8x16:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_i8x16 i8x16 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_i16x8:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_i16x8 i16x8 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_i32x4:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_i32x4 i32x4 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_i64x2:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_i64x2 i64x2 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_f32x4:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_f32x4 f32x4 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_f64x2:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_f64x2 f64x2 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_i8x32:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_i8x32 i8x32 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_i16x16:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_i16x16 i16x16 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_i32x8:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_i32x8 i32x8 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_i64x4:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_i64x4 i64x4 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_f32x8:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_f32x8 f32x8 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_f64x4:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_f64x4 f64x4 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_i8x64:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_i8x64 i8x64 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_i16x32:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_i16x32 i16x32 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_i32x16:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_i32x16 i32x16 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_i64x8:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_i64x8 i64x8 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_f32x16:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_f32x16 f32x16 zmm_reg "vmovaps");

// CHECK-LABEL: zmm_reg_f64x8:
// CHECK: #APP
// CHECK: vmovaps zmm{{[0-9]+}}, zmm{{[0-9]+}}
// CHECK: #NO_APP
check!(zmm_reg_f64x8 f64x8 zmm_reg "vmovaps");

// CHECK-LABEL: kreg_i8:
// CHECK: #APP
// CHECK: kmovb k{{[0-9]+}}, k{{[0-9]+}}
// CHECK: #NO_APP
check!(kreg_i8 i8 kreg "kmovb");

// CHECK-LABEL: kreg_i16:
// CHECK: #APP
// CHECK: kmovw k{{[0-9]+}}, k{{[0-9]+}}
// CHECK: #NO_APP
check!(kreg_i16 i16 kreg "kmovw");

// CHECK-LABEL: kreg_i32:
// CHECK: #APP
// CHECK: kmovd k{{[0-9]+}}, k{{[0-9]+}}
// CHECK: #NO_APP
check!(kreg_i32 i32 kreg "kmovd");

// CHECK-LABEL: kreg_i64:
// CHECK: #APP
// CHECK: kmovq k{{[0-9]+}}, k{{[0-9]+}}
// CHECK: #NO_APP
check!(kreg_i64 i64 kreg "kmovq");

// CHECK-LABEL: kreg_ptr:
// CHECK: #APP
// CHECK: kmovq k{{[0-9]+}}, k{{[0-9]+}}
// CHECK: #NO_APP
check!(kreg_ptr ptr kreg "kmovq");
