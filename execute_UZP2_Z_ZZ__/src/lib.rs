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
use Zeros::*;
use CheckSVEEnabled::*;
use Z_read::*;
use Elem_read::*;
use Z_set::*;
use common::*;
pub fn execute_UZP2_Z_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
    part: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_195412: i64,
        gs_195420: i64,
        p: i64,
        u_3547: i64,
        esizeshadow_2910: i64,
        pairs: i64,
        VLshadow_2912: i64,
        result: Bits,
        operand1: Bits,
        VLshadow_2911: i64,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
        part: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        m,
        n,
        part,
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
        // D s_0_3: write-var esizeshadow#2910 <= s_0_2
        fn_state.esizeshadow_2910 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2911 <= s_0_6
        fn_state.VLshadow_2911 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2911:i64
        let s_1_0: i64 = fn_state.VLshadow_2911;
        // D s_1_1: write-var VLshadow#2912 <= s_1_0
        fn_state.VLshadow_2912 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esizeshadow#2910:i64
        let s_1_3: i64 = fn_state.esizeshadow_2910;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) * (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2912:i64
        let s_1_7: i64 = fn_state.VLshadow_2912;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: div s_1_8 s_1_9
        let s_1_10: i128 = ((s_1_8) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var pairs <= s_1_11
        fn_state.pairs = s_1_11;
        // D s_1_13: read-var VLshadow#2912:i64
        let s_1_13: i64 = fn_state.VLshadow_2912;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var n:i64
        let s_1_16: i64 = fn_state.n;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call Z_read(s_1_17, s_1_18)
        let s_1_19: Bits = Z_read(state, tracer, s_1_17, s_1_18);
        // D s_1_20: write-var operand1 <= s_1_19
        fn_state.operand1 = s_1_19;
        // D s_1_21: read-var VLshadow#2912:i64
        let s_1_21: i64 = fn_state.VLshadow_2912;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: read-var m:i64
        let s_1_24: i64 = fn_state.m;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast zx s_1_23 -> i
        let s_1_26: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_27: call Z_read(s_1_25, s_1_26)
        let s_1_27: Bits = Z_read(state, tracer, s_1_25, s_1_26);
        // D s_1_28: write-var operand2 <= s_1_27
        fn_state.operand2 = s_1_27;
        // D s_1_29: read-var VLshadow#2912:i64
        let s_1_29: i64 = fn_state.VLshadow_2912;
        // D s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_31: call Zeros(s_1_30)
        let s_1_31: Bits = Zeros(state, tracer, s_1_30);
        // D s_1_32: write-var result <= s_1_31
        fn_state.result = s_1_31;
        // C s_1_33: const #0s : i64
        let s_1_33: i64 = 0;
        // C s_1_34: const #1s : i
        let s_1_34: i128 = 1;
        // D s_1_35: read-var pairs:i64
        let s_1_35: i64 = fn_state.pairs;
        // D s_1_36: cast zx s_1_35 -> i
        let s_1_36: i128 = (i128::try_from(s_1_35).unwrap());
        // D s_1_37: sub s_1_36 s_1_34
        let s_1_37: i128 = ((s_1_36) - (s_1_34));
        // D s_1_38: cast reint s_1_37 -> i64
        let s_1_38: i64 = (s_1_37 as i64);
        // D s_1_39: write-var gs#195412 <= s_1_38
        fn_state.gs_195412 = s_1_38;
        // D s_1_40: write-var p <= s_1_33
        fn_state.p = s_1_33;
        // N s_1_41: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var p:i64
        let s_2_0: i64 = fn_state.p;
        // D s_2_1: read-var gs#195412:i64
        let s_2_1: i64 = fn_state.gs_195412;
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
        // D s_3_0: read-var esizeshadow#2910:i64
        let s_3_0: i64 = fn_state.esizeshadow_2910;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // C s_3_3: const #2s : i
        let s_3_3: i128 = 2;
        // D s_3_4: read-var p:i64
        let s_3_4: i64 = fn_state.p;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: mul s_3_3 s_3_5
        let s_3_6: i128 = ((s_3_3) * (s_3_5));
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: read-var part:i64
        let s_3_9: i64 = fn_state.part;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: add s_3_8 s_3_10
        let s_3_11: i128 = (s_3_8 + s_3_10);
        // D s_3_12: cast reint s_3_11 -> i64
        let s_3_12: i64 = (s_3_11 as i64);
        // D s_3_13: read-var esizeshadow#2910:i64
        let s_3_13: i64 = fn_state.esizeshadow_2910;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: cast zx s_3_12 -> i
        let s_3_16: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_17: cast zx s_3_15 -> i
        let s_3_17: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_18: read-var operand1:bv
        let s_3_18: Bits = fn_state.operand1;
        // D s_3_19: call Elem_read(s_3_18, s_3_16, s_3_17)
        let s_3_19: Bits = Elem_read(state, tracer, s_3_18, s_3_16, s_3_17);
        // D s_3_20: read-var p:i64
        let s_3_20: i64 = fn_state.p;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: cast zx s_3_2 -> i
        let s_3_22: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_23: read-var result:bv
        let s_3_23: Bits = fn_state.result;
        // D s_3_24: call Elem_set(s_3_23, s_3_21, s_3_22, s_3_19)
        let s_3_24: Bits = Elem_set(state, tracer, s_3_23, s_3_21, s_3_22, s_3_19);
        // D s_3_25: write-var result <= s_3_24
        fn_state.result = s_3_24;
        // D s_3_26: read-var p:i64
        let s_3_26: i64 = fn_state.p;
        // C s_3_27: const #1s : i64
        let s_3_27: i64 = 1;
        // D s_3_28: add s_3_26 s_3_27
        let s_3_28: i64 = (s_3_26 + s_3_27);
        // D s_3_29: write-var p <= s_3_28
        fn_state.p = s_3_28;
        // N s_3_30: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i64
        let s_4_0: i64 = 0;
        // C s_4_1: const #1s : i
        let s_4_1: i128 = 1;
        // D s_4_2: read-var pairs:i64
        let s_4_2: i64 = fn_state.pairs;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: sub s_4_3 s_4_1
        let s_4_4: i128 = ((s_4_3) - (s_4_1));
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: write-var gs#195420 <= s_4_5
        fn_state.gs_195420 = s_4_5;
        // D s_4_7: write-var u#3547 <= s_4_0
        fn_state.u_3547 = s_4_0;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var u#3547:i64
        let s_5_0: i64 = fn_state.u_3547;
        // D s_5_1: read-var gs#195420:i64
        let s_5_1: i64 = fn_state.gs_195420;
        // D s_5_2: cmp-gt s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) > (s_5_1));
        // N s_5_3: branch s_5_2 b7 b6
        if s_5_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var pairs:i64
        let s_6_0: i64 = fn_state.pairs;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var u#3547:i64
        let s_6_2: i64 = fn_state.u_3547;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: add s_6_1 s_6_3
        let s_6_4: i128 = (s_6_1 + s_6_3);
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: read-var esizeshadow#2910:i64
        let s_6_6: i64 = fn_state.esizeshadow_2910;
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_8: cast reint s_6_7 -> i64
        let s_6_8: i64 = (s_6_7 as i64);
        // C s_6_9: const #2s : i
        let s_6_9: i128 = 2;
        // D s_6_10: read-var u#3547:i64
        let s_6_10: i64 = fn_state.u_3547;
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_12: mul s_6_9 s_6_11
        let s_6_12: i128 = ((s_6_9) * (s_6_11));
        // D s_6_13: cast reint s_6_12 -> i64
        let s_6_13: i64 = (s_6_12 as i64);
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (i128::try_from(s_6_13).unwrap());
        // D s_6_15: read-var part:i64
        let s_6_15: i64 = fn_state.part;
        // D s_6_16: cast zx s_6_15 -> i
        let s_6_16: i128 = (i128::try_from(s_6_15).unwrap());
        // D s_6_17: add s_6_14 s_6_16
        let s_6_17: i128 = (s_6_14 + s_6_16);
        // D s_6_18: cast reint s_6_17 -> i64
        let s_6_18: i64 = (s_6_17 as i64);
        // D s_6_19: read-var esizeshadow#2910:i64
        let s_6_19: i64 = fn_state.esizeshadow_2910;
        // D s_6_20: cast zx s_6_19 -> i
        let s_6_20: i128 = (i128::try_from(s_6_19).unwrap());
        // D s_6_21: cast reint s_6_20 -> i64
        let s_6_21: i64 = (s_6_20 as i64);
        // D s_6_22: cast zx s_6_18 -> i
        let s_6_22: i128 = (i128::try_from(s_6_18).unwrap());
        // D s_6_23: cast zx s_6_21 -> i
        let s_6_23: i128 = (i128::try_from(s_6_21).unwrap());
        // D s_6_24: read-var operand2:bv
        let s_6_24: Bits = fn_state.operand2;
        // D s_6_25: call Elem_read(s_6_24, s_6_22, s_6_23)
        let s_6_25: Bits = Elem_read(state, tracer, s_6_24, s_6_22, s_6_23);
        // D s_6_26: cast zx s_6_5 -> i
        let s_6_26: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_27: cast zx s_6_8 -> i
        let s_6_27: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_28: read-var result:bv
        let s_6_28: Bits = fn_state.result;
        // D s_6_29: call Elem_set(s_6_28, s_6_26, s_6_27, s_6_25)
        let s_6_29: Bits = Elem_set(state, tracer, s_6_28, s_6_26, s_6_27, s_6_25);
        // D s_6_30: write-var result <= s_6_29
        fn_state.result = s_6_29;
        // D s_6_31: read-var u#3547:i64
        let s_6_31: i64 = fn_state.u_3547;
        // C s_6_32: const #1s : i64
        let s_6_32: i64 = 1;
        // D s_6_33: add s_6_31 s_6_32
        let s_6_33: i64 = (s_6_31 + s_6_32);
        // D s_6_34: write-var u#3547 <= s_6_33
        fn_state.u_3547 = s_6_33;
        // N s_6_35: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VLshadow#2912:i64
        let s_7_0: i64 = fn_state.VLshadow_2912;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast zx s_7_2 -> i
        let s_7_5: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_6: read-var result:bv
        let s_7_6: Bits = fn_state.result;
        // D s_7_7: call Z_set(s_7_4, s_7_5, s_7_6)
        let s_7_7: () = Z_set(state, tracer, s_7_4, s_7_5, s_7_6);
        // N s_7_8: return
        return;
    }
}
