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
use Din_read::*;
use u__UNKNOWN_bits::*;
use CheckAdvSIMDEnabled::*;
use common::*;
pub fn execute_aarch32_instrs_VSWP_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        gs_322734: i64,
        d: i64,
        m: i64,
        regs: i64,
    }
    let fn_state = FunctionState {
        d,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckAdvSIMDEnabled(s_0_0)
        let s_0_1: () = CheckAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: read-var regs:i64
        let s_1_2: i64 = fn_state.regs;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#322734 <= s_1_5
        fn_state.gs_322734 = s_1_5;
        // D s_1_7: write-var r <= s_1_0
        fn_state.r = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#322734:i64
        let s_2_1: i64 = fn_state.gs_322734;
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
        // D s_3_0: read-var d:i64
        let s_3_0: i64 = fn_state.d;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var m:i64
        let s_3_2: i64 = fn_state.m;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b6 b4
        if s_3_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var d:i64
        let s_4_0: i64 = fn_state.d;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var r:i64
        let s_4_2: i64 = fn_state.r;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: add s_4_1 s_4_3
        let s_4_4: i128 = (s_4_1 + s_4_3);
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: read-var m:i64
        let s_4_6: i64 = fn_state.m;
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: read-var r:i64
        let s_4_8: i64 = fn_state.r;
        // D s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (i128::try_from(s_4_8).unwrap());
        // D s_4_10: add s_4_7 s_4_9
        let s_4_10: i128 = (s_4_7 + s_4_9);
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_13: call Din_read(s_4_12)
        let s_4_13: u64 = Din_read(state, tracer, s_4_12);
        // D s_4_14: cast zx s_4_5 -> i
        let s_4_14: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_15: call D_set(s_4_14, s_4_13)
        let s_4_15: () = D_set(state, tracer, s_4_14, s_4_13);
        // D s_4_16: read-var m:i64
        let s_4_16: i64 = fn_state.m;
        // D s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (i128::try_from(s_4_16).unwrap());
        // D s_4_18: read-var r:i64
        let s_4_18: i64 = fn_state.r;
        // D s_4_19: cast zx s_4_18 -> i
        let s_4_19: i128 = (i128::try_from(s_4_18).unwrap());
        // D s_4_20: add s_4_17 s_4_19
        let s_4_20: i128 = (s_4_17 + s_4_19);
        // D s_4_21: cast reint s_4_20 -> i64
        let s_4_21: i64 = (s_4_20 as i64);
        // D s_4_22: read-var d:i64
        let s_4_22: i64 = fn_state.d;
        // D s_4_23: cast zx s_4_22 -> i
        let s_4_23: i128 = (i128::try_from(s_4_22).unwrap());
        // D s_4_24: read-var r:i64
        let s_4_24: i64 = fn_state.r;
        // D s_4_25: cast zx s_4_24 -> i
        let s_4_25: i128 = (i128::try_from(s_4_24).unwrap());
        // D s_4_26: add s_4_23 s_4_25
        let s_4_26: i128 = (s_4_23 + s_4_25);
        // D s_4_27: cast reint s_4_26 -> i64
        let s_4_27: i64 = (s_4_26 as i64);
        // D s_4_28: cast zx s_4_27 -> i
        let s_4_28: i128 = (i128::try_from(s_4_27).unwrap());
        // D s_4_29: call Din_read(s_4_28)
        let s_4_29: u64 = Din_read(state, tracer, s_4_28);
        // D s_4_30: cast zx s_4_21 -> i
        let s_4_30: i128 = (i128::try_from(s_4_21).unwrap());
        // D s_4_31: call D_set(s_4_30, s_4_29)
        let s_4_31: () = D_set(state, tracer, s_4_30, s_4_29);
        // N s_4_32: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var r:i64
        let s_5_0: i64 = fn_state.r;
        // C s_5_1: const #1s : i64
        let s_5_1: i64 = 1;
        // D s_5_2: add s_5_0 s_5_1
        let s_5_2: i64 = (s_5_0 + s_5_1);
        // D s_5_3: write-var r <= s_5_2
        fn_state.r = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var d:i64
        let s_6_0: i64 = fn_state.d;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var r:i64
        let s_6_2: i64 = fn_state.r;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: add s_6_1 s_6_3
        let s_6_4: i128 = (s_6_1 + s_6_3);
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // C s_6_6: const #64s : i64
        let s_6_6: i64 = 64;
        // C s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // S s_6_8: call __UNKNOWN_bits(s_6_7)
        let s_6_8: Bits = u__UNKNOWN_bits(state, tracer, s_6_7);
        // S s_6_9: cast reint s_6_8 -> u64
        let s_6_9: u64 = (s_6_8.value() as u64);
        // D s_6_10: cast zx s_6_5 -> i
        let s_6_10: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_11: call D_set(s_6_10, s_6_9)
        let s_6_11: () = D_set(state, tracer, s_6_10, s_6_9);
        // N s_6_12: jump b5
        return block_5(state, tracer, fn_state);
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
