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
use execute_aarch32_instrs_LDM_Op_A_txt::*;
use LastInITBlock::*;
use ConditionPassed::*;
use BitCount::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_LDM_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    W: bool,
    Rn: u8,
    P: bool,
    M: bool,
    register_list: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_296842: bool,
        gs_296845: bool,
        gs_296841: bool,
        gs_296851: bool,
        registers: u16,
        n: i64,
        gs_296843: bool,
        wback: bool,
        gs_296850: bool,
        W: bool,
        Rn: u8,
        P: bool,
        M: bool,
        register_list: u16,
    }
    let fn_state = FunctionState {
        W,
        Rn,
        P,
        M,
        register_list,
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
        // D s_2_0: read-var Rn:u8
        let s_2_0: u8 = fn_state.Rn;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var n <= s_2_3
        fn_state.n = s_2_3;
        // D s_2_5: read-var P:u8
        let s_2_5: bool = fn_state.P;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 1u16);
        // D s_2_7: read-var M:u8
        let s_2_7: bool = fn_state.M;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 1u16);
        // D s_2_9: cast reint s_2_6 -> u128
        let s_2_9: u128 = (s_2_6.value() as u128);
        // D s_2_10: size-of s_2_6
        let s_2_10: u16 = s_2_6.length();
        // D s_2_11: cast reint s_2_8 -> u128
        let s_2_11: u128 = (s_2_8.value() as u128);
        // D s_2_12: size-of s_2_8
        let s_2_12: u16 = s_2_8.length();
        // D s_2_13: lsl s_2_9 s_2_12
        let s_2_13: u128 = s_2_9 << s_2_12;
        // D s_2_14: or s_2_13 s_2_11
        let s_2_14: u128 = ((s_2_13) | (s_2_11));
        // D s_2_15: add s_2_10 s_2_12
        let s_2_15: u16 = (s_2_10 + s_2_12);
        // D s_2_16: create-bits s_2_14 s_2_15
        let s_2_16: Bits = Bits::new(s_2_14, s_2_15);
        // D s_2_17: cast reint s_2_16 -> u8
        let s_2_17: u8 = (s_2_16.value() as u8);
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 2u16);
        // D s_2_19: read-var register_list:u14
        let s_2_19: u16 = fn_state.register_list;
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 14u16);
        // D s_2_21: cast reint s_2_18 -> u128
        let s_2_21: u128 = (s_2_18.value() as u128);
        // D s_2_22: size-of s_2_18
        let s_2_22: u16 = s_2_18.length();
        // D s_2_23: cast reint s_2_20 -> u128
        let s_2_23: u128 = (s_2_20.value() as u128);
        // D s_2_24: size-of s_2_20
        let s_2_24: u16 = s_2_20.length();
        // D s_2_25: lsl s_2_21 s_2_24
        let s_2_25: u128 = s_2_21 << s_2_24;
        // D s_2_26: or s_2_25 s_2_23
        let s_2_26: u128 = ((s_2_25) | (s_2_23));
        // D s_2_27: add s_2_22 s_2_24
        let s_2_27: u16 = (s_2_22 + s_2_24);
        // D s_2_28: create-bits s_2_26 s_2_27
        let s_2_28: Bits = Bits::new(s_2_26, s_2_27);
        // D s_2_29: cast reint s_2_28 -> u16
        let s_2_29: u16 = (s_2_28.value() as u16);
        // D s_2_30: write-var registers <= s_2_29
        fn_state.registers = s_2_29;
        // D s_2_31: read-var W:u8
        let s_2_31: bool = fn_state.W;
        // D s_2_32: cast zx s_2_31 -> bv
        let s_2_32: Bits = Bits::new(s_2_31 as u128, 1u16);
        // C s_2_33: const #1u : u8
        let s_2_33: bool = true;
        // C s_2_34: cast zx s_2_33 -> bv
        let s_2_34: Bits = Bits::new(s_2_33 as u128, 1u16);
        // D s_2_35: cmp-eq s_2_32 s_2_34
        let s_2_35: bool = ((s_2_32) == (s_2_34));
        // D s_2_36: write-var wback <= s_2_35
        fn_state.wback = s_2_35;
        // C s_2_37: const #15s : i
        let s_2_37: i128 = 15;
        // D s_2_38: read-var n:i64
        let s_2_38: i64 = fn_state.n;
        // D s_2_39: cast zx s_2_38 -> i
        let s_2_39: i128 = (i128::try_from(s_2_38).unwrap());
        // D s_2_40: cmp-eq s_2_39 s_2_37
        let s_2_40: bool = ((s_2_39) == (s_2_37));
        // N s_2_41: branch s_2_40 b28 b3
        if s_2_40 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var registers:u16
        let s_3_0: u16 = fn_state.registers;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 16u16);
        // D s_3_2: call BitCount(s_3_1)
        let s_3_2: i128 = BitCount(state, tracer, s_3_1);
        // C s_3_3: const #2s : i
        let s_3_3: i128 = 2;
        // D s_3_4: cmp-lt s_3_2 s_3_3
        let s_3_4: bool = ((s_3_2) < (s_3_3));
        // D s_3_5: write-var gs#296841 <= s_3_4
        fn_state.gs_296841 = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#296841:u8
        let s_4_0: bool = fn_state.gs_296841;
        // N s_4_1: branch s_4_0 b27 b5
        if s_4_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var P:u8
        let s_5_0: bool = fn_state.P;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #1u : u8
        let s_5_2: bool = true;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b26 b6
        if s_5_4 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#296842 <= s_6_0
        fn_state.gs_296842 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#296842:u8
        let s_7_0: bool = fn_state.gs_296842;
        // D s_7_1: write-var gs#296843 <= s_7_0
        fn_state.gs_296843 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#296843:u8
        let s_8_0: bool = fn_state.gs_296843;
        // N s_8_1: branch s_8_0 b25 b9
        if s_8_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var wback:u8
        let s_9_0: bool = fn_state.wback;
        // N s_9_1: branch s_9_0 b24 b10
        if s_9_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#296845 <= s_10_0
        fn_state.gs_296845 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#296845:u8
        let s_11_0: bool = fn_state.gs_296845;
        // N s_11_1: branch s_11_0 b23 b12
        if s_11_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #13s : i
        let s_12_0: i128 = 13;
        // D s_12_1: read-var registers:u16
        let s_12_1: u16 = fn_state.registers;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 16u16);
        // C s_12_3: const #1u : u64
        let s_12_3: u64 = 1;
        // D s_12_4: bit-extract s_12_2 s_12_0 s_12_3
        let s_12_4: Bits = (Bits::new(
            ((s_12_2) >> (s_12_0)).value(),
            u16::try_from(s_12_3).unwrap(),
        ));
        // D s_12_5: cast reint s_12_4 -> u8
        let s_12_5: bool = ((s_12_4.value()) != 0);
        // C s_12_6: const #0s : i
        let s_12_6: i128 = 0;
        // C s_12_7: const #0u : u64
        let s_12_7: u64 = 0;
        // D s_12_8: cast zx s_12_5 -> u64
        let s_12_8: u64 = (s_12_5 as u64);
        // C s_12_9: const #1u : u64
        let s_12_9: u64 = 1;
        // D s_12_10: and s_12_8 s_12_9
        let s_12_10: u64 = ((s_12_8) & (s_12_9));
        // D s_12_11: cmp-eq s_12_10 s_12_9
        let s_12_11: bool = ((s_12_10) == (s_12_9));
        // D s_12_12: lsl s_12_8 s_12_6
        let s_12_12: u64 = s_12_8 << s_12_6;
        // D s_12_13: or s_12_7 s_12_12
        let s_12_13: u64 = ((s_12_7) | (s_12_12));
        // D s_12_14: cmpl s_12_12
        let s_12_14: u64 = !s_12_12;
        // D s_12_15: and s_12_7 s_12_14
        let s_12_15: u64 = ((s_12_7) & (s_12_14));
        // D s_12_16: select s_12_11 s_12_13 s_12_15
        let s_12_16: u64 = if s_12_11 { s_12_13 } else { s_12_15 };
        // D s_12_17: cast trunc s_12_16 -> u8
        let s_12_17: bool = ((s_12_16) != 0);
        // D s_12_18: cast zx s_12_17 -> bv
        let s_12_18: Bits = Bits::new(s_12_17 as u128, 1u16);
        // C s_12_19: const #1u : u8
        let s_12_19: bool = true;
        // C s_12_20: cast zx s_12_19 -> bv
        let s_12_20: Bits = Bits::new(s_12_19 as u128, 1u16);
        // D s_12_21: cmp-eq s_12_18 s_12_20
        let s_12_21: bool = ((s_12_18) == (s_12_20));
        // N s_12_22: branch s_12_21 b22 b13
        if s_12_21 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #15s : i
        let s_13_0: i128 = 15;
        // D s_13_1: read-var registers:u16
        let s_13_1: u16 = fn_state.registers;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 16u16);
        // C s_13_3: const #1u : u64
        let s_13_3: u64 = 1;
        // D s_13_4: bit-extract s_13_2 s_13_0 s_13_3
        let s_13_4: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_3).unwrap(),
        ));
        // D s_13_5: cast reint s_13_4 -> u8
        let s_13_5: bool = ((s_13_4.value()) != 0);
        // C s_13_6: const #0s : i
        let s_13_6: i128 = 0;
        // C s_13_7: const #0u : u64
        let s_13_7: u64 = 0;
        // D s_13_8: cast zx s_13_5 -> u64
        let s_13_8: u64 = (s_13_5 as u64);
        // C s_13_9: const #1u : u64
        let s_13_9: u64 = 1;
        // D s_13_10: and s_13_8 s_13_9
        let s_13_10: u64 = ((s_13_8) & (s_13_9));
        // D s_13_11: cmp-eq s_13_10 s_13_9
        let s_13_11: bool = ((s_13_10) == (s_13_9));
        // D s_13_12: lsl s_13_8 s_13_6
        let s_13_12: u64 = s_13_8 << s_13_6;
        // D s_13_13: or s_13_7 s_13_12
        let s_13_13: u64 = ((s_13_7) | (s_13_12));
        // D s_13_14: cmpl s_13_12
        let s_13_14: u64 = !s_13_12;
        // D s_13_15: and s_13_7 s_13_14
        let s_13_15: u64 = ((s_13_7) & (s_13_14));
        // D s_13_16: select s_13_11 s_13_13 s_13_15
        let s_13_16: u64 = if s_13_11 { s_13_13 } else { s_13_15 };
        // D s_13_17: cast trunc s_13_16 -> u8
        let s_13_17: bool = ((s_13_16) != 0);
        // D s_13_18: cast zx s_13_17 -> bv
        let s_13_18: Bits = Bits::new(s_13_17 as u128, 1u16);
        // C s_13_19: const #1u : u8
        let s_13_19: bool = true;
        // C s_13_20: cast zx s_13_19 -> bv
        let s_13_20: Bits = Bits::new(s_13_19 as u128, 1u16);
        // D s_13_21: cmp-eq s_13_18 s_13_20
        let s_13_21: bool = ((s_13_18) == (s_13_20));
        // N s_13_22: branch s_13_21 b21 b14
        if s_13_21 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#296850 <= s_14_0
        fn_state.gs_296850 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#296850:u8
        let s_15_0: bool = fn_state.gs_296850;
        // N s_15_1: branch s_15_0 b20 b16
        if s_15_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#296851 <= s_16_0
        fn_state.gs_296851 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#296851:u8
        let s_17_0: bool = fn_state.gs_296851;
        // N s_17_1: branch s_17_0 b19 b18
        if s_17_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var n:i64
        let s_18_0: i64 = fn_state.n;
        // D s_18_1: read-var registers:u16
        let s_18_1: u16 = fn_state.registers;
        // D s_18_2: read-var wback:u8
        let s_18_2: bool = fn_state.wback;
        // D s_18_3: call execute_aarch32_instrs_LDM_Op_A_txt(s_18_0, s_18_1, s_18_2)
        let s_18_3: () = execute_aarch32_instrs_LDM_Op_A_txt(
            state,
            tracer,
            s_18_0,
            s_18_1,
            s_18_2,
        );
        // N s_18_4: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call LastInITBlock(s_20_0)
        let s_20_1: bool = LastInITBlock(state, tracer, s_20_0);
        // S s_20_2: not s_20_1
        let s_20_2: bool = !s_20_1;
        // D s_20_3: write-var gs#296851 <= s_20_2
        fn_state.gs_296851 = s_20_2;
        // N s_20_4: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call InITBlock(s_21_0)
        let s_21_1: bool = InITBlock(state, tracer, s_21_0);
        // D s_21_2: write-var gs#296850 <= s_21_1
        fn_state.gs_296850 = s_21_1;
        // N s_21_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var registers:u16
        let s_24_0: u16 = fn_state.registers;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 16u16);
        // D s_24_2: read-var n:i64
        let s_24_2: i64 = fn_state.n;
        // D s_24_3: cast zx s_24_2 -> i
        let s_24_3: i128 = (i128::try_from(s_24_2).unwrap());
        // C s_24_4: const #1u : u64
        let s_24_4: u64 = 1;
        // D s_24_5: bit-extract s_24_1 s_24_3 s_24_4
        let s_24_5: Bits = (Bits::new(
            ((s_24_1) >> (s_24_3)).value(),
            u16::try_from(s_24_4).unwrap(),
        ));
        // D s_24_6: cast reint s_24_5 -> u8
        let s_24_6: bool = ((s_24_5.value()) != 0);
        // C s_24_7: const #0s : i
        let s_24_7: i128 = 0;
        // C s_24_8: const #0u : u64
        let s_24_8: u64 = 0;
        // D s_24_9: cast zx s_24_6 -> u64
        let s_24_9: u64 = (s_24_6 as u64);
        // C s_24_10: const #1u : u64
        let s_24_10: u64 = 1;
        // D s_24_11: and s_24_9 s_24_10
        let s_24_11: u64 = ((s_24_9) & (s_24_10));
        // D s_24_12: cmp-eq s_24_11 s_24_10
        let s_24_12: bool = ((s_24_11) == (s_24_10));
        // D s_24_13: lsl s_24_9 s_24_7
        let s_24_13: u64 = s_24_9 << s_24_7;
        // D s_24_14: or s_24_8 s_24_13
        let s_24_14: u64 = ((s_24_8) | (s_24_13));
        // D s_24_15: cmpl s_24_13
        let s_24_15: u64 = !s_24_13;
        // D s_24_16: and s_24_8 s_24_15
        let s_24_16: u64 = ((s_24_8) & (s_24_15));
        // D s_24_17: select s_24_12 s_24_14 s_24_16
        let s_24_17: u64 = if s_24_12 { s_24_14 } else { s_24_16 };
        // D s_24_18: cast trunc s_24_17 -> u8
        let s_24_18: bool = ((s_24_17) != 0);
        // D s_24_19: cast zx s_24_18 -> bv
        let s_24_19: Bits = Bits::new(s_24_18 as u128, 1u16);
        // C s_24_20: const #1u : u8
        let s_24_20: bool = true;
        // C s_24_21: cast zx s_24_20 -> bv
        let s_24_21: Bits = Bits::new(s_24_20 as u128, 1u16);
        // D s_24_22: cmp-eq s_24_19 s_24_21
        let s_24_22: bool = ((s_24_19) == (s_24_21));
        // D s_24_23: write-var gs#296845 <= s_24_22
        fn_state.gs_296845 = s_24_22;
        // N s_24_24: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var M:u8
        let s_26_0: bool = fn_state.M;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#296842 <= s_26_4
        fn_state.gs_296842 = s_26_4;
        // N s_26_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#296843 <= s_27_0
        fn_state.gs_296843 = s_27_0;
        // N s_27_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#296841 <= s_28_0
        fn_state.gs_296841 = s_28_0;
        // N s_28_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
