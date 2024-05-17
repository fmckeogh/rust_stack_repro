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
use ROL::*;
use V_set::*;
use V_read::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_sha2op_sha1_hash<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        n,
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
        // S s_0_1: call AArch64_CheckFPAdvSIMDEnabled(s_0_0)
        let s_0_1: () = AArch64_CheckFPAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #32s : i64
        let s_1_0: i64 = 32;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call V_read(s_1_2, s_1_0)
        let s_1_3: Bits = V_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u32
        let s_1_4: u32 = (s_1_3.value() as u32);
        // C s_1_5: const #32s : i64
        let s_1_5: i64 = 32;
        // C s_1_6: const #30s : i
        let s_1_6: i128 = 30;
        // D s_1_7: cast zx s_1_4 -> bv
        let s_1_7: Bits = Bits::new(s_1_4 as u128, 32u16);
        // D s_1_8: call ROL(s_1_7, s_1_6)
        let s_1_8: Bits = ROL(state, tracer, s_1_7, s_1_6);
        // D s_1_9: cast reint s_1_8 -> u32
        let s_1_9: u32 = (s_1_8.value() as u32);
        // D s_1_10: read-var d:i64
        let s_1_10: i64 = fn_state.d;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: cast zx s_1_9 -> bv
        let s_1_12: Bits = Bits::new(s_1_9 as u128, 32u16);
        // D s_1_13: call V_set(s_1_11, s_1_5, s_1_12)
        let s_1_13: () = V_set(state, tracer, s_1_11, s_1_5, s_1_12);
        // N s_1_14: return
        return;
    }
}
