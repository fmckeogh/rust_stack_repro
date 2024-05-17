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
use HaveQRDMLAHExt::*;
use execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd::*;
use common::*;
pub fn decode_sqrdmlah_advsimd_vec_aarch64_instrs_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, S: bool, Rm: u8, size: u8) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_171134: bool,
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        S: bool,
        Rm: u8,
        size: u8,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        S,
        Rm,
        size,
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
        // S s_0_1: call HaveQRDMLAHExt(s_0_0)
        let s_0_1: bool = HaveQRDMLAHExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b7 b1
        if s_0_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Rd:u8
        let s_1_0: u8 = fn_state.Rd;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var d <= s_1_3
        fn_state.d = s_1_3;
        // D s_1_5: read-var Rn:u8
        let s_1_5: u8 = fn_state.Rn;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 5u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var n <= s_1_8
        fn_state.n = s_1_8;
        // D s_1_10: read-var Rm:u8
        let s_1_10: u8 = fn_state.Rm;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 5u16);
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (s_1_11.value() as i128);
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var m <= s_1_13
        fn_state.m = s_1_13;
        // D s_1_15: read-var size:u8
        let s_1_15: u8 = fn_state.size;
        // D s_1_16: cast zx s_1_15 -> bv
        let s_1_16: Bits = Bits::new(s_1_15 as u128, 2u16);
        // C s_1_17: const #3u : u8
        let s_1_17: u8 = 3;
        // C s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 2u16);
        // D s_1_19: cmp-eq s_1_16 s_1_18
        let s_1_19: bool = ((s_1_16) == (s_1_18));
        // N s_1_20: branch s_1_19 b6 b2
        if s_1_19 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #0u : u8
        let s_2_2: u8 = 0;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // D s_2_5: write-var gs#171134 <= s_2_4
        fn_state.gs_171134 = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#171134:u8
        let s_3_0: bool = fn_state.gs_171134;
        // N s_3_1: branch s_3_0 b5 b4
        if s_3_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var size:u8
        let s_4_0: u8 = fn_state.size;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // C s_4_4: const #8s : i64
        let s_4_4: i64 = 8;
        // D s_4_5: lsl s_4_4 s_4_3
        let s_4_5: i64 = s_4_4 << s_4_3;
        // D s_4_6: read-var S:u8
        let s_4_6: bool = fn_state.S;
        // D s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 1u16);
        // C s_4_8: const #1u : u8
        let s_4_8: bool = true;
        // C s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 1u16);
        // D s_4_10: cmp-eq s_4_7 s_4_9
        let s_4_10: bool = ((s_4_7) == (s_4_9));
        // D s_4_11: cast zx s_4_5 -> i
        let s_4_11: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_12: cast reint s_4_11 -> i64
        let s_4_12: i64 = (s_4_11 as i64);
        // D s_4_13: cast zx s_4_5 -> i
        let s_4_13: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_14: cast reint s_4_13 -> i64
        let s_4_14: i64 = (s_4_13 as i64);
        // C s_4_15: const #1s : i
        let s_4_15: i128 = 1;
        // D s_4_16: read-var d:i64
        let s_4_16: i64 = fn_state.d;
        // D s_4_17: read-var m:i64
        let s_4_17: i64 = fn_state.m;
        // D s_4_18: read-var n:i64
        let s_4_18: i64 = fn_state.n;
        // C s_4_19: const #1u : u8
        let s_4_19: bool = true;
        // D s_4_20: call execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd(s_4_16, s_4_12, s_4_15, s_4_14, s_4_17, s_4_18, s_4_19, s_4_10)
        let s_4_20: () = execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd(
            state,
            tracer,
            s_4_16,
            s_4_12,
            s_4_15,
            s_4_14,
            s_4_17,
            s_4_18,
            s_4_19,
            s_4_10,
        );
        // N s_4_21: return
        return;
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
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var gs#171134 <= s_6_0
        fn_state.gs_171134 = s_6_0;
        // N s_6_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
}
