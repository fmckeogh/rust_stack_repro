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
use execute_aarch32_instrs_STC_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_STC_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    P: bool,
    U: bool,
    W: bool,
    Rn: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        add: bool,
        gs_302125: bool,
        imm32: u32,
        gs_302126: bool,
        wback: bool,
        gs_302114: bool,
        n: i64,
        index: bool,
        gs_302115: bool,
        P: bool,
        U: bool,
        W: bool,
        Rn: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        P,
        U,
        W,
        Rn,
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
        // D s_3_1: write-var gs#302114 <= s_3_0
        fn_state.gs_302114 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#302114:u8
        let s_4_0: bool = fn_state.gs_302114;
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
        // D s_5_1: write-var gs#302115 <= s_5_0
        fn_state.gs_302115 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#302115:u8
        let s_6_0: bool = fn_state.gs_302115;
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
        // D s_7_0: read-var Rn:u8
        let s_7_0: u8 = fn_state.Rn;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 4u16);
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (s_7_1.value() as i128);
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // D s_7_4: write-var n <= s_7_3
        fn_state.n = s_7_3;
        // C s_7_5: const #32s : i
        let s_7_5: i128 = 32;
        // C s_7_6: const #0s : i
        let s_7_6: i128 = 0;
        // C s_7_7: const #8s : i
        let s_7_7: i128 = 8;
        // C s_7_8: const #2s : i
        let s_7_8: i128 = 2;
        // D s_7_9: read-var imm8:u8
        let s_7_9: u8 = fn_state.imm8;
        // D s_7_10: cast zx s_7_9 -> bv
        let s_7_10: Bits = Bits::new(s_7_9 as u128, 8u16);
        // D s_7_11: call place_slice(s_7_5, s_7_10, s_7_6, s_7_7, s_7_8)
        let s_7_11: Bits = place_slice(
            state,
            tracer,
            s_7_5,
            s_7_10,
            s_7_6,
            s_7_7,
            s_7_8,
        );
        // D s_7_12: cast reint s_7_11 -> u32
        let s_7_12: u32 = (s_7_11.value() as u32);
        // D s_7_13: write-var imm32 <= s_7_12
        fn_state.imm32 = s_7_12;
        // D s_7_14: read-var P:u8
        let s_7_14: bool = fn_state.P;
        // D s_7_15: cast zx s_7_14 -> bv
        let s_7_15: Bits = Bits::new(s_7_14 as u128, 1u16);
        // C s_7_16: const #1u : u8
        let s_7_16: bool = true;
        // C s_7_17: cast zx s_7_16 -> bv
        let s_7_17: Bits = Bits::new(s_7_16 as u128, 1u16);
        // D s_7_18: cmp-eq s_7_15 s_7_17
        let s_7_18: bool = ((s_7_15) == (s_7_17));
        // D s_7_19: write-var index <= s_7_18
        fn_state.index = s_7_18;
        // D s_7_20: read-var U:u8
        let s_7_20: bool = fn_state.U;
        // D s_7_21: cast zx s_7_20 -> bv
        let s_7_21: Bits = Bits::new(s_7_20 as u128, 1u16);
        // C s_7_22: const #1u : u8
        let s_7_22: bool = true;
        // C s_7_23: cast zx s_7_22 -> bv
        let s_7_23: Bits = Bits::new(s_7_22 as u128, 1u16);
        // D s_7_24: cmp-eq s_7_21 s_7_23
        let s_7_24: bool = ((s_7_21) == (s_7_23));
        // D s_7_25: write-var add <= s_7_24
        fn_state.add = s_7_24;
        // D s_7_26: read-var W:u8
        let s_7_26: bool = fn_state.W;
        // D s_7_27: cast zx s_7_26 -> bv
        let s_7_27: Bits = Bits::new(s_7_26 as u128, 1u16);
        // C s_7_28: const #1u : u8
        let s_7_28: bool = true;
        // C s_7_29: cast zx s_7_28 -> bv
        let s_7_29: Bits = Bits::new(s_7_28 as u128, 1u16);
        // D s_7_30: cmp-eq s_7_27 s_7_29
        let s_7_30: bool = ((s_7_27) == (s_7_29));
        // D s_7_31: write-var wback <= s_7_30
        fn_state.wback = s_7_30;
        // C s_7_32: const #15s : i
        let s_7_32: i128 = 15;
        // D s_7_33: read-var n:i64
        let s_7_33: i64 = fn_state.n;
        // D s_7_34: cast zx s_7_33 -> i
        let s_7_34: i128 = (i128::try_from(s_7_33).unwrap());
        // D s_7_35: cmp-eq s_7_34 s_7_32
        let s_7_35: bool = ((s_7_34) == (s_7_32));
        // N s_7_36: branch s_7_35 b12 b8
        if s_7_35 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#302126 <= s_8_0
        fn_state.gs_302126 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#302126:u8
        let s_9_0: bool = fn_state.gs_302126;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var add:u8
        let s_10_0: bool = fn_state.add;
        // C s_10_1: const #14s : i64
        let s_10_1: i64 = 14;
        // D s_10_2: read-var imm32:u32
        let s_10_2: u32 = fn_state.imm32;
        // D s_10_3: read-var index:u8
        let s_10_3: bool = fn_state.index;
        // D s_10_4: read-var n:i64
        let s_10_4: i64 = fn_state.n;
        // D s_10_5: read-var wback:u8
        let s_10_5: bool = fn_state.wback;
        // D s_10_6: call execute_aarch32_instrs_STC_Op_A_txt(s_10_0, s_10_1, s_10_2, s_10_3, s_10_4, s_10_5)
        let s_10_6: () = execute_aarch32_instrs_STC_Op_A_txt(
            state,
            tracer,
            s_10_0,
            s_10_1,
            s_10_2,
            s_10_3,
            s_10_4,
            s_10_5,
        );
        // N s_10_7: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var wback:u8
        let s_12_0: bool = fn_state.wback;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call CurrentInstrSet(s_13_0)
        let s_13_1: u32 = CurrentInstrSet(state, tracer, s_13_0);
        // C s_13_2: const #1u : u32
        let s_13_2: u32 = 1;
        // S s_13_3: cmp-eq s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) == (s_13_2));
        // D s_13_4: write-var gs#302125 <= s_13_3
        fn_state.gs_302125 = s_13_3;
        // N s_13_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#302125:u8
        let s_14_0: bool = fn_state.gs_302125;
        // D s_14_1: write-var gs#302126 <= s_14_0
        fn_state.gs_302126 = s_14_0;
        // N s_14_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#302125 <= s_15_0
        fn_state.gs_302125 = s_15_0;
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
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
        // D s_17_5: write-var gs#302115 <= s_17_4
        fn_state.gs_302115 = s_17_4;
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
        // D s_18_5: write-var gs#302114 <= s_18_4
        fn_state.gs_302114 = s_18_4;
        // N s_18_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
