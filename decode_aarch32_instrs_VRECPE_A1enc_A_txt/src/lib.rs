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
use common::*;
pub fn decode_aarch32_instrs_VRECPE_A1enc_A_txt<T: Tracer>(
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
        esize: i64,
        gs_317550: bool,
        gs_317554: bool,
        gs_317553: bool,
        ga_359939: i64,
        elementsshadow_7744: i64,
        d: i64,
        elements: i64,
        gs_317555: bool,
        esizeshadow_7743: i64,
        gs_317551: bool,
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
        // N s_2_5: branch s_2_4 b26 b3
        if s_2_4 {
            return block_26(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#317551 <= s_3_0
        fn_state.gs_317551 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#317551:u8
        let s_4_0: bool = fn_state.gs_317551;
        // N s_4_1: branch s_4_0 b25 b5
        if s_4_0 {
            return block_25(state, tracer, fn_state);
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
        // N s_5_5: branch s_5_4 b24 b6
        if s_5_4 {
            return block_24(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#317553 <= s_6_0
        fn_state.gs_317553 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#317553:u8
        let s_7_0: bool = fn_state.gs_317553;
        // N s_7_1: branch s_7_0 b23 b8
        if s_7_0 {
            return block_23(state, tracer, fn_state);
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
        // N s_8_5: branch s_8_4 b22 b9
        if s_8_4 {
            return block_22(state, tracer, fn_state);
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
        // D s_9_5: write-var gs#317554 <= s_9_4
        fn_state.gs_317554 = s_9_4;
        // N s_9_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#317554:u8
        let s_10_0: bool = fn_state.gs_317554;
        // D s_10_1: write-var gs#317555 <= s_10_0
        fn_state.gs_317555 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#317555:u8
        let s_11_0: bool = fn_state.gs_317555;
        // N s_11_1: branch s_11_0 b21 b12
        if s_11_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var F:u8
        let s_12_0: bool = fn_state.F;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #1u : u8
        let s_12_2: bool = true;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var floating_point <= s_12_4
        fn_state.floating_point = s_12_4;
        // C s_12_6: const #16s : i64
        let s_12_6: i64 = 16;
        // D s_12_7: write-var esize <= s_12_6
        fn_state.esize = s_12_6;
        // C s_12_8: const #2s : i64
        let s_12_8: i64 = 2;
        // D s_12_9: write-var elements <= s_12_8
        fn_state.elements = s_12_8;
        // D s_12_10: read-var size:u8
        let s_12_10: u8 = fn_state.size;
        // D s_12_11: cast zx s_12_10 -> bv
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 2u16);
        // C s_12_12: const #1u : u8
        let s_12_12: u8 = 1;
        // C s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 2u16);
        // D s_12_14: cmp-eq s_12_11 s_12_13
        let s_12_14: bool = ((s_12_11) == (s_12_13));
        // D s_12_15: not s_12_14
        let s_12_15: bool = !s_12_14;
        // N s_12_16: branch s_12_15 b18 b13
        if s_12_15 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #16s : i64
        let s_13_0: i64 = 16;
        // D s_13_1: write-var esize <= s_13_0
        fn_state.esize = s_13_0;
        // C s_13_2: const #4s : i64
        let s_13_2: i64 = 4;
        // D s_13_3: write-var elements <= s_13_2
        fn_state.elements = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var esize:i64
        let s_14_0: i64 = fn_state.esize;
        // D s_14_1: write-var esizeshadow#7743 <= s_14_0
        fn_state.esizeshadow_7743 = s_14_0;
        // D s_14_2: read-var elements:i64
        let s_14_2: i64 = fn_state.elements;
        // D s_14_3: write-var elementsshadow#7744 <= s_14_2
        fn_state.elementsshadow_7744 = s_14_2;
        // D s_14_4: read-var D:u8
        let s_14_4: bool = fn_state.D;
        // D s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // D s_14_6: read-var Vd:u8
        let s_14_6: u8 = fn_state.Vd;
        // D s_14_7: cast zx s_14_6 -> bv
        let s_14_7: Bits = Bits::new(s_14_6 as u128, 4u16);
        // D s_14_8: cast reint s_14_5 -> u128
        let s_14_8: u128 = (s_14_5.value() as u128);
        // D s_14_9: size-of s_14_5
        let s_14_9: u16 = s_14_5.length();
        // D s_14_10: cast reint s_14_7 -> u128
        let s_14_10: u128 = (s_14_7.value() as u128);
        // D s_14_11: size-of s_14_7
        let s_14_11: u16 = s_14_7.length();
        // D s_14_12: lsl s_14_8 s_14_11
        let s_14_12: u128 = s_14_8 << s_14_11;
        // D s_14_13: or s_14_12 s_14_10
        let s_14_13: u128 = ((s_14_12) | (s_14_10));
        // D s_14_14: add s_14_9 s_14_11
        let s_14_14: u16 = (s_14_9 + s_14_11);
        // D s_14_15: create-bits s_14_13 s_14_14
        let s_14_15: Bits = Bits::new(s_14_13, s_14_14);
        // D s_14_16: cast reint s_14_15 -> u8
        let s_14_16: u8 = (s_14_15.value() as u8);
        // D s_14_17: cast zx s_14_16 -> bv
        let s_14_17: Bits = Bits::new(s_14_16 as u128, 5u16);
        // D s_14_18: cast zx s_14_17 -> i
        let s_14_18: i128 = (s_14_17.value() as i128);
        // D s_14_19: cast reint s_14_18 -> i64
        let s_14_19: i64 = (s_14_18 as i64);
        // D s_14_20: write-var d <= s_14_19
        fn_state.d = s_14_19;
        // D s_14_21: read-var M:u8
        let s_14_21: bool = fn_state.M;
        // D s_14_22: cast zx s_14_21 -> bv
        let s_14_22: Bits = Bits::new(s_14_21 as u128, 1u16);
        // D s_14_23: read-var Vm:u8
        let s_14_23: u8 = fn_state.Vm;
        // D s_14_24: cast zx s_14_23 -> bv
        let s_14_24: Bits = Bits::new(s_14_23 as u128, 4u16);
        // D s_14_25: cast reint s_14_22 -> u128
        let s_14_25: u128 = (s_14_22.value() as u128);
        // D s_14_26: size-of s_14_22
        let s_14_26: u16 = s_14_22.length();
        // D s_14_27: cast reint s_14_24 -> u128
        let s_14_27: u128 = (s_14_24.value() as u128);
        // D s_14_28: size-of s_14_24
        let s_14_28: u16 = s_14_24.length();
        // D s_14_29: lsl s_14_25 s_14_28
        let s_14_29: u128 = s_14_25 << s_14_28;
        // D s_14_30: or s_14_29 s_14_27
        let s_14_30: u128 = ((s_14_29) | (s_14_27));
        // D s_14_31: add s_14_26 s_14_28
        let s_14_31: u16 = (s_14_26 + s_14_28);
        // D s_14_32: create-bits s_14_30 s_14_31
        let s_14_32: Bits = Bits::new(s_14_30, s_14_31);
        // D s_14_33: cast reint s_14_32 -> u8
        let s_14_33: u8 = (s_14_32.value() as u8);
        // D s_14_34: cast zx s_14_33 -> bv
        let s_14_34: Bits = Bits::new(s_14_33 as u128, 5u16);
        // D s_14_35: cast zx s_14_34 -> i
        let s_14_35: i128 = (s_14_34.value() as i128);
        // D s_14_36: cast reint s_14_35 -> i64
        let s_14_36: i64 = (s_14_35 as i64);
        // D s_14_37: write-var m <= s_14_36
        fn_state.m = s_14_36;
        // D s_14_38: read-var Q:u8
        let s_14_38: bool = fn_state.Q;
        // D s_14_39: cast zx s_14_38 -> bv
        let s_14_39: Bits = Bits::new(s_14_38 as u128, 1u16);
        // C s_14_40: const #0u : u8
        let s_14_40: bool = false;
        // C s_14_41: cast zx s_14_40 -> bv
        let s_14_41: Bits = Bits::new(s_14_40 as u128, 1u16);
        // D s_14_42: cmp-eq s_14_39 s_14_41
        let s_14_42: bool = ((s_14_39) == (s_14_41));
        // N s_14_43: branch s_14_42 b17 b15
        if s_14_42 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #2s : i64
        let s_15_0: i64 = 2;
        // D s_15_1: write-var ga#359939 <= s_15_0
        fn_state.ga_359939 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var ga#359939:i64
        let s_16_0: i64 = fn_state.ga_359939;
        // D s_16_1: read-var esizeshadow#7743:i64
        let s_16_1: i64 = fn_state.esizeshadow_7743;
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // D s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // D s_16_4: read-var d:i64
        let s_16_4: i64 = fn_state.d;
        // D s_16_5: read-var elementsshadow#7744:i64
        let s_16_5: i64 = fn_state.elementsshadow_7744;
        // D s_16_6: read-var floating_point:u8
        let s_16_6: bool = fn_state.floating_point;
        // D s_16_7: read-var m:i64
        let s_16_7: i64 = fn_state.m;
        // D s_16_8: call execute_aarch32_instrs_VRECPE_Op_A_txt(s_16_4, s_16_5, s_16_3, s_16_6, s_16_7, s_16_0)
        let s_16_8: () = execute_aarch32_instrs_VRECPE_Op_A_txt(
            state,
            tracer,
            s_16_4,
            s_16_5,
            s_16_3,
            s_16_6,
            s_16_7,
            s_16_0,
        );
        // N s_16_9: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1s : i64
        let s_17_0: i64 = 1;
        // D s_17_1: write-var ga#359939 <= s_17_0
        fn_state.ga_359939 = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var size:u8
        let s_18_0: u8 = fn_state.size;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 2u16);
        // C s_18_2: const #2u : u8
        let s_18_2: u8 = 2;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 2u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: not s_18_4
        let s_18_5: bool = !s_18_4;
        // N s_18_6: branch s_18_5 b20 b19
        if s_18_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #32s : i64
        let s_19_0: i64 = 32;
        // D s_19_1: write-var esize <= s_19_0
        fn_state.esize = s_19_0;
        // C s_19_2: const #2s : i64
        let s_19_2: i64 = 2;
        // D s_19_3: write-var elements <= s_19_2
        fn_state.elements = s_19_2;
        // N s_19_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#317554 <= s_22_0
        fn_state.gs_317554 = s_22_0;
        // N s_22_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#317555 <= s_23_0
        fn_state.gs_317555 = s_23_0;
        // N s_23_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var F:u8
        let s_24_0: bool = fn_state.F;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #0u : u8
        let s_24_2: bool = false;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: write-var gs#317553 <= s_24_4
        fn_state.gs_317553 = s_24_4;
        // N s_24_6: jump b7
        return block_7(state, tracer, fn_state);
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
        // C s_26_0: const #0s : i
        let s_26_0: i128 = 0;
        // D s_26_1: read-var Vd:u8
        let s_26_1: u8 = fn_state.Vd;
        // D s_26_2: cast zx s_26_1 -> bv
        let s_26_2: Bits = Bits::new(s_26_1 as u128, 4u16);
        // C s_26_3: const #1u : u64
        let s_26_3: u64 = 1;
        // D s_26_4: bit-extract s_26_2 s_26_0 s_26_3
        let s_26_4: Bits = (Bits::new(
            ((s_26_2) >> (s_26_0)).value(),
            u16::try_from(s_26_3).unwrap(),
        ));
        // D s_26_5: cast reint s_26_4 -> u8
        let s_26_5: bool = ((s_26_4.value()) != 0);
        // C s_26_6: const #0s : i
        let s_26_6: i128 = 0;
        // C s_26_7: const #0u : u64
        let s_26_7: u64 = 0;
        // D s_26_8: cast zx s_26_5 -> u64
        let s_26_8: u64 = (s_26_5 as u64);
        // C s_26_9: const #1u : u64
        let s_26_9: u64 = 1;
        // D s_26_10: and s_26_8 s_26_9
        let s_26_10: u64 = ((s_26_8) & (s_26_9));
        // D s_26_11: cmp-eq s_26_10 s_26_9
        let s_26_11: bool = ((s_26_10) == (s_26_9));
        // D s_26_12: lsl s_26_8 s_26_6
        let s_26_12: u64 = s_26_8 << s_26_6;
        // D s_26_13: or s_26_7 s_26_12
        let s_26_13: u64 = ((s_26_7) | (s_26_12));
        // D s_26_14: cmpl s_26_12
        let s_26_14: u64 = !s_26_12;
        // D s_26_15: and s_26_7 s_26_14
        let s_26_15: u64 = ((s_26_7) & (s_26_14));
        // D s_26_16: select s_26_11 s_26_13 s_26_15
        let s_26_16: u64 = if s_26_11 { s_26_13 } else { s_26_15 };
        // D s_26_17: cast trunc s_26_16 -> u8
        let s_26_17: bool = ((s_26_16) != 0);
        // D s_26_18: cast zx s_26_17 -> bv
        let s_26_18: Bits = Bits::new(s_26_17 as u128, 1u16);
        // C s_26_19: const #1u : u8
        let s_26_19: bool = true;
        // C s_26_20: cast zx s_26_19 -> bv
        let s_26_20: Bits = Bits::new(s_26_19 as u128, 1u16);
        // D s_26_21: cmp-eq s_26_18 s_26_20
        let s_26_21: bool = ((s_26_18) == (s_26_20));
        // N s_26_22: branch s_26_21 b29 b27
        if s_26_21 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0s : i
        let s_27_0: i128 = 0;
        // D s_27_1: read-var Vm:u8
        let s_27_1: u8 = fn_state.Vm;
        // D s_27_2: cast zx s_27_1 -> bv
        let s_27_2: Bits = Bits::new(s_27_1 as u128, 4u16);
        // C s_27_3: const #1u : u64
        let s_27_3: u64 = 1;
        // D s_27_4: bit-extract s_27_2 s_27_0 s_27_3
        let s_27_4: Bits = (Bits::new(
            ((s_27_2) >> (s_27_0)).value(),
            u16::try_from(s_27_3).unwrap(),
        ));
        // D s_27_5: cast reint s_27_4 -> u8
        let s_27_5: bool = ((s_27_4.value()) != 0);
        // C s_27_6: const #0s : i
        let s_27_6: i128 = 0;
        // C s_27_7: const #0u : u64
        let s_27_7: u64 = 0;
        // D s_27_8: cast zx s_27_5 -> u64
        let s_27_8: u64 = (s_27_5 as u64);
        // C s_27_9: const #1u : u64
        let s_27_9: u64 = 1;
        // D s_27_10: and s_27_8 s_27_9
        let s_27_10: u64 = ((s_27_8) & (s_27_9));
        // D s_27_11: cmp-eq s_27_10 s_27_9
        let s_27_11: bool = ((s_27_10) == (s_27_9));
        // D s_27_12: lsl s_27_8 s_27_6
        let s_27_12: u64 = s_27_8 << s_27_6;
        // D s_27_13: or s_27_7 s_27_12
        let s_27_13: u64 = ((s_27_7) | (s_27_12));
        // D s_27_14: cmpl s_27_12
        let s_27_14: u64 = !s_27_12;
        // D s_27_15: and s_27_7 s_27_14
        let s_27_15: u64 = ((s_27_7) & (s_27_14));
        // D s_27_16: select s_27_11 s_27_13 s_27_15
        let s_27_16: u64 = if s_27_11 { s_27_13 } else { s_27_15 };
        // D s_27_17: cast trunc s_27_16 -> u8
        let s_27_17: bool = ((s_27_16) != 0);
        // D s_27_18: cast zx s_27_17 -> bv
        let s_27_18: Bits = Bits::new(s_27_17 as u128, 1u16);
        // C s_27_19: const #1u : u8
        let s_27_19: bool = true;
        // C s_27_20: cast zx s_27_19 -> bv
        let s_27_20: Bits = Bits::new(s_27_19 as u128, 1u16);
        // D s_27_21: cmp-eq s_27_18 s_27_20
        let s_27_21: bool = ((s_27_18) == (s_27_20));
        // D s_27_22: write-var gs#317550 <= s_27_21
        fn_state.gs_317550 = s_27_21;
        // N s_27_23: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#317550:u8
        let s_28_0: bool = fn_state.gs_317550;
        // D s_28_1: write-var gs#317551 <= s_28_0
        fn_state.gs_317551 = s_28_0;
        // N s_28_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#317550 <= s_29_0
        fn_state.gs_317550 = s_29_0;
        // N s_29_2: jump b28
        return block_28(state, tracer, fn_state);
    }
}
