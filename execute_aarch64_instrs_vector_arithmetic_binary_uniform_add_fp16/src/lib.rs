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
use V_set::*;
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use FPCR_read::*;
use FPAdd::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_add_fp16<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    pair: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_253006: Bits,
        element2: Bits,
        e: i64,
        element1: Bits,
        concat: Bits,
        esizeshadow_1244: i64,
        gs_148972: i64,
        datasizeshadow_1245: i64,
        result: Bits,
        operand1: Bits,
        ga_253005: i64,
        operand2: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        pair: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        m,
        n,
        pair,
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
        // D s_0_3: write-var esizeshadow#1244 <= s_0_2
        fn_state.esizeshadow_1244 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1245 <= s_0_6
        fn_state.datasizeshadow_1245 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1245:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1245;
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
        // D s_1_7: read-var datasizeshadow#1245:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1245;
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
        // D s_1_14: read-var operand2:bv
        let s_1_14: Bits = fn_state.operand2;
        // D s_1_15: read-var operand1:bv
        let s_1_15: Bits = fn_state.operand1;
        // D s_1_16: cast reint s_1_14 -> u128
        let s_1_16: u128 = (s_1_14.value() as u128);
        // D s_1_17: size-of s_1_14
        let s_1_17: u16 = s_1_14.length();
        // D s_1_18: cast reint s_1_15 -> u128
        let s_1_18: u128 = (s_1_15.value() as u128);
        // D s_1_19: size-of s_1_15
        let s_1_19: u16 = s_1_15.length();
        // D s_1_20: lsl s_1_16 s_1_19
        let s_1_20: u128 = s_1_16 << s_1_19;
        // D s_1_21: or s_1_20 s_1_18
        let s_1_21: u128 = ((s_1_20) | (s_1_18));
        // D s_1_22: add s_1_17 s_1_19
        let s_1_22: u16 = (s_1_17 + s_1_19);
        // D s_1_23: create-bits s_1_21 s_1_22
        let s_1_23: Bits = Bits::new(s_1_21, s_1_22);
        // D s_1_24: write-var concat <= s_1_23
        fn_state.concat = s_1_23;
        // C s_1_25: const #0s : i64
        let s_1_25: i64 = 0;
        // C s_1_26: const #1s : i
        let s_1_26: i128 = 1;
        // D s_1_27: read-var elements:i
        let s_1_27: i128 = fn_state.elements;
        // D s_1_28: sub s_1_27 s_1_26
        let s_1_28: i128 = ((s_1_27) - (s_1_26));
        // D s_1_29: cast reint s_1_28 -> i64
        let s_1_29: i64 = (s_1_28 as i64);
        // D s_1_30: write-var gs#148972 <= s_1_29
        fn_state.gs_148972 = s_1_29;
        // D s_1_31: write-var e <= s_1_25
        fn_state.e = s_1_25;
        // N s_1_32: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#148972:i64
        let s_2_1: i64 = fn_state.gs_148972;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b8 b3
        if s_2_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var pair:u8
        let s_3_0: bool = fn_state.pair;
        // N s_3_1: branch s_3_0 b7 b4
        if s_3_0 {
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
        // D s_4_0: read-var esizeshadow#1244:i64
        let s_4_0: i64 = fn_state.esizeshadow_1244;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var e:i64
        let s_4_3: i64 = fn_state.e;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: read-var operand1:bv
        let s_4_6: Bits = fn_state.operand1;
        // D s_4_7: call Elem_read(s_4_6, s_4_4, s_4_5)
        let s_4_7: Bits = Elem_read(state, tracer, s_4_6, s_4_4, s_4_5);
        // D s_4_8: write-var element1 <= s_4_7
        fn_state.element1 = s_4_7;
        // D s_4_9: read-var esizeshadow#1244:i64
        let s_4_9: i64 = fn_state.esizeshadow_1244;
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // D s_4_12: read-var e:i64
        let s_4_12: i64 = fn_state.e;
        // D s_4_13: cast zx s_4_12 -> i
        let s_4_13: i128 = (i128::try_from(s_4_12).unwrap());
        // D s_4_14: cast zx s_4_11 -> i
        let s_4_14: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_15: read-var operand2:bv
        let s_4_15: Bits = fn_state.operand2;
        // D s_4_16: call Elem_read(s_4_15, s_4_13, s_4_14)
        let s_4_16: Bits = Elem_read(state, tracer, s_4_15, s_4_13, s_4_14);
        // D s_4_17: write-var element2 <= s_4_16
        fn_state.element2 = s_4_16;
        // N s_4_18: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#1244:i64
        let s_5_0: i64 = fn_state.esizeshadow_1244;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: write-var ga#253005 <= s_5_2
        fn_state.ga_253005 = s_5_2;
        // C s_5_4: const #() : ()
        let s_5_4: () = ();
        // S s_5_5: call FPCR_read(s_5_4)
        let s_5_5: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_5_4);
        // D s_5_6: read-var element1:bv
        let s_5_6: Bits = fn_state.element1;
        // D s_5_7: read-var element2:bv
        let s_5_7: Bits = fn_state.element2;
        // D s_5_8: call FPAdd(s_5_6, s_5_7, s_5_5)
        let s_5_8: Bits = FPAdd(state, tracer, s_5_6, s_5_7, s_5_5);
        // D s_5_9: write-var ga#253006 <= s_5_8
        fn_state.ga_253006 = s_5_8;
        // N s_5_10: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var ga#253005:i64
        let s_6_2: i64 = fn_state.ga_253005;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: read-var result:bv
        let s_6_4: Bits = fn_state.result;
        // D s_6_5: read-var ga#253006:bv
        let s_6_5: Bits = fn_state.ga_253006;
        // D s_6_6: call Elem_set(s_6_4, s_6_1, s_6_3, s_6_5)
        let s_6_6: Bits = Elem_set(state, tracer, s_6_4, s_6_1, s_6_3, s_6_5);
        // D s_6_7: write-var result <= s_6_6
        fn_state.result = s_6_6;
        // D s_6_8: read-var e:i64
        let s_6_8: i64 = fn_state.e;
        // C s_6_9: const #1s : i64
        let s_6_9: i64 = 1;
        // D s_6_10: add s_6_8 s_6_9
        let s_6_10: i64 = (s_6_8 + s_6_9);
        // D s_6_11: write-var e <= s_6_10
        fn_state.e = s_6_10;
        // N s_6_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2s : i
        let s_7_0: i128 = 2;
        // D s_7_1: read-var e:i64
        let s_7_1: i64 = fn_state.e;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: mul s_7_0 s_7_2
        let s_7_3: i128 = ((s_7_0) * (s_7_2));
        // D s_7_4: read-var esizeshadow#1244:i64
        let s_7_4: i64 = fn_state.esizeshadow_1244;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: cast reint s_7_5 -> i64
        let s_7_6: i64 = (s_7_5 as i64);
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: read-var concat:bv
        let s_7_8: Bits = fn_state.concat;
        // D s_7_9: call Elem_read(s_7_8, s_7_3, s_7_7)
        let s_7_9: Bits = Elem_read(state, tracer, s_7_8, s_7_3, s_7_7);
        // D s_7_10: write-var element1 <= s_7_9
        fn_state.element1 = s_7_9;
        // C s_7_11: const #2s : i
        let s_7_11: i128 = 2;
        // D s_7_12: read-var e:i64
        let s_7_12: i64 = fn_state.e;
        // D s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_14: mul s_7_11 s_7_13
        let s_7_14: i128 = ((s_7_11) * (s_7_13));
        // C s_7_15: const #1s : i
        let s_7_15: i128 = 1;
        // D s_7_16: add s_7_14 s_7_15
        let s_7_16: i128 = (s_7_14 + s_7_15);
        // D s_7_17: read-var esizeshadow#1244:i64
        let s_7_17: i64 = fn_state.esizeshadow_1244;
        // D s_7_18: cast zx s_7_17 -> i
        let s_7_18: i128 = (i128::try_from(s_7_17).unwrap());
        // D s_7_19: cast reint s_7_18 -> i64
        let s_7_19: i64 = (s_7_18 as i64);
        // D s_7_20: cast zx s_7_19 -> i
        let s_7_20: i128 = (i128::try_from(s_7_19).unwrap());
        // D s_7_21: read-var concat:bv
        let s_7_21: Bits = fn_state.concat;
        // D s_7_22: call Elem_read(s_7_21, s_7_16, s_7_20)
        let s_7_22: Bits = Elem_read(state, tracer, s_7_21, s_7_16, s_7_20);
        // D s_7_23: write-var element2 <= s_7_22
        fn_state.element2 = s_7_22;
        // N s_7_24: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var datasizeshadow#1245:i64
        let s_8_0: i64 = fn_state.datasizeshadow_1245;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: read-var d:i64
        let s_8_3: i64 = fn_state.d;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: read-var result:bv
        let s_8_5: Bits = fn_state.result;
        // D s_8_6: call V_set(s_8_4, s_8_2, s_8_5)
        let s_8_6: () = V_set(state, tracer, s_8_4, s_8_2, s_8_5);
        // N s_8_7: return
        return;
    }
}
