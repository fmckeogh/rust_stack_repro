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
use V_read::*;
use V_set::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_sha3_bcax<T: Tracer>(
    state: &mut State,
    tracer: &T,
    a: i64,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        a: i64,
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        a,
        d,
        m,
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
        // D s_1_1: read-var m:i64
        let s_1_1: i64 = fn_state.m;
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
        // C s_1_10: const #128s : i64
        let s_1_10: i64 = 128;
        // D s_1_11: read-var a:i64
        let s_1_11: i64 = fn_state.a;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call V_read(s_1_12, s_1_10)
        let s_1_13: Bits = V_read(state, tracer, s_1_12, s_1_10);
        // D s_1_14: cast reint s_1_13 -> u128
        let s_1_14: u128 = (s_1_13.value() as u128);
        // C s_1_15: const #128s : i64
        let s_1_15: i64 = 128;
        // D s_1_16: cast zx s_1_14 -> bv
        let s_1_16: Bits = Bits::new(s_1_14 as u128, 128u16);
        // D s_1_17: not s_1_16
        let s_1_17: Bits = !s_1_16;
        // D s_1_18: cast reint s_1_17 -> u128
        let s_1_18: u128 = (s_1_17.value() as u128);
        // D s_1_19: cast zx s_1_4 -> bv
        let s_1_19: Bits = Bits::new(s_1_4 as u128, 128u16);
        // D s_1_20: cast zx s_1_18 -> bv
        let s_1_20: Bits = Bits::new(s_1_18 as u128, 128u16);
        // D s_1_21: and s_1_19 s_1_20
        let s_1_21: Bits = ((s_1_19) & (s_1_20));
        // D s_1_22: cast reint s_1_21 -> u128
        let s_1_22: u128 = (s_1_21.value() as u128);
        // D s_1_23: cast zx s_1_9 -> bv
        let s_1_23: Bits = Bits::new(s_1_9 as u128, 128u16);
        // D s_1_24: cast zx s_1_22 -> bv
        let s_1_24: Bits = Bits::new(s_1_22 as u128, 128u16);
        // D s_1_25: xor s_1_23 s_1_24
        let s_1_25: Bits = ((s_1_23) ^ (s_1_24));
        // D s_1_26: cast reint s_1_25 -> u128
        let s_1_26: u128 = (s_1_25.value() as u128);
        // D s_1_27: read-var d:i64
        let s_1_27: i64 = fn_state.d;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: cast zx s_1_26 -> bv
        let s_1_29: Bits = Bits::new(s_1_26 as u128, 128u16);
        // D s_1_30: call V_set(s_1_28, s_1_15, s_1_29)
        let s_1_30: () = V_set(state, tracer, s_1_28, s_1_15, s_1_29);
        // N s_1_31: return
        return;
    }
}
