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
use UnsignedRecipEstimate::*;
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_unary_special_recip_int<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        datasizeshadow_2010: i64,
        e: i64,
        result: Bits,
        gs_174218: i64,
        d: i64,
        datasize: i64,
        elements: i128,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        n,
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
        // D s_0_3: write-var datasizeshadow#2010 <= s_0_2
        fn_state.datasizeshadow_2010 = s_0_2;
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
        // D s_1_0: read-var datasizeshadow#2010:i64
        let s_1_0: i64 = fn_state.datasizeshadow_2010;
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
        // D s_1_6: write-var operand <= s_1_5
        fn_state.operand = s_1_5;
        // C s_1_7: const #0s : i64
        let s_1_7: i64 = 0;
        // C s_1_8: const #1s : i
        let s_1_8: i128 = 1;
        // D s_1_9: read-var elements:i
        let s_1_9: i128 = fn_state.elements;
        // D s_1_10: sub s_1_9 s_1_8
        let s_1_10: i128 = ((s_1_9) - (s_1_8));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var gs#174218 <= s_1_11
        fn_state.gs_174218 = s_1_11;
        // D s_1_13: write-var e <= s_1_7
        fn_state.e = s_1_7;
        // N s_1_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#174218:i64
        let s_2_1: i64 = fn_state.gs_174218;
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
        // C s_3_0: const #32s : i64
        let s_3_0: i64 = 32;
        // D s_3_1: read-var e:i64
        let s_3_1: i64 = fn_state.e;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // C s_3_3: cast zx s_3_0 -> i
        let s_3_3: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_4: read-var operand:bv
        let s_3_4: Bits = fn_state.operand;
        // D s_3_5: call Elem_read(s_3_4, s_3_2, s_3_3)
        let s_3_5: Bits = Elem_read(state, tracer, s_3_4, s_3_2, s_3_3);
        // D s_3_6: cast reint s_3_5 -> u32
        let s_3_6: u32 = (s_3_5.value() as u32);
        // C s_3_7: const #32s : i64
        let s_3_7: i64 = 32;
        // D s_3_8: cast zx s_3_6 -> bv
        let s_3_8: Bits = Bits::new(s_3_6 as u128, 32u16);
        // D s_3_9: call UnsignedRecipEstimate(s_3_8)
        let s_3_9: Bits = UnsignedRecipEstimate(state, tracer, s_3_8);
        // D s_3_10: cast reint s_3_9 -> u32
        let s_3_10: u32 = (s_3_9.value() as u32);
        // D s_3_11: read-var e:i64
        let s_3_11: i64 = fn_state.e;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // C s_3_13: cast zx s_3_7 -> i
        let s_3_13: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_14: cast zx s_3_10 -> bv
        let s_3_14: Bits = Bits::new(s_3_10 as u128, 32u16);
        // D s_3_15: read-var result:bv
        let s_3_15: Bits = fn_state.result;
        // D s_3_16: call Elem_set(s_3_15, s_3_12, s_3_13, s_3_14)
        let s_3_16: Bits = Elem_set(state, tracer, s_3_15, s_3_12, s_3_13, s_3_14);
        // D s_3_17: write-var result <= s_3_16
        fn_state.result = s_3_16;
        // D s_3_18: read-var e:i64
        let s_3_18: i64 = fn_state.e;
        // C s_3_19: const #1s : i64
        let s_3_19: i64 = 1;
        // D s_3_20: add s_3_18 s_3_19
        let s_3_20: i64 = (s_3_18 + s_3_19);
        // D s_3_21: write-var e <= s_3_20
        fn_state.e = s_3_20;
        // N s_3_22: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasizeshadow#2010:i64
        let s_4_0: i64 = fn_state.datasizeshadow_2010;
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
