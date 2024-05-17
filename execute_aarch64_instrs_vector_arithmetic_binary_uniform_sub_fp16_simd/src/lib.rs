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
use FPCR_read::*;
use FPAbs::*;
use V_set::*;
use FPSub::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_sub_fp16_simd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    abs: bool,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_256746: i64,
        fpcr: ProductType5c790c8ef59cc8b2,
        e: i64,
        gs_154965: i64,
        datasizeshadow_1472: i64,
        diff: Bits,
        ga_256747: Bits,
        result: Bits,
        operand1: Bits,
        esizeshadow_1471: i64,
        operand2: Bits,
        abs: bool,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        abs,
        d,
        datasize,
        elements,
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
        // D s_0_3: write-var esizeshadow#1471 <= s_0_2
        fn_state.esizeshadow_1471 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1472 <= s_0_6
        fn_state.datasizeshadow_1472 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1472:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1472;
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
        // D s_1_7: read-var datasizeshadow#1472:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1472;
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
        // C s_1_14: const #() : ()
        let s_1_14: () = ();
        // S s_1_15: call FPCR_read(s_1_14)
        let s_1_15: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_1_14);
        // D s_1_16: write-var fpcr <= s_1_15
        fn_state.fpcr = s_1_15;
        // C s_1_17: const #0s : i64
        let s_1_17: i64 = 0;
        // C s_1_18: const #1s : i
        let s_1_18: i128 = 1;
        // D s_1_19: read-var elements:i
        let s_1_19: i128 = fn_state.elements;
        // D s_1_20: sub s_1_19 s_1_18
        let s_1_20: i128 = ((s_1_19) - (s_1_18));
        // D s_1_21: cast reint s_1_20 -> i64
        let s_1_21: i64 = (s_1_20 as i64);
        // D s_1_22: write-var gs#154965 <= s_1_21
        fn_state.gs_154965 = s_1_21;
        // D s_1_23: write-var e <= s_1_17
        fn_state.e = s_1_17;
        // N s_1_24: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#154965:i64
        let s_2_1: i64 = fn_state.gs_154965;
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
        // D s_3_0: read-var esizeshadow#1471:i64
        let s_3_0: i64 = fn_state.esizeshadow_1471;
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
        // D s_3_8: read-var esizeshadow#1471:i64
        let s_3_8: i64 = fn_state.esizeshadow_1471;
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
        // D s_3_16: read-var fpcr:struct
        let s_3_16: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_3_17: call FPSub(s_3_7, s_3_15, s_3_16)
        let s_3_17: Bits = FPSub(state, tracer, s_3_7, s_3_15, s_3_16);
        // D s_3_18: write-var diff <= s_3_17
        fn_state.diff = s_3_17;
        // N s_3_19: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var esizeshadow#1471:i64
        let s_4_0: i64 = fn_state.esizeshadow_1471;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: write-var ga#256746 <= s_4_2
        fn_state.ga_256746 = s_4_2;
        // D s_4_4: read-var abs:u8
        let s_4_4: bool = fn_state.abs;
        // N s_4_5: branch s_4_4 b7 b5
        if s_4_4 {
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
        // D s_5_0: read-var diff:bv
        let s_5_0: Bits = fn_state.diff;
        // D s_5_1: write-var ga#256747 <= s_5_0
        fn_state.ga_256747 = s_5_0;
        // N s_5_2: jump b6
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
        // D s_6_2: read-var ga#256746:i64
        let s_6_2: i64 = fn_state.ga_256746;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: read-var result:bv
        let s_6_4: Bits = fn_state.result;
        // D s_6_5: read-var ga#256747:bv
        let s_6_5: Bits = fn_state.ga_256747;
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
        // D s_7_0: read-var diff:bv
        let s_7_0: Bits = fn_state.diff;
        // D s_7_1: call FPAbs(s_7_0)
        let s_7_1: Bits = FPAbs(state, tracer, s_7_0);
        // D s_7_2: write-var ga#256747 <= s_7_1
        fn_state.ga_256747 = s_7_1;
        // N s_7_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var datasizeshadow#1472:i64
        let s_8_0: i64 = fn_state.datasizeshadow_1472;
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
