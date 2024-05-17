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
use execute_aarch64_instrs_vector_reduce_fp16_max_sisd::*;
use common::*;
pub fn decode_fmaxp_advsimd_pair_aarch64_instrs_vector_reduce_fp_max_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    sz: bool,
    o1: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        datasize: i64,
        esize: i64,
        op: u32,
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        sz: bool,
        o1: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        sz,
        o1,
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
        // C s_0_17: const #2s : i
        let s_0_17: i128 = 2;
        // D s_0_18: read-var esize:i64
        let s_0_18: i64 = fn_state.esize;
        // D s_0_19: cast zx s_0_18 -> i
        let s_0_19: i128 = (i128::try_from(s_0_18).unwrap());
        // D s_0_20: mul s_0_19 s_0_17
        let s_0_20: i128 = ((s_0_19) * (s_0_17));
        // D s_0_21: cast reint s_0_20 -> i64
        let s_0_21: i64 = (s_0_20 as i64);
        // D s_0_22: write-var datasize <= s_0_21
        fn_state.datasize = s_0_21;
        // D s_0_23: read-var o1:u8
        let s_0_23: bool = fn_state.o1;
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 1u16);
        // C s_0_25: const #1u : u8
        let s_0_25: bool = true;
        // C s_0_26: cast zx s_0_25 -> bv
        let s_0_26: Bits = Bits::new(s_0_25 as u128, 1u16);
        // D s_0_27: cmp-eq s_0_24 s_0_26
        let s_0_27: bool = ((s_0_24) == (s_0_26));
        // N s_0_28: branch s_0_27 b3 b1
        if s_0_27 {
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
        // C s_1_0: const #3u : u32
        let s_1_0: u32 = 3;
        // D s_1_1: write-var op <= s_1_0
        fn_state.op = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var datasize:i64
        let s_2_0: i64 = fn_state.datasize;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: cast reint s_2_1 -> i64
        let s_2_2: i64 = (s_2_1 as i64);
        // D s_2_3: read-var esize:i64
        let s_2_3: i64 = fn_state.esize;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: cast reint s_2_4 -> i64
        let s_2_5: i64 = (s_2_4 as i64);
        // D s_2_6: read-var d:i64
        let s_2_6: i64 = fn_state.d;
        // D s_2_7: read-var n:i64
        let s_2_7: i64 = fn_state.n;
        // D s_2_8: read-var op:u32
        let s_2_8: u32 = fn_state.op;
        // D s_2_9: call execute_aarch64_instrs_vector_reduce_fp16_max_sisd(s_2_6, s_2_2, s_2_5, s_2_7, s_2_8)
        let s_2_9: () = execute_aarch64_instrs_vector_reduce_fp16_max_sisd(
            state,
            tracer,
            s_2_6,
            s_2_2,
            s_2_5,
            s_2_7,
            s_2_8,
        );
        // N s_2_10: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #2u : u32
        let s_3_0: u32 = 2;
        // D s_3_1: write-var op <= s_3_0
        fn_state.op = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}