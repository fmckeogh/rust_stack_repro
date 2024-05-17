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
use BFMatMulAdd::*;
use V_read::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_bfmmla<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_684123: Bits,
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
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
        // S s_0_1: call CheckFPAdvSIMDEnabled64(s_0_0)
        let s_0_1: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_0);
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
        // C s_1_5: const #128s : i64
        let s_1_5: i64 = 128;
        // D s_1_6: read-var m:i64
        let s_1_6: i64 = fn_state.m;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: call V_read(s_1_7, s_1_5)
        let s_1_8: Bits = V_read(state, tracer, s_1_7, s_1_5);
        // D s_1_9: cast reint s_1_8 -> u128
        let s_1_9: u128 = (s_1_8.value() as u128);
        // C s_1_10: const #128s : i64
        let s_1_10: i64 = 128;
        // D s_1_11: read-var d:i64
        let s_1_11: i64 = fn_state.d;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call V_read(s_1_12, s_1_10)
        let s_1_13: Bits = V_read(state, tracer, s_1_12, s_1_10);
        // D s_1_14: cast reint s_1_13 -> u128
        let s_1_14: u128 = (s_1_13.value() as u128);
        // D s_1_15: cast zx s_1_14 -> bv
        let s_1_15: Bits = Bits::new(s_1_14 as u128, 128u16);
        // D s_1_16: cast zx s_1_4 -> bv
        let s_1_16: Bits = Bits::new(s_1_4 as u128, 128u16);
        // D s_1_17: cast zx s_1_9 -> bv
        let s_1_17: Bits = Bits::new(s_1_9 as u128, 128u16);
        // D s_1_18: call BFMatMulAdd(s_1_15, s_1_16, s_1_17)
        let s_1_18: Bits = BFMatMulAdd(state, tracer, s_1_15, s_1_16, s_1_17);
        // D s_1_19: write-var gs#684123 <= s_1_18
        fn_state.gs_684123 = s_1_18;
        // N s_1_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#684123:bv
        let s_2_0: Bits = fn_state.gs_684123;
        // D s_2_1: cast reint s_2_0 -> u128
        let s_2_1: u128 = (s_2_0.value() as u128);
        // D s_2_2: read-var d:i64
        let s_2_2: i64 = fn_state.d;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: cast zx s_2_1 -> bv
        let s_2_4: Bits = Bits::new(s_2_1 as u128, 128u16);
        // C s_2_5: const #128s : i64
        let s_2_5: i64 = 128;
        // D s_2_6: call V_set(s_2_3, s_2_5, s_2_4)
        let s_2_6: () = V_set(state, tracer, s_2_3, s_2_5, s_2_4);
        // N s_2_7: return
        return;
    }
}
