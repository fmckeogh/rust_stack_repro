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
use execute_aarch64_instrs_vector_arithmetic_unary_add_pairwise::*;
use common::*;
pub fn decode_uaddlp_advsimd_aarch64_instrs_vector_arithmetic_unary_add_pairwise<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    op: bool,
    size: u8,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        ga_264851: i64,
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        op: bool,
        size: u8,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        op,
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
        // C s_0_12: const #3u : u8
        let s_0_12: u8 = 3;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 2u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b5 b1
        if s_0_14 {
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
        // D s_2_1: write-var ga#264851 <= s_2_0
        fn_state.ga_264851 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#264851:i64
        let s_3_0: i64 = fn_state.ga_264851;
        // C s_3_1: const #2s : i
        let s_3_1: i128 = 2;
        // D s_3_2: read-var esize:i64
        let s_3_2: i64 = fn_state.esize;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: mul s_3_1 s_3_3
        let s_3_4: i128 = ((s_3_1) * (s_3_3));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: cast zx s_3_0 -> i
        let s_3_6: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_7: cast zx s_3_5 -> i
        let s_3_7: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_8: div s_3_6 s_3_7
        let s_3_8: i128 = ((s_3_6) / (s_3_7));
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: read-var op:u8
        let s_3_10: bool = fn_state.op;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 1u16);
        // C s_3_12: const #1u : u8
        let s_3_12: bool = true;
        // C s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 1u16);
        // D s_3_14: cmp-eq s_3_11 s_3_13
        let s_3_14: bool = ((s_3_11) == (s_3_13));
        // D s_3_15: read-var U:u8
        let s_3_15: bool = fn_state.U;
        // D s_3_16: cast zx s_3_15 -> bv
        let s_3_16: Bits = Bits::new(s_3_15 as u128, 1u16);
        // C s_3_17: const #1u : u8
        let s_3_17: bool = true;
        // C s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 1u16);
        // D s_3_19: cmp-eq s_3_16 s_3_18
        let s_3_19: bool = ((s_3_16) == (s_3_18));
        // D s_3_20: cast zx s_3_0 -> i
        let s_3_20: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // D s_3_22: read-var esize:i64
        let s_3_22: i64 = fn_state.esize;
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // D s_3_25: cast zx s_3_9 -> i
        let s_3_25: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_26: read-var d:i64
        let s_3_26: i64 = fn_state.d;
        // D s_3_27: read-var n:i64
        let s_3_27: i64 = fn_state.n;
        // D s_3_28: call execute_aarch64_instrs_vector_arithmetic_unary_add_pairwise(s_3_14, s_3_26, s_3_21, s_3_25, s_3_24, s_3_27, s_3_19)
        let s_3_28: () = execute_aarch64_instrs_vector_arithmetic_unary_add_pairwise(
            state,
            tracer,
            s_3_14,
            s_3_26,
            s_3_21,
            s_3_25,
            s_3_24,
            s_3_27,
            s_3_19,
        );
        // N s_3_29: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #128s : i64
        let s_4_0: i64 = 128;
        // D s_4_1: write-var ga#264851 <= s_4_0
        fn_state.ga_264851 = s_4_0;
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
