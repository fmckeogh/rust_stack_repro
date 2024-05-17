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
use execute_aarch64_instrs_vector_transfer_vector_table::*;
use common::*;
pub fn decode_tbx_advsimd_aarch64_instrs_vector_transfer_vector_table<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    op: bool,
    len: u8,
    Rm: u8,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        m: i64,
        n: i64,
        ga_269889: i64,
        Rd: u8,
        Rn: u8,
        op: bool,
        len: u8,
        Rm: u8,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        op,
        len,
        Rm,
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
        // D s_0_15: read-var Q:u8
        let s_0_15: bool = fn_state.Q;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 1u16);
        // C s_0_17: const #1u : u8
        let s_0_17: bool = true;
        // C s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_19: cmp-eq s_0_16 s_0_18
        let s_0_19: bool = ((s_0_16) == (s_0_18));
        // N s_0_20: branch s_0_19 b3 b1
        if s_0_19 {
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
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: write-var ga#269889 <= s_1_0
        fn_state.ga_269889 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#269889:i64
        let s_2_0: i64 = fn_state.ga_269889;
        // C s_2_1: const #8s : i
        let s_2_1: i128 = 8;
        // D s_2_2: cast zx s_2_0 -> i
        let s_2_2: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_3: div s_2_2 s_2_1
        let s_2_3: i128 = ((s_2_2) / (s_2_1));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: read-var len:u8
        let s_2_5: u8 = fn_state.len;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 2u16);
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (s_2_6.value() as i128);
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // C s_2_9: const #1s : i
        let s_2_9: i128 = 1;
        // D s_2_10: cast zx s_2_8 -> i
        let s_2_10: i128 = (i128::try_from(s_2_8).unwrap());
        // D s_2_11: add s_2_10 s_2_9
        let s_2_11: i128 = (s_2_10 + s_2_9);
        // D s_2_12: cast reint s_2_11 -> i64
        let s_2_12: i64 = (s_2_11 as i64);
        // D s_2_13: read-var op:u8
        let s_2_13: bool = fn_state.op;
        // D s_2_14: cast zx s_2_13 -> bv
        let s_2_14: Bits = Bits::new(s_2_13 as u128, 1u16);
        // C s_2_15: const #0u : u8
        let s_2_15: bool = false;
        // C s_2_16: cast zx s_2_15 -> bv
        let s_2_16: Bits = Bits::new(s_2_15 as u128, 1u16);
        // D s_2_17: cmp-eq s_2_14 s_2_16
        let s_2_17: bool = ((s_2_14) == (s_2_16));
        // D s_2_18: cast zx s_2_0 -> i
        let s_2_18: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_19: cast reint s_2_18 -> i64
        let s_2_19: i64 = (s_2_18 as i64);
        // D s_2_20: read-var d:i64
        let s_2_20: i64 = fn_state.d;
        // D s_2_21: read-var m:i64
        let s_2_21: i64 = fn_state.m;
        // D s_2_22: read-var n:i64
        let s_2_22: i64 = fn_state.n;
        // D s_2_23: call execute_aarch64_instrs_vector_transfer_vector_table(s_2_20, s_2_19, s_2_4, s_2_17, s_2_21, s_2_22, s_2_12)
        let s_2_23: () = execute_aarch64_instrs_vector_transfer_vector_table(
            state,
            tracer,
            s_2_20,
            s_2_19,
            s_2_4,
            s_2_17,
            s_2_21,
            s_2_22,
            s_2_12,
        );
        // N s_2_24: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #128s : i64
        let s_3_0: i64 = 128;
        // D s_3_1: write-var ga#269889 <= s_3_0
        fn_state.ga_269889 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
