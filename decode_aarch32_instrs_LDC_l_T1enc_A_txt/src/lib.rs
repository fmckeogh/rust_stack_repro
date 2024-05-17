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
use CurrentInstrSet::*;
use ConditionPassed::*;
use place_slice::*;
use execute_aarch32_instrs_LDC_l_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDC_l_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    P: bool,
    U: bool,
    W: bool,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
        gs_296792: bool,
        gs_296782: bool,
        gs_296791: bool,
        index: bool,
        gs_296783: bool,
        add: bool,
        P: bool,
        U: bool,
        W: bool,
        imm8: u8,
    }
    let fn_state = FunctionState {
        P,
        U,
        W,
        imm8,
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
        // D s_2_0: read-var P:u8
        let s_2_0: bool = fn_state.P;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b18 b3
        if s_2_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#296782 <= s_3_0
        fn_state.gs_296782 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#296782:u8
        let s_4_0: bool = fn_state.gs_296782;
        // N s_4_1: branch s_4_0 b17 b5
        if s_4_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#296783 <= s_5_0
        fn_state.gs_296783 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#296783:u8
        let s_6_0: bool = fn_state.gs_296783;
        // N s_6_1: branch s_6_0 b16 b7
        if s_6_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var P:u8
        let s_7_0: bool = fn_state.P;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: write-var index <= s_7_4
        fn_state.index = s_7_4;
        // D s_7_6: read-var U:u8
        let s_7_6: bool = fn_state.U;
        // D s_7_7: cast zx s_7_6 -> bv
        let s_7_7: Bits = Bits::new(s_7_6 as u128, 1u16);
        // C s_7_8: const #1u : u8
        let s_7_8: bool = true;
        // C s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 1u16);
        // D s_7_10: cmp-eq s_7_7 s_7_9
        let s_7_10: bool = ((s_7_7) == (s_7_9));
        // D s_7_11: write-var add <= s_7_10
        fn_state.add = s_7_10;
        // C s_7_12: const #32s : i
        let s_7_12: i128 = 32;
        // C s_7_13: const #0s : i
        let s_7_13: i128 = 0;
        // C s_7_14: const #8s : i
        let s_7_14: i128 = 8;
        // C s_7_15: const #2s : i
        let s_7_15: i128 = 2;
        // D s_7_16: read-var imm8:u8
        let s_7_16: u8 = fn_state.imm8;
        // D s_7_17: cast zx s_7_16 -> bv
        let s_7_17: Bits = Bits::new(s_7_16 as u128, 8u16);
        // D s_7_18: call place_slice(s_7_12, s_7_17, s_7_13, s_7_14, s_7_15)
        let s_7_18: Bits = place_slice(
            state,
            tracer,
            s_7_12,
            s_7_17,
            s_7_13,
            s_7_14,
            s_7_15,
        );
        // D s_7_19: cast reint s_7_18 -> u32
        let s_7_19: u32 = (s_7_18.value() as u32);
        // D s_7_20: write-var imm32 <= s_7_19
        fn_state.imm32 = s_7_19;
        // D s_7_21: read-var W:u8
        let s_7_21: bool = fn_state.W;
        // D s_7_22: cast zx s_7_21 -> bv
        let s_7_22: Bits = Bits::new(s_7_21 as u128, 1u16);
        // C s_7_23: const #1u : u8
        let s_7_23: bool = true;
        // C s_7_24: cast zx s_7_23 -> bv
        let s_7_24: Bits = Bits::new(s_7_23 as u128, 1u16);
        // D s_7_25: cmp-eq s_7_22 s_7_24
        let s_7_25: bool = ((s_7_22) == (s_7_24));
        // N s_7_26: branch s_7_25 b15 b8
        if s_7_25 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var P:u8
        let s_8_0: bool = fn_state.P;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #0u : u8
        let s_8_2: bool = false;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b14 b9
        if s_8_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#296791 <= s_9_0
        fn_state.gs_296791 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#296791:u8
        let s_10_0: bool = fn_state.gs_296791;
        // D s_10_1: write-var gs#296792 <= s_10_0
        fn_state.gs_296792 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#296792:u8
        let s_11_0: bool = fn_state.gs_296792;
        // N s_11_1: branch s_11_0 b13 b12
        if s_11_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var add:u8
        let s_12_0: bool = fn_state.add;
        // C s_12_1: const #14s : i64
        let s_12_1: i64 = 14;
        // D s_12_2: read-var imm32:u32
        let s_12_2: u32 = fn_state.imm32;
        // D s_12_3: read-var index:u8
        let s_12_3: bool = fn_state.index;
        // D s_12_4: call execute_aarch32_instrs_LDC_l_Op_A_txt(s_12_0, s_12_1, s_12_2, s_12_3)
        let s_12_4: () = execute_aarch32_instrs_LDC_l_Op_A_txt(
            state,
            tracer,
            s_12_0,
            s_12_1,
            s_12_2,
            s_12_3,
        );
        // N s_12_5: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call CurrentInstrSet(s_14_0)
        let s_14_1: u32 = CurrentInstrSet(state, tracer, s_14_0);
        // C s_14_2: const #1u : u32
        let s_14_2: u32 = 1;
        // S s_14_3: cmp-eq s_14_1 s_14_2
        let s_14_3: bool = ((s_14_1) == (s_14_2));
        // D s_14_4: write-var gs#296791 <= s_14_3
        fn_state.gs_296791 = s_14_3;
        // N s_14_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#296792 <= s_15_0
        fn_state.gs_296792 = s_15_0;
        // N s_15_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var W:u8
        let s_17_0: bool = fn_state.W;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 1u16);
        // C s_17_2: const #0u : u8
        let s_17_2: bool = false;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: write-var gs#296783 <= s_17_4
        fn_state.gs_296783 = s_17_4;
        // N s_17_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var U:u8
        let s_18_0: bool = fn_state.U;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #0u : u8
        let s_18_2: bool = false;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: write-var gs#296782 <= s_18_4
        fn_state.gs_296782 = s_18_4;
        // N s_18_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
