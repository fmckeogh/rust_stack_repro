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
use V_read::*;
use Elem_read::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_transfer_vector_permute_transpose<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    esize: i64,
    m: i64,
    n: i64,
    pairs: i128,
    part: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        datasizeshadow_1999: i64,
        p: i64,
        esizeshadow_1998: i64,
        gs_173986: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        d: i64,
        datasize: i64,
        esize: i64,
        m: i64,
        n: i64,
        pairs: i128,
        part: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        esize,
        m,
        n,
        pairs,
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
        // D s_0_3: write-var esizeshadow#1998 <= s_0_2
        fn_state.esizeshadow_1998 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1999 <= s_0_6
        fn_state.datasizeshadow_1999 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckFPAdvSIMDEnabled64(s_0_8)
        let s_0_9: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1999:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1999;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call V_read(s_1_4, s_1_2)
        let s_1_5: Bits = V_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: write-var operand1 <= s_1_5
        fn_state.operand1 = s_1_5;
        // D s_1_7: read-var datasizeshadow#1999:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1999;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var m:i64
        let s_1_10: i64 = fn_state.m;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: call V_read(s_1_11, s_1_9)
        let s_1_12: Bits = V_read(state, tracer, s_1_11, s_1_9);
        // D s_1_13: write-var operand2 <= s_1_12
        fn_state.operand2 = s_1_12;
        // C s_1_14: const #0s : i64
        let s_1_14: i64 = 0;
        // C s_1_15: const #1s : i
        let s_1_15: i128 = 1;
        // D s_1_16: read-var pairs:i
        let s_1_16: i128 = fn_state.pairs;
        // D s_1_17: sub s_1_16 s_1_15
        let s_1_17: i128 = ((s_1_16) - (s_1_15));
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: write-var gs#173986 <= s_1_18
        fn_state.gs_173986 = s_1_18;
        // D s_1_20: write-var p <= s_1_14
        fn_state.p = s_1_14;
        // N s_1_21: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var p:i64
        let s_2_0: i64 = fn_state.p;
        // D s_2_1: read-var gs#173986:i64
        let s_2_1: i64 = fn_state.gs_173986;
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
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var p:i64
        let s_3_1: i64 = fn_state.p;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // C s_3_4: const #0s : i
        let s_3_4: i128 = 0;
        // D s_3_5: add s_3_3 s_3_4
        let s_3_5: i128 = (s_3_3 + s_3_4);
        // D s_3_6: read-var esizeshadow#1998:i64
        let s_3_6: i64 = fn_state.esizeshadow_1998;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // C s_3_9: const #2s : i
        let s_3_9: i128 = 2;
        // D s_3_10: read-var p:i64
        let s_3_10: i64 = fn_state.p;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: mul s_3_9 s_3_11
        let s_3_12: i128 = ((s_3_9) * (s_3_11));
        // D s_3_13: read-var part:i64
        let s_3_13: i64 = fn_state.part;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: add s_3_12 s_3_14
        let s_3_15: i128 = (s_3_12 + s_3_14);
        // D s_3_16: read-var esizeshadow#1998:i64
        let s_3_16: i64 = fn_state.esizeshadow_1998;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: read-var operand1:bv
        let s_3_20: Bits = fn_state.operand1;
        // D s_3_21: call Elem_read(s_3_20, s_3_15, s_3_19)
        let s_3_21: Bits = Elem_read(state, tracer, s_3_20, s_3_15, s_3_19);
        // D s_3_22: cast zx s_3_8 -> i
        let s_3_22: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_23: read-var result:bv
        let s_3_23: Bits = fn_state.result;
        // D s_3_24: call Elem_set(s_3_23, s_3_5, s_3_22, s_3_21)
        let s_3_24: Bits = Elem_set(state, tracer, s_3_23, s_3_5, s_3_22, s_3_21);
        // D s_3_25: write-var result <= s_3_24
        fn_state.result = s_3_24;
        // C s_3_26: const #2s : i
        let s_3_26: i128 = 2;
        // D s_3_27: read-var p:i64
        let s_3_27: i64 = fn_state.p;
        // D s_3_28: cast zx s_3_27 -> i
        let s_3_28: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_29: mul s_3_26 s_3_28
        let s_3_29: i128 = ((s_3_26) * (s_3_28));
        // C s_3_30: const #1s : i
        let s_3_30: i128 = 1;
        // D s_3_31: add s_3_29 s_3_30
        let s_3_31: i128 = (s_3_29 + s_3_30);
        // D s_3_32: read-var esizeshadow#1998:i64
        let s_3_32: i64 = fn_state.esizeshadow_1998;
        // D s_3_33: cast zx s_3_32 -> i
        let s_3_33: i128 = (i128::try_from(s_3_32).unwrap());
        // D s_3_34: cast reint s_3_33 -> i64
        let s_3_34: i64 = (s_3_33 as i64);
        // C s_3_35: const #2s : i
        let s_3_35: i128 = 2;
        // D s_3_36: read-var p:i64
        let s_3_36: i64 = fn_state.p;
        // D s_3_37: cast zx s_3_36 -> i
        let s_3_37: i128 = (i128::try_from(s_3_36).unwrap());
        // D s_3_38: mul s_3_35 s_3_37
        let s_3_38: i128 = ((s_3_35) * (s_3_37));
        // D s_3_39: read-var part:i64
        let s_3_39: i64 = fn_state.part;
        // D s_3_40: cast zx s_3_39 -> i
        let s_3_40: i128 = (i128::try_from(s_3_39).unwrap());
        // D s_3_41: add s_3_38 s_3_40
        let s_3_41: i128 = (s_3_38 + s_3_40);
        // D s_3_42: read-var esizeshadow#1998:i64
        let s_3_42: i64 = fn_state.esizeshadow_1998;
        // D s_3_43: cast zx s_3_42 -> i
        let s_3_43: i128 = (i128::try_from(s_3_42).unwrap());
        // D s_3_44: cast reint s_3_43 -> i64
        let s_3_44: i64 = (s_3_43 as i64);
        // D s_3_45: cast zx s_3_44 -> i
        let s_3_45: i128 = (i128::try_from(s_3_44).unwrap());
        // D s_3_46: read-var operand2:bv
        let s_3_46: Bits = fn_state.operand2;
        // D s_3_47: call Elem_read(s_3_46, s_3_41, s_3_45)
        let s_3_47: Bits = Elem_read(state, tracer, s_3_46, s_3_41, s_3_45);
        // D s_3_48: cast zx s_3_34 -> i
        let s_3_48: i128 = (i128::try_from(s_3_34).unwrap());
        // D s_3_49: read-var result:bv
        let s_3_49: Bits = fn_state.result;
        // D s_3_50: call Elem_set(s_3_49, s_3_31, s_3_48, s_3_47)
        let s_3_50: Bits = Elem_set(state, tracer, s_3_49, s_3_31, s_3_48, s_3_47);
        // D s_3_51: write-var result <= s_3_50
        fn_state.result = s_3_50;
        // D s_3_52: read-var p:i64
        let s_3_52: i64 = fn_state.p;
        // C s_3_53: const #1s : i64
        let s_3_53: i64 = 1;
        // D s_3_54: add s_3_52 s_3_53
        let s_3_54: i64 = (s_3_52 + s_3_53);
        // D s_3_55: write-var p <= s_3_54
        fn_state.p = s_3_54;
        // N s_3_56: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasizeshadow#1999:i64
        let s_4_0: i64 = fn_state.datasizeshadow_1999;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: read-var result:bv
        let s_4_5: Bits = fn_state.result;
        // D s_4_6: call V_set(s_4_4, s_4_2, s_4_5)
        let s_4_6: () = V_set(state, tracer, s_4_4, s_4_2, s_4_5);
        // N s_4_7: return
        return;
    }
}
