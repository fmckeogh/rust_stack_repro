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
use HaveBF16Ext::*;
use execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_int_bfdot::*;
use common::*;
pub fn decode_bfdot_advsimd_vec_aarch64_instrs_vector_arithmetic_binary_uniform_mul_int_bfdot<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, Rm: u8, Q: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        n: i64,
        ga_249857: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        Rm: u8,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        Rm,
        Q,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveBF16Ext(s_0_0)
        let s_0_1: bool = HaveBF16Ext(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b5 b1
        if s_0_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Rn:u8
        let s_1_0: u8 = fn_state.Rn;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var n <= s_1_3
        fn_state.n = s_1_3;
        // D s_1_5: read-var Rm:u8
        let s_1_5: u8 = fn_state.Rm;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 5u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var m <= s_1_8
        fn_state.m = s_1_8;
        // D s_1_10: read-var Rd:u8
        let s_1_10: u8 = fn_state.Rd;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 5u16);
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (s_1_11.value() as i128);
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var d <= s_1_13
        fn_state.d = s_1_13;
        // D s_1_15: read-var Q:u8
        let s_1_15: bool = fn_state.Q;
        // D s_1_16: cast zx s_1_15 -> bv
        let s_1_16: Bits = Bits::new(s_1_15 as u128, 1u16);
        // C s_1_17: const #1u : u8
        let s_1_17: bool = true;
        // C s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // D s_1_19: cmp-eq s_1_16 s_1_18
        let s_1_19: bool = ((s_1_16) == (s_1_18));
        // N s_1_20: branch s_1_19 b4 b2
        if s_1_19 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: write-var ga#249857 <= s_2_0
        fn_state.ga_249857 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#249857:i64
        let s_3_0: i64 = fn_state.ga_249857;
        // C s_3_1: const #32s : i
        let s_3_1: i128 = 32;
        // D s_3_2: cast zx s_3_0 -> i
        let s_3_2: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_3: div s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) / (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: cast zx s_3_0 -> i
        let s_3_5: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // D s_3_7: read-var d:i64
        let s_3_7: i64 = fn_state.d;
        // D s_3_8: read-var m:i64
        let s_3_8: i64 = fn_state.m;
        // D s_3_9: read-var n:i64
        let s_3_9: i64 = fn_state.n;
        // D s_3_10: call execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_int_bfdot(s_3_7, s_3_6, s_3_4, s_3_8, s_3_9)
        let s_3_10: () = execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_int_bfdot(
            state,
            tracer,
            s_3_7,
            s_3_6,
            s_3_4,
            s_3_8,
            s_3_9,
        );
        // N s_3_11: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #128s : i64
        let s_4_0: i64 = 128;
        // D s_4_1: write-var ga#249857 <= s_4_0
        fn_state.ga_249857 = s_4_0;
        // N s_4_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
}
