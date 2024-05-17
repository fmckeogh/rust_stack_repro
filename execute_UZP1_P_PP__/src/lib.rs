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
use Elem_read::*;
use CheckSVEEnabled::*;
use P_set::*;
use P_read::*;
use common::*;
pub fn execute_UZP1_P_PP__<T: Tracer>(
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
        p: i64,
        gs_194849: i64,
        gs_194839: i64,
        PL: i64,
        pairs: i64,
        u_3504: i64,
        result: Bits,
        operand1: Bits,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckSVEEnabled(s_0_0)
        let s_0_1: () = CheckSVEEnabled(state, tracer, s_0_0);
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
        // C s_1_6: const #2s : i
        let s_1_6: i128 = 2;
        // D s_1_7: read-var esize:i64
        let s_1_7: i64 = fn_state.esize;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: mul s_1_8 s_1_6
        let s_1_9: i128 = ((s_1_8) * (s_1_6));
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: cast zx s_1_0 -> i
        let s_1_11: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_12: cast zx s_1_10 -> i
        let s_1_12: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_13: div s_1_11 s_1_12
        let s_1_13: i128 = ((s_1_11) / (s_1_12));
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: write-var pairs <= s_1_14
        fn_state.pairs = s_1_14;
        // D s_1_16: read-var PL:i64
        let s_1_16: i64 = fn_state.PL;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: read-var n:i64
        let s_1_19: i64 = fn_state.n;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast zx s_1_18 -> i
        let s_1_21: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_22: call P_read(s_1_20, s_1_21)
        let s_1_22: Bits = P_read(state, tracer, s_1_20, s_1_21);
        // D s_1_23: write-var operand1 <= s_1_22
        fn_state.operand1 = s_1_22;
        // D s_1_24: read-var PL:i64
        let s_1_24: i64 = fn_state.PL;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // D s_1_27: read-var m:i64
        let s_1_27: i64 = fn_state.m;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: cast zx s_1_26 -> i
        let s_1_29: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_30: call P_read(s_1_28, s_1_29)
        let s_1_30: Bits = P_read(state, tracer, s_1_28, s_1_29);
        // D s_1_31: write-var operand2 <= s_1_30
        fn_state.operand2 = s_1_30;
        // C s_1_32: const #0s : i64
        let s_1_32: i64 = 0;
        // C s_1_33: const #1s : i
        let s_1_33: i128 = 1;
        // D s_1_34: read-var pairs:i64
        let s_1_34: i64 = fn_state.pairs;
        // D s_1_35: cast zx s_1_34 -> i
        let s_1_35: i128 = (i128::try_from(s_1_34).unwrap());
        // D s_1_36: sub s_1_35 s_1_33
        let s_1_36: i128 = ((s_1_35) - (s_1_33));
        // D s_1_37: cast reint s_1_36 -> i64
        let s_1_37: i64 = (s_1_36 as i64);
        // D s_1_38: write-var gs#194839 <= s_1_37
        fn_state.gs_194839 = s_1_37;
        // D s_1_39: write-var p <= s_1_32
        fn_state.p = s_1_32;
        // N s_1_40: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var p:i64
        let s_2_0: i64 = fn_state.p;
        // D s_2_1: read-var gs#194839:i64
        let s_2_1: i64 = fn_state.gs_194839;
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
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // D s_3_1: read-var esize:i64
        let s_3_1: i64 = fn_state.esize;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: div s_3_2 s_3_0
        let s_3_3: i128 = ((s_3_2) / (s_3_0));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // C s_3_7: const #2s : i
        let s_3_7: i128 = 2;
        // D s_3_8: read-var p:i64
        let s_3_8: i64 = fn_state.p;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: mul s_3_7 s_3_9
        let s_3_10: i128 = ((s_3_7) * (s_3_9));
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: read-var part:i64
        let s_3_13: i64 = fn_state.part;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: add s_3_12 s_3_14
        let s_3_15: i128 = (s_3_12 + s_3_14);
        // D s_3_16: cast reint s_3_15 -> i64
        let s_3_16: i64 = (s_3_15 as i64);
        // C s_3_17: const #8s : i
        let s_3_17: i128 = 8;
        // D s_3_18: read-var esize:i64
        let s_3_18: i64 = fn_state.esize;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: div s_3_19 s_3_17
        let s_3_20: i128 = ((s_3_19) / (s_3_17));
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // D s_3_22: cast zx s_3_21 -> i
        let s_3_22: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_23: cast reint s_3_22 -> i64
        let s_3_23: i64 = (s_3_22 as i64);
        // D s_3_24: cast zx s_3_16 -> i
        let s_3_24: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_25: cast zx s_3_23 -> i
        let s_3_25: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_26: read-var operand1:bv
        let s_3_26: Bits = fn_state.operand1;
        // D s_3_27: call Elem_read(s_3_26, s_3_24, s_3_25)
        let s_3_27: Bits = Elem_read(state, tracer, s_3_26, s_3_24, s_3_25);
        // D s_3_28: read-var p:i64
        let s_3_28: i64 = fn_state.p;
        // D s_3_29: cast zx s_3_28 -> i
        let s_3_29: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_30: cast zx s_3_6 -> i
        let s_3_30: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_31: read-var result:bv
        let s_3_31: Bits = fn_state.result;
        // D s_3_32: call Elem_set(s_3_31, s_3_29, s_3_30, s_3_27)
        let s_3_32: Bits = Elem_set(state, tracer, s_3_31, s_3_29, s_3_30, s_3_27);
        // D s_3_33: write-var result <= s_3_32
        fn_state.result = s_3_32;
        // D s_3_34: read-var p:i64
        let s_3_34: i64 = fn_state.p;
        // C s_3_35: const #1s : i64
        let s_3_35: i64 = 1;
        // D s_3_36: add s_3_34 s_3_35
        let s_3_36: i64 = (s_3_34 + s_3_35);
        // D s_3_37: write-var p <= s_3_36
        fn_state.p = s_3_36;
        // N s_3_38: jump b2
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
        // D s_4_6: write-var gs#194849 <= s_4_5
        fn_state.gs_194849 = s_4_5;
        // D s_4_7: write-var u#3504 <= s_4_0
        fn_state.u_3504 = s_4_0;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var u#3504:i64
        let s_5_0: i64 = fn_state.u_3504;
        // D s_5_1: read-var gs#194849:i64
        let s_5_1: i64 = fn_state.gs_194849;
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
        // D s_6_2: read-var u#3504:i64
        let s_6_2: i64 = fn_state.u_3504;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: add s_6_1 s_6_3
        let s_6_4: i128 = (s_6_1 + s_6_3);
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // C s_6_6: const #8s : i
        let s_6_6: i128 = 8;
        // D s_6_7: read-var esize:i64
        let s_6_7: i64 = fn_state.esize;
        // D s_6_8: cast zx s_6_7 -> i
        let s_6_8: i128 = (i128::try_from(s_6_7).unwrap());
        // D s_6_9: div s_6_8 s_6_6
        let s_6_9: i128 = ((s_6_8) / (s_6_6));
        // D s_6_10: cast reint s_6_9 -> i64
        let s_6_10: i64 = (s_6_9 as i64);
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_12: cast reint s_6_11 -> i64
        let s_6_12: i64 = (s_6_11 as i64);
        // C s_6_13: const #2s : i
        let s_6_13: i128 = 2;
        // D s_6_14: read-var u#3504:i64
        let s_6_14: i64 = fn_state.u_3504;
        // D s_6_15: cast zx s_6_14 -> i
        let s_6_15: i128 = (i128::try_from(s_6_14).unwrap());
        // D s_6_16: mul s_6_13 s_6_15
        let s_6_16: i128 = ((s_6_13) * (s_6_15));
        // D s_6_17: cast reint s_6_16 -> i64
        let s_6_17: i64 = (s_6_16 as i64);
        // D s_6_18: cast zx s_6_17 -> i
        let s_6_18: i128 = (i128::try_from(s_6_17).unwrap());
        // D s_6_19: read-var part:i64
        let s_6_19: i64 = fn_state.part;
        // D s_6_20: cast zx s_6_19 -> i
        let s_6_20: i128 = (i128::try_from(s_6_19).unwrap());
        // D s_6_21: add s_6_18 s_6_20
        let s_6_21: i128 = (s_6_18 + s_6_20);
        // D s_6_22: cast reint s_6_21 -> i64
        let s_6_22: i64 = (s_6_21 as i64);
        // C s_6_23: const #8s : i
        let s_6_23: i128 = 8;
        // D s_6_24: read-var esize:i64
        let s_6_24: i64 = fn_state.esize;
        // D s_6_25: cast zx s_6_24 -> i
        let s_6_25: i128 = (i128::try_from(s_6_24).unwrap());
        // D s_6_26: div s_6_25 s_6_23
        let s_6_26: i128 = ((s_6_25) / (s_6_23));
        // D s_6_27: cast reint s_6_26 -> i64
        let s_6_27: i64 = (s_6_26 as i64);
        // D s_6_28: cast zx s_6_27 -> i
        let s_6_28: i128 = (i128::try_from(s_6_27).unwrap());
        // D s_6_29: cast reint s_6_28 -> i64
        let s_6_29: i64 = (s_6_28 as i64);
        // D s_6_30: cast zx s_6_22 -> i
        let s_6_30: i128 = (i128::try_from(s_6_22).unwrap());
        // D s_6_31: cast zx s_6_29 -> i
        let s_6_31: i128 = (i128::try_from(s_6_29).unwrap());
        // D s_6_32: read-var operand2:bv
        let s_6_32: Bits = fn_state.operand2;
        // D s_6_33: call Elem_read(s_6_32, s_6_30, s_6_31)
        let s_6_33: Bits = Elem_read(state, tracer, s_6_32, s_6_30, s_6_31);
        // D s_6_34: cast zx s_6_5 -> i
        let s_6_34: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_35: cast zx s_6_12 -> i
        let s_6_35: i128 = (i128::try_from(s_6_12).unwrap());
        // D s_6_36: read-var result:bv
        let s_6_36: Bits = fn_state.result;
        // D s_6_37: call Elem_set(s_6_36, s_6_34, s_6_35, s_6_33)
        let s_6_37: Bits = Elem_set(state, tracer, s_6_36, s_6_34, s_6_35, s_6_33);
        // D s_6_38: write-var result <= s_6_37
        fn_state.result = s_6_37;
        // D s_6_39: read-var u#3504:i64
        let s_6_39: i64 = fn_state.u_3504;
        // C s_6_40: const #1s : i64
        let s_6_40: i64 = 1;
        // D s_6_41: add s_6_39 s_6_40
        let s_6_41: i64 = (s_6_39 + s_6_40);
        // D s_6_42: write-var u#3504 <= s_6_41
        fn_state.u_3504 = s_6_41;
        // N s_6_43: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var PL:i64
        let s_7_0: i64 = fn_state.PL;
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
        // D s_7_7: call P_set(s_7_4, s_7_5, s_7_6)
        let s_7_7: () = P_set(state, tracer, s_7_4, s_7_5, s_7_6);
        // N s_7_8: return
        return;
    }
}
