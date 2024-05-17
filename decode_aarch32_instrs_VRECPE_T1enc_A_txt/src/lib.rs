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
use execute_aarch32_instrs_VRECPE_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_VRECPE_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    size: u8,
    Vd: u8,
    F: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        floating_point: bool,
        m: i64,
        gs_317593: bool,
        gs_317592: bool,
        elementsshadow_7746: i64,
        esizeshadow_7745: i64,
        esize: i64,
        gs_317588: bool,
        gs_317591: bool,
        d: i64,
        elements: i64,
        gs_317594: bool,
        gs_317589: bool,
        ga_359963: i64,
        D: bool,
        size: u8,
        Vd: u8,
        F: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        size,
        Vd,
        F,
        Q,
        M,
        Vm,
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
        // D s_2_0: read-var Q:u8
        let s_2_0: bool = fn_state.Q;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b31 b3
        if s_2_4 {
            return block_31(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#317589 <= s_3_0
        fn_state.gs_317589 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#317589:u8
        let s_4_0: bool = fn_state.gs_317589;
        // N s_4_1: branch s_4_0 b30 b5
        if s_4_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var size:u8
        let s_5_0: u8 = fn_state.size;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #1u : u8
        let s_5_2: u8 = 1;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b29 b6
        if s_5_4 {
            return block_29(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#317591 <= s_6_0
        fn_state.gs_317591 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#317591:u8
        let s_7_0: bool = fn_state.gs_317591;
        // N s_7_1: branch s_7_0 b28 b8
        if s_7_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var size:u8
        let s_8_0: u8 = fn_state.size;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #0u : u8
        let s_8_2: u8 = 0;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b27 b9
        if s_8_4 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var size:u8
        let s_9_0: u8 = fn_state.size;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 2u16);
        // C s_9_2: const #3u : u8
        let s_9_2: u8 = 3;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 2u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: write-var gs#317592 <= s_9_4
        fn_state.gs_317592 = s_9_4;
        // N s_9_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#317592:u8
        let s_10_0: bool = fn_state.gs_317592;
        // D s_10_1: write-var gs#317593 <= s_10_0
        fn_state.gs_317593 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#317593:u8
        let s_11_0: bool = fn_state.gs_317593;
        // N s_11_1: branch s_11_0 b26 b12
        if s_11_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var size:u8
        let s_12_0: u8 = fn_state.size;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 2u16);
        // C s_12_2: const #1u : u8
        let s_12_2: u8 = 1;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 2u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // N s_12_5: branch s_12_4 b25 b13
        if s_12_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#317594 <= s_13_0
        fn_state.gs_317594 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#317594:u8
        let s_14_0: bool = fn_state.gs_317594;
        // N s_14_1: branch s_14_0 b24 b15
        if s_14_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var F:u8
        let s_15_0: bool = fn_state.F;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 1u16);
        // C s_15_2: const #1u : u8
        let s_15_2: bool = true;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: write-var floating_point <= s_15_4
        fn_state.floating_point = s_15_4;
        // C s_15_6: const #16s : i64
        let s_15_6: i64 = 16;
        // D s_15_7: write-var esize <= s_15_6
        fn_state.esize = s_15_6;
        // C s_15_8: const #2s : i64
        let s_15_8: i64 = 2;
        // D s_15_9: write-var elements <= s_15_8
        fn_state.elements = s_15_8;
        // D s_15_10: read-var size:u8
        let s_15_10: u8 = fn_state.size;
        // D s_15_11: cast zx s_15_10 -> bv
        let s_15_11: Bits = Bits::new(s_15_10 as u128, 2u16);
        // C s_15_12: const #1u : u8
        let s_15_12: u8 = 1;
        // C s_15_13: cast zx s_15_12 -> bv
        let s_15_13: Bits = Bits::new(s_15_12 as u128, 2u16);
        // D s_15_14: cmp-eq s_15_11 s_15_13
        let s_15_14: bool = ((s_15_11) == (s_15_13));
        // D s_15_15: not s_15_14
        let s_15_15: bool = !s_15_14;
        // N s_15_16: branch s_15_15 b21 b16
        if s_15_15 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #16s : i64
        let s_16_0: i64 = 16;
        // D s_16_1: write-var esize <= s_16_0
        fn_state.esize = s_16_0;
        // C s_16_2: const #4s : i64
        let s_16_2: i64 = 4;
        // D s_16_3: write-var elements <= s_16_2
        fn_state.elements = s_16_2;
        // N s_16_4: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var esize:i64
        let s_17_0: i64 = fn_state.esize;
        // D s_17_1: write-var esizeshadow#7745 <= s_17_0
        fn_state.esizeshadow_7745 = s_17_0;
        // D s_17_2: read-var elements:i64
        let s_17_2: i64 = fn_state.elements;
        // D s_17_3: write-var elementsshadow#7746 <= s_17_2
        fn_state.elementsshadow_7746 = s_17_2;
        // D s_17_4: read-var D:u8
        let s_17_4: bool = fn_state.D;
        // D s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 1u16);
        // D s_17_6: read-var Vd:u8
        let s_17_6: u8 = fn_state.Vd;
        // D s_17_7: cast zx s_17_6 -> bv
        let s_17_7: Bits = Bits::new(s_17_6 as u128, 4u16);
        // D s_17_8: cast reint s_17_5 -> u128
        let s_17_8: u128 = (s_17_5.value() as u128);
        // D s_17_9: size-of s_17_5
        let s_17_9: u16 = s_17_5.length();
        // D s_17_10: cast reint s_17_7 -> u128
        let s_17_10: u128 = (s_17_7.value() as u128);
        // D s_17_11: size-of s_17_7
        let s_17_11: u16 = s_17_7.length();
        // D s_17_12: lsl s_17_8 s_17_11
        let s_17_12: u128 = s_17_8 << s_17_11;
        // D s_17_13: or s_17_12 s_17_10
        let s_17_13: u128 = ((s_17_12) | (s_17_10));
        // D s_17_14: add s_17_9 s_17_11
        let s_17_14: u16 = (s_17_9 + s_17_11);
        // D s_17_15: create-bits s_17_13 s_17_14
        let s_17_15: Bits = Bits::new(s_17_13, s_17_14);
        // D s_17_16: cast reint s_17_15 -> u8
        let s_17_16: u8 = (s_17_15.value() as u8);
        // D s_17_17: cast zx s_17_16 -> bv
        let s_17_17: Bits = Bits::new(s_17_16 as u128, 5u16);
        // D s_17_18: cast zx s_17_17 -> i
        let s_17_18: i128 = (s_17_17.value() as i128);
        // D s_17_19: cast reint s_17_18 -> i64
        let s_17_19: i64 = (s_17_18 as i64);
        // D s_17_20: write-var d <= s_17_19
        fn_state.d = s_17_19;
        // D s_17_21: read-var M:u8
        let s_17_21: bool = fn_state.M;
        // D s_17_22: cast zx s_17_21 -> bv
        let s_17_22: Bits = Bits::new(s_17_21 as u128, 1u16);
        // D s_17_23: read-var Vm:u8
        let s_17_23: u8 = fn_state.Vm;
        // D s_17_24: cast zx s_17_23 -> bv
        let s_17_24: Bits = Bits::new(s_17_23 as u128, 4u16);
        // D s_17_25: cast reint s_17_22 -> u128
        let s_17_25: u128 = (s_17_22.value() as u128);
        // D s_17_26: size-of s_17_22
        let s_17_26: u16 = s_17_22.length();
        // D s_17_27: cast reint s_17_24 -> u128
        let s_17_27: u128 = (s_17_24.value() as u128);
        // D s_17_28: size-of s_17_24
        let s_17_28: u16 = s_17_24.length();
        // D s_17_29: lsl s_17_25 s_17_28
        let s_17_29: u128 = s_17_25 << s_17_28;
        // D s_17_30: or s_17_29 s_17_27
        let s_17_30: u128 = ((s_17_29) | (s_17_27));
        // D s_17_31: add s_17_26 s_17_28
        let s_17_31: u16 = (s_17_26 + s_17_28);
        // D s_17_32: create-bits s_17_30 s_17_31
        let s_17_32: Bits = Bits::new(s_17_30, s_17_31);
        // D s_17_33: cast reint s_17_32 -> u8
        let s_17_33: u8 = (s_17_32.value() as u8);
        // D s_17_34: cast zx s_17_33 -> bv
        let s_17_34: Bits = Bits::new(s_17_33 as u128, 5u16);
        // D s_17_35: cast zx s_17_34 -> i
        let s_17_35: i128 = (s_17_34.value() as i128);
        // D s_17_36: cast reint s_17_35 -> i64
        let s_17_36: i64 = (s_17_35 as i64);
        // D s_17_37: write-var m <= s_17_36
        fn_state.m = s_17_36;
        // D s_17_38: read-var Q:u8
        let s_17_38: bool = fn_state.Q;
        // D s_17_39: cast zx s_17_38 -> bv
        let s_17_39: Bits = Bits::new(s_17_38 as u128, 1u16);
        // C s_17_40: const #0u : u8
        let s_17_40: bool = false;
        // C s_17_41: cast zx s_17_40 -> bv
        let s_17_41: Bits = Bits::new(s_17_40 as u128, 1u16);
        // D s_17_42: cmp-eq s_17_39 s_17_41
        let s_17_42: bool = ((s_17_39) == (s_17_41));
        // N s_17_43: branch s_17_42 b20 b18
        if s_17_42 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #2s : i64
        let s_18_0: i64 = 2;
        // D s_18_1: write-var ga#359963 <= s_18_0
        fn_state.ga_359963 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var ga#359963:i64
        let s_19_0: i64 = fn_state.ga_359963;
        // D s_19_1: read-var esizeshadow#7745:i64
        let s_19_1: i64 = fn_state.esizeshadow_7745;
        // D s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (i128::try_from(s_19_1).unwrap());
        // D s_19_3: cast reint s_19_2 -> i64
        let s_19_3: i64 = (s_19_2 as i64);
        // D s_19_4: read-var d:i64
        let s_19_4: i64 = fn_state.d;
        // D s_19_5: read-var elementsshadow#7746:i64
        let s_19_5: i64 = fn_state.elementsshadow_7746;
        // D s_19_6: read-var floating_point:u8
        let s_19_6: bool = fn_state.floating_point;
        // D s_19_7: read-var m:i64
        let s_19_7: i64 = fn_state.m;
        // D s_19_8: call execute_aarch32_instrs_VRECPE_Op_A_txt(s_19_4, s_19_5, s_19_3, s_19_6, s_19_7, s_19_0)
        let s_19_8: () = execute_aarch32_instrs_VRECPE_Op_A_txt(
            state,
            tracer,
            s_19_4,
            s_19_5,
            s_19_3,
            s_19_6,
            s_19_7,
            s_19_0,
        );
        // N s_19_9: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1s : i64
        let s_20_0: i64 = 1;
        // D s_20_1: write-var ga#359963 <= s_20_0
        fn_state.ga_359963 = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var size:u8
        let s_21_0: u8 = fn_state.size;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 2u16);
        // C s_21_2: const #2u : u8
        let s_21_2: u8 = 2;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 2u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: not s_21_4
        let s_21_5: bool = !s_21_4;
        // N s_21_6: branch s_21_5 b23 b22
        if s_21_5 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #32s : i64
        let s_22_0: i64 = 32;
        // D s_22_1: write-var esize <= s_22_0
        fn_state.esize = s_22_0;
        // C s_22_2: const #2s : i64
        let s_22_2: i64 = 2;
        // D s_22_3: write-var elements <= s_22_2
        fn_state.elements = s_22_2;
        // N s_22_4: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call InITBlock(s_25_0)
        let s_25_1: bool = InITBlock(state, tracer, s_25_0);
        // D s_25_2: write-var gs#317594 <= s_25_1
        fn_state.gs_317594 = s_25_1;
        // N s_25_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#317592 <= s_27_0
        fn_state.gs_317592 = s_27_0;
        // N s_27_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#317593 <= s_28_0
        fn_state.gs_317593 = s_28_0;
        // N s_28_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var F:u8
        let s_29_0: bool = fn_state.F;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #0u : u8
        let s_29_2: bool = false;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#317591 <= s_29_4
        fn_state.gs_317591 = s_29_4;
        // N s_29_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: panic
        panic!("{:?}", ());
        // N s_30_1: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0s : i
        let s_31_0: i128 = 0;
        // D s_31_1: read-var Vd:u8
        let s_31_1: u8 = fn_state.Vd;
        // D s_31_2: cast zx s_31_1 -> bv
        let s_31_2: Bits = Bits::new(s_31_1 as u128, 4u16);
        // C s_31_3: const #1u : u64
        let s_31_3: u64 = 1;
        // D s_31_4: bit-extract s_31_2 s_31_0 s_31_3
        let s_31_4: Bits = (Bits::new(
            ((s_31_2) >> (s_31_0)).value(),
            u16::try_from(s_31_3).unwrap(),
        ));
        // D s_31_5: cast reint s_31_4 -> u8
        let s_31_5: bool = ((s_31_4.value()) != 0);
        // C s_31_6: const #0s : i
        let s_31_6: i128 = 0;
        // C s_31_7: const #0u : u64
        let s_31_7: u64 = 0;
        // D s_31_8: cast zx s_31_5 -> u64
        let s_31_8: u64 = (s_31_5 as u64);
        // C s_31_9: const #1u : u64
        let s_31_9: u64 = 1;
        // D s_31_10: and s_31_8 s_31_9
        let s_31_10: u64 = ((s_31_8) & (s_31_9));
        // D s_31_11: cmp-eq s_31_10 s_31_9
        let s_31_11: bool = ((s_31_10) == (s_31_9));
        // D s_31_12: lsl s_31_8 s_31_6
        let s_31_12: u64 = s_31_8 << s_31_6;
        // D s_31_13: or s_31_7 s_31_12
        let s_31_13: u64 = ((s_31_7) | (s_31_12));
        // D s_31_14: cmpl s_31_12
        let s_31_14: u64 = !s_31_12;
        // D s_31_15: and s_31_7 s_31_14
        let s_31_15: u64 = ((s_31_7) & (s_31_14));
        // D s_31_16: select s_31_11 s_31_13 s_31_15
        let s_31_16: u64 = if s_31_11 { s_31_13 } else { s_31_15 };
        // D s_31_17: cast trunc s_31_16 -> u8
        let s_31_17: bool = ((s_31_16) != 0);
        // D s_31_18: cast zx s_31_17 -> bv
        let s_31_18: Bits = Bits::new(s_31_17 as u128, 1u16);
        // C s_31_19: const #1u : u8
        let s_31_19: bool = true;
        // C s_31_20: cast zx s_31_19 -> bv
        let s_31_20: Bits = Bits::new(s_31_19 as u128, 1u16);
        // D s_31_21: cmp-eq s_31_18 s_31_20
        let s_31_21: bool = ((s_31_18) == (s_31_20));
        // N s_31_22: branch s_31_21 b34 b32
        if s_31_21 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0s : i
        let s_32_0: i128 = 0;
        // D s_32_1: read-var Vm:u8
        let s_32_1: u8 = fn_state.Vm;
        // D s_32_2: cast zx s_32_1 -> bv
        let s_32_2: Bits = Bits::new(s_32_1 as u128, 4u16);
        // C s_32_3: const #1u : u64
        let s_32_3: u64 = 1;
        // D s_32_4: bit-extract s_32_2 s_32_0 s_32_3
        let s_32_4: Bits = (Bits::new(
            ((s_32_2) >> (s_32_0)).value(),
            u16::try_from(s_32_3).unwrap(),
        ));
        // D s_32_5: cast reint s_32_4 -> u8
        let s_32_5: bool = ((s_32_4.value()) != 0);
        // C s_32_6: const #0s : i
        let s_32_6: i128 = 0;
        // C s_32_7: const #0u : u64
        let s_32_7: u64 = 0;
        // D s_32_8: cast zx s_32_5 -> u64
        let s_32_8: u64 = (s_32_5 as u64);
        // C s_32_9: const #1u : u64
        let s_32_9: u64 = 1;
        // D s_32_10: and s_32_8 s_32_9
        let s_32_10: u64 = ((s_32_8) & (s_32_9));
        // D s_32_11: cmp-eq s_32_10 s_32_9
        let s_32_11: bool = ((s_32_10) == (s_32_9));
        // D s_32_12: lsl s_32_8 s_32_6
        let s_32_12: u64 = s_32_8 << s_32_6;
        // D s_32_13: or s_32_7 s_32_12
        let s_32_13: u64 = ((s_32_7) | (s_32_12));
        // D s_32_14: cmpl s_32_12
        let s_32_14: u64 = !s_32_12;
        // D s_32_15: and s_32_7 s_32_14
        let s_32_15: u64 = ((s_32_7) & (s_32_14));
        // D s_32_16: select s_32_11 s_32_13 s_32_15
        let s_32_16: u64 = if s_32_11 { s_32_13 } else { s_32_15 };
        // D s_32_17: cast trunc s_32_16 -> u8
        let s_32_17: bool = ((s_32_16) != 0);
        // D s_32_18: cast zx s_32_17 -> bv
        let s_32_18: Bits = Bits::new(s_32_17 as u128, 1u16);
        // C s_32_19: const #1u : u8
        let s_32_19: bool = true;
        // C s_32_20: cast zx s_32_19 -> bv
        let s_32_20: Bits = Bits::new(s_32_19 as u128, 1u16);
        // D s_32_21: cmp-eq s_32_18 s_32_20
        let s_32_21: bool = ((s_32_18) == (s_32_20));
        // D s_32_22: write-var gs#317588 <= s_32_21
        fn_state.gs_317588 = s_32_21;
        // N s_32_23: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#317588:u8
        let s_33_0: bool = fn_state.gs_317588;
        // D s_33_1: write-var gs#317589 <= s_33_0
        fn_state.gs_317589 = s_33_0;
        // N s_33_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#317588 <= s_34_0
        fn_state.gs_317588 = s_34_0;
        // N s_34_2: jump b33
        return block_33(state, tracer, fn_state);
    }
}
