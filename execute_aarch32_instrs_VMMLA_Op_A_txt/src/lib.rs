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
use Q_read::*;
use BFMatMulAdd::*;
use CheckAdvSIMDEnabled::*;
use Q_set::*;
use common::*;
pub fn execute_aarch32_instrs_VMMLA_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_917335: Bits,
        ga_367125: i64,
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
        // S s_0_1: call CheckAdvSIMDEnabled(s_0_0)
        let s_0_1: () = CheckAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1s : i64
        let s_1_0: i64 = 1;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: lsr s_1_1 s_1_0
        let s_1_2: i64 = s_1_1 >> s_1_0;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: call Q_read(s_1_3)
        let s_1_4: u128 = Q_read(state, tracer, s_1_3);
        // C s_1_5: const #1s : i64
        let s_1_5: i64 = 1;
        // D s_1_6: read-var m:i64
        let s_1_6: i64 = fn_state.m;
        // D s_1_7: lsr s_1_6 s_1_5
        let s_1_7: i64 = s_1_6 >> s_1_5;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call Q_read(s_1_8)
        let s_1_9: u128 = Q_read(state, tracer, s_1_8);
        // C s_1_10: const #1s : i64
        let s_1_10: i64 = 1;
        // D s_1_11: read-var d:i64
        let s_1_11: i64 = fn_state.d;
        // D s_1_12: lsr s_1_11 s_1_10
        let s_1_12: i64 = s_1_11 >> s_1_10;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: call Q_read(s_1_13)
        let s_1_14: u128 = Q_read(state, tracer, s_1_13);
        // C s_1_15: const #1s : i64
        let s_1_15: i64 = 1;
        // D s_1_16: read-var d:i64
        let s_1_16: i64 = fn_state.d;
        // D s_1_17: lsr s_1_16 s_1_15
        let s_1_17: i64 = s_1_16 >> s_1_15;
        // D s_1_18: write-var ga#367125 <= s_1_17
        fn_state.ga_367125 = s_1_17;
        // D s_1_19: cast zx s_1_14 -> bv
        let s_1_19: Bits = Bits::new(s_1_14 as u128, 128u16);
        // D s_1_20: cast zx s_1_4 -> bv
        let s_1_20: Bits = Bits::new(s_1_4 as u128, 128u16);
        // D s_1_21: cast zx s_1_9 -> bv
        let s_1_21: Bits = Bits::new(s_1_9 as u128, 128u16);
        // D s_1_22: call BFMatMulAdd(s_1_19, s_1_20, s_1_21)
        let s_1_22: Bits = BFMatMulAdd(state, tracer, s_1_19, s_1_20, s_1_21);
        // D s_1_23: write-var gs#917335 <= s_1_22
        fn_state.gs_917335 = s_1_22;
        // N s_1_24: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#917335:bv
        let s_2_0: Bits = fn_state.gs_917335;
        // D s_2_1: cast reint s_2_0 -> u128
        let s_2_1: u128 = (s_2_0.value() as u128);
        // D s_2_2: read-var ga#367125:i64
        let s_2_2: i64 = fn_state.ga_367125;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: call Q_set(s_2_3, s_2_1)
        let s_2_4: () = Q_set(state, tracer, s_2_3, s_2_1);
        // N s_2_5: return
        return;
    }
}
