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
use AESSubBytes::*;
use AESInvShiftRows::*;
use V_set::*;
use V_read::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use AESShiftRows::*;
use AESInvSubBytes::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_aes_round<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    decrypt: bool,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
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
        // D s_1_1: read-var d:i64
        let s_1_1: i64 = fn_state.d;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call V_read(s_1_2, s_1_0)
        let s_1_3: Bits = V_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u128
        let s_1_4: u128 = (s_1_3.value() as u128);
        // C s_1_5: const #128s : i64
        let s_1_5: i64 = 128;
        // D s_1_6: read-var n:i64
        let s_1_6: i64 = fn_state.n;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: call V_read(s_1_7, s_1_5)
        let s_1_8: Bits = V_read(state, tracer, s_1_7, s_1_5);
        // D s_1_9: cast reint s_1_8 -> u128
        let s_1_9: u128 = (s_1_8.value() as u128);
        // D s_1_10: cast zx s_1_4 -> bv
        let s_1_10: Bits = Bits::new(s_1_4 as u128, 128u16);
        // D s_1_11: cast zx s_1_9 -> bv
        let s_1_11: Bits = Bits::new(s_1_9 as u128, 128u16);
        // D s_1_12: xor s_1_10 s_1_11
        let s_1_12: Bits = ((s_1_10) ^ (s_1_11));
        // D s_1_13: cast reint s_1_12 -> u128
        let s_1_13: u128 = (s_1_12.value() as u128);
        // D s_1_14: write-var result <= s_1_13
        fn_state.result = s_1_13;
        // D s_1_15: read-var decrypt:u8
        let s_1_15: bool = fn_state.decrypt;
        // N s_1_16: branch s_1_15 b4 b2
        if s_1_15 {
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
        // D s_2_0: read-var result:u128
        let s_2_0: u128 = fn_state.result;
        // D s_2_1: call AESShiftRows(s_2_0)
        let s_2_1: u128 = AESShiftRows(state, tracer, s_2_0);
        // D s_2_2: call AESSubBytes(s_2_1)
        let s_2_2: u128 = AESSubBytes(state, tracer, s_2_1);
        // D s_2_3: write-var result <= s_2_2
        fn_state.result = s_2_2;
        // N s_2_4: jump b3
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
        // D s_4_0: read-var result:u128
        let s_4_0: u128 = fn_state.result;
        // D s_4_1: call AESInvShiftRows(s_4_0)
        let s_4_1: u128 = AESInvShiftRows(state, tracer, s_4_0);
        // D s_4_2: call AESInvSubBytes(s_4_1)
        let s_4_2: u128 = AESInvSubBytes(state, tracer, s_4_1);
        // D s_4_3: write-var result <= s_4_2
        fn_state.result = s_4_2;
        // N s_4_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
