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
use HaveLSE128::*;
use execute_aarch64_instrs_memory_atomicops_swp_128::*;
use common::*;
pub fn decode_swpp_aarch64_instrs_memory_atomicops_swp_128<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    opc: u8,
    o3: bool,
    Rt2: u8,
    R: bool,
    A: bool,
    S: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rt: u8,
        Rn: u8,
        opc: u8,
        o3: bool,
        Rt2: u8,
        R: bool,
        A: bool,
        S: bool,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        opc,
        o3,
        Rt2,
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
        // S s_0_1: call HaveLSE128(s_0_0)
        let s_0_1: bool = HaveLSE128(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b6 b1
        if s_0_2 {
            return block_6(state, tracer, fn_state);
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
        // C s_1_2: const #31u : u8
        let s_1_2: u8 = 31;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 5u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b5 b2
        if s_1_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Rt2:u8
        let s_2_0: u8 = fn_state.Rt2;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 5u16);
        // C s_2_2: const #31u : u8
        let s_2_2: u8 = 31;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 5u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b4 b3
        if s_2_4 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rt:u8
        let s_3_0: u8 = fn_state.Rt;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 5u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: const #31s : i
        let s_3_4: i128 = 31;
        // D s_3_5: cast zx s_3_3 -> i
        let s_3_5: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_6: call neq_int(s_3_5, s_3_4)
        let s_3_6: bool = neq_int(state, tracer, s_3_5, s_3_4);
        // N s_3_7: assert s_3_6
        let s_3_7: () = assert!(s_3_6);
        // D s_3_8: read-var Rt2:u8
        let s_3_8: u8 = fn_state.Rt2;
        // D s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 5u16);
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (s_3_9.value() as i128);
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: read-var Rn:u8
        let s_3_12: u8 = fn_state.Rn;
        // D s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 5u16);
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (s_3_13.value() as i128);
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: read-var A:u8
        let s_3_16: bool = fn_state.A;
        // D s_3_17: cast zx s_3_16 -> bv
        let s_3_17: Bits = Bits::new(s_3_16 as u128, 1u16);
        // C s_3_18: const #1u : u8
        let s_3_18: bool = true;
        // C s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 1u16);
        // D s_3_20: cmp-eq s_3_17 s_3_19
        let s_3_20: bool = ((s_3_17) == (s_3_19));
        // D s_3_21: read-var R:u8
        let s_3_21: bool = fn_state.R;
        // D s_3_22: cast zx s_3_21 -> bv
        let s_3_22: Bits = Bits::new(s_3_21 as u128, 1u16);
        // C s_3_23: const #1u : u8
        let s_3_23: bool = true;
        // C s_3_24: cast zx s_3_23 -> bv
        let s_3_24: Bits = Bits::new(s_3_23 as u128, 1u16);
        // D s_3_25: cmp-eq s_3_22 s_3_24
        let s_3_25: bool = ((s_3_22) == (s_3_24));
        // C s_3_26: const #31s : i
        let s_3_26: i128 = 31;
        // D s_3_27: cast zx s_3_15 -> i
        let s_3_27: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_28: call neq_int(s_3_27, s_3_26)
        let s_3_28: bool = neq_int(state, tracer, s_3_27, s_3_26);
        // D s_3_29: call execute_aarch64_instrs_memory_atomicops_swp_128(s_3_20, s_3_15, s_3_25, s_3_3, s_3_11, s_3_28)
        let s_3_29: () = execute_aarch64_instrs_memory_atomicops_swp_128(
            state,
            tracer,
            s_3_20,
            s_3_15,
            s_3_25,
            s_3_3,
            s_3_11,
            s_3_28,
        );
        // N s_3_30: return
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
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
}
