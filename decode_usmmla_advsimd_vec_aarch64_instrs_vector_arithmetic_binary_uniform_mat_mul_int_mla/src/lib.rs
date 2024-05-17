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
use HaveInt8MatMulExt::*;
use execute_aarch64_instrs_vector_arithmetic_binary_uniform_mat_mul_int_mla::*;
use common::*;
pub fn decode_usmmla_advsimd_vec_aarch64_instrs_vector_arithmetic_binary_uniform_mat_mul_int_mla<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, B: bool, Rm: u8, U: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        op1_unsigned: bool,
        ga_266998: u8,
        op2_unsigned: bool,
        Rd: u8,
        Rn: u8,
        B: bool,
        Rm: u8,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        B,
        Rm,
        U,
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
        // S s_0_1: call HaveInt8MatMulExt(s_0_0)
        let s_0_1: bool = HaveInt8MatMulExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b9 b1
        if s_0_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var B:u8
        let s_1_0: bool = fn_state.B;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // D s_1_2: read-var U:u8
        let s_1_2: bool = fn_state.U;
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cast reint s_1_1 -> u128
        let s_1_4: u128 = (s_1_1.value() as u128);
        // D s_1_5: size-of s_1_1
        let s_1_5: u16 = s_1_1.length();
        // D s_1_6: cast reint s_1_3 -> u128
        let s_1_6: u128 = (s_1_3.value() as u128);
        // D s_1_7: size-of s_1_3
        let s_1_7: u16 = s_1_3.length();
        // D s_1_8: lsl s_1_4 s_1_7
        let s_1_8: u128 = s_1_4 << s_1_7;
        // D s_1_9: or s_1_8 s_1_6
        let s_1_9: u128 = ((s_1_8) | (s_1_6));
        // D s_1_10: add s_1_5 s_1_7
        let s_1_10: u16 = (s_1_5 + s_1_7);
        // D s_1_11: create-bits s_1_9 s_1_10
        let s_1_11: Bits = Bits::new(s_1_9, s_1_10);
        // D s_1_12: cast reint s_1_11 -> u8
        let s_1_12: u8 = (s_1_11.value() as u8);
        // D s_1_13: write-var ga#266998 <= s_1_12
        fn_state.ga_266998 = s_1_12;
        // D s_1_14: read-var ga#266998:u8
        let s_1_14: u8 = fn_state.ga_266998;
        // D s_1_15: cast zx s_1_14 -> bv
        let s_1_15: Bits = Bits::new(s_1_14 as u128, 2u16);
        // C s_1_16: const #0u : u8
        let s_1_16: u8 = 0;
        // C s_1_17: cast zx s_1_16 -> bv
        let s_1_17: Bits = Bits::new(s_1_16 as u128, 2u16);
        // D s_1_18: cmp-eq s_1_15 s_1_17
        let s_1_18: bool = ((s_1_15) == (s_1_17));
        // D s_1_19: not s_1_18
        let s_1_19: bool = !s_1_18;
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
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var op1_unsigned <= s_2_0
        fn_state.op1_unsigned = s_2_0;
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // D s_2_3: write-var op2_unsigned <= s_2_2
        fn_state.op2_unsigned = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var op2_unsigned:u8
        let s_3_0: bool = fn_state.op2_unsigned;
        // D s_3_1: read-var op1_unsigned:u8
        let s_3_1: bool = fn_state.op1_unsigned;
        // D s_3_2: read-var Rn:u8
        let s_3_2: u8 = fn_state.Rn;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 5u16);
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (s_3_3.value() as i128);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var Rm:u8
        let s_3_6: u8 = fn_state.Rm;
        // D s_3_7: cast zx s_3_6 -> bv
        let s_3_7: Bits = Bits::new(s_3_6 as u128, 5u16);
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (s_3_7.value() as i128);
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: read-var Rd:u8
        let s_3_10: u8 = fn_state.Rd;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 5u16);
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (s_3_11.value() as i128);
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: call execute_aarch64_instrs_vector_arithmetic_binary_uniform_mat_mul_int_mla(s_3_13, s_3_9, s_3_5, s_3_1, s_3_0)
        let s_3_14: () = execute_aarch64_instrs_vector_arithmetic_binary_uniform_mat_mul_int_mla(
            state,
            tracer,
            s_3_13,
            s_3_9,
            s_3_5,
            s_3_1,
            s_3_0,
        );
        // N s_3_15: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#266998:u8
        let s_4_0: u8 = fn_state.ga_266998;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #1u : u8
        let s_4_2: u8 = 1;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: not s_4_4
        let s_4_5: bool = !s_4_4;
        // N s_4_6: branch s_4_5 b6 b5
        if s_4_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var op1_unsigned <= s_5_0
        fn_state.op1_unsigned = s_5_0;
        // C s_5_2: const #1u : u8
        let s_5_2: bool = true;
        // D s_5_3: write-var op2_unsigned <= s_5_2
        fn_state.op2_unsigned = s_5_2;
        // N s_5_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#266998:u8
        let s_6_0: u8 = fn_state.ga_266998;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: not s_6_4
        let s_6_5: bool = !s_6_4;
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var op1_unsigned <= s_7_0
        fn_state.op1_unsigned = s_7_0;
        // C s_7_2: const #0u : u8
        let s_7_2: bool = false;
        // D s_7_3: write-var op2_unsigned <= s_7_2
        fn_state.op2_unsigned = s_7_2;
        // N s_7_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
}
