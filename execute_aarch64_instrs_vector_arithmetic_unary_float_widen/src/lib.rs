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
use FPCR_read::*;
use FPConvert__1::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_unary_float_widen<T: Tracer>(
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
        gs_151361: i64,
        esizeshadow_1347: i64,
        ga_254401: Bits,
        result: Bits,
        ga_254400: i64,
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
        // D s_0_3: write-var esizeshadow#1347 <= s_0_2
        fn_state.esizeshadow_1347 = s_0_2;
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
        // D s_1_9: write-var operand <= s_1_8
        fn_state.operand = s_1_8;
        // C s_1_10: const #0s : i64
        let s_1_10: i64 = 0;
        // C s_1_11: const #1s : i
        let s_1_11: i128 = 1;
        // D s_1_12: read-var elements:i
        let s_1_12: i128 = fn_state.elements;
        // D s_1_13: sub s_1_12 s_1_11
        let s_1_13: i128 = ((s_1_12) - (s_1_11));
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: write-var gs#151361 <= s_1_14
        fn_state.gs_151361 = s_1_14;
        // D s_1_16: write-var e <= s_1_10
        fn_state.e = s_1_10;
        // N s_1_17: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#151361:i64
        let s_2_1: i64 = fn_state.gs_151361;
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
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var esizeshadow#1347:i64
        let s_3_1: i64 = fn_state.esizeshadow_1347;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // D s_3_7: write-var ga#254400 <= s_3_6
        fn_state.ga_254400 = s_3_6;
        // D s_3_8: read-var esizeshadow#1347:i64
        let s_3_8: i64 = fn_state.esizeshadow_1347;
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
        // C s_3_18: const #2s : i
        let s_3_18: i128 = 2;
        // D s_3_19: read-var esizeshadow#1347:i64
        let s_3_19: i64 = fn_state.esizeshadow_1347;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: mul s_3_18 s_3_20
        let s_3_21: i128 = ((s_3_18) * (s_3_20));
        // D s_3_22: cast reint s_3_21 -> i64
        let s_3_22: i64 = (s_3_21 as i64);
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // D s_3_25: call FPConvert__1(s_3_15, s_3_17, s_3_24)
        let s_3_25: Bits = FPConvert__1(state, tracer, s_3_15, s_3_17, s_3_24);
        // D s_3_26: write-var ga#254401 <= s_3_25
        fn_state.ga_254401 = s_3_25;
        // N s_3_27: jump b4
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
        // D s_4_2: read-var ga#254400:i64
        let s_4_2: i64 = fn_state.ga_254400;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var result:bv
        let s_4_4: Bits = fn_state.result;
        // D s_4_5: read-var ga#254401:bv
        let s_4_5: Bits = fn_state.ga_254401;
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
        // C s_5_0: const #2s : i
        let s_5_0: i128 = 2;
        // D s_5_1: read-var datasize:i64
        let s_5_1: i64 = fn_state.datasize;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mul s_5_0 s_5_2
        let s_5_3: i128 = ((s_5_0) * (s_5_2));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: cast reint s_5_5 -> i64
        let s_5_6: i64 = (s_5_5 as i64);
        // D s_5_7: read-var d:i64
        let s_5_7: i64 = fn_state.d;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: read-var result:bv
        let s_5_9: Bits = fn_state.result;
        // D s_5_10: call V_set(s_5_8, s_5_6, s_5_9)
        let s_5_10: () = V_set(state, tracer, s_5_8, s_5_6, s_5_9);
        // N s_5_11: return
        return;
    }
}
