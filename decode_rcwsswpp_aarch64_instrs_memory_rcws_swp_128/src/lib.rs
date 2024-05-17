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
use HaveTHExt::*;
use Have128BitDescriptorExt::*;
use execute_aarch64_instrs_memory_rcws_swp_128::*;
use common::*;
pub fn decode_rcwsswpp_aarch64_instrs_memory_rcws_swp_128<T: Tracer>(
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
        gs_166453: bool,
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
        // S s_0_1: call Have128BitDescriptorExt(s_0_0)
        let s_0_1: bool = Have128BitDescriptorExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b9 b1
        if s_0_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveTHExt(s_1_0)
        let s_1_1: bool = HaveTHExt(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // D s_1_3: write-var gs#166453 <= s_1_2
        fn_state.gs_166453 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#166453:u8
        let s_2_0: bool = fn_state.gs_166453;
        // N s_2_1: branch s_2_0 b8 b3
        if s_2_0 {
            return block_8(state, tracer, fn_state);
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
        // C s_3_2: const #31u : u8
        let s_3_2: u8 = 31;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 5u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b7 b4
        if s_3_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Rt2:u8
        let s_4_0: u8 = fn_state.Rt2;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 5u16);
        // C s_4_2: const #31u : u8
        let s_4_2: u8 = 31;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 5u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b6 b5
        if s_4_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Rt:u8
        let s_5_0: u8 = fn_state.Rt;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 5u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: const #31s : i
        let s_5_4: i128 = 31;
        // D s_5_5: cast zx s_5_3 -> i
        let s_5_5: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_6: call neq_int(s_5_5, s_5_4)
        let s_5_6: bool = neq_int(state, tracer, s_5_5, s_5_4);
        // N s_5_7: assert s_5_6
        let s_5_7: () = assert!(s_5_6);
        // D s_5_8: read-var Rt2:u8
        let s_5_8: u8 = fn_state.Rt2;
        // D s_5_9: cast zx s_5_8 -> bv
        let s_5_9: Bits = Bits::new(s_5_8 as u128, 5u16);
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (s_5_9.value() as i128);
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // D s_5_12: read-var Rn:u8
        let s_5_12: u8 = fn_state.Rn;
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 5u16);
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (s_5_13.value() as i128);
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // C s_5_16: const #1u : u8
        let s_5_16: bool = true;
        // D s_5_17: read-var A:u8
        let s_5_17: bool = fn_state.A;
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 1u16);
        // C s_5_19: const #1u : u8
        let s_5_19: bool = true;
        // C s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 1u16);
        // D s_5_21: cmp-eq s_5_18 s_5_20
        let s_5_21: bool = ((s_5_18) == (s_5_20));
        // D s_5_22: read-var R:u8
        let s_5_22: bool = fn_state.R;
        // D s_5_23: cast zx s_5_22 -> bv
        let s_5_23: Bits = Bits::new(s_5_22 as u128, 1u16);
        // C s_5_24: const #1u : u8
        let s_5_24: bool = true;
        // C s_5_25: cast zx s_5_24 -> bv
        let s_5_25: Bits = Bits::new(s_5_24 as u128, 1u16);
        // D s_5_26: cmp-eq s_5_23 s_5_25
        let s_5_26: bool = ((s_5_23) == (s_5_25));
        // C s_5_27: const #31s : i
        let s_5_27: i128 = 31;
        // D s_5_28: cast zx s_5_15 -> i
        let s_5_28: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_29: call neq_int(s_5_28, s_5_27)
        let s_5_29: bool = neq_int(state, tracer, s_5_28, s_5_27);
        // D s_5_30: call execute_aarch64_instrs_memory_rcws_swp_128(s_5_21, s_5_15, s_5_26, s_5_16, s_5_3, s_5_11, s_5_29)
        let s_5_30: () = execute_aarch64_instrs_memory_rcws_swp_128(
            state,
            tracer,
            s_5_21,
            s_5_15,
            s_5_26,
            s_5_16,
            s_5_3,
            s_5_11,
            s_5_29,
        );
        // N s_5_31: return
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
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#166453 <= s_9_0
        fn_state.gs_166453 = s_9_0;
        // N s_9_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
