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
use execute_aarch64_instrs_integer_arithmetic_mul_uniform_add_sub::*;
use common::*;
pub fn decode_msub_aarch64_instrs_integer_arithmetic_mul_uniform_add_sub<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    Ra: u8,
    o0: bool,
    Rm: u8,
    sf: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        ga_263360: i64,
        n: i64,
        a: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        Ra: u8,
        o0: bool,
        Rm: u8,
        sf: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        Ra,
        o0,
        Rm,
        sf,
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
        // D s_0_15: read-var Ra:u8
        let s_0_15: u8 = fn_state.Ra;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 5u16);
        // D s_0_17: cast zx s_0_16 -> i
        let s_0_17: i128 = (s_0_16.value() as i128);
        // D s_0_18: cast reint s_0_17 -> i64
        let s_0_18: i64 = (s_0_17 as i64);
        // D s_0_19: write-var a <= s_0_18
        fn_state.a = s_0_18;
        // D s_0_20: read-var sf:u8
        let s_0_20: bool = fn_state.sf;
        // D s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 1u16);
        // C s_0_22: const #1u : u8
        let s_0_22: bool = true;
        // C s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 1u16);
        // D s_0_24: cmp-eq s_0_21 s_0_23
        let s_0_24: bool = ((s_0_21) == (s_0_23));
        // N s_0_25: branch s_0_24 b3 b1
        if s_0_24 {
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
        // C s_1_0: const #32s : i64
        let s_1_0: i64 = 32;
        // D s_1_1: write-var ga#263360 <= s_1_0
        fn_state.ga_263360 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#263360:i64
        let s_2_0: i64 = fn_state.ga_263360;
        // D s_2_1: read-var o0:u8
        let s_2_1: bool = fn_state.o0;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 1u16);
        // C s_2_3: const #1u : u8
        let s_2_3: bool = true;
        // C s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 1u16);
        // D s_2_5: cmp-eq s_2_2 s_2_4
        let s_2_5: bool = ((s_2_2) == (s_2_4));
        // D s_2_6: cast zx s_2_0 -> i
        let s_2_6: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_7: cast reint s_2_6 -> i64
        let s_2_7: i64 = (s_2_6 as i64);
        // D s_2_8: cast zx s_2_0 -> i
        let s_2_8: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: read-var a:i64
        let s_2_10: i64 = fn_state.a;
        // D s_2_11: read-var d:i64
        let s_2_11: i64 = fn_state.d;
        // D s_2_12: read-var m:i64
        let s_2_12: i64 = fn_state.m;
        // D s_2_13: read-var n:i64
        let s_2_13: i64 = fn_state.n;
        // D s_2_14: call execute_aarch64_instrs_integer_arithmetic_mul_uniform_add_sub(s_2_10, s_2_11, s_2_7, s_2_9, s_2_12, s_2_13, s_2_5)
        let s_2_14: () = execute_aarch64_instrs_integer_arithmetic_mul_uniform_add_sub(
            state,
            tracer,
            s_2_10,
            s_2_11,
            s_2_7,
            s_2_9,
            s_2_12,
            s_2_13,
            s_2_5,
        );
        // N s_2_15: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: write-var ga#263360 <= s_3_0
        fn_state.ga_263360 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
