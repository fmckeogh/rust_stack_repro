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
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_TBX_Z_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: i128,
        VLshadow_4097: i64,
        e: i64,
        esizeshadow_4096: i64,
        elements: i64,
        gs_215778: i64,
        result: Bits,
        operand1: Bits,
        VLshadow_4098: i64,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        m,
        n,
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
        // D s_0_3: write-var esizeshadow#4096 <= s_0_2
        fn_state.esizeshadow_4096 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#4097 <= s_0_6
        fn_state.VLshadow_4097 = s_0_6;
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
        // D s_1_0: read-var VLshadow#4097:i64
        let s_1_0: i64 = fn_state.VLshadow_4097;
        // D s_1_1: write-var VLshadow#4098 <= s_1_0
        fn_state.VLshadow_4098 = s_1_0;
        // D s_1_2: read-var VLshadow#4098:i64
        let s_1_2: i64 = fn_state.VLshadow_4098;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#4096:i64
        let s_1_4: i64 = fn_state.esizeshadow_4096;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: write-var elements <= s_1_7
        fn_state.elements = s_1_7;
        // D s_1_9: read-var VLshadow#4098:i64
        let s_1_9: i64 = fn_state.VLshadow_4098;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var n:i64
        let s_1_12: i64 = fn_state.n;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast zx s_1_11 -> i
        let s_1_14: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_15: call Z_read(s_1_13, s_1_14)
        let s_1_15: Bits = Z_read(state, tracer, s_1_13, s_1_14);
        // D s_1_16: write-var operand1 <= s_1_15
        fn_state.operand1 = s_1_15;
        // D s_1_17: read-var VLshadow#4098:i64
        let s_1_17: i64 = fn_state.VLshadow_4098;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // D s_1_20: read-var m:i64
        let s_1_20: i64 = fn_state.m;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: cast zx s_1_19 -> i
        let s_1_22: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_23: call Z_read(s_1_21, s_1_22)
        let s_1_23: Bits = Z_read(state, tracer, s_1_21, s_1_22);
        // D s_1_24: write-var operand2 <= s_1_23
        fn_state.operand2 = s_1_23;
        // D s_1_25: read-var VLshadow#4098:i64
        let s_1_25: i64 = fn_state.VLshadow_4098;
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_27: cast reint s_1_26 -> i64
        let s_1_27: i64 = (s_1_26 as i64);
        // D s_1_28: read-var d:i64
        let s_1_28: i64 = fn_state.d;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: cast zx s_1_27 -> i
        let s_1_30: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_31: call Z_read(s_1_29, s_1_30)
        let s_1_31: Bits = Z_read(state, tracer, s_1_29, s_1_30);
        // D s_1_32: write-var result <= s_1_31
        fn_state.result = s_1_31;
        // C s_1_33: const #0s : i64
        let s_1_33: i64 = 0;
        // C s_1_34: const #1s : i
        let s_1_34: i128 = 1;
        // D s_1_35: read-var elements:i64
        let s_1_35: i64 = fn_state.elements;
        // D s_1_36: cast zx s_1_35 -> i
        let s_1_36: i128 = (i128::try_from(s_1_35).unwrap());
        // D s_1_37: sub s_1_36 s_1_34
        let s_1_37: i128 = ((s_1_36) - (s_1_34));
        // D s_1_38: cast reint s_1_37 -> i64
        let s_1_38: i64 = (s_1_37 as i64);
        // D s_1_39: write-var gs#215778 <= s_1_38
        fn_state.gs_215778 = s_1_38;
        // D s_1_40: write-var e <= s_1_33
        fn_state.e = s_1_33;
        // N s_1_41: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#215778:i64
        let s_2_1: i64 = fn_state.gs_215778;
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
        // D s_3_0: read-var esizeshadow#4096:i64
        let s_3_0: i64 = fn_state.esizeshadow_4096;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var operand2:bv
        let s_3_6: Bits = fn_state.operand2;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (s_3_7.value() as i128);
        // D s_3_9: write-var element2 <= s_3_8
        fn_state.element2 = s_3_8;
        // D s_3_10: read-var elements:i64
        let s_3_10: i64 = fn_state.elements;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: read-var element2:i
        let s_3_12: i128 = fn_state.element2;
        // D s_3_13: cmp-lt s_3_12 s_3_11
        let s_3_13: bool = ((s_3_12) < (s_3_11));
        // N s_3_14: branch s_3_13 b6 b4
        if s_3_13 {
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
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // C s_5_1: const #1s : i64
        let s_5_1: i64 = 1;
        // D s_5_2: add s_5_0 s_5_1
        let s_5_2: i64 = (s_5_0 + s_5_1);
        // D s_5_3: write-var e <= s_5_2
        fn_state.e = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#4096:i64
        let s_6_0: i64 = fn_state.esizeshadow_4096;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var esizeshadow#4096:i64
        let s_6_3: i64 = fn_state.esizeshadow_4096;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: read-var operand1:bv
        let s_6_7: Bits = fn_state.operand1;
        // D s_6_8: read-var element2:i
        let s_6_8: i128 = fn_state.element2;
        // D s_6_9: call Elem_read(s_6_7, s_6_8, s_6_6)
        let s_6_9: Bits = Elem_read(state, tracer, s_6_7, s_6_8, s_6_6);
        // D s_6_10: read-var e:i64
        let s_6_10: i64 = fn_state.e;
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_12: cast zx s_6_2 -> i
        let s_6_12: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_13: read-var result:bv
        let s_6_13: Bits = fn_state.result;
        // D s_6_14: call Elem_set(s_6_13, s_6_11, s_6_12, s_6_9)
        let s_6_14: Bits = Elem_set(state, tracer, s_6_13, s_6_11, s_6_12, s_6_9);
        // D s_6_15: write-var result <= s_6_14
        fn_state.result = s_6_14;
        // N s_6_16: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VLshadow#4098:i64
        let s_7_0: i64 = fn_state.VLshadow_4098;
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