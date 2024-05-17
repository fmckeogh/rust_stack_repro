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
use execute_aarch64_instrs_vector_arithmetic_unary_cmp_int_bulk_sisd::*;
use common::*;
pub fn decode_cmge_advsimd_zero_aarch64_instrs_vector_arithmetic_unary_cmp_int_bulk_sisd<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, op: bool, size: u8, U: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        datasize: i64,
        n: i64,
        comparison: u32,
        d: i64,
        ga_251138: u8,
        Rd: u8,
        Rn: u8,
        op: bool,
        size: u8,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        op,
        size,
        U,
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
        // D s_0_10: read-var size:u8
        let s_0_10: u8 = fn_state.size;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // C s_0_12: const #3u : u8
        let s_0_12: u8 = 3;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 2u16);
        // D s_0_14: cmp-ne s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) != (s_0_13));
        // N s_0_15: branch s_0_14 b9 b1
        if s_0_14 {
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
        // D s_1_7: read-var esize:i64
        let s_1_7: i64 = fn_state.esize;
        // D s_1_8: write-var datasize <= s_1_7
        fn_state.datasize = s_1_7;
        // D s_1_9: read-var op:u8
        let s_1_9: bool = fn_state.op;
        // D s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 1u16);
        // D s_1_11: read-var U:u8
        let s_1_11: bool = fn_state.U;
        // D s_1_12: cast zx s_1_11 -> bv
        let s_1_12: Bits = Bits::new(s_1_11 as u128, 1u16);
        // D s_1_13: cast reint s_1_10 -> u128
        let s_1_13: u128 = (s_1_10.value() as u128);
        // D s_1_14: size-of s_1_10
        let s_1_14: u16 = s_1_10.length();
        // D s_1_15: cast reint s_1_12 -> u128
        let s_1_15: u128 = (s_1_12.value() as u128);
        // D s_1_16: size-of s_1_12
        let s_1_16: u16 = s_1_12.length();
        // D s_1_17: lsl s_1_13 s_1_16
        let s_1_17: u128 = s_1_13 << s_1_16;
        // D s_1_18: or s_1_17 s_1_15
        let s_1_18: u128 = ((s_1_17) | (s_1_15));
        // D s_1_19: add s_1_14 s_1_16
        let s_1_19: u16 = (s_1_14 + s_1_16);
        // D s_1_20: create-bits s_1_18 s_1_19
        let s_1_20: Bits = Bits::new(s_1_18, s_1_19);
        // D s_1_21: cast reint s_1_20 -> u8
        let s_1_21: u8 = (s_1_20.value() as u8);
        // D s_1_22: write-var ga#251138 <= s_1_21
        fn_state.ga_251138 = s_1_21;
        // D s_1_23: read-var ga#251138:u8
        let s_1_23: u8 = fn_state.ga_251138;
        // D s_1_24: cast zx s_1_23 -> bv
        let s_1_24: Bits = Bits::new(s_1_23 as u128, 2u16);
        // C s_1_25: const #0u : u8
        let s_1_25: u8 = 0;
        // C s_1_26: cast zx s_1_25 -> bv
        let s_1_26: Bits = Bits::new(s_1_25 as u128, 2u16);
        // D s_1_27: cmp-eq s_1_24 s_1_26
        let s_1_27: bool = ((s_1_24) == (s_1_26));
        // D s_1_28: not s_1_27
        let s_1_28: bool = !s_1_27;
        // N s_1_29: branch s_1_28 b4 b2
        if s_1_28 {
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
        // C s_2_0: const #0u : u32
        let s_2_0: u32 = 0;
        // D s_2_1: write-var comparison <= s_2_0
        fn_state.comparison = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var comparison:u32
        let s_3_0: u32 = fn_state.comparison;
        // D s_3_1: read-var datasize:i64
        let s_3_1: i64 = fn_state.datasize;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: read-var esize:i64
        let s_3_4: i64 = fn_state.esize;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // C s_3_7: const #1s : i
        let s_3_7: i128 = 1;
        // D s_3_8: read-var d:i64
        let s_3_8: i64 = fn_state.d;
        // D s_3_9: read-var n:i64
        let s_3_9: i64 = fn_state.n;
        // D s_3_10: call execute_aarch64_instrs_vector_arithmetic_unary_cmp_int_bulk_sisd(s_3_0, s_3_8, s_3_3, s_3_7, s_3_6, s_3_9)
        let s_3_10: () = execute_aarch64_instrs_vector_arithmetic_unary_cmp_int_bulk_sisd(
            state,
            tracer,
            s_3_0,
            s_3_8,
            s_3_3,
            s_3_7,
            s_3_6,
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
        // D s_4_0: read-var ga#251138:u8
        let s_4_0: u8 = fn_state.ga_251138;
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
        // C s_5_0: const #1u : u32
        let s_5_0: u32 = 1;
        // D s_5_1: write-var comparison <= s_5_0
        fn_state.comparison = s_5_0;
        // N s_5_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#251138:u8
        let s_6_0: u8 = fn_state.ga_251138;
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
        // C s_7_0: const #2u : u32
        let s_7_0: u32 = 2;
        // D s_7_1: write-var comparison <= s_7_0
        fn_state.comparison = s_7_0;
        // N s_7_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #3u : u32
        let s_8_0: u32 = 3;
        // D s_8_1: write-var comparison <= s_8_0
        fn_state.comparison = s_8_0;
        // N s_8_2: jump b3
        return block_3(state, tracer, fn_state);
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
