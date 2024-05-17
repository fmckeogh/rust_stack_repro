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
use execute_aarch64_instrs_vector_reduce_fp16_max_simd::*;
use common::*;
pub fn decode_fmaxv_advsimd_aarch64_instrs_vector_reduce_fp16_max_simd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    o1: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        datasize: i64,
        ga_255413: i64,
        op: u32,
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        o1: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        o1,
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
        // D s_0_10: read-var Q:u8
        let s_0_10: bool = fn_state.Q;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // C s_0_12: const #1u : u8
        let s_0_12: bool = true;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b6 b1
        if s_0_14 {
            return block_6(state, tracer, fn_state);
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
        // D s_1_1: write-var ga#255413 <= s_1_0
        fn_state.ga_255413 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#255413:i64
        let s_2_0: i64 = fn_state.ga_255413;
        // D s_2_1: write-var datasize <= s_2_0
        fn_state.datasize = s_2_0;
        // D s_2_2: read-var o1:u8
        let s_2_2: bool = fn_state.o1;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #1u : u8
        let s_2_4: bool = true;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b5 b3
        if s_2_6 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #3u : u32
        let s_3_0: u32 = 3;
        // D s_3_1: write-var op <= s_3_0
        fn_state.op = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasize:i64
        let s_4_0: i64 = fn_state.datasize;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // C s_4_3: const #16s : i64
        let s_4_3: i64 = 16;
        // D s_4_4: read-var d:i64
        let s_4_4: i64 = fn_state.d;
        // D s_4_5: read-var n:i64
        let s_4_5: i64 = fn_state.n;
        // D s_4_6: read-var op:u32
        let s_4_6: u32 = fn_state.op;
        // D s_4_7: call execute_aarch64_instrs_vector_reduce_fp16_max_simd(s_4_4, s_4_2, s_4_3, s_4_5, s_4_6)
        let s_4_7: () = execute_aarch64_instrs_vector_reduce_fp16_max_simd(
            state,
            tracer,
            s_4_4,
            s_4_2,
            s_4_3,
            s_4_5,
            s_4_6,
        );
        // N s_4_8: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #2u : u32
        let s_5_0: u32 = 2;
        // D s_5_1: write-var op <= s_5_0
        fn_state.op = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #128s : i64
        let s_6_0: i64 = 128;
        // D s_6_1: write-var ga#255413 <= s_6_0
        fn_state.ga_255413 = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
