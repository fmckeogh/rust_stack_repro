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
use execute_aarch64_instrs_vector_arithmetic_binary_uniform_diff::*;
use common::*;
pub fn decode_saba_advsimd_aarch64_instrs_vector_arithmetic_binary_uniform_diff<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    ac: bool,
    Rm: u8,
    size: u8,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        n: i64,
        d: i64,
        ga_264685: i64,
        Rd: u8,
        Rn: u8,
        ac: bool,
        Rm: u8,
        size: u8,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        ac,
        Rm,
        size,
        U,
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
        // D s_0_10: read-var Rm:u8
        let s_0_10: u8 = fn_state.Rm;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 5u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: write-var m <= s_0_13
        fn_state.m = s_0_13;
        // D s_0_15: read-var size:u8
        let s_0_15: u8 = fn_state.size;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 2u16);
        // C s_0_17: const #3u : u8
        let s_0_17: u8 = 3;
        // C s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 2u16);
        // D s_0_19: cmp-eq s_0_16 s_0_18
        let s_0_19: bool = ((s_0_16) == (s_0_18));
        // N s_0_20: branch s_0_19 b5 b1
        if s_0_19 {
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
        // D s_1_0: read-var size:u8
        let s_1_0: u8 = fn_state.size;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #8s : i64
        let s_1_4: i64 = 8;
        // D s_1_5: lsl s_1_4 s_1_3
        let s_1_5: i64 = s_1_4 << s_1_3;
        // D s_1_6: write-var esize <= s_1_5
        fn_state.esize = s_1_5;
        // D s_1_7: read-var Q:u8
        let s_1_7: bool = fn_state.Q;
        // D s_1_8: cast zx s_1_7 -> bv
        let s_1_8: Bits = Bits::new(s_1_7 as u128, 1u16);
        // C s_1_9: const #1u : u8
        let s_1_9: bool = true;
        // C s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 1u16);
        // D s_1_11: cmp-eq s_1_8 s_1_10
        let s_1_11: bool = ((s_1_8) == (s_1_10));
        // N s_1_12: branch s_1_11 b4 b2
        if s_1_11 {
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
        // D s_2_1: write-var ga#264685 <= s_2_0
        fn_state.ga_264685 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#264685:i64
        let s_3_0: i64 = fn_state.ga_264685;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var esize:i64
        let s_3_2: i64 = fn_state.esize;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: div s_3_1 s_3_3
        let s_3_4: i128 = ((s_3_1) / (s_3_3));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var U:u8
        let s_3_6: bool = fn_state.U;
        // D s_3_7: cast zx s_3_6 -> bv
        let s_3_7: Bits = Bits::new(s_3_6 as u128, 1u16);
        // C s_3_8: const #1u : u8
        let s_3_8: bool = true;
        // C s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 1u16);
        // D s_3_10: cmp-eq s_3_7 s_3_9
        let s_3_10: bool = ((s_3_7) == (s_3_9));
        // D s_3_11: read-var ac:u8
        let s_3_11: bool = fn_state.ac;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 1u16);
        // C s_3_13: const #1u : u8
        let s_3_13: bool = true;
        // C s_3_14: cast zx s_3_13 -> bv
        let s_3_14: Bits = Bits::new(s_3_13 as u128, 1u16);
        // D s_3_15: cmp-eq s_3_12 s_3_14
        let s_3_15: bool = ((s_3_12) == (s_3_14));
        // D s_3_16: cast zx s_3_0 -> i
        let s_3_16: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_17: cast reint s_3_16 -> i64
        let s_3_17: i64 = (s_3_16 as i64);
        // D s_3_18: read-var esize:i64
        let s_3_18: i64 = fn_state.esize;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // D s_3_21: cast zx s_3_5 -> i
        let s_3_21: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_22: read-var d:i64
        let s_3_22: i64 = fn_state.d;
        // D s_3_23: read-var m:i64
        let s_3_23: i64 = fn_state.m;
        // D s_3_24: read-var n:i64
        let s_3_24: i64 = fn_state.n;
        // D s_3_25: call execute_aarch64_instrs_vector_arithmetic_binary_uniform_diff(s_3_15, s_3_22, s_3_17, s_3_21, s_3_20, s_3_23, s_3_24, s_3_10)
        let s_3_25: () = execute_aarch64_instrs_vector_arithmetic_binary_uniform_diff(
            state,
            tracer,
            s_3_15,
            s_3_22,
            s_3_17,
            s_3_21,
            s_3_20,
            s_3_23,
            s_3_24,
            s_3_10,
        );
        // N s_3_26: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #128s : i64
        let s_4_0: i64 = 128;
        // D s_4_1: write-var ga#264685 <= s_4_0
        fn_state.ga_264685 = s_4_0;
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
