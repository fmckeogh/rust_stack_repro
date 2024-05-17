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
use V_set::*;
use V_read::*;
use AESMixColumns::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use AESInvMixColumns::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_aes_mix<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    decrypt: bool,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: u128,
        result: u128,
        d: i64,
        decrypt: bool,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        decrypt,
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
        // C s_1_0: const #128s : i64
        let s_1_0: i64 = 128;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call V_read(s_1_2, s_1_0)
        let s_1_3: Bits = V_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u128
        let s_1_4: u128 = (s_1_3.value() as u128);
        // D s_1_5: write-var operand <= s_1_4
        fn_state.operand = s_1_4;
        // D s_1_6: read-var decrypt:u8
        let s_1_6: bool = fn_state.decrypt;
        // N s_1_7: branch s_1_6 b4 b2
        if s_1_6 {
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
        // D s_2_0: read-var operand:u128
        let s_2_0: u128 = fn_state.operand;
        // D s_2_1: call AESMixColumns(s_2_0)
        let s_2_1: u128 = AESMixColumns(state, tracer, s_2_0);
        // D s_2_2: write-var result <= s_2_1
        fn_state.result = s_2_1;
        // N s_2_3: jump b3
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
        // D s_4_0: read-var operand:u128
        let s_4_0: u128 = fn_state.operand;
        // D s_4_1: call AESInvMixColumns(s_4_0)
        let s_4_1: u128 = AESInvMixColumns(state, tracer, s_4_0);
        // D s_4_2: write-var result <= s_4_1
        fn_state.result = s_4_1;
        // N s_4_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
