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
use u__id::*;
use execute_aarch64_instrs_vector_arithmetic_unary_float_narrow::*;
use fdiv_int::*;
use common::*;
pub fn decode_fcvtn_advsimd_aarch64_instrs_vector_arithmetic_unary_float_narrow<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, sz: bool, Q: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        elements: i128,
        gs_151822: bool,
        esize: i128,
        part: i64,
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        sz: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        sz,
        Q,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rd:u8
        let s_0_0: u8 = fn_state.Rd;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var d <= s_0_3
        fn_state.d = s_0_3;
        // D s_0_5: read-var Rn:u8
        let s_0_5: u8 = fn_state.Rn;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var n <= s_0_8
        fn_state.n = s_0_8;
        // D s_0_10: read-var sz:u8
        let s_0_10: bool = fn_state.sz;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // C s_0_14: const #16s : i
        let s_0_14: i128 = 16;
        // D s_0_15: cast zx s_0_13 -> i
        let s_0_15: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_16: lsl s_0_14 s_0_15
        let s_0_16: i128 = s_0_14 << s_0_15;
        // D s_0_17: write-var esize <= s_0_16
        fn_state.esize = s_0_16;
        // D s_0_18: read-var Q:u8
        let s_0_18: bool = fn_state.Q;
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 1u16);
        // D s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (s_0_19.value() as i128);
        // D s_0_21: cast reint s_0_20 -> i64
        let s_0_21: i64 = (s_0_20 as i64);
        // D s_0_22: write-var part <= s_0_21
        fn_state.part = s_0_21;
        // C s_0_23: const #64s : i
        let s_0_23: i128 = 64;
        // D s_0_24: read-var esize:i
        let s_0_24: i128 = fn_state.esize;
        // D s_0_25: call fdiv_int(s_0_23, s_0_24)
        let s_0_25: i128 = fdiv_int(state, tracer, s_0_23, s_0_24);
        // D s_0_26: write-var elements <= s_0_25
        fn_state.elements = s_0_25;
        // D s_0_27: read-var esize:i
        let s_0_27: i128 = fn_state.esize;
        // D s_0_28: call __id(s_0_27)
        let s_0_28: i128 = u__id(state, tracer, s_0_27);
        // C s_0_29: const #16s : i
        let s_0_29: i128 = 16;
        // D s_0_30: cmp-eq s_0_28 s_0_29
        let s_0_30: bool = ((s_0_28) == (s_0_29));
        // N s_0_31: branch s_0_30 b3 b1
        if s_0_30 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var esize:i
        let s_1_0: i128 = fn_state.esize;
        // D s_1_1: call __id(s_1_0)
        let s_1_1: i128 = u__id(state, tracer, s_1_0);
        // C s_1_2: const #32s : i
        let s_1_2: i128 = 32;
        // D s_1_3: cmp-eq s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) == (s_1_2));
        // D s_1_4: write-var gs#151822 <= s_1_3
        fn_state.gs_151822 = s_1_3;
        // N s_1_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#151822:u8
        let s_2_0: bool = fn_state.gs_151822;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var esize:i
        let s_2_2: i128 = fn_state.esize;
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #64s : i64
        let s_2_4: i64 = 64;
        // D s_2_5: read-var d:i64
        let s_2_5: i64 = fn_state.d;
        // D s_2_6: read-var elements:i
        let s_2_6: i128 = fn_state.elements;
        // D s_2_7: read-var n:i64
        let s_2_7: i64 = fn_state.n;
        // D s_2_8: read-var part:i64
        let s_2_8: i64 = fn_state.part;
        // D s_2_9: call execute_aarch64_instrs_vector_arithmetic_unary_float_narrow(s_2_5, s_2_4, s_2_6, s_2_3, s_2_7, s_2_8)
        let s_2_9: () = execute_aarch64_instrs_vector_arithmetic_unary_float_narrow(
            state,
            tracer,
            s_2_5,
            s_2_4,
            s_2_6,
            s_2_3,
            s_2_7,
            s_2_8,
        );
        // N s_2_10: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var gs#151822 <= s_3_0
        fn_state.gs_151822 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
