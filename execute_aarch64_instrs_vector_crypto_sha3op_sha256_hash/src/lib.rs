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
use SHA256hash::*;
use V_set::*;
use V_read::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_sha3op_sha256_hash<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
    part1: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: u128,
        d: i64,
        m: i64,
        n: i64,
        part1: bool,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
        part1,
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
        // D s_1_0: read-var part1:u8
        let s_1_0: bool = fn_state.part1;
        // N s_1_1: branch s_1_0 b4 b2
        if s_1_0 {
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
        // C s_2_0: const #128s : i64
        let s_2_0: i64 = 128;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call V_read(s_2_2, s_2_0)
        let s_2_3: Bits = V_read(state, tracer, s_2_2, s_2_0);
        // D s_2_4: cast reint s_2_3 -> u128
        let s_2_4: u128 = (s_2_3.value() as u128);
        // C s_2_5: const #128s : i64
        let s_2_5: i64 = 128;
        // D s_2_6: read-var d:i64
        let s_2_6: i64 = fn_state.d;
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_8: call V_read(s_2_7, s_2_5)
        let s_2_8: Bits = V_read(state, tracer, s_2_7, s_2_5);
        // D s_2_9: cast reint s_2_8 -> u128
        let s_2_9: u128 = (s_2_8.value() as u128);
        // C s_2_10: const #128s : i64
        let s_2_10: i64 = 128;
        // D s_2_11: read-var m:i64
        let s_2_11: i64 = fn_state.m;
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_13: call V_read(s_2_12, s_2_10)
        let s_2_13: Bits = V_read(state, tracer, s_2_12, s_2_10);
        // D s_2_14: cast reint s_2_13 -> u128
        let s_2_14: u128 = (s_2_13.value() as u128);
        // C s_2_15: const #0u : u8
        let s_2_15: bool = false;
        // D s_2_16: call SHA256hash(s_2_4, s_2_9, s_2_14, s_2_15)
        let s_2_16: u128 = SHA256hash(state, tracer, s_2_4, s_2_9, s_2_14, s_2_15);
        // D s_2_17: write-var result <= s_2_16
        fn_state.result = s_2_16;
        // N s_2_18: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #128s : i64
        let s_3_0: i64 = 128;
        // D s_3_1: read-var d:i64
        let s_3_1: i64 = fn_state.d;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: read-var result:u128
        let s_3_3: u128 = fn_state.result;
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 128u16);
        // D s_3_5: call V_set(s_3_2, s_3_0, s_3_4)
        let s_3_5: () = V_set(state, tracer, s_3_2, s_3_0, s_3_4);
        // N s_3_6: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #128s : i64
        let s_4_0: i64 = 128;
        // D s_4_1: read-var d:i64
        let s_4_1: i64 = fn_state.d;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: call V_read(s_4_2, s_4_0)
        let s_4_3: Bits = V_read(state, tracer, s_4_2, s_4_0);
        // D s_4_4: cast reint s_4_3 -> u128
        let s_4_4: u128 = (s_4_3.value() as u128);
        // C s_4_5: const #128s : i64
        let s_4_5: i64 = 128;
        // D s_4_6: read-var n:i64
        let s_4_6: i64 = fn_state.n;
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: call V_read(s_4_7, s_4_5)
        let s_4_8: Bits = V_read(state, tracer, s_4_7, s_4_5);
        // D s_4_9: cast reint s_4_8 -> u128
        let s_4_9: u128 = (s_4_8.value() as u128);
        // C s_4_10: const #128s : i64
        let s_4_10: i64 = 128;
        // D s_4_11: read-var m:i64
        let s_4_11: i64 = fn_state.m;
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_13: call V_read(s_4_12, s_4_10)
        let s_4_13: Bits = V_read(state, tracer, s_4_12, s_4_10);
        // D s_4_14: cast reint s_4_13 -> u128
        let s_4_14: u128 = (s_4_13.value() as u128);
        // C s_4_15: const #1u : u8
        let s_4_15: bool = true;
        // D s_4_16: call SHA256hash(s_4_4, s_4_9, s_4_14, s_4_15)
        let s_4_16: u128 = SHA256hash(state, tracer, s_4_4, s_4_9, s_4_14, s_4_15);
        // D s_4_17: write-var result <= s_4_16
        fn_state.result = s_4_16;
        // N s_4_18: jump b3
        return block_3(state, tracer, fn_state);
    }
}
