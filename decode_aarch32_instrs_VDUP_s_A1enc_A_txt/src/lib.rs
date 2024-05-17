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
use execute_aarch32_instrs_VDUP_s_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VDUP_s_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    imm4: u8,
    Vd: u8,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        ga_352778: i64,
        esize: i64,
        index: i64,
        esizeshadow_7448: i64,
        gs_308658: bool,
        elementsshadow_7449: i64,
        gs_308665: bool,
        d: i64,
        elements: i64,
        indexshadow_7447: i64,
        D: bool,
        imm4: u8,
        Vd: u8,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        imm4,
        Vd,
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
        // D s_2_0: read-var imm4:u8
        let s_2_0: u8 = fn_state.imm4;
        // C s_2_1: const #0s : i
        let s_2_1: i128 = 0;
        // D s_2_2: cast zx s_2_0 -> bv
        let s_2_2: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_3: const #1s : i64
        let s_2_3: i64 = 1;
        // C s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // C s_2_5: const #2s : i
        let s_2_5: i128 = 2;
        // C s_2_6: add s_2_5 s_2_4
        let s_2_6: i128 = (s_2_5 + s_2_4);
        // D s_2_7: bit-extract s_2_2 s_2_1 s_2_6
        let s_2_7: Bits = (Bits::new(
            ((s_2_2) >> (s_2_1)).value(),
            u16::try_from(s_2_6).unwrap(),
        ));
        // D s_2_8: cast reint s_2_7 -> u8
        let s_2_8: u8 = (s_2_7.value() as u8);
        // D s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 3u16);
        // C s_2_10: const #0u : u8
        let s_2_10: u8 = 0;
        // C s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 3u16);
        // D s_2_12: cmp-eq s_2_9 s_2_11
        let s_2_12: bool = ((s_2_9) == (s_2_11));
        // D s_2_13: not s_2_12
        let s_2_13: bool = !s_2_12;
        // N s_2_14: branch s_2_13 b22 b3
        if s_2_13 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var gs#308658 <= s_3_0
        fn_state.gs_308658 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#308658:u8
        let s_4_0: bool = fn_state.gs_308658;
        // N s_4_1: branch s_4_0 b21 b5
        if s_4_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Q:u8
        let s_5_0: bool = fn_state.Q;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #1u : u8
        let s_5_2: bool = true;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b20 b6
        if s_5_4 {
            return block_20(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#308665 <= s_6_0
        fn_state.gs_308665 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#308665:u8
        let s_7_0: bool = fn_state.gs_308665;
        // N s_7_1: branch s_7_0 b19 b8
        if s_7_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #8s : i64
        let s_8_0: i64 = 8;
        // D s_8_1: write-var esize <= s_8_0
        fn_state.esize = s_8_0;
        // C s_8_2: const #2s : i64
        let s_8_2: i64 = 2;
        // D s_8_3: write-var elements <= s_8_2
        fn_state.elements = s_8_2;
        // D s_8_4: read-var imm4:u8
        let s_8_4: u8 = fn_state.imm4;
        // C s_8_5: const #0s : i
        let s_8_5: i128 = 0;
        // D s_8_6: cast zx s_8_4 -> bv
        let s_8_6: Bits = Bits::new(s_8_4 as u128, 4u16);
        // C s_8_7: const #1s : i64
        let s_8_7: i64 = 1;
        // C s_8_8: cast zx s_8_7 -> i
        let s_8_8: i128 = (i128::try_from(s_8_7).unwrap());
        // C s_8_9: const #0s : i
        let s_8_9: i128 = 0;
        // C s_8_10: add s_8_9 s_8_8
        let s_8_10: i128 = (s_8_9 + s_8_8);
        // D s_8_11: bit-extract s_8_6 s_8_5 s_8_10
        let s_8_11: Bits = (Bits::new(
            ((s_8_6) >> (s_8_5)).value(),
            u16::try_from(s_8_10).unwrap(),
        ));
        // D s_8_12: cast reint s_8_11 -> u8
        let s_8_12: bool = ((s_8_11.value()) != 0);
        // D s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 1u16);
        // C s_8_14: const #1u : u8
        let s_8_14: bool = true;
        // C s_8_15: cast zx s_8_14 -> bv
        let s_8_15: Bits = Bits::new(s_8_14 as u128, 1u16);
        // D s_8_16: cmp-eq s_8_13 s_8_15
        let s_8_16: bool = ((s_8_13) == (s_8_15));
        // D s_8_17: not s_8_16
        let s_8_17: bool = !s_8_16;
        // N s_8_18: branch s_8_17 b14 b9
        if s_8_17 {
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
        // C s_9_0: const #8s : i64
        let s_9_0: i64 = 8;
        // D s_9_1: write-var esize <= s_9_0
        fn_state.esize = s_9_0;
        // C s_9_2: const #8s : i64
        let s_9_2: i64 = 8;
        // D s_9_3: write-var elements <= s_9_2
        fn_state.elements = s_9_2;
        // C s_9_4: const #1s : i
        let s_9_4: i128 = 1;
        // D s_9_5: read-var imm4:u8
        let s_9_5: u8 = fn_state.imm4;
        // D s_9_6: cast zx s_9_5 -> bv
        let s_9_6: Bits = Bits::new(s_9_5 as u128, 4u16);
        // C s_9_7: const #1s : i64
        let s_9_7: i64 = 1;
        // C s_9_8: cast zx s_9_7 -> i
        let s_9_8: i128 = (i128::try_from(s_9_7).unwrap());
        // C s_9_9: const #2s : i
        let s_9_9: i128 = 2;
        // C s_9_10: add s_9_9 s_9_8
        let s_9_10: i128 = (s_9_9 + s_9_8);
        // D s_9_11: bit-extract s_9_6 s_9_4 s_9_10
        let s_9_11: Bits = (Bits::new(
            ((s_9_6) >> (s_9_4)).value(),
            u16::try_from(s_9_10).unwrap(),
        ));
        // D s_9_12: cast reint s_9_11 -> u8
        let s_9_12: u8 = (s_9_11.value() as u8);
        // D s_9_13: cast zx s_9_12 -> bv
        let s_9_13: Bits = Bits::new(s_9_12 as u128, 3u16);
        // D s_9_14: cast zx s_9_13 -> i
        let s_9_14: i128 = (s_9_13.value() as i128);
        // D s_9_15: cast reint s_9_14 -> i64
        let s_9_15: i64 = (s_9_14 as i64);
        // D s_9_16: write-var index <= s_9_15
        fn_state.index = s_9_15;
        // N s_9_17: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var index:i64
        let s_10_0: i64 = fn_state.index;
        // D s_10_1: write-var indexshadow#7447 <= s_10_0
        fn_state.indexshadow_7447 = s_10_0;
        // D s_10_2: read-var esize:i64
        let s_10_2: i64 = fn_state.esize;
        // D s_10_3: write-var esizeshadow#7448 <= s_10_2
        fn_state.esizeshadow_7448 = s_10_2;
        // D s_10_4: read-var elements:i64
        let s_10_4: i64 = fn_state.elements;
        // D s_10_5: write-var elementsshadow#7449 <= s_10_4
        fn_state.elementsshadow_7449 = s_10_4;
        // D s_10_6: read-var D:u8
        let s_10_6: bool = fn_state.D;
        // D s_10_7: cast zx s_10_6 -> bv
        let s_10_7: Bits = Bits::new(s_10_6 as u128, 1u16);
        // D s_10_8: read-var Vd:u8
        let s_10_8: u8 = fn_state.Vd;
        // D s_10_9: cast zx s_10_8 -> bv
        let s_10_9: Bits = Bits::new(s_10_8 as u128, 4u16);
        // D s_10_10: cast reint s_10_7 -> u128
        let s_10_10: u128 = (s_10_7.value() as u128);
        // D s_10_11: size-of s_10_7
        let s_10_11: u16 = s_10_7.length();
        // D s_10_12: cast reint s_10_9 -> u128
        let s_10_12: u128 = (s_10_9.value() as u128);
        // D s_10_13: size-of s_10_9
        let s_10_13: u16 = s_10_9.length();
        // D s_10_14: lsl s_10_10 s_10_13
        let s_10_14: u128 = s_10_10 << s_10_13;
        // D s_10_15: or s_10_14 s_10_12
        let s_10_15: u128 = ((s_10_14) | (s_10_12));
        // D s_10_16: add s_10_11 s_10_13
        let s_10_16: u16 = (s_10_11 + s_10_13);
        // D s_10_17: create-bits s_10_15 s_10_16
        let s_10_17: Bits = Bits::new(s_10_15, s_10_16);
        // D s_10_18: cast reint s_10_17 -> u8
        let s_10_18: u8 = (s_10_17.value() as u8);
        // D s_10_19: cast zx s_10_18 -> bv
        let s_10_19: Bits = Bits::new(s_10_18 as u128, 5u16);
        // D s_10_20: cast zx s_10_19 -> i
        let s_10_20: i128 = (s_10_19.value() as i128);
        // D s_10_21: cast reint s_10_20 -> i64
        let s_10_21: i64 = (s_10_20 as i64);
        // D s_10_22: write-var d <= s_10_21
        fn_state.d = s_10_21;
        // D s_10_23: read-var M:u8
        let s_10_23: bool = fn_state.M;
        // D s_10_24: cast zx s_10_23 -> bv
        let s_10_24: Bits = Bits::new(s_10_23 as u128, 1u16);
        // D s_10_25: read-var Vm:u8
        let s_10_25: u8 = fn_state.Vm;
        // D s_10_26: cast zx s_10_25 -> bv
        let s_10_26: Bits = Bits::new(s_10_25 as u128, 4u16);
        // D s_10_27: cast reint s_10_24 -> u128
        let s_10_27: u128 = (s_10_24.value() as u128);
        // D s_10_28: size-of s_10_24
        let s_10_28: u16 = s_10_24.length();
        // D s_10_29: cast reint s_10_26 -> u128
        let s_10_29: u128 = (s_10_26.value() as u128);
        // D s_10_30: size-of s_10_26
        let s_10_30: u16 = s_10_26.length();
        // D s_10_31: lsl s_10_27 s_10_30
        let s_10_31: u128 = s_10_27 << s_10_30;
        // D s_10_32: or s_10_31 s_10_29
        let s_10_32: u128 = ((s_10_31) | (s_10_29));
        // D s_10_33: add s_10_28 s_10_30
        let s_10_33: u16 = (s_10_28 + s_10_30);
        // D s_10_34: create-bits s_10_32 s_10_33
        let s_10_34: Bits = Bits::new(s_10_32, s_10_33);
        // D s_10_35: cast reint s_10_34 -> u8
        let s_10_35: u8 = (s_10_34.value() as u8);
        // D s_10_36: cast zx s_10_35 -> bv
        let s_10_36: Bits = Bits::new(s_10_35 as u128, 5u16);
        // D s_10_37: cast zx s_10_36 -> i
        let s_10_37: i128 = (s_10_36.value() as i128);
        // D s_10_38: cast reint s_10_37 -> i64
        let s_10_38: i64 = (s_10_37 as i64);
        // D s_10_39: write-var m <= s_10_38
        fn_state.m = s_10_38;
        // D s_10_40: read-var Q:u8
        let s_10_40: bool = fn_state.Q;
        // D s_10_41: cast zx s_10_40 -> bv
        let s_10_41: Bits = Bits::new(s_10_40 as u128, 1u16);
        // C s_10_42: const #0u : u8
        let s_10_42: bool = false;
        // C s_10_43: cast zx s_10_42 -> bv
        let s_10_43: Bits = Bits::new(s_10_42 as u128, 1u16);
        // D s_10_44: cmp-eq s_10_41 s_10_43
        let s_10_44: bool = ((s_10_41) == (s_10_43));
        // N s_10_45: branch s_10_44 b13 b11
        if s_10_44 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #2s : i64
        let s_11_0: i64 = 2;
        // D s_11_1: write-var ga#352778 <= s_11_0
        fn_state.ga_352778 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#352778:i64
        let s_12_0: i64 = fn_state.ga_352778;
        // D s_12_1: read-var esizeshadow#7448:i64
        let s_12_1: i64 = fn_state.esizeshadow_7448;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // D s_12_4: read-var d:i64
        let s_12_4: i64 = fn_state.d;
        // D s_12_5: read-var elementsshadow#7449:i64
        let s_12_5: i64 = fn_state.elementsshadow_7449;
        // D s_12_6: read-var indexshadow#7447:i64
        let s_12_6: i64 = fn_state.indexshadow_7447;
        // D s_12_7: read-var m:i64
        let s_12_7: i64 = fn_state.m;
        // D s_12_8: call execute_aarch32_instrs_VDUP_s_Op_A_txt(s_12_4, s_12_5, s_12_3, s_12_6, s_12_7, s_12_0)
        let s_12_8: () = execute_aarch32_instrs_VDUP_s_Op_A_txt(
            state,
            tracer,
            s_12_4,
            s_12_5,
            s_12_3,
            s_12_6,
            s_12_7,
            s_12_0,
        );
        // N s_12_9: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1s : i64
        let s_13_0: i64 = 1;
        // D s_13_1: write-var ga#352778 <= s_13_0
        fn_state.ga_352778 = s_13_0;
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var imm4:u8
        let s_14_0: u8 = fn_state.imm4;
        // C s_14_1: const #0s : i
        let s_14_1: i128 = 0;
        // D s_14_2: cast zx s_14_0 -> bv
        let s_14_2: Bits = Bits::new(s_14_0 as u128, 4u16);
        // C s_14_3: const #1s : i64
        let s_14_3: i64 = 1;
        // C s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // C s_14_5: const #1s : i
        let s_14_5: i128 = 1;
        // C s_14_6: add s_14_5 s_14_4
        let s_14_6: i128 = (s_14_5 + s_14_4);
        // D s_14_7: bit-extract s_14_2 s_14_1 s_14_6
        let s_14_7: Bits = (Bits::new(
            ((s_14_2) >> (s_14_1)).value(),
            u16::try_from(s_14_6).unwrap(),
        ));
        // D s_14_8: cast reint s_14_7 -> u8
        let s_14_8: u8 = (s_14_7.value() as u8);
        // D s_14_9: cast zx s_14_8 -> bv
        let s_14_9: Bits = Bits::new(s_14_8 as u128, 2u16);
        // C s_14_10: const #2u : u8
        let s_14_10: u8 = 2;
        // C s_14_11: cast zx s_14_10 -> bv
        let s_14_11: Bits = Bits::new(s_14_10 as u128, 2u16);
        // D s_14_12: cmp-eq s_14_9 s_14_11
        let s_14_12: bool = ((s_14_9) == (s_14_11));
        // D s_14_13: not s_14_12
        let s_14_13: bool = !s_14_12;
        // N s_14_14: branch s_14_13 b16 b15
        if s_14_13 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #16s : i64
        let s_15_0: i64 = 16;
        // D s_15_1: write-var esize <= s_15_0
        fn_state.esize = s_15_0;
        // C s_15_2: const #4s : i64
        let s_15_2: i64 = 4;
        // D s_15_3: write-var elements <= s_15_2
        fn_state.elements = s_15_2;
        // C s_15_4: const #2s : i
        let s_15_4: i128 = 2;
        // D s_15_5: read-var imm4:u8
        let s_15_5: u8 = fn_state.imm4;
        // D s_15_6: cast zx s_15_5 -> bv
        let s_15_6: Bits = Bits::new(s_15_5 as u128, 4u16);
        // C s_15_7: const #1s : i64
        let s_15_7: i64 = 1;
        // C s_15_8: cast zx s_15_7 -> i
        let s_15_8: i128 = (i128::try_from(s_15_7).unwrap());
        // C s_15_9: const #1s : i
        let s_15_9: i128 = 1;
        // C s_15_10: add s_15_9 s_15_8
        let s_15_10: i128 = (s_15_9 + s_15_8);
        // D s_15_11: bit-extract s_15_6 s_15_4 s_15_10
        let s_15_11: Bits = (Bits::new(
            ((s_15_6) >> (s_15_4)).value(),
            u16::try_from(s_15_10).unwrap(),
        ));
        // D s_15_12: cast reint s_15_11 -> u8
        let s_15_12: u8 = (s_15_11.value() as u8);
        // D s_15_13: cast zx s_15_12 -> bv
        let s_15_13: Bits = Bits::new(s_15_12 as u128, 2u16);
        // D s_15_14: cast zx s_15_13 -> i
        let s_15_14: i128 = (s_15_13.value() as i128);
        // D s_15_15: cast reint s_15_14 -> i64
        let s_15_15: i64 = (s_15_14 as i64);
        // D s_15_16: write-var index <= s_15_15
        fn_state.index = s_15_15;
        // N s_15_17: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var imm4:u8
        let s_16_0: u8 = fn_state.imm4;
        // C s_16_1: const #0s : i
        let s_16_1: i128 = 0;
        // D s_16_2: cast zx s_16_0 -> bv
        let s_16_2: Bits = Bits::new(s_16_0 as u128, 4u16);
        // C s_16_3: const #1s : i64
        let s_16_3: i64 = 1;
        // C s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // C s_16_5: const #2s : i
        let s_16_5: i128 = 2;
        // C s_16_6: add s_16_5 s_16_4
        let s_16_6: i128 = (s_16_5 + s_16_4);
        // D s_16_7: bit-extract s_16_2 s_16_1 s_16_6
        let s_16_7: Bits = (Bits::new(
            ((s_16_2) >> (s_16_1)).value(),
            u16::try_from(s_16_6).unwrap(),
        ));
        // D s_16_8: cast reint s_16_7 -> u8
        let s_16_8: u8 = (s_16_7.value() as u8);
        // D s_16_9: cast zx s_16_8 -> bv
        let s_16_9: Bits = Bits::new(s_16_8 as u128, 3u16);
        // C s_16_10: const #4u : u8
        let s_16_10: u8 = 4;
        // C s_16_11: cast zx s_16_10 -> bv
        let s_16_11: Bits = Bits::new(s_16_10 as u128, 3u16);
        // D s_16_12: cmp-eq s_16_9 s_16_11
        let s_16_12: bool = ((s_16_9) == (s_16_11));
        // D s_16_13: not s_16_12
        let s_16_13: bool = !s_16_12;
        // N s_16_14: branch s_16_13 b18 b17
        if s_16_13 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #32s : i64
        let s_17_0: i64 = 32;
        // D s_17_1: write-var esize <= s_17_0
        fn_state.esize = s_17_0;
        // C s_17_2: const #2s : i64
        let s_17_2: i64 = 2;
        // D s_17_3: write-var elements <= s_17_2
        fn_state.elements = s_17_2;
        // C s_17_4: const #3s : i
        let s_17_4: i128 = 3;
        // D s_17_5: read-var imm4:u8
        let s_17_5: u8 = fn_state.imm4;
        // D s_17_6: cast zx s_17_5 -> bv
        let s_17_6: Bits = Bits::new(s_17_5 as u128, 4u16);
        // C s_17_7: const #1u : u64
        let s_17_7: u64 = 1;
        // D s_17_8: bit-extract s_17_6 s_17_4 s_17_7
        let s_17_8: Bits = (Bits::new(
            ((s_17_6) >> (s_17_4)).value(),
            u16::try_from(s_17_7).unwrap(),
        ));
        // D s_17_9: cast reint s_17_8 -> u8
        let s_17_9: bool = ((s_17_8.value()) != 0);
        // C s_17_10: const #0s : i
        let s_17_10: i128 = 0;
        // C s_17_11: const #0u : u64
        let s_17_11: u64 = 0;
        // D s_17_12: cast zx s_17_9 -> u64
        let s_17_12: u64 = (s_17_9 as u64);
        // C s_17_13: const #1u : u64
        let s_17_13: u64 = 1;
        // D s_17_14: and s_17_12 s_17_13
        let s_17_14: u64 = ((s_17_12) & (s_17_13));
        // D s_17_15: cmp-eq s_17_14 s_17_13
        let s_17_15: bool = ((s_17_14) == (s_17_13));
        // D s_17_16: lsl s_17_12 s_17_10
        let s_17_16: u64 = s_17_12 << s_17_10;
        // D s_17_17: or s_17_11 s_17_16
        let s_17_17: u64 = ((s_17_11) | (s_17_16));
        // D s_17_18: cmpl s_17_16
        let s_17_18: u64 = !s_17_16;
        // D s_17_19: and s_17_11 s_17_18
        let s_17_19: u64 = ((s_17_11) & (s_17_18));
        // D s_17_20: select s_17_15 s_17_17 s_17_19
        let s_17_20: u64 = if s_17_15 { s_17_17 } else { s_17_19 };
        // D s_17_21: cast trunc s_17_20 -> u8
        let s_17_21: bool = ((s_17_20) != 0);
        // D s_17_22: cast zx s_17_21 -> bv
        let s_17_22: Bits = Bits::new(s_17_21 as u128, 1u16);
        // D s_17_23: cast zx s_17_22 -> i
        let s_17_23: i128 = (s_17_22.value() as i128);
        // D s_17_24: cast reint s_17_23 -> i64
        let s_17_24: i64 = (s_17_23 as i64);
        // D s_17_25: write-var index <= s_17_24
        fn_state.index = s_17_24;
        // N s_17_26: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b10
        return block_10(state, tracer, fn_state);
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
        // C s_20_0: const #0s : i
        let s_20_0: i128 = 0;
        // D s_20_1: read-var Vd:u8
        let s_20_1: u8 = fn_state.Vd;
        // D s_20_2: cast zx s_20_1 -> bv
        let s_20_2: Bits = Bits::new(s_20_1 as u128, 4u16);
        // C s_20_3: const #1u : u64
        let s_20_3: u64 = 1;
        // D s_20_4: bit-extract s_20_2 s_20_0 s_20_3
        let s_20_4: Bits = (Bits::new(
            ((s_20_2) >> (s_20_0)).value(),
            u16::try_from(s_20_3).unwrap(),
        ));
        // D s_20_5: cast reint s_20_4 -> u8
        let s_20_5: bool = ((s_20_4.value()) != 0);
        // C s_20_6: const #0s : i
        let s_20_6: i128 = 0;
        // C s_20_7: const #0u : u64
        let s_20_7: u64 = 0;
        // D s_20_8: cast zx s_20_5 -> u64
        let s_20_8: u64 = (s_20_5 as u64);
        // C s_20_9: const #1u : u64
        let s_20_9: u64 = 1;
        // D s_20_10: and s_20_8 s_20_9
        let s_20_10: u64 = ((s_20_8) & (s_20_9));
        // D s_20_11: cmp-eq s_20_10 s_20_9
        let s_20_11: bool = ((s_20_10) == (s_20_9));
        // D s_20_12: lsl s_20_8 s_20_6
        let s_20_12: u64 = s_20_8 << s_20_6;
        // D s_20_13: or s_20_7 s_20_12
        let s_20_13: u64 = ((s_20_7) | (s_20_12));
        // D s_20_14: cmpl s_20_12
        let s_20_14: u64 = !s_20_12;
        // D s_20_15: and s_20_7 s_20_14
        let s_20_15: u64 = ((s_20_7) & (s_20_14));
        // D s_20_16: select s_20_11 s_20_13 s_20_15
        let s_20_16: u64 = if s_20_11 { s_20_13 } else { s_20_15 };
        // D s_20_17: cast trunc s_20_16 -> u8
        let s_20_17: bool = ((s_20_16) != 0);
        // D s_20_18: cast zx s_20_17 -> bv
        let s_20_18: Bits = Bits::new(s_20_17 as u128, 1u16);
        // C s_20_19: const #1u : u8
        let s_20_19: bool = true;
        // C s_20_20: cast zx s_20_19 -> bv
        let s_20_20: Bits = Bits::new(s_20_19 as u128, 1u16);
        // D s_20_21: cmp-eq s_20_18 s_20_20
        let s_20_21: bool = ((s_20_18) == (s_20_20));
        // D s_20_22: write-var gs#308665 <= s_20_21
        fn_state.gs_308665 = s_20_21;
        // N s_20_23: jump b7
        return block_7(state, tracer, fn_state);
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
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#308658 <= s_22_0
        fn_state.gs_308658 = s_22_0;
        // N s_22_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
