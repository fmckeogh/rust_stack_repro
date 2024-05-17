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
use execute_aarch64_instrs_vector_arithmetic_unary_rev::*;
use common::*;
pub fn decode_rev64_advsimd_aarch64_instrs_vector_arithmetic_unary_rev<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    o0: bool,
    size: u8,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        op: u8,
        n: i64,
        d: i64,
        container_size: i64,
        datasize: i64,
        ga_264540: i64,
        Rd: u8,
        Rn: u8,
        o0: bool,
        size: u8,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        o0,
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
        // D s_0_10: read-var size:u8
        let s_0_10: u8 = fn_state.size;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // C s_0_14: const #8s : i64
        let s_0_14: i64 = 8;
        // D s_0_15: lsl s_0_14 s_0_13
        let s_0_15: i64 = s_0_14 << s_0_13;
        // D s_0_16: write-var esize <= s_0_15
        fn_state.esize = s_0_15;
        // D s_0_17: read-var Q:u8
        let s_0_17: bool = fn_state.Q;
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // C s_0_19: const #1u : u8
        let s_0_19: bool = true;
        // C s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 1u16);
        // D s_0_21: cmp-eq s_0_18 s_0_20
        let s_0_21: bool = ((s_0_18) == (s_0_20));
        // N s_0_22: branch s_0_21 b12 b1
        if s_0_21 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: write-var ga#264540 <= s_1_0
        fn_state.ga_264540 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#264540:i64
        let s_2_0: i64 = fn_state.ga_264540;
        // D s_2_1: write-var datasize <= s_2_0
        fn_state.datasize = s_2_0;
        // D s_2_2: read-var o0:u8
        let s_2_2: bool = fn_state.o0;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: read-var U:u8
        let s_2_4: bool = fn_state.U;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: cast reint s_2_5 -> u128
        let s_2_8: u128 = (s_2_5.value() as u128);
        // D s_2_9: size-of s_2_5
        let s_2_9: u16 = s_2_5.length();
        // D s_2_10: lsl s_2_6 s_2_9
        let s_2_10: u128 = s_2_6 << s_2_9;
        // D s_2_11: or s_2_10 s_2_8
        let s_2_11: u128 = ((s_2_10) | (s_2_8));
        // D s_2_12: add s_2_7 s_2_9
        let s_2_12: u16 = (s_2_7 + s_2_9);
        // D s_2_13: create-bits s_2_11 s_2_12
        let s_2_13: Bits = Bits::new(s_2_11, s_2_12);
        // D s_2_14: cast reint s_2_13 -> u8
        let s_2_14: u8 = (s_2_13.value() as u8);
        // D s_2_15: write-var op <= s_2_14
        fn_state.op = s_2_14;
        // D s_2_16: read-var op:u8
        let s_2_16: u8 = fn_state.op;
        // D s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 2u16);
        // D s_2_18: cast zx s_2_17 -> i
        let s_2_18: i128 = (s_2_17.value() as i128);
        // D s_2_19: cast reint s_2_18 -> i64
        let s_2_19: i64 = (s_2_18 as i64);
        // D s_2_20: read-var size:u8
        let s_2_20: u8 = fn_state.size;
        // D s_2_21: cast zx s_2_20 -> bv
        let s_2_21: Bits = Bits::new(s_2_20 as u128, 2u16);
        // D s_2_22: cast zx s_2_21 -> i
        let s_2_22: i128 = (s_2_21.value() as i128);
        // D s_2_23: cast reint s_2_22 -> i64
        let s_2_23: i64 = (s_2_22 as i64);
        // D s_2_24: cast zx s_2_19 -> i
        let s_2_24: i128 = (i128::try_from(s_2_19).unwrap());
        // D s_2_25: cast zx s_2_23 -> i
        let s_2_25: i128 = (i128::try_from(s_2_23).unwrap());
        // D s_2_26: add s_2_24 s_2_25
        let s_2_26: i128 = (s_2_24 + s_2_25);
        // D s_2_27: cast reint s_2_26 -> i64
        let s_2_27: i64 = (s_2_26 as i64);
        // C s_2_28: const #3s : i
        let s_2_28: i128 = 3;
        // D s_2_29: cast zx s_2_27 -> i
        let s_2_29: i128 = (i128::try_from(s_2_27).unwrap());
        // D s_2_30: cmp-ge s_2_29 s_2_28
        let s_2_30: bool = ((s_2_29) >= (s_2_28));
        // N s_2_31: branch s_2_30 b11 b3
        if s_2_30 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16s : i64
        let s_3_0: i64 = 16;
        // D s_3_1: write-var container_size <= s_3_0
        fn_state.container_size = s_3_0;
        // D s_3_2: read-var op:u8
        let s_3_2: u8 = fn_state.op;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // C s_3_4: const #2u : u8
        let s_3_4: u8 = 2;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // D s_3_6: cmp-eq s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) == (s_3_5));
        // D s_3_7: not s_3_6
        let s_3_7: bool = !s_3_6;
        // N s_3_8: branch s_3_7 b6 b4
        if s_3_7 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #16s : i64
        let s_4_0: i64 = 16;
        // D s_4_1: write-var container_size <= s_4_0
        fn_state.container_size = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var container_size:i64
        let s_5_0: i64 = fn_state.container_size;
        // D s_5_1: read-var datasize:i64
        let s_5_1: i64 = fn_state.datasize;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cast zx s_5_0 -> i
        let s_5_3: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_4: div s_5_2 s_5_3
        let s_5_4: i128 = ((s_5_2) / (s_5_3));
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: cast zx s_5_0 -> i
        let s_5_6: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_7: read-var esize:i64
        let s_5_7: i64 = fn_state.esize;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: div s_5_6 s_5_8
        let s_5_9: i128 = ((s_5_6) / (s_5_8));
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: read-var datasize:i64
        let s_5_11: i64 = fn_state.datasize;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: read-var esize:i64
        let s_5_14: i64 = fn_state.esize;
        // D s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_16: cast reint s_5_15 -> i64
        let s_5_16: i64 = (s_5_15 as i64);
        // D s_5_17: cast zx s_5_5 -> i
        let s_5_17: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_18: cast zx s_5_10 -> i
        let s_5_18: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_19: read-var d:i64
        let s_5_19: i64 = fn_state.d;
        // D s_5_20: read-var n:i64
        let s_5_20: i64 = fn_state.n;
        // D s_5_21: call execute_aarch64_instrs_vector_arithmetic_unary_rev(s_5_17, s_5_19, s_5_13, s_5_18, s_5_16, s_5_20)
        let s_5_21: () = execute_aarch64_instrs_vector_arithmetic_unary_rev(
            state,
            tracer,
            s_5_17,
            s_5_19,
            s_5_13,
            s_5_18,
            s_5_16,
            s_5_20,
        );
        // N s_5_22: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var op:u8
        let s_6_0: u8 = fn_state.op;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #1u : u8
        let s_6_2: u8 = 1;
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
        // C s_7_0: const #32s : i64
        let s_7_0: i64 = 32;
        // D s_7_1: write-var container_size <= s_7_0
        fn_state.container_size = s_7_0;
        // N s_7_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var op:u8
        let s_8_0: u8 = fn_state.op;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #0u : u8
        let s_8_2: u8 = 0;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b10 b9
        if s_8_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #64s : i64
        let s_9_0: i64 = 64;
        // D s_9_1: write-var container_size <= s_9_0
        fn_state.container_size = s_9_0;
        // N s_9_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #128s : i64
        let s_12_0: i64 = 128;
        // D s_12_1: write-var ga#264540 <= s_12_0
        fn_state.ga_264540 = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
