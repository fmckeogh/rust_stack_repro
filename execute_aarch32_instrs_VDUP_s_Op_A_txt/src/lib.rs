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
use D_set::*;
use Elem_set::*;
use Elem_read::*;
use CheckAdvSIMDEnabled::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VDUP_s_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i64,
    esize: i64,
    index: i64,
    m: i64,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        scalar: Bits,
        r: i64,
        esizeshadow_7446: i64,
        e: i64,
        gs_308646: i64,
        gs_308652: i64,
        d: i128,
        d__arg: i64,
        elements: i64,
        esize: i64,
        index: i64,
        m: i64,
        regs: i64,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        index,
        m,
        regs,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#7446 <= s_0_2
        fn_state.esizeshadow_7446 = s_0_2;
        // D s_0_4: read-var d__arg:i64
        let s_0_4: i64 = fn_state.d__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var d <= s_0_5
        fn_state.d = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call CheckAdvSIMDEnabled(s_0_7)
        let s_0_8: () = CheckAdvSIMDEnabled(state, tracer, s_0_7);
        // N s_0_9: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var m:i64
        let s_1_0: i64 = fn_state.m;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call D_read(s_1_1)
        let s_1_2: u64 = D_read(state, tracer, s_1_1);
        // D s_1_3: read-var esizeshadow#7446:i64
        let s_1_3: i64 = fn_state.esizeshadow_7446;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: cast zx s_1_2 -> bv
        let s_1_6: Bits = Bits::new(s_1_2 as u128, 64u16);
        // D s_1_7: read-var index:i64
        let s_1_7: i64 = fn_state.index;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_5 -> i
        let s_1_9: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_10: call Elem_read(s_1_6, s_1_8, s_1_9)
        let s_1_10: Bits = Elem_read(state, tracer, s_1_6, s_1_8, s_1_9);
        // D s_1_11: write-var scalar <= s_1_10
        fn_state.scalar = s_1_10;
        // C s_1_12: const #0s : i64
        let s_1_12: i64 = 0;
        // C s_1_13: const #1s : i
        let s_1_13: i128 = 1;
        // D s_1_14: read-var regs:i64
        let s_1_14: i64 = fn_state.regs;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: sub s_1_15 s_1_13
        let s_1_16: i128 = ((s_1_15) - (s_1_13));
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: write-var gs#308646 <= s_1_17
        fn_state.gs_308646 = s_1_17;
        // D s_1_19: write-var r <= s_1_12
        fn_state.r = s_1_12;
        // N s_1_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#308646:i64
        let s_2_1: i64 = fn_state.gs_308646;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var elements:i64
        let s_3_2: i64 = fn_state.elements;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#308652 <= s_3_5
        fn_state.gs_308652 = s_3_5;
        // D s_3_7: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#308652:i64
        let s_4_1: i64 = fn_state.gs_308652;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var r:i64
        let s_5_0: i64 = fn_state.r;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var d:i
        let s_5_2: i128 = fn_state.d;
        // D s_5_3: add s_5_2 s_5_1
        let s_5_3: i128 = (s_5_2 + s_5_1);
        // D s_5_4: read-var r:i64
        let s_5_4: i64 = fn_state.r;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: read-var d:i
        let s_5_6: i128 = fn_state.d;
        // D s_5_7: add s_5_6 s_5_5
        let s_5_7: i128 = (s_5_6 + s_5_5);
        // D s_5_8: call D_read(s_5_7)
        let s_5_8: u64 = D_read(state, tracer, s_5_7);
        // D s_5_9: read-var esizeshadow#7446:i64
        let s_5_9: i64 = fn_state.esizeshadow_7446;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // D s_5_12: cast zx s_5_8 -> bv
        let s_5_12: Bits = Bits::new(s_5_8 as u128, 64u16);
        // D s_5_13: read-var e:i64
        let s_5_13: i64 = fn_state.e;
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_15: cast zx s_5_11 -> i
        let s_5_15: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_16: read-var scalar:bv
        let s_5_16: Bits = fn_state.scalar;
        // D s_5_17: call Elem_set(s_5_12, s_5_14, s_5_15, s_5_16)
        let s_5_17: Bits = Elem_set(state, tracer, s_5_12, s_5_14, s_5_15, s_5_16);
        // D s_5_18: cast reint s_5_17 -> u64
        let s_5_18: u64 = (s_5_17.value() as u64);
        // D s_5_19: call D_set(s_5_3, s_5_18)
        let s_5_19: () = D_set(state, tracer, s_5_3, s_5_18);
        // D s_5_20: read-var e:i64
        let s_5_20: i64 = fn_state.e;
        // C s_5_21: const #1s : i64
        let s_5_21: i64 = 1;
        // D s_5_22: add s_5_20 s_5_21
        let s_5_22: i64 = (s_5_20 + s_5_21);
        // D s_5_23: write-var e <= s_5_22
        fn_state.e = s_5_22;
        // N s_5_24: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var r:i64
        let s_6_0: i64 = fn_state.r;
        // C s_6_1: const #1s : i64
        let s_6_1: i64 = 1;
        // D s_6_2: add s_6_0 s_6_1
        let s_6_2: i64 = (s_6_0 + s_6_1);
        // D s_6_3: write-var r <= s_6_2
        fn_state.r = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
}
