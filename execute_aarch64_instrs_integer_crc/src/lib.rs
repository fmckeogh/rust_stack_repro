#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use X_set::*;
use BitReverse::*;
use X_read::*;
use Zeros::*;
use integer_subrange::*;
use Poly32Mod2::*;
use common::*;
pub fn execute_aarch64_instrs_integer_crc<T: Tracer>(
    state: &mut State,
    tracer: &T,
    crc32c: bool,
    d: i64,
    m: i64,
    n: i64,
    size: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        acc: u32,
        ga_252298: i64,
        val_name: Bits,
        sizeshadow_1176: i64,
        crc32c: bool,
        d: i64,
        m: i64,
        n: i64,
        size: i64,
    }
    let fn_state = FunctionState {
        crc32c,
        d,
        m,
        n,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var size:i64
        let s_0_0: i64 = fn_state.size;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var sizeshadow#1176 <= s_0_2
        fn_state.sizeshadow_1176 = s_0_2;
        // C s_0_4: const #32s : i64
        let s_0_4: i64 = 32;
        // D s_0_5: read-var n:i64
        let s_0_5: i64 = fn_state.n;
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_7: call X_read(s_0_6, s_0_4)
        let s_0_7: Bits = X_read(state, tracer, s_0_6, s_0_4);
        // D s_0_8: cast reint s_0_7 -> u32
        let s_0_8: u32 = (s_0_7.value() as u32);
        // D s_0_9: write-var acc <= s_0_8
        fn_state.acc = s_0_8;
        // D s_0_10: read-var sizeshadow#1176:i64
        let s_0_10: i64 = fn_state.sizeshadow_1176;
        // D s_0_11: cast zx s_0_10 -> i
        let s_0_11: i128 = (i128::try_from(s_0_10).unwrap());
        // D s_0_12: cast reint s_0_11 -> i64
        let s_0_12: i64 = (s_0_11 as i64);
        // D s_0_13: read-var m:i64
        let s_0_13: i64 = fn_state.m;
        // D s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_15: call X_read(s_0_14, s_0_12)
        let s_0_15: Bits = X_read(state, tracer, s_0_14, s_0_12);
        // D s_0_16: write-var val_name <= s_0_15
        fn_state.val_name = s_0_15;
        // D s_0_17: read-var crc32c:u8
        let s_0_17: bool = fn_state.crc32c;
        // N s_0_18: branch s_0_17 b3 b1
        if s_0_17 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #79764919s : i64
        let s_1_0: i64 = 79764919;
        // D s_1_1: write-var ga#252298 <= s_1_0
        fn_state.ga_252298 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #31s : i
        let s_2_0: i128 = 31;
        // C s_2_1: const #0s : i
        let s_2_1: i128 = 0;
        // D s_2_2: read-var ga#252298:i64
        let s_2_2: i64 = fn_state.ga_252298;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: call integer_subrange(s_2_3, s_2_0, s_2_1)
        let s_2_4: Bits = integer_subrange(state, tracer, s_2_3, s_2_0, s_2_1);
        // D s_2_5: cast reint s_2_4 -> u32
        let s_2_5: u32 = (s_2_4.value() as u32);
        // D s_2_6: read-var acc:u32
        let s_2_6: u32 = fn_state.acc;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 32u16);
        // D s_2_8: call BitReverse(s_2_7)
        let s_2_8: Bits = BitReverse(state, tracer, s_2_7);
        // D s_2_9: cast reint s_2_8 -> u32
        let s_2_9: u32 = (s_2_8.value() as u32);
        // D s_2_10: read-var sizeshadow#1176:i64
        let s_2_10: i64 = fn_state.sizeshadow_1176;
        // D s_2_11: cast zx s_2_10 -> i
        let s_2_11: i128 = (i128::try_from(s_2_10).unwrap());
        // D s_2_12: call Zeros(s_2_11)
        let s_2_12: Bits = Zeros(state, tracer, s_2_11);
        // D s_2_13: cast zx s_2_9 -> bv
        let s_2_13: Bits = Bits::new(s_2_9 as u128, 32u16);
        // D s_2_14: cast reint s_2_13 -> u128
        let s_2_14: u128 = (s_2_13.value() as u128);
        // D s_2_15: size-of s_2_13
        let s_2_15: u16 = s_2_13.length();
        // D s_2_16: cast reint s_2_12 -> u128
        let s_2_16: u128 = (s_2_12.value() as u128);
        // D s_2_17: size-of s_2_12
        let s_2_17: u16 = s_2_12.length();
        // D s_2_18: lsl s_2_14 s_2_17
        let s_2_18: u128 = s_2_14 << s_2_17;
        // D s_2_19: or s_2_18 s_2_16
        let s_2_19: u128 = ((s_2_18) | (s_2_16));
        // D s_2_20: add s_2_15 s_2_17
        let s_2_20: u16 = (s_2_15 + s_2_17);
        // D s_2_21: create-bits s_2_19 s_2_20
        let s_2_21: Bits = Bits::new(s_2_19, s_2_20);
        // D s_2_22: read-var val_name:bv
        let s_2_22: Bits = fn_state.val_name;
        // D s_2_23: call BitReverse(s_2_22)
        let s_2_23: Bits = BitReverse(state, tracer, s_2_22);
        // C s_2_24: const #0u : u32
        let s_2_24: u32 = 0;
        // C s_2_25: cast zx s_2_24 -> bv
        let s_2_25: Bits = Bits::new(s_2_24 as u128, 32u16);
        // D s_2_26: cast reint s_2_23 -> u128
        let s_2_26: u128 = (s_2_23.value() as u128);
        // D s_2_27: size-of s_2_23
        let s_2_27: u16 = s_2_23.length();
        // C s_2_28: cast reint s_2_25 -> u128
        let s_2_28: u128 = (s_2_25.value() as u128);
        // D s_2_29: size-of s_2_25
        let s_2_29: u16 = s_2_25.length();
        // D s_2_30: lsl s_2_26 s_2_29
        let s_2_30: u128 = s_2_26 << s_2_29;
        // D s_2_31: or s_2_30 s_2_28
        let s_2_31: u128 = ((s_2_30) | (s_2_28));
        // D s_2_32: add s_2_27 s_2_29
        let s_2_32: u16 = (s_2_27 + s_2_29);
        // D s_2_33: create-bits s_2_31 s_2_32
        let s_2_33: Bits = Bits::new(s_2_31, s_2_32);
        // C s_2_34: const #32s : i64
        let s_2_34: i64 = 32;
        // D s_2_35: xor s_2_21 s_2_33
        let s_2_35: Bits = ((s_2_21) ^ (s_2_33));
        // D s_2_36: call Poly32Mod2(s_2_35, s_2_5)
        let s_2_36: u32 = Poly32Mod2(state, tracer, s_2_35, s_2_5);
        // D s_2_37: cast zx s_2_36 -> bv
        let s_2_37: Bits = Bits::new(s_2_36 as u128, 32u16);
        // D s_2_38: call BitReverse(s_2_37)
        let s_2_38: Bits = BitReverse(state, tracer, s_2_37);
        // D s_2_39: cast reint s_2_38 -> u32
        let s_2_39: u32 = (s_2_38.value() as u32);
        // D s_2_40: read-var d:i64
        let s_2_40: i64 = fn_state.d;
        // D s_2_41: cast zx s_2_40 -> i
        let s_2_41: i128 = (i128::try_from(s_2_40).unwrap());
        // D s_2_42: cast zx s_2_39 -> bv
        let s_2_42: Bits = Bits::new(s_2_39 as u128, 32u16);
        // D s_2_43: call X_set(s_2_41, s_2_34, s_2_42)
        let s_2_43: () = X_set(state, tracer, s_2_41, s_2_34, s_2_42);
        // N s_2_44: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #517762881s : i64
        let s_3_0: i64 = 517762881;
        // D s_3_1: write-var ga#252298 <= s_3_0
        fn_state.ga_252298 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
