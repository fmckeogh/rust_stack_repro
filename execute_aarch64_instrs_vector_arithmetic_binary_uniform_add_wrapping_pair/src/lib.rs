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
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_add_wrapping_pair<
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
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        esizeshadow_1032: i64,
        concat: Bits,
        gs_143229: i64,
        datasizeshadow_1033: i64,
        result: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
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
        // D s_0_3: write-var esizeshadow#1032 <= s_0_2
        fn_state.esizeshadow_1032 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1033 <= s_0_6
        fn_state.datasizeshadow_1033 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1033:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1033;
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
        // D s_1_6: read-var datasizeshadow#1033:i64
        let s_1_6: i64 = fn_state.datasizeshadow_1033;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: read-var m:i64
        let s_1_9: i64 = fn_state.m;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: call V_read(s_1_10, s_1_8)
        let s_1_11: Bits = V_read(state, tracer, s_1_10, s_1_8);
        // D s_1_12: cast reint s_1_11 -> u128
        let s_1_12: u128 = (s_1_11.value() as u128);
        // D s_1_13: size-of s_1_11
        let s_1_13: u16 = s_1_11.length();
        // D s_1_14: cast reint s_1_5 -> u128
        let s_1_14: u128 = (s_1_5.value() as u128);
        // D s_1_15: size-of s_1_5
        let s_1_15: u16 = s_1_5.length();
        // D s_1_16: lsl s_1_12 s_1_15
        let s_1_16: u128 = s_1_12 << s_1_15;
        // D s_1_17: or s_1_16 s_1_14
        let s_1_17: u128 = ((s_1_16) | (s_1_14));
        // D s_1_18: add s_1_13 s_1_15
        let s_1_18: u16 = (s_1_13 + s_1_15);
        // D s_1_19: create-bits s_1_17 s_1_18
        let s_1_19: Bits = Bits::new(s_1_17, s_1_18);
        // D s_1_20: write-var concat <= s_1_19
        fn_state.concat = s_1_19;
        // C s_1_21: const #0s : i64
        let s_1_21: i64 = 0;
        // C s_1_22: const #1s : i
        let s_1_22: i128 = 1;
        // D s_1_23: read-var elements:i
        let s_1_23: i128 = fn_state.elements;
        // D s_1_24: sub s_1_23 s_1_22
        let s_1_24: i128 = ((s_1_23) - (s_1_22));
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: write-var gs#143229 <= s_1_25
        fn_state.gs_143229 = s_1_25;
        // D s_1_27: write-var e <= s_1_21
        fn_state.e = s_1_21;
        // N s_1_28: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#143229:i64
        let s_2_1: i64 = fn_state.gs_143229;
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
        // D s_3_1: read-var e:i64
        let s_3_1: i64 = fn_state.e;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: read-var esizeshadow#1032:i64
        let s_3_4: i64 = fn_state.esizeshadow_1032;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: read-var concat:bv
        let s_3_8: Bits = fn_state.concat;
        // D s_3_9: call Elem_read(s_3_8, s_3_3, s_3_7)
        let s_3_9: Bits = Elem_read(state, tracer, s_3_8, s_3_3, s_3_7);
        // C s_3_10: const #2s : i
        let s_3_10: i128 = 2;
        // D s_3_11: read-var e:i64
        let s_3_11: i64 = fn_state.e;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: mul s_3_10 s_3_12
        let s_3_13: i128 = ((s_3_10) * (s_3_12));
        // C s_3_14: const #1s : i
        let s_3_14: i128 = 1;
        // D s_3_15: add s_3_13 s_3_14
        let s_3_15: i128 = (s_3_13 + s_3_14);
        // D s_3_16: read-var esizeshadow#1032:i64
        let s_3_16: i64 = fn_state.esizeshadow_1032;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: read-var concat:bv
        let s_3_20: Bits = fn_state.concat;
        // D s_3_21: call Elem_read(s_3_20, s_3_15, s_3_19)
        let s_3_21: Bits = Elem_read(state, tracer, s_3_20, s_3_15, s_3_19);
        // D s_3_22: read-var esizeshadow#1032:i64
        let s_3_22: i64 = fn_state.esizeshadow_1032;
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // D s_3_25: add s_3_9 s_3_21
        let s_3_25: Bits = (s_3_9 + s_3_21);
        // D s_3_26: read-var e:i64
        let s_3_26: i64 = fn_state.e;
        // D s_3_27: cast zx s_3_26 -> i
        let s_3_27: i128 = (i128::try_from(s_3_26).unwrap());
        // D s_3_28: cast zx s_3_24 -> i
        let s_3_28: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_29: read-var result:bv
        let s_3_29: Bits = fn_state.result;
        // D s_3_30: call Elem_set(s_3_29, s_3_27, s_3_28, s_3_25)
        let s_3_30: Bits = Elem_set(state, tracer, s_3_29, s_3_27, s_3_28, s_3_25);
        // D s_3_31: write-var result <= s_3_30
        fn_state.result = s_3_30;
        // D s_3_32: read-var e:i64
        let s_3_32: i64 = fn_state.e;
        // C s_3_33: const #1s : i64
        let s_3_33: i64 = 1;
        // D s_3_34: add s_3_32 s_3_33
        let s_3_34: i64 = (s_3_32 + s_3_33);
        // D s_3_35: write-var e <= s_3_34
        fn_state.e = s_3_34;
        // N s_3_36: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasizeshadow#1033:i64
        let s_4_0: i64 = fn_state.datasizeshadow_1033;
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
