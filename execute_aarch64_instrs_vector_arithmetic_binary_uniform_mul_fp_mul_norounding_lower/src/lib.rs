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
use V_read::*;
use FPCR_read::*;
use V_set::*;
use FPMulAddH::*;
use FPNeg::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    part: i64,
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: Bits,
        e: i64,
        element1: Bits,
        ga_255724: i64,
        operand3: Bits,
        gs_153353: i64,
        result: Bits,
        operand1: Bits,
        datasizeshadow_1403: i64,
        operand2: Bits,
        gs_695190: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        part: i64,
        sub_op: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        m,
        n,
        part,
        sub_op,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var datasizeshadow#1403 <= s_0_2
        fn_state.datasizeshadow_1403 = s_0_2;
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
        // C s_1_0: const #2s : i
        let s_1_0: i128 = 2;
        // D s_1_1: read-var datasizeshadow#1403:i64
        let s_1_1: i64 = fn_state.datasizeshadow_1403;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: div s_1_2 s_1_0
        let s_1_3: i128 = ((s_1_2) / (s_1_0));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var n:i64
        let s_1_7: i64 = fn_state.n;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var part:i64
        let s_1_9: i64 = fn_state.part;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: cast zx s_1_6 -> i
        let s_1_11: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_12: call Vpart_read(s_1_8, s_1_10, s_1_11)
        let s_1_12: Bits = Vpart_read(state, tracer, s_1_8, s_1_10, s_1_11);
        // D s_1_13: write-var operand1 <= s_1_12
        fn_state.operand1 = s_1_12;
        // C s_1_14: const #2s : i
        let s_1_14: i128 = 2;
        // D s_1_15: read-var datasizeshadow#1403:i64
        let s_1_15: i64 = fn_state.datasizeshadow_1403;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: div s_1_16 s_1_14
        let s_1_17: i128 = ((s_1_16) / (s_1_14));
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: read-var m:i64
        let s_1_21: i64 = fn_state.m;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: read-var part:i64
        let s_1_23: i64 = fn_state.part;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast zx s_1_20 -> i
        let s_1_25: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_26: call Vpart_read(s_1_22, s_1_24, s_1_25)
        let s_1_26: Bits = Vpart_read(state, tracer, s_1_22, s_1_24, s_1_25);
        // D s_1_27: write-var operand2 <= s_1_26
        fn_state.operand2 = s_1_26;
        // D s_1_28: read-var datasizeshadow#1403:i64
        let s_1_28: i64 = fn_state.datasizeshadow_1403;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: read-var d:i64
        let s_1_31: i64 = fn_state.d;
        // D s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (i128::try_from(s_1_31).unwrap());
        // D s_1_33: call V_read(s_1_32, s_1_30)
        let s_1_33: Bits = V_read(state, tracer, s_1_32, s_1_30);
        // D s_1_34: write-var operand3 <= s_1_33
        fn_state.operand3 = s_1_33;
        // C s_1_35: const #0s : i64
        let s_1_35: i64 = 0;
        // C s_1_36: const #1s : i
        let s_1_36: i128 = 1;
        // D s_1_37: read-var elements:i
        let s_1_37: i128 = fn_state.elements;
        // D s_1_38: sub s_1_37 s_1_36
        let s_1_38: i128 = ((s_1_37) - (s_1_36));
        // D s_1_39: cast reint s_1_38 -> i64
        let s_1_39: i64 = (s_1_38 as i64);
        // D s_1_40: write-var gs#153353 <= s_1_39
        fn_state.gs_153353 = s_1_39;
        // D s_1_41: write-var e <= s_1_35
        fn_state.e = s_1_35;
        // N s_1_42: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#153353:i64
        let s_2_1: i64 = fn_state.gs_153353;
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
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
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
        // D s_3_7: read-var e:i64
        let s_3_7: i64 = fn_state.e;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: cast zx s_3_6 -> i
        let s_3_9: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_10: read-var operand1:bv
        let s_3_10: Bits = fn_state.operand1;
        // D s_3_11: call Elem_read(s_3_10, s_3_8, s_3_9)
        let s_3_11: Bits = Elem_read(state, tracer, s_3_10, s_3_8, s_3_9);
        // D s_3_12: write-var element1 <= s_3_11
        fn_state.element1 = s_3_11;
        // C s_3_13: const #2s : i
        let s_3_13: i128 = 2;
        // D s_3_14: read-var esize:i64
        let s_3_14: i64 = fn_state.esize;
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: div s_3_15 s_3_13
        let s_3_16: i128 = ((s_3_15) / (s_3_13));
        // D s_3_17: cast reint s_3_16 -> i64
        let s_3_17: i64 = (s_3_16 as i64);
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: cast reint s_3_18 -> i64
        let s_3_19: i64 = (s_3_18 as i64);
        // D s_3_20: read-var e:i64
        let s_3_20: i64 = fn_state.e;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: cast zx s_3_19 -> i
        let s_3_22: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_23: read-var operand2:bv
        let s_3_23: Bits = fn_state.operand2;
        // D s_3_24: call Elem_read(s_3_23, s_3_21, s_3_22)
        let s_3_24: Bits = Elem_read(state, tracer, s_3_23, s_3_21, s_3_22);
        // D s_3_25: write-var element2 <= s_3_24
        fn_state.element2 = s_3_24;
        // D s_3_26: read-var sub_op:u8
        let s_3_26: bool = fn_state.sub_op;
        // N s_3_27: branch s_3_26 b7 b4
        if s_3_26 {
            return block_7(state, tracer, fn_state);
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
        // D s_5_0: read-var esize:i64
        let s_5_0: i64 = fn_state.esize;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: write-var ga#255724 <= s_5_2
        fn_state.ga_255724 = s_5_2;
        // D s_5_4: read-var esize:i64
        let s_5_4: i64 = fn_state.esize;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: cast reint s_5_5 -> i64
        let s_5_6: i64 = (s_5_5 as i64);
        // D s_5_7: read-var e:i64
        let s_5_7: i64 = fn_state.e;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: cast zx s_5_6 -> i
        let s_5_9: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_10: read-var operand3:bv
        let s_5_10: Bits = fn_state.operand3;
        // D s_5_11: call Elem_read(s_5_10, s_5_8, s_5_9)
        let s_5_11: Bits = Elem_read(state, tracer, s_5_10, s_5_8, s_5_9);
        // D s_5_12: cast reint s_5_11 -> u32
        let s_5_12: u32 = (s_5_11.value() as u32);
        // C s_5_13: const #() : ()
        let s_5_13: () = ();
        // S s_5_14: call FPCR_read(s_5_13)
        let s_5_14: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_5_13);
        // D s_5_15: cast zx s_5_12 -> bv
        let s_5_15: Bits = Bits::new(s_5_12 as u128, 32u16);
        // D s_5_16: read-var element1:bv
        let s_5_16: Bits = fn_state.element1;
        // D s_5_17: read-var element2:bv
        let s_5_17: Bits = fn_state.element2;
        // D s_5_18: call FPMulAddH(s_5_15, s_5_16, s_5_17, s_5_14)
        let s_5_18: Bits = FPMulAddH(state, tracer, s_5_15, s_5_16, s_5_17, s_5_14);
        // D s_5_19: write-var gs#695190 <= s_5_18
        fn_state.gs_695190 = s_5_18;
        // N s_5_20: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#695190:bv
        let s_6_0: Bits = fn_state.gs_695190;
        // D s_6_1: cast reint s_6_0 -> u32
        let s_6_1: u32 = (s_6_0.value() as u32);
        // D s_6_2: read-var e:i64
        let s_6_2: i64 = fn_state.e;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: read-var ga#255724:i64
        let s_6_4: i64 = fn_state.ga_255724;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: cast zx s_6_1 -> bv
        let s_6_6: Bits = Bits::new(s_6_1 as u128, 32u16);
        // D s_6_7: read-var result:bv
        let s_6_7: Bits = fn_state.result;
        // D s_6_8: call Elem_set(s_6_7, s_6_3, s_6_5, s_6_6)
        let s_6_8: Bits = Elem_set(state, tracer, s_6_7, s_6_3, s_6_5, s_6_6);
        // D s_6_9: write-var result <= s_6_8
        fn_state.result = s_6_8;
        // D s_6_10: read-var e:i64
        let s_6_10: i64 = fn_state.e;
        // C s_6_11: const #1s : i64
        let s_6_11: i64 = 1;
        // D s_6_12: add s_6_10 s_6_11
        let s_6_12: i64 = (s_6_10 + s_6_11);
        // D s_6_13: write-var e <= s_6_12
        fn_state.e = s_6_12;
        // N s_6_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var element1:bv
        let s_7_0: Bits = fn_state.element1;
        // D s_7_1: call FPNeg(s_7_0)
        let s_7_1: Bits = FPNeg(state, tracer, s_7_0);
        // D s_7_2: write-var element1 <= s_7_1
        fn_state.element1 = s_7_1;
        // N s_7_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var datasizeshadow#1403:i64
        let s_9_0: i64 = fn_state.datasizeshadow_1403;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var d:i64
        let s_9_3: i64 = fn_state.d;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: read-var result:bv
        let s_9_5: Bits = fn_state.result;
        // D s_9_6: call V_set(s_9_4, s_9_2, s_9_5)
        let s_9_6: () = V_set(state, tracer, s_9_4, s_9_2, s_9_5);
        // N s_9_7: return
        return;
    }
}