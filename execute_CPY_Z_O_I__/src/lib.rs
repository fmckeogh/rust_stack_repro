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
use P_read::*;
use ActivePredicateElement::*;
use integer_subrange::*;
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_CPY_Z_O_I__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    imm: i128,
    merging: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_2793: i64,
        e: i64,
        VLshadow_2794: i64,
        dest: Bits,
        esizeshadow_2792: i64,
        result: Bits,
        gs_193039: i64,
        mask: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        imm: i128,
        merging: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
        imm,
        merging,
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
        // D s_0_3: write-var esizeshadow#2792 <= s_0_2
        fn_state.esizeshadow_2792 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2793 <= s_0_6
        fn_state.VLshadow_2793 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2793:i64
        let s_1_0: i64 = fn_state.VLshadow_2793;
        // D s_1_1: write-var VLshadow#2794 <= s_1_0
        fn_state.VLshadow_2794 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2794:i64
        let s_1_3: i64 = fn_state.VLshadow_2794;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2794:i64
        let s_1_7: i64 = fn_state.VLshadow_2794;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#2792:i64
        let s_1_9: i64 = fn_state.esizeshadow_2792;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: cast zx s_1_6 -> i
        let s_1_13: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: read-var g:i64
        let s_1_15: i64 = fn_state.g;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_14 -> i
        let s_1_17: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_18: call P_read(s_1_16, s_1_17)
        let s_1_18: Bits = P_read(state, tracer, s_1_16, s_1_17);
        // D s_1_19: write-var mask <= s_1_18
        fn_state.mask = s_1_18;
        // D s_1_20: read-var VLshadow#2794:i64
        let s_1_20: i64 = fn_state.VLshadow_2794;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: read-var d:i64
        let s_1_23: i64 = fn_state.d;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast zx s_1_22 -> i
        let s_1_25: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_26: call Z_read(s_1_24, s_1_25)
        let s_1_26: Bits = Z_read(state, tracer, s_1_24, s_1_25);
        // D s_1_27: write-var dest <= s_1_26
        fn_state.dest = s_1_26;
        // C s_1_28: const #0s : i64
        let s_1_28: i64 = 0;
        // C s_1_29: const #1s : i
        let s_1_29: i128 = 1;
        // D s_1_30: cast zx s_1_12 -> i
        let s_1_30: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_31: sub s_1_30 s_1_29
        let s_1_31: i128 = ((s_1_30) - (s_1_29));
        // D s_1_32: cast reint s_1_31 -> i64
        let s_1_32: i64 = (s_1_31 as i64);
        // D s_1_33: write-var gs#193039 <= s_1_32
        fn_state.gs_193039 = s_1_32;
        // D s_1_34: write-var e <= s_1_28
        fn_state.e = s_1_28;
        // N s_1_35: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#193039:i64
        let s_2_1: i64 = fn_state.gs_193039;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b9 b3
        if s_2_2 {
            return block_9(state, tracer, fn_state);
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
        // D s_3_2: read-var esizeshadow#2792:i64
        let s_3_2: i64 = fn_state.esizeshadow_2792;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: read-var mask:bv
        let s_3_4: Bits = fn_state.mask;
        // D s_3_5: call ActivePredicateElement(s_3_4, s_3_1, s_3_3)
        let s_3_5: bool = ActivePredicateElement(state, tracer, s_3_4, s_3_1, s_3_3);
        // N s_3_6: branch s_3_5 b8 b4
        if s_3_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var merging:u8
        let s_4_0: bool = fn_state.merging;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#2792:i64
        let s_5_0: i64 = fn_state.esizeshadow_2792;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var esizeshadow#2792:i64
        let s_5_3: i64 = fn_state.esizeshadow_2792;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: call Zeros(s_5_4)
        let s_5_5: Bits = Zeros(state, tracer, s_5_4);
        // D s_5_6: read-var e:i64
        let s_5_6: i64 = fn_state.e;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: cast zx s_5_2 -> i
        let s_5_8: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_9: read-var result:bv
        let s_5_9: Bits = fn_state.result;
        // D s_5_10: call Elem_set(s_5_9, s_5_7, s_5_8, s_5_5)
        let s_5_10: Bits = Elem_set(state, tracer, s_5_9, s_5_7, s_5_8, s_5_5);
        // D s_5_11: write-var result <= s_5_10
        fn_state.result = s_5_10;
        // N s_5_12: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // C s_6_1: const #1s : i64
        let s_6_1: i64 = 1;
        // D s_6_2: add s_6_0 s_6_1
        let s_6_2: i64 = (s_6_0 + s_6_1);
        // D s_6_3: write-var e <= s_6_2
        fn_state.e = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esizeshadow#2792:i64
        let s_7_0: i64 = fn_state.esizeshadow_2792;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var esizeshadow#2792:i64
        let s_7_3: i64 = fn_state.esizeshadow_2792;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: read-var e:i64
        let s_7_6: i64 = fn_state.e;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: cast zx s_7_5 -> i
        let s_7_8: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_9: read-var dest:bv
        let s_7_9: Bits = fn_state.dest;
        // D s_7_10: call Elem_read(s_7_9, s_7_7, s_7_8)
        let s_7_10: Bits = Elem_read(state, tracer, s_7_9, s_7_7, s_7_8);
        // D s_7_11: read-var e:i64
        let s_7_11: i64 = fn_state.e;
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: cast zx s_7_2 -> i
        let s_7_13: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_14: read-var result:bv
        let s_7_14: Bits = fn_state.result;
        // D s_7_15: call Elem_set(s_7_14, s_7_12, s_7_13, s_7_10)
        let s_7_15: Bits = Elem_set(state, tracer, s_7_14, s_7_12, s_7_13, s_7_10);
        // D s_7_16: write-var result <= s_7_15
        fn_state.result = s_7_15;
        // N s_7_17: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esizeshadow#2792:i64
        let s_8_0: i64 = fn_state.esizeshadow_2792;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // C s_8_3: const #1s : i
        let s_8_3: i128 = 1;
        // D s_8_4: read-var esizeshadow#2792:i64
        let s_8_4: i64 = fn_state.esizeshadow_2792;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: sub s_8_5 s_8_3
        let s_8_6: i128 = ((s_8_5) - (s_8_3));
        // D s_8_7: cast reint s_8_6 -> i64
        let s_8_7: i64 = (s_8_6 as i64);
        // C s_8_8: const #0s : i
        let s_8_8: i128 = 0;
        // D s_8_9: cast zx s_8_7 -> i
        let s_8_9: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_10: read-var imm:i
        let s_8_10: i128 = fn_state.imm;
        // D s_8_11: call integer_subrange(s_8_10, s_8_9, s_8_8)
        let s_8_11: Bits = integer_subrange(state, tracer, s_8_10, s_8_9, s_8_8);
        // D s_8_12: read-var e:i64
        let s_8_12: i64 = fn_state.e;
        // D s_8_13: cast zx s_8_12 -> i
        let s_8_13: i128 = (i128::try_from(s_8_12).unwrap());
        // D s_8_14: cast zx s_8_2 -> i
        let s_8_14: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_15: read-var result:bv
        let s_8_15: Bits = fn_state.result;
        // D s_8_16: call Elem_set(s_8_15, s_8_13, s_8_14, s_8_11)
        let s_8_16: Bits = Elem_set(state, tracer, s_8_15, s_8_13, s_8_14, s_8_11);
        // D s_8_17: write-var result <= s_8_16
        fn_state.result = s_8_16;
        // N s_8_18: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VLshadow#2794:i64
        let s_9_0: i64 = fn_state.VLshadow_2794;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var d:i64
        let s_9_3: i64 = fn_state.d;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: cast zx s_9_2 -> i
        let s_9_5: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_6: read-var result:bv
        let s_9_6: Bits = fn_state.result;
        // D s_9_7: call Z_set(s_9_4, s_9_5, s_9_6)
        let s_9_7: () = Z_set(state, tracer, s_9_4, s_9_5, s_9_6);
        // N s_9_8: return
        return;
    }
}
