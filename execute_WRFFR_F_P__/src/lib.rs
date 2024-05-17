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
use FFR_set::*;
use u__UNKNOWN_bits::*;
use HighestSetBit::*;
use CheckNonStreamingSVEEnabled::*;
use P_read::*;
use is_ones_subrange::*;
use common::*;
pub fn execute_WRFFR_F_P__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        hsb: i128,
        gs_197631: bool,
        PL: i64,
        VL: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        n,
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
        // S s_0_1: call CheckNonStreamingSVEEnabled(s_0_0)
        let s_0_1: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VL:i64
        let s_1_0: i64 = fn_state.VL;
        // C s_1_1: const #8s : i
        let s_1_1: i128 = 8;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: div s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) / (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: write-var PL <= s_1_4
        fn_state.PL = s_1_4;
        // D s_1_6: read-var PL:i64
        let s_1_6: i64 = fn_state.PL;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: read-var n:i64
        let s_1_9: i64 = fn_state.n;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: cast zx s_1_8 -> i
        let s_1_11: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_12: call P_read(s_1_10, s_1_11)
        let s_1_12: Bits = P_read(state, tracer, s_1_10, s_1_11);
        // D s_1_13: write-var operand <= s_1_12
        fn_state.operand = s_1_12;
        // D s_1_14: read-var operand:bv
        let s_1_14: Bits = fn_state.operand;
        // D s_1_15: call HighestSetBit(s_1_14)
        let s_1_15: i128 = HighestSetBit(state, tracer, s_1_14);
        // D s_1_16: write-var hsb <= s_1_15
        fn_state.hsb = s_1_15;
        // C s_1_17: const #0s : i
        let s_1_17: i128 = 0;
        // D s_1_18: read-var hsb:i
        let s_1_18: i128 = fn_state.hsb;
        // D s_1_19: cmp-lt s_1_18 s_1_17
        let s_1_19: bool = ((s_1_18) < (s_1_17));
        // N s_1_20: branch s_1_19 b6 b2
        if s_1_19 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0s : i
        let s_2_0: i128 = 0;
        // D s_2_1: read-var operand:bv
        let s_2_1: Bits = fn_state.operand;
        // D s_2_2: read-var hsb:i
        let s_2_2: i128 = fn_state.hsb;
        // D s_2_3: call is_ones_subrange(s_2_1, s_2_2, s_2_0)
        let s_2_3: bool = is_ones_subrange(state, tracer, s_2_1, s_2_2, s_2_0);
        // D s_2_4: write-var gs#197631 <= s_2_3
        fn_state.gs_197631 = s_2_3;
        // N s_2_5: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#197631:u8
        let s_3_0: bool = fn_state.gs_197631;
        // N s_3_1: branch s_3_0 b5 b4
        if s_3_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var PL:i64
        let s_4_0: i64 = fn_state.PL;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var PL:i64
        let s_4_3: i64 = fn_state.PL;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: call __UNKNOWN_bits(s_4_6)
        let s_4_7: Bits = u__UNKNOWN_bits(state, tracer, s_4_6);
        // D s_4_8: cast zx s_4_2 -> i
        let s_4_8: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_9: call FFR_set(s_4_8, s_4_7)
        let s_4_9: () = FFR_set(state, tracer, s_4_8, s_4_7);
        // N s_4_10: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var PL:i64
        let s_5_0: i64 = fn_state.PL;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var operand:bv
        let s_5_4: Bits = fn_state.operand;
        // D s_5_5: call FFR_set(s_5_3, s_5_4)
        let s_5_5: () = FFR_set(state, tracer, s_5_3, s_5_4);
        // N s_5_6: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var gs#197631 <= s_6_0
        fn_state.gs_197631 = s_6_0;
        // N s_6_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
