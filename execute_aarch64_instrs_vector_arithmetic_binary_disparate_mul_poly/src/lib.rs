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
use Vpart_read::*;
use Elem_set::*;
use Elem_read::*;
use PolynomialMult::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_disparate_mul_poly<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    part: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        esizeshadow_1779: i64,
        gs_165697: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        part: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
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
        // D s_0_3: write-var esizeshadow#1779 <= s_0_2
        fn_state.esizeshadow_1779 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckFPAdvSIMDEnabled64(s_0_4)
        let s_0_5: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasize:i64
        let s_1_0: i64 = fn_state.datasize;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: read-var part:i64
        let s_1_5: i64 = fn_state.part;
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_7: cast zx s_1_2 -> i
        let s_1_7: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_8: call Vpart_read(s_1_4, s_1_6, s_1_7)
        let s_1_8: Bits = Vpart_read(state, tracer, s_1_4, s_1_6, s_1_7);
        // D s_1_9: write-var operand1 <= s_1_8
        fn_state.operand1 = s_1_8;
        // D s_1_10: read-var datasize:i64
        let s_1_10: i64 = fn_state.datasize;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: read-var m:i64
        let s_1_13: i64 = fn_state.m;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: read-var part:i64
        let s_1_15: i64 = fn_state.part;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_12 -> i
        let s_1_17: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_18: call Vpart_read(s_1_14, s_1_16, s_1_17)
        let s_1_18: Bits = Vpart_read(state, tracer, s_1_14, s_1_16, s_1_17);
        // D s_1_19: write-var operand2 <= s_1_18
        fn_state.operand2 = s_1_18;
        // C s_1_20: const #0s : i64
        let s_1_20: i64 = 0;
        // C s_1_21: const #1s : i
        let s_1_21: i128 = 1;
        // D s_1_22: read-var elements:i
        let s_1_22: i128 = fn_state.elements;
        // D s_1_23: sub s_1_22 s_1_21
        let s_1_23: i128 = ((s_1_22) - (s_1_21));
        // D s_1_24: cast reint s_1_23 -> i64
        let s_1_24: i64 = (s_1_23 as i64);
        // D s_1_25: write-var gs#165697 <= s_1_24
        fn_state.gs_165697 = s_1_24;
        // D s_1_26: write-var e <= s_1_20
        fn_state.e = s_1_20;
        // N s_1_27: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#165697:i64
        let s_2_1: i64 = fn_state.gs_165697;
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
        // D s_3_0: read-var esizeshadow#1779:i64
        let s_3_0: i64 = fn_state.esizeshadow_1779;
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
        // D s_3_6: read-var operand1:bv
        let s_3_6: Bits = fn_state.operand1;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: read-var esizeshadow#1779:i64
        let s_3_8: i64 = fn_state.esizeshadow_1779;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: read-var e:i64
        let s_3_11: i64 = fn_state.e;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: cast zx s_3_10 -> i
        let s_3_13: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_14: read-var operand2:bv
        let s_3_14: Bits = fn_state.operand2;
        // D s_3_15: call Elem_read(s_3_14, s_3_12, s_3_13)
        let s_3_15: Bits = Elem_read(state, tracer, s_3_14, s_3_12, s_3_13);
        // C s_3_16: const #2s : i
        let s_3_16: i128 = 2;
        // D s_3_17: read-var esizeshadow#1779:i64
        let s_3_17: i64 = fn_state.esizeshadow_1779;
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: mul s_3_16 s_3_18
        let s_3_19: i128 = ((s_3_16) * (s_3_18));
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: cast reint s_3_21 -> i64
        let s_3_22: i64 = (s_3_21 as i64);
        // D s_3_23: call PolynomialMult(s_3_7, s_3_15)
        let s_3_23: Bits = PolynomialMult(state, tracer, s_3_7, s_3_15);
        // D s_3_24: read-var e:i64
        let s_3_24: i64 = fn_state.e;
        // D s_3_25: cast zx s_3_24 -> i
        let s_3_25: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_26: cast zx s_3_22 -> i
        let s_3_26: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_27: read-var result:bv
        let s_3_27: Bits = fn_state.result;
        // D s_3_28: call Elem_set(s_3_27, s_3_25, s_3_26, s_3_23)
        let s_3_28: Bits = Elem_set(state, tracer, s_3_27, s_3_25, s_3_26, s_3_23);
        // D s_3_29: write-var result <= s_3_28
        fn_state.result = s_3_28;
        // D s_3_30: read-var e:i64
        let s_3_30: i64 = fn_state.e;
        // C s_3_31: const #1s : i64
        let s_3_31: i64 = 1;
        // D s_3_32: add s_3_30 s_3_31
        let s_3_32: i64 = (s_3_30 + s_3_31);
        // D s_3_33: write-var e <= s_3_32
        fn_state.e = s_3_32;
        // N s_3_34: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2s : i
        let s_4_0: i128 = 2;
        // D s_4_1: read-var datasize:i64
        let s_4_1: i64 = fn_state.datasize;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: mul s_4_0 s_4_2
        let s_4_3: i128 = ((s_4_0) * (s_4_2));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: cast reint s_4_5 -> i64
        let s_4_6: i64 = (s_4_5 as i64);
        // D s_4_7: read-var d:i64
        let s_4_7: i64 = fn_state.d;
        // D s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (i128::try_from(s_4_7).unwrap());
        // D s_4_9: read-var result:bv
        let s_4_9: Bits = fn_state.result;
        // D s_4_10: call V_set(s_4_8, s_4_6, s_4_9)
        let s_4_10: () = V_set(state, tracer, s_4_8, s_4_6, s_4_9);
        // N s_4_11: return
        return;
    }
}
