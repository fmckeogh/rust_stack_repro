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
use neq_int::*;
use execute_aarch64_instrs_memory_rcw_cas::*;
use HaveTHExt::*;
use common::*;
pub fn decode_rcwcas_aarch64_instrs_memory_rcw_cas<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    Rs: u8,
    R: bool,
    A: bool,
    S: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rt: u8,
        Rn: u8,
        Rs: u8,
        R: bool,
        A: bool,
        S: bool,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        Rs,
        R,
        A,
        S,
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
        // S s_0_1: call HaveTHExt(s_0_0)
        let s_0_1: bool = HaveTHExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b2 b1
        if s_0_2 {
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
        // D s_1_0: read-var Rt:u8
        let s_1_0: u8 = fn_state.Rt;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: read-var Rn:u8
        let s_1_4: u8 = fn_state.Rn;
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 5u16);
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (s_1_5.value() as i128);
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var Rs:u8
        let s_1_8: u8 = fn_state.Rs;
        // D s_1_9: cast zx s_1_8 -> bv
        let s_1_9: Bits = Bits::new(s_1_8 as u128, 5u16);
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (s_1_9.value() as i128);
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // C s_1_12: const #0u : u8
        let s_1_12: bool = false;
        // D s_1_13: read-var A:u8
        let s_1_13: bool = fn_state.A;
        // D s_1_14: cast zx s_1_13 -> bv
        let s_1_14: Bits = Bits::new(s_1_13 as u128, 1u16);
        // C s_1_15: const #1u : u8
        let s_1_15: bool = true;
        // C s_1_16: cast zx s_1_15 -> bv
        let s_1_16: Bits = Bits::new(s_1_15 as u128, 1u16);
        // D s_1_17: cmp-eq s_1_14 s_1_16
        let s_1_17: bool = ((s_1_14) == (s_1_16));
        // D s_1_18: read-var R:u8
        let s_1_18: bool = fn_state.R;
        // D s_1_19: cast zx s_1_18 -> bv
        let s_1_19: Bits = Bits::new(s_1_18 as u128, 1u16);
        // C s_1_20: const #1u : u8
        let s_1_20: bool = true;
        // C s_1_21: cast zx s_1_20 -> bv
        let s_1_21: Bits = Bits::new(s_1_20 as u128, 1u16);
        // D s_1_22: cmp-eq s_1_19 s_1_21
        let s_1_22: bool = ((s_1_19) == (s_1_21));
        // C s_1_23: const #31s : i
        let s_1_23: i128 = 31;
        // D s_1_24: cast zx s_1_7 -> i
        let s_1_24: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_25: call neq_int(s_1_24, s_1_23)
        let s_1_25: bool = neq_int(state, tracer, s_1_24, s_1_23);
        // D s_1_26: call execute_aarch64_instrs_memory_rcw_cas(s_1_17, s_1_7, s_1_22, s_1_11, s_1_12, s_1_3, s_1_25)
        let s_1_26: () = execute_aarch64_instrs_memory_rcw_cas(
            state,
            tracer,
            s_1_17,
            s_1_7,
            s_1_22,
            s_1_11,
            s_1_12,
            s_1_3,
            s_1_25,
        );
        // N s_1_27: return
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
