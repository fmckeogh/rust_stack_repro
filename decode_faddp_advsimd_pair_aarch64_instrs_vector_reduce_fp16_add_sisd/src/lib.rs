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
use HaveFP16Ext::*;
use execute_aarch64_instrs_vector_reduce_fp16_add_sisd::*;
use common::*;
pub fn decode_faddp_advsimd_pair_aarch64_instrs_vector_reduce_fp16_add_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    sz: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        sz: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        sz,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveFP16Ext(s_0_0)
        let s_0_1: bool = HaveFP16Ext(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b4 b1
        if s_0_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Rd:u8
        let s_1_0: u8 = fn_state.Rd;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var d <= s_1_3
        fn_state.d = s_1_3;
        // D s_1_5: read-var Rn:u8
        let s_1_5: u8 = fn_state.Rn;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 5u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var n <= s_1_8
        fn_state.n = s_1_8;
        // D s_1_10: read-var sz:u8
        let s_1_10: bool = fn_state.sz;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 1u16);
        // C s_1_12: const #1u : u8
        let s_1_12: bool = true;
        // C s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 1u16);
        // D s_1_14: cmp-eq s_1_11 s_1_13
        let s_1_14: bool = ((s_1_11) == (s_1_13));
        // N s_1_15: branch s_1_14 b3 b2
        if s_1_14 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #2s : i
        let s_2_0: i128 = 2;
        // C s_2_1: const #16s : i64
        let s_2_1: i64 = 16;
        // C s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // C s_2_3: mul s_2_2 s_2_0
        let s_2_3: i128 = ((s_2_2) * (s_2_0));
        // C s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // C s_2_5: const #4u : u32
        let s_2_5: u32 = 4;
        // C s_2_6: cast zx s_2_4 -> i
        let s_2_6: i128 = (i128::try_from(s_2_4).unwrap());
        // C s_2_7: cast reint s_2_6 -> i64
        let s_2_7: i64 = (s_2_6 as i64);
        // C s_2_8: const #16s : i64
        let s_2_8: i64 = 16;
        // C s_2_9: cast zx s_2_8 -> i
        let s_2_9: i128 = (i128::try_from(s_2_8).unwrap());
        // C s_2_10: cast reint s_2_9 -> i64
        let s_2_10: i64 = (s_2_9 as i64);
        // D s_2_11: read-var d:i64
        let s_2_11: i64 = fn_state.d;
        // D s_2_12: read-var n:i64
        let s_2_12: i64 = fn_state.n;
        // D s_2_13: call execute_aarch64_instrs_vector_reduce_fp16_add_sisd(s_2_11, s_2_7, s_2_10, s_2_12, s_2_5)
        let s_2_13: () = execute_aarch64_instrs_vector_reduce_fp16_add_sisd(
            state,
            tracer,
            s_2_11,
            s_2_7,
            s_2_10,
            s_2_12,
            s_2_5,
        );
        // N s_2_14: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: panic
        panic!("{:?}", ());
        // N s_3_1: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
}
