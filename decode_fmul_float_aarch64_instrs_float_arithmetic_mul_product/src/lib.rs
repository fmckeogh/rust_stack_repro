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
use execute_aarch64_instrs_float_arithmetic_mul_product::*;
use common::*;
pub fn decode_fmul_float_aarch64_instrs_float_arithmetic_mul_product<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    op: bool,
    Rm: u8,
    ftype: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        op: bool,
        Rm: u8,
        ftype: u8,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        op,
        Rm,
        ftype,
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
        // C s_0_15: const #16s : i64
        let s_0_15: i64 = 16;
        // D s_0_16: write-var esize <= s_0_15
        fn_state.esize = s_0_15;
        // D s_0_17: read-var ftype:u8
        let s_0_17: u8 = fn_state.ftype;
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 2u16);
        // C s_0_19: const #0u : u8
        let s_0_19: u8 = 0;
        // C s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 2u16);
        // D s_0_21: cmp-eq s_0_18 s_0_20
        let s_0_21: bool = ((s_0_18) == (s_0_20));
        // D s_0_22: not s_0_21
        let s_0_22: bool = !s_0_21;
        // N s_0_23: branch s_0_22 b3 b1
        if s_0_22 {
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
        // D s_1_1: write-var esize <= s_1_0
        fn_state.esize = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var esize:i64
        let s_2_0: i64 = fn_state.esize;
        // D s_2_1: read-var op:u8
        let s_2_1: bool = fn_state.op;
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
        // D s_2_8: read-var d:i64
        let s_2_8: i64 = fn_state.d;
        // D s_2_9: read-var m:i64
        let s_2_9: i64 = fn_state.m;
        // D s_2_10: read-var n:i64
        let s_2_10: i64 = fn_state.n;
        // D s_2_11: call execute_aarch64_instrs_float_arithmetic_mul_product(s_2_8, s_2_7, s_2_9, s_2_10, s_2_5)
        let s_2_11: () = execute_aarch64_instrs_float_arithmetic_mul_product(
            state,
            tracer,
            s_2_8,
            s_2_7,
            s_2_9,
            s_2_10,
            s_2_5,
        );
        // N s_2_12: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ftype:u8
        let s_3_0: u8 = fn_state.ftype;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #1u : u8
        let s_3_2: u8 = 1;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: not s_3_4
        let s_3_5: bool = !s_3_4;
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
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
        // C s_4_0: const #64s : i64
        let s_4_0: i64 = 64;
        // D s_4_1: write-var esize <= s_4_0
        fn_state.esize = s_4_0;
        // N s_4_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ftype:u8
        let s_5_0: u8 = fn_state.ftype;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #2u : u8
        let s_5_2: u8 = 2;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16s : i64
        let s_7_0: i64 = 16;
        // D s_7_1: write-var esize <= s_7_0
        fn_state.esize = s_7_0;
        // N s_7_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
