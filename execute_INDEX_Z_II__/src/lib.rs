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
use Elem_set::*;
use CheckSVEEnabled::*;
use integer_subrange::*;
use Z_set::*;
use common::*;
pub fn execute_INDEX_Z_II__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    imm1: i128,
    imm2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        VLshadow_2851: i64,
        esizeshadow_2849: i64,
        gs_193974: i64,
        result: Bits,
        VLshadow_2850: i64,
        VL: i64,
        d: i64,
        esize: i64,
        imm1: i128,
        imm2: i128,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        imm1,
        imm2,
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
        // D s_0_3: write-var esizeshadow#2849 <= s_0_2
        fn_state.esizeshadow_2849 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2850 <= s_0_6
        fn_state.VLshadow_2850 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckSVEEnabled(s_0_8)
        let s_0_9: () = CheckSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2850:i64
        let s_1_0: i64 = fn_state.VLshadow_2850;
        // D s_1_1: write-var VLshadow#2851 <= s_1_0
        fn_state.VLshadow_2851 = s_1_0;
        // D s_1_2: read-var VLshadow#2851:i64
        let s_1_2: i64 = fn_state.VLshadow_2851;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#2849:i64
        let s_1_4: i64 = fn_state.esizeshadow_2849;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // C s_1_8: const #0s : i64
        let s_1_8: i64 = 0;
        // C s_1_9: const #1s : i
        let s_1_9: i128 = 1;
        // D s_1_10: cast zx s_1_7 -> i
        let s_1_10: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_11: sub s_1_10 s_1_9
        let s_1_11: i128 = ((s_1_10) - (s_1_9));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var gs#193974 <= s_1_12
        fn_state.gs_193974 = s_1_12;
        // D s_1_14: write-var e <= s_1_8
        fn_state.e = s_1_8;
        // N s_1_15: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#193974:i64
        let s_2_1: i64 = fn_state.gs_193974;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var imm2:i
        let s_3_2: i128 = fn_state.imm2;
        // D s_3_3: mul s_3_1 s_3_2
        let s_3_3: i128 = ((s_3_1) * (s_3_2));
        // D s_3_4: read-var imm1:i
        let s_3_4: i128 = fn_state.imm1;
        // D s_3_5: add s_3_4 s_3_3
        let s_3_5: i128 = (s_3_4 + s_3_3);
        // D s_3_6: read-var esizeshadow#2849:i64
        let s_3_6: i64 = fn_state.esizeshadow_2849;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // C s_3_9: const #1s : i
        let s_3_9: i128 = 1;
        // D s_3_10: read-var esizeshadow#2849:i64
        let s_3_10: i64 = fn_state.esizeshadow_2849;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: sub s_3_11 s_3_9
        let s_3_12: i128 = ((s_3_11) - (s_3_9));
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // C s_3_14: const #0s : i
        let s_3_14: i128 = 0;
        // D s_3_15: cast zx s_3_13 -> i
        let s_3_15: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_16: call integer_subrange(s_3_5, s_3_15, s_3_14)
        let s_3_16: Bits = integer_subrange(state, tracer, s_3_5, s_3_15, s_3_14);
        // D s_3_17: read-var e:i64
        let s_3_17: i64 = fn_state.e;
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: cast zx s_3_8 -> i
        let s_3_19: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_20: read-var result:bv
        let s_3_20: Bits = fn_state.result;
        // D s_3_21: call Elem_set(s_3_20, s_3_18, s_3_19, s_3_16)
        let s_3_21: Bits = Elem_set(state, tracer, s_3_20, s_3_18, s_3_19, s_3_16);
        // D s_3_22: write-var result <= s_3_21
        fn_state.result = s_3_21;
        // D s_3_23: read-var e:i64
        let s_3_23: i64 = fn_state.e;
        // C s_3_24: const #1s : i64
        let s_3_24: i64 = 1;
        // D s_3_25: add s_3_23 s_3_24
        let s_3_25: i64 = (s_3_23 + s_3_24);
        // D s_3_26: write-var e <= s_3_25
        fn_state.e = s_3_25;
        // N s_3_27: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2851:i64
        let s_4_0: i64 = fn_state.VLshadow_2851;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: read-var result:bv
        let s_4_6: Bits = fn_state.result;
        // D s_4_7: call Z_set(s_4_4, s_4_5, s_4_6)
        let s_4_7: () = Z_set(state, tracer, s_4_4, s_4_5, s_4_6);
        // N s_4_8: return
        return;
    }
}
