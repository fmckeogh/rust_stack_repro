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
use execute_aarch64_instrs_vector_crypto_sha2op_sha1_sched1::*;
use HaveSHA1Ext::*;
use common::*;
pub fn decode_sha1su1_advsimd_aarch64_instrs_vector_crypto_sha2op_sha1_sched1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
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
        // C s_0_10: const #() : ()
        let s_0_10: () = ();
        // S s_0_11: call HaveSHA1Ext(s_0_10)
        let s_0_11: bool = HaveSHA1Ext(state, tracer, s_0_10);
        // S s_0_12: not s_0_11
        let s_0_12: bool = !s_0_11;
        // N s_0_13: branch s_0_12 b2 b1
        if s_0_12 {
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
        // D s_1_0: read-var d:i64
        let s_1_0: i64 = fn_state.d;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: call execute_aarch64_instrs_vector_crypto_sha2op_sha1_sched1(s_1_0, s_1_1)
        let s_1_2: () = execute_aarch64_instrs_vector_crypto_sha2op_sha1_sched1(
            state,
            tracer,
            s_1_0,
            s_1_1,
        );
        // N s_1_3: return
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
