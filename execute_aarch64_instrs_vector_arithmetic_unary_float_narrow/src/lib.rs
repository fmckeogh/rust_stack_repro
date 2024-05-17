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
use Vpart_set::*;
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use FPCR_read::*;
use FPConvert__1::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_unary_float_narrow<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    n: i64,
    part: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        result: Bits,
        esizeshadow_1350: i64,
        ga_254718: Bits,
        gs_151807: i64,
        ga_254717: i64,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        n: i64,
        part: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
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
        // D s_0_3: write-var esizeshadow#1350 <= s_0_2
        fn_state.esizeshadow_1350 = s_0_2;
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
        // D s_1_1: read-var datasize:i64
        let s_1_1: i64 = fn_state.datasize;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: mul s_1_0 s_1_2
        let s_1_3: i128 = ((s_1_0) * (s_1_2));
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
        // D s_1_9: call V_read(s_1_8, s_1_6)
        let s_1_9: Bits = V_read(state, tracer, s_1_8, s_1_6);
        // D s_1_10: write-var operand <= s_1_9
        fn_state.operand = s_1_9;
        // C s_1_11: const #0s : i64
        let s_1_11: i64 = 0;
        // C s_1_12: const #1s : i
        let s_1_12: i128 = 1;
        // D s_1_13: read-var elements:i
        let s_1_13: i128 = fn_state.elements;
        // D s_1_14: sub s_1_13 s_1_12
        let s_1_14: i128 = ((s_1_13) - (s_1_12));
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: write-var gs#151807 <= s_1_15
        fn_state.gs_151807 = s_1_15;
        // D s_1_17: write-var e <= s_1_11
        fn_state.e = s_1_11;
        // N s_1_18: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#151807:i64
        let s_2_1: i64 = fn_state.gs_151807;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b5 b3
        if s_2_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#1350:i64
        let s_3_0: i64 = fn_state.esizeshadow_1350;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: write-var ga#254717 <= s_3_2
        fn_state.ga_254717 = s_3_2;
        // C s_3_4: const #2s : i
        let s_3_4: i128 = 2;
        // D s_3_5: read-var esizeshadow#1350:i64
        let s_3_5: i64 = fn_state.esizeshadow_1350;
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: mul s_3_4 s_3_6
        let s_3_7: i128 = ((s_3_4) * (s_3_6));
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
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
        // D s_3_14: read-var operand:bv
        let s_3_14: Bits = fn_state.operand;
        // D s_3_15: call Elem_read(s_3_14, s_3_12, s_3_13)
        let s_3_15: Bits = Elem_read(state, tracer, s_3_14, s_3_12, s_3_13);
        // C s_3_16: const #() : ()
        let s_3_16: () = ();
        // S s_3_17: call FPCR_read(s_3_16)
        let s_3_17: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_3_16);
        // D s_3_18: read-var esizeshadow#1350:i64
        let s_3_18: i64 = fn_state.esizeshadow_1350;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // D s_3_21: call FPConvert__1(s_3_15, s_3_17, s_3_20)
        let s_3_21: Bits = FPConvert__1(state, tracer, s_3_15, s_3_17, s_3_20);
        // D s_3_22: write-var ga#254718 <= s_3_21
        fn_state.ga_254718 = s_3_21;
        // N s_3_23: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var ga#254717:i64
        let s_4_2: i64 = fn_state.ga_254717;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var result:bv
        let s_4_4: Bits = fn_state.result;
        // D s_4_5: read-var ga#254718:bv
        let s_4_5: Bits = fn_state.ga_254718;
        // D s_4_6: call Elem_set(s_4_4, s_4_1, s_4_3, s_4_5)
        let s_4_6: Bits = Elem_set(state, tracer, s_4_4, s_4_1, s_4_3, s_4_5);
        // D s_4_7: write-var result <= s_4_6
        fn_state.result = s_4_6;
        // D s_4_8: read-var e:i64
        let s_4_8: i64 = fn_state.e;
        // C s_4_9: const #1s : i64
        let s_4_9: i64 = 1;
        // D s_4_10: add s_4_8 s_4_9
        let s_4_10: i64 = (s_4_8 + s_4_9);
        // D s_4_11: write-var e <= s_4_10
        fn_state.e = s_4_10;
        // N s_4_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var datasize:i64
        let s_5_0: i64 = fn_state.datasize;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var d:i64
        let s_5_3: i64 = fn_state.d;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: read-var part:i64
        let s_5_5: i64 = fn_state.part;
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: cast zx s_5_2 -> i
        let s_5_7: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_8: read-var result:bv
        let s_5_8: Bits = fn_state.result;
        // D s_5_9: call Vpart_set(s_5_4, s_5_6, s_5_7, s_5_8)
        let s_5_9: () = Vpart_set(state, tracer, s_5_4, s_5_6, s_5_7, s_5_8);
        // N s_5_10: return
        return;
    }
}
