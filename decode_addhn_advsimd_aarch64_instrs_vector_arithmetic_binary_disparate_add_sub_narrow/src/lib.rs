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
use execute_aarch64_instrs_vector_arithmetic_binary_disparate_add_sub_narrow::*;
use common::*;
pub fn decode_addhn_advsimd_aarch64_instrs_vector_arithmetic_binary_disparate_add_sub_narrow<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    o1: bool,
    Rm: u8,
    size: u8,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        o1: bool,
        Rm: u8,
        size: u8,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        o1,
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
        // N s_0_20: branch s_0_19 b2 b1
        if s_0_19 {
            return block_2(state, tracer, fn_state);
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
        // D s_1_6: read-var Q:u8
        let s_1_6: bool = fn_state.Q;
        // D s_1_7: cast zx s_1_6 -> bv
        let s_1_7: Bits = Bits::new(s_1_6 as u128, 1u16);
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (s_1_7.value() as i128);
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // C s_1_10: const #64s : i
        let s_1_10: i128 = 64;
        // D s_1_11: cast zx s_1_5 -> i
        let s_1_11: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_12: div s_1_10 s_1_11
        let s_1_12: i128 = ((s_1_10) / (s_1_11));
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: read-var o1:u8
        let s_1_14: bool = fn_state.o1;
        // D s_1_15: cast zx s_1_14 -> bv
        let s_1_15: Bits = Bits::new(s_1_14 as u128, 1u16);
        // C s_1_16: const #1u : u8
        let s_1_16: bool = true;
        // C s_1_17: cast zx s_1_16 -> bv
        let s_1_17: Bits = Bits::new(s_1_16 as u128, 1u16);
        // D s_1_18: cmp-eq s_1_15 s_1_17
        let s_1_18: bool = ((s_1_15) == (s_1_17));
        // D s_1_19: read-var U:u8
        let s_1_19: bool = fn_state.U;
        // D s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 1u16);
        // C s_1_21: const #1u : u8
        let s_1_21: bool = true;
        // C s_1_22: cast zx s_1_21 -> bv
        let s_1_22: Bits = Bits::new(s_1_21 as u128, 1u16);
        // D s_1_23: cmp-eq s_1_20 s_1_22
        let s_1_23: bool = ((s_1_20) == (s_1_22));
        // D s_1_24: cast zx s_1_5 -> i
        let s_1_24: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // C s_1_26: const #64s : i64
        let s_1_26: i64 = 64;
        // D s_1_27: cast zx s_1_13 -> i
        let s_1_27: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_28: read-var d:i64
        let s_1_28: i64 = fn_state.d;
        // D s_1_29: read-var m:i64
        let s_1_29: i64 = fn_state.m;
        // D s_1_30: read-var n:i64
        let s_1_30: i64 = fn_state.n;
        // D s_1_31: call execute_aarch64_instrs_vector_arithmetic_binary_disparate_add_sub_narrow(s_1_28, s_1_26, s_1_27, s_1_25, s_1_29, s_1_30, s_1_9, s_1_23, s_1_18)
        let s_1_31: () = execute_aarch64_instrs_vector_arithmetic_binary_disparate_add_sub_narrow(
            state,
            tracer,
            s_1_28,
            s_1_26,
            s_1_27,
            s_1_25,
            s_1_29,
            s_1_30,
            s_1_9,
            s_1_23,
            s_1_18,
        );
        // N s_1_32: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
}
