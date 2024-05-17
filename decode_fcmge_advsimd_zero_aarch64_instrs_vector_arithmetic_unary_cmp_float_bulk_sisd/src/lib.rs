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
use execute_aarch64_instrs_vector_arithmetic_unary_cmp_fp16_bulk_sisd::*;
use common::*;
pub fn decode_fcmge_advsimd_zero_aarch64_instrs_vector_arithmetic_unary_cmp_float_bulk_sisd<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, op: bool, sz: bool, U: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_253233: u8,
        esize: i64,
        datasize: i64,
        n: i64,
        comparison: u32,
        d: i64,
        Rd: u8,
        Rn: u8,
        op: bool,
        sz: bool,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        op,
        sz,
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
        // D s_0_10: read-var sz:u8
        let s_0_10: bool = fn_state.sz;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // C s_0_14: const #32s : i64
        let s_0_14: i64 = 32;
        // D s_0_15: lsl s_0_14 s_0_13
        let s_0_15: i64 = s_0_14 << s_0_13;
        // D s_0_16: write-var esize <= s_0_15
        fn_state.esize = s_0_15;
        // D s_0_17: read-var esize:i64
        let s_0_17: i64 = fn_state.esize;
        // D s_0_18: write-var datasize <= s_0_17
        fn_state.datasize = s_0_17;
        // D s_0_19: read-var op:u8
        let s_0_19: bool = fn_state.op;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 1u16);
        // D s_0_21: read-var U:u8
        let s_0_21: bool = fn_state.U;
        // D s_0_22: cast zx s_0_21 -> bv
        let s_0_22: Bits = Bits::new(s_0_21 as u128, 1u16);
        // D s_0_23: cast reint s_0_20 -> u128
        let s_0_23: u128 = (s_0_20.value() as u128);
        // D s_0_24: size-of s_0_20
        let s_0_24: u16 = s_0_20.length();
        // D s_0_25: cast reint s_0_22 -> u128
        let s_0_25: u128 = (s_0_22.value() as u128);
        // D s_0_26: size-of s_0_22
        let s_0_26: u16 = s_0_22.length();
        // D s_0_27: lsl s_0_23 s_0_26
        let s_0_27: u128 = s_0_23 << s_0_26;
        // D s_0_28: or s_0_27 s_0_25
        let s_0_28: u128 = ((s_0_27) | (s_0_25));
        // D s_0_29: add s_0_24 s_0_26
        let s_0_29: u16 = (s_0_24 + s_0_26);
        // D s_0_30: create-bits s_0_28 s_0_29
        let s_0_30: Bits = Bits::new(s_0_28, s_0_29);
        // D s_0_31: cast reint s_0_30 -> u8
        let s_0_31: u8 = (s_0_30.value() as u8);
        // D s_0_32: write-var ga#253233 <= s_0_31
        fn_state.ga_253233 = s_0_31;
        // D s_0_33: read-var ga#253233:u8
        let s_0_33: u8 = fn_state.ga_253233;
        // D s_0_34: cast zx s_0_33 -> bv
        let s_0_34: Bits = Bits::new(s_0_33 as u128, 2u16);
        // C s_0_35: const #0u : u8
        let s_0_35: u8 = 0;
        // C s_0_36: cast zx s_0_35 -> bv
        let s_0_36: Bits = Bits::new(s_0_35 as u128, 2u16);
        // D s_0_37: cmp-eq s_0_34 s_0_36
        let s_0_37: bool = ((s_0_34) == (s_0_36));
        // D s_0_38: not s_0_37
        let s_0_38: bool = !s_0_37;
        // N s_0_39: branch s_0_38 b3 b1
        if s_0_38 {
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
        // C s_1_0: const #0u : u32
        let s_1_0: u32 = 0;
        // D s_1_1: write-var comparison <= s_1_0
        fn_state.comparison = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var comparison:u32
        let s_2_0: u32 = fn_state.comparison;
        // D s_2_1: read-var datasize:i64
        let s_2_1: i64 = fn_state.datasize;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: read-var esize:i64
        let s_2_4: i64 = fn_state.esize;
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_6: cast reint s_2_5 -> i64
        let s_2_6: i64 = (s_2_5 as i64);
        // C s_2_7: const #1s : i
        let s_2_7: i128 = 1;
        // D s_2_8: read-var d:i64
        let s_2_8: i64 = fn_state.d;
        // D s_2_9: read-var n:i64
        let s_2_9: i64 = fn_state.n;
        // D s_2_10: call execute_aarch64_instrs_vector_arithmetic_unary_cmp_fp16_bulk_sisd(s_2_0, s_2_8, s_2_3, s_2_7, s_2_6, s_2_9)
        let s_2_10: () = execute_aarch64_instrs_vector_arithmetic_unary_cmp_fp16_bulk_sisd(
            state,
            tracer,
            s_2_0,
            s_2_8,
            s_2_3,
            s_2_7,
            s_2_6,
            s_2_9,
        );
        // N s_2_11: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#253233:u8
        let s_3_0: u8 = fn_state.ga_253233;
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
        // C s_4_0: const #1u : u32
        let s_4_0: u32 = 1;
        // D s_4_1: write-var comparison <= s_4_0
        fn_state.comparison = s_4_0;
        // N s_4_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#253233:u8
        let s_5_0: u8 = fn_state.ga_253233;
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
        // C s_6_0: const #2u : u32
        let s_6_0: u32 = 2;
        // D s_6_1: write-var comparison <= s_6_0
        fn_state.comparison = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #3u : u32
        let s_7_0: u32 = 3;
        // D s_7_1: write-var comparison <= s_7_0
        fn_state.comparison = s_7_0;
        // N s_7_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
