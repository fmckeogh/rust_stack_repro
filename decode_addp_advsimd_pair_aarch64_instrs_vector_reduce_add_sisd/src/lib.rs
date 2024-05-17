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
use execute_aarch64_instrs_vector_reduce_add_sisd::*;
use common::*;
pub fn decode_addp_advsimd_pair_aarch64_instrs_vector_reduce_add_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    size: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        size: u8,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        size,
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
        // N s_0_15: branch s_0_14 b2 b1
        if s_0_14 {
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
        // C s_1_6: const #2s : i
        let s_1_6: i128 = 2;
        // D s_1_7: cast zx s_1_5 -> i
        let s_1_7: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_8: mul s_1_7 s_1_6
        let s_1_8: i128 = ((s_1_7) * (s_1_6));
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: cast zx s_1_5 -> i
        let s_1_12: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: read-var d:i64
        let s_1_14: i64 = fn_state.d;
        // D s_1_15: read-var n:i64
        let s_1_15: i64 = fn_state.n;
        // C s_1_16: const #5u : u32
        let s_1_16: u32 = 5;
        // D s_1_17: call execute_aarch64_instrs_vector_reduce_add_sisd(s_1_14, s_1_11, s_1_13, s_1_15, s_1_16)
        let s_1_17: () = execute_aarch64_instrs_vector_reduce_add_sisd(
            state,
            tracer,
            s_1_14,
            s_1_11,
            s_1_13,
            s_1_15,
            s_1_16,
        );
        // N s_1_18: return
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
