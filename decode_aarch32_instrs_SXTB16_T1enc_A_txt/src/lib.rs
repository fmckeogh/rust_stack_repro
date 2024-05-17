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
use ConditionPassed::*;
use execute_aarch32_instrs_SXTB16_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_SXTB16_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    rotate: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        rotation: i64,
        gs_303553: bool,
        d: i64,
        Rd: u8,
        rotate: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        Rd,
        rotate,
        Rm,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
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
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Rd:u8
        let s_2_0: u8 = fn_state.Rd;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var d <= s_2_3
        fn_state.d = s_2_3;
        // D s_2_5: read-var Rm:u8
        let s_2_5: u8 = fn_state.Rm;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 4u16);
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (s_2_6.value() as i128);
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // D s_2_9: write-var m <= s_2_8
        fn_state.m = s_2_8;
        // D s_2_10: read-var rotate:u8
        let s_2_10: u8 = fn_state.rotate;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 2u16);
        // C s_2_12: const #0u : u8
        let s_2_12: u8 = 0;
        // C s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 3u16);
        // D s_2_14: cast reint s_2_11 -> u128
        let s_2_14: u128 = (s_2_11.value() as u128);
        // D s_2_15: size-of s_2_11
        let s_2_15: u16 = s_2_11.length();
        // C s_2_16: cast reint s_2_13 -> u128
        let s_2_16: u128 = (s_2_13.value() as u128);
        // D s_2_17: size-of s_2_13
        let s_2_17: u16 = s_2_13.length();
        // D s_2_18: lsl s_2_14 s_2_17
        let s_2_18: u128 = s_2_14 << s_2_17;
        // D s_2_19: or s_2_18 s_2_16
        let s_2_19: u128 = ((s_2_18) | (s_2_16));
        // D s_2_20: add s_2_15 s_2_17
        let s_2_20: u16 = (s_2_15 + s_2_17);
        // D s_2_21: create-bits s_2_19 s_2_20
        let s_2_21: Bits = Bits::new(s_2_19, s_2_20);
        // D s_2_22: cast reint s_2_21 -> u8
        let s_2_22: u8 = (s_2_21.value() as u8);
        // D s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 5u16);
        // D s_2_24: cast zx s_2_23 -> i
        let s_2_24: i128 = (s_2_23.value() as i128);
        // D s_2_25: cast reint s_2_24 -> i64
        let s_2_25: i64 = (s_2_24 as i64);
        // D s_2_26: write-var rotation <= s_2_25
        fn_state.rotation = s_2_25;
        // C s_2_27: const #15s : i
        let s_2_27: i128 = 15;
        // D s_2_28: read-var d:i64
        let s_2_28: i64 = fn_state.d;
        // D s_2_29: cast zx s_2_28 -> i
        let s_2_29: i128 = (i128::try_from(s_2_28).unwrap());
        // D s_2_30: cmp-eq s_2_29 s_2_27
        let s_2_30: bool = ((s_2_29) == (s_2_27));
        // N s_2_31: branch s_2_30 b7 b3
        if s_2_30 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #15s : i
        let s_3_0: i128 = 15;
        // D s_3_1: read-var m:i64
        let s_3_1: i64 = fn_state.m;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // D s_3_4: write-var gs#303553 <= s_3_3
        fn_state.gs_303553 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#303553:u8
        let s_4_0: bool = fn_state.gs_303553;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
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
        // D s_5_0: read-var d:i64
        let s_5_0: i64 = fn_state.d;
        // D s_5_1: read-var m:i64
        let s_5_1: i64 = fn_state.m;
        // D s_5_2: read-var rotation:i64
        let s_5_2: i64 = fn_state.rotation;
        // D s_5_3: call execute_aarch32_instrs_SXTB16_Op_A_txt(s_5_0, s_5_1, s_5_2)
        let s_5_3: () = execute_aarch32_instrs_SXTB16_Op_A_txt(
            state,
            tracer,
            s_5_0,
            s_5_1,
            s_5_2,
        );
        // N s_5_4: return
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
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#303553 <= s_7_0
        fn_state.gs_303553 = s_7_0;
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
