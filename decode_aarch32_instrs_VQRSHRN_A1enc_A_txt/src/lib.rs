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
use execute_aarch32_instrs_VQRSHRN_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VQRSHRN_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    D: bool,
    imm6: u8,
    Vd: u8,
    op: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_316912: bool,
        esize: i64,
        gs_316950: bool,
        shift_amountshadow_7719: i128,
        gs_316907: bool,
        elementsshadow_7721: i64,
        esizeshadow_7720: i64,
        elements: i64,
        shift_amount: i128,
        U: bool,
        D: bool,
        imm6: u8,
        Vd: u8,
        op: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        U,
        D,
        imm6,
        Vd,
        op,
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
        // D s_2_0: read-var imm6:u8
        let s_2_0: u8 = fn_state.imm6;
        // C s_2_1: const #3s : i
        let s_2_1: i128 = 3;
        // D s_2_2: cast zx s_2_0 -> bv
        let s_2_2: Bits = Bits::new(s_2_0 as u128, 6u16);
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
        // N s_2_14: branch s_2_13 b24 b3
        if s_2_13 {
            return block_24(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#316907 <= s_3_0
        fn_state.gs_316907 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#316907:u8
        let s_4_0: bool = fn_state.gs_316907;
        // N s_4_1: branch s_4_0 b23 b5
        if s_4_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var U:u8
        let s_5_0: bool = fn_state.U;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b22 b6
        if s_5_4 {
            return block_22(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#316912 <= s_6_0
        fn_state.gs_316912 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#316912:u8
        let s_7_0: bool = fn_state.gs_316912;
        // N s_7_1: branch s_7_0 b21 b8
        if s_7_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0s : i
        let s_8_0: i128 = 0;
        // D s_8_1: read-var Vm:u8
        let s_8_1: u8 = fn_state.Vm;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 4u16);
        // C s_8_3: const #1u : u64
        let s_8_3: u64 = 1;
        // D s_8_4: bit-extract s_8_2 s_8_0 s_8_3
        let s_8_4: Bits = (Bits::new(
            ((s_8_2) >> (s_8_0)).value(),
            u16::try_from(s_8_3).unwrap(),
        ));
        // D s_8_5: cast reint s_8_4 -> u8
        let s_8_5: bool = ((s_8_4.value()) != 0);
        // C s_8_6: const #0s : i
        let s_8_6: i128 = 0;
        // C s_8_7: const #0u : u64
        let s_8_7: u64 = 0;
        // D s_8_8: cast zx s_8_5 -> u64
        let s_8_8: u64 = (s_8_5 as u64);
        // C s_8_9: const #1u : u64
        let s_8_9: u64 = 1;
        // D s_8_10: and s_8_8 s_8_9
        let s_8_10: u64 = ((s_8_8) & (s_8_9));
        // D s_8_11: cmp-eq s_8_10 s_8_9
        let s_8_11: bool = ((s_8_10) == (s_8_9));
        // D s_8_12: lsl s_8_8 s_8_6
        let s_8_12: u64 = s_8_8 << s_8_6;
        // D s_8_13: or s_8_7 s_8_12
        let s_8_13: u64 = ((s_8_7) | (s_8_12));
        // D s_8_14: cmpl s_8_12
        let s_8_14: u64 = !s_8_12;
        // D s_8_15: and s_8_7 s_8_14
        let s_8_15: u64 = ((s_8_7) & (s_8_14));
        // D s_8_16: select s_8_11 s_8_13 s_8_15
        let s_8_16: u64 = if s_8_11 { s_8_13 } else { s_8_15 };
        // D s_8_17: cast trunc s_8_16 -> u8
        let s_8_17: bool = ((s_8_16) != 0);
        // D s_8_18: cast zx s_8_17 -> bv
        let s_8_18: Bits = Bits::new(s_8_17 as u128, 1u16);
        // C s_8_19: const #1u : u8
        let s_8_19: bool = true;
        // C s_8_20: cast zx s_8_19 -> bv
        let s_8_20: Bits = Bits::new(s_8_19 as u128, 1u16);
        // D s_8_21: cmp-eq s_8_18 s_8_20
        let s_8_21: bool = ((s_8_18) == (s_8_20));
        // N s_8_22: branch s_8_21 b20 b9
        if s_8_21 {
            return block_20(state, tracer, fn_state);
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
        // C s_9_2: const #2s : i64
        let s_9_2: i64 = 2;
        // D s_9_3: write-var elements <= s_9_2
        fn_state.elements = s_9_2;
        // D s_9_4: read-var imm6:u8
        let s_9_4: u8 = fn_state.imm6;
        // C s_9_5: const #3s : i
        let s_9_5: i128 = 3;
        // D s_9_6: cast zx s_9_4 -> bv
        let s_9_6: Bits = Bits::new(s_9_4 as u128, 6u16);
        // C s_9_7: const #1s : i64
        let s_9_7: i64 = 1;
        // C s_9_8: cast zx s_9_7 -> i
        let s_9_8: i128 = (i128::try_from(s_9_7).unwrap());
        // C s_9_9: const #2s : i
        let s_9_9: i128 = 2;
        // C s_9_10: add s_9_9 s_9_8
        let s_9_10: i128 = (s_9_9 + s_9_8);
        // D s_9_11: bit-extract s_9_6 s_9_5 s_9_10
        let s_9_11: Bits = (Bits::new(
            ((s_9_6) >> (s_9_5)).value(),
            u16::try_from(s_9_10).unwrap(),
        ));
        // D s_9_12: cast reint s_9_11 -> u8
        let s_9_12: u8 = (s_9_11.value() as u8);
        // D s_9_13: cast zx s_9_12 -> bv
        let s_9_13: Bits = Bits::new(s_9_12 as u128, 3u16);
        // C s_9_14: const #1u : u8
        let s_9_14: u8 = 1;
        // C s_9_15: cast zx s_9_14 -> bv
        let s_9_15: Bits = Bits::new(s_9_14 as u128, 3u16);
        // D s_9_16: cmp-eq s_9_13 s_9_15
        let s_9_16: bool = ((s_9_13) == (s_9_15));
        // D s_9_17: not s_9_16
        let s_9_17: bool = !s_9_16;
        // N s_9_18: branch s_9_17 b15 b10
        if s_9_17 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #8s : i64
        let s_10_0: i64 = 8;
        // D s_10_1: write-var esize <= s_10_0
        fn_state.esize = s_10_0;
        // C s_10_2: const #8s : i64
        let s_10_2: i64 = 8;
        // D s_10_3: write-var elements <= s_10_2
        fn_state.elements = s_10_2;
        // D s_10_4: read-var imm6:u8
        let s_10_4: u8 = fn_state.imm6;
        // D s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 6u16);
        // D s_10_6: cast zx s_10_5 -> i
        let s_10_6: i128 = (s_10_5.value() as i128);
        // D s_10_7: cast reint s_10_6 -> i64
        let s_10_7: i64 = (s_10_6 as i64);
        // C s_10_8: const #16s : i
        let s_10_8: i128 = 16;
        // D s_10_9: cast zx s_10_7 -> i
        let s_10_9: i128 = (i128::try_from(s_10_7).unwrap());
        // D s_10_10: sub s_10_8 s_10_9
        let s_10_10: i128 = ((s_10_8) - (s_10_9));
        // D s_10_11: write-var shift_amount <= s_10_10
        fn_state.shift_amount = s_10_10;
        // N s_10_12: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var shift_amount:i
        let s_11_0: i128 = fn_state.shift_amount;
        // D s_11_1: write-var shift_amountshadow#7719 <= s_11_0
        fn_state.shift_amountshadow_7719 = s_11_0;
        // D s_11_2: read-var esize:i64
        let s_11_2: i64 = fn_state.esize;
        // D s_11_3: write-var esizeshadow#7720 <= s_11_2
        fn_state.esizeshadow_7720 = s_11_2;
        // D s_11_4: read-var elements:i64
        let s_11_4: i64 = fn_state.elements;
        // D s_11_5: write-var elementsshadow#7721 <= s_11_4
        fn_state.elementsshadow_7721 = s_11_4;
        // D s_11_6: read-var U:u8
        let s_11_6: bool = fn_state.U;
        // D s_11_7: cast zx s_11_6 -> bv
        let s_11_7: Bits = Bits::new(s_11_6 as u128, 1u16);
        // C s_11_8: const #1u : u8
        let s_11_8: bool = true;
        // C s_11_9: cast zx s_11_8 -> bv
        let s_11_9: Bits = Bits::new(s_11_8 as u128, 1u16);
        // D s_11_10: cmp-eq s_11_7 s_11_9
        let s_11_10: bool = ((s_11_7) == (s_11_9));
        // N s_11_11: branch s_11_10 b14 b12
        if s_11_10 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#316950 <= s_12_0
        fn_state.gs_316950 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#316950:u8
        let s_13_0: bool = fn_state.gs_316950;
        // D s_13_1: read-var U:u8
        let s_13_1: bool = fn_state.U;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 1u16);
        // C s_13_3: const #1u : u8
        let s_13_3: bool = true;
        // C s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 1u16);
        // D s_13_5: cmp-eq s_13_2 s_13_4
        let s_13_5: bool = ((s_13_2) == (s_13_4));
        // D s_13_6: read-var D:u8
        let s_13_6: bool = fn_state.D;
        // D s_13_7: cast zx s_13_6 -> bv
        let s_13_7: Bits = Bits::new(s_13_6 as u128, 1u16);
        // D s_13_8: read-var Vd:u8
        let s_13_8: u8 = fn_state.Vd;
        // D s_13_9: cast zx s_13_8 -> bv
        let s_13_9: Bits = Bits::new(s_13_8 as u128, 4u16);
        // D s_13_10: cast reint s_13_7 -> u128
        let s_13_10: u128 = (s_13_7.value() as u128);
        // D s_13_11: size-of s_13_7
        let s_13_11: u16 = s_13_7.length();
        // D s_13_12: cast reint s_13_9 -> u128
        let s_13_12: u128 = (s_13_9.value() as u128);
        // D s_13_13: size-of s_13_9
        let s_13_13: u16 = s_13_9.length();
        // D s_13_14: lsl s_13_10 s_13_13
        let s_13_14: u128 = s_13_10 << s_13_13;
        // D s_13_15: or s_13_14 s_13_12
        let s_13_15: u128 = ((s_13_14) | (s_13_12));
        // D s_13_16: add s_13_11 s_13_13
        let s_13_16: u16 = (s_13_11 + s_13_13);
        // D s_13_17: create-bits s_13_15 s_13_16
        let s_13_17: Bits = Bits::new(s_13_15, s_13_16);
        // D s_13_18: cast reint s_13_17 -> u8
        let s_13_18: u8 = (s_13_17.value() as u8);
        // D s_13_19: cast zx s_13_18 -> bv
        let s_13_19: Bits = Bits::new(s_13_18 as u128, 5u16);
        // D s_13_20: cast zx s_13_19 -> i
        let s_13_20: i128 = (s_13_19.value() as i128);
        // D s_13_21: cast reint s_13_20 -> i64
        let s_13_21: i64 = (s_13_20 as i64);
        // D s_13_22: read-var M:u8
        let s_13_22: bool = fn_state.M;
        // D s_13_23: cast zx s_13_22 -> bv
        let s_13_23: Bits = Bits::new(s_13_22 as u128, 1u16);
        // D s_13_24: read-var Vm:u8
        let s_13_24: u8 = fn_state.Vm;
        // D s_13_25: cast zx s_13_24 -> bv
        let s_13_25: Bits = Bits::new(s_13_24 as u128, 4u16);
        // D s_13_26: cast reint s_13_23 -> u128
        let s_13_26: u128 = (s_13_23.value() as u128);
        // D s_13_27: size-of s_13_23
        let s_13_27: u16 = s_13_23.length();
        // D s_13_28: cast reint s_13_25 -> u128
        let s_13_28: u128 = (s_13_25.value() as u128);
        // D s_13_29: size-of s_13_25
        let s_13_29: u16 = s_13_25.length();
        // D s_13_30: lsl s_13_26 s_13_29
        let s_13_30: u128 = s_13_26 << s_13_29;
        // D s_13_31: or s_13_30 s_13_28
        let s_13_31: u128 = ((s_13_30) | (s_13_28));
        // D s_13_32: add s_13_27 s_13_29
        let s_13_32: u16 = (s_13_27 + s_13_29);
        // D s_13_33: create-bits s_13_31 s_13_32
        let s_13_33: Bits = Bits::new(s_13_31, s_13_32);
        // D s_13_34: cast reint s_13_33 -> u8
        let s_13_34: u8 = (s_13_33.value() as u8);
        // D s_13_35: cast zx s_13_34 -> bv
        let s_13_35: Bits = Bits::new(s_13_34 as u128, 5u16);
        // D s_13_36: cast zx s_13_35 -> i
        let s_13_36: i128 = (s_13_35.value() as i128);
        // D s_13_37: cast reint s_13_36 -> i64
        let s_13_37: i64 = (s_13_36 as i64);
        // D s_13_38: read-var esizeshadow#7720:i64
        let s_13_38: i64 = fn_state.esizeshadow_7720;
        // D s_13_39: cast zx s_13_38 -> i
        let s_13_39: i128 = (i128::try_from(s_13_38).unwrap());
        // D s_13_40: cast reint s_13_39 -> i64
        let s_13_40: i64 = (s_13_39 as i64);
        // D s_13_41: read-var elementsshadow#7721:i64
        let s_13_41: i64 = fn_state.elementsshadow_7721;
        // D s_13_42: read-var shift_amountshadow#7719:i
        let s_13_42: i128 = fn_state.shift_amountshadow_7719;
        // D s_13_43: call execute_aarch32_instrs_VQRSHRN_Op_A_txt(s_13_21, s_13_5, s_13_41, s_13_40, s_13_37, s_13_42, s_13_0)
        let s_13_43: () = execute_aarch32_instrs_VQRSHRN_Op_A_txt(
            state,
            tracer,
            s_13_21,
            s_13_5,
            s_13_41,
            s_13_40,
            s_13_37,
            s_13_42,
            s_13_0,
        );
        // N s_13_44: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var op:u8
        let s_14_0: bool = fn_state.op;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: write-var gs#316950 <= s_14_4
        fn_state.gs_316950 = s_14_4;
        // N s_14_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var imm6:u8
        let s_15_0: u8 = fn_state.imm6;
        // C s_15_1: const #4s : i
        let s_15_1: i128 = 4;
        // D s_15_2: cast zx s_15_0 -> bv
        let s_15_2: Bits = Bits::new(s_15_0 as u128, 6u16);
        // C s_15_3: const #1s : i64
        let s_15_3: i64 = 1;
        // C s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // C s_15_5: const #1s : i
        let s_15_5: i128 = 1;
        // C s_15_6: add s_15_5 s_15_4
        let s_15_6: i128 = (s_15_5 + s_15_4);
        // D s_15_7: bit-extract s_15_2 s_15_1 s_15_6
        let s_15_7: Bits = (Bits::new(
            ((s_15_2) >> (s_15_1)).value(),
            u16::try_from(s_15_6).unwrap(),
        ));
        // D s_15_8: cast reint s_15_7 -> u8
        let s_15_8: u8 = (s_15_7.value() as u8);
        // D s_15_9: cast zx s_15_8 -> bv
        let s_15_9: Bits = Bits::new(s_15_8 as u128, 2u16);
        // C s_15_10: const #1u : u8
        let s_15_10: u8 = 1;
        // C s_15_11: cast zx s_15_10 -> bv
        let s_15_11: Bits = Bits::new(s_15_10 as u128, 2u16);
        // D s_15_12: cmp-eq s_15_9 s_15_11
        let s_15_12: bool = ((s_15_9) == (s_15_11));
        // D s_15_13: not s_15_12
        let s_15_13: bool = !s_15_12;
        // N s_15_14: branch s_15_13 b17 b16
        if s_15_13 {
            return block_17(state, tracer, fn_state);
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
        // D s_16_4: read-var imm6:u8
        let s_16_4: u8 = fn_state.imm6;
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 6u16);
        // D s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (s_16_5.value() as i128);
        // D s_16_7: cast reint s_16_6 -> i64
        let s_16_7: i64 = (s_16_6 as i64);
        // C s_16_8: const #32s : i
        let s_16_8: i128 = 32;
        // D s_16_9: cast zx s_16_7 -> i
        let s_16_9: i128 = (i128::try_from(s_16_7).unwrap());
        // D s_16_10: sub s_16_8 s_16_9
        let s_16_10: i128 = ((s_16_8) - (s_16_9));
        // D s_16_11: write-var shift_amount <= s_16_10
        fn_state.shift_amount = s_16_10;
        // N s_16_12: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var imm6:u8
        let s_17_0: u8 = fn_state.imm6;
        // C s_17_1: const #5s : i
        let s_17_1: i128 = 5;
        // D s_17_2: cast zx s_17_0 -> bv
        let s_17_2: Bits = Bits::new(s_17_0 as u128, 6u16);
        // C s_17_3: const #1s : i64
        let s_17_3: i64 = 1;
        // C s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // C s_17_5: const #0s : i
        let s_17_5: i128 = 0;
        // C s_17_6: add s_17_5 s_17_4
        let s_17_6: i128 = (s_17_5 + s_17_4);
        // D s_17_7: bit-extract s_17_2 s_17_1 s_17_6
        let s_17_7: Bits = (Bits::new(
            ((s_17_2) >> (s_17_1)).value(),
            u16::try_from(s_17_6).unwrap(),
        ));
        // D s_17_8: cast reint s_17_7 -> u8
        let s_17_8: bool = ((s_17_7.value()) != 0);
        // D s_17_9: cast zx s_17_8 -> bv
        let s_17_9: Bits = Bits::new(s_17_8 as u128, 1u16);
        // C s_17_10: const #1u : u8
        let s_17_10: bool = true;
        // C s_17_11: cast zx s_17_10 -> bv
        let s_17_11: Bits = Bits::new(s_17_10 as u128, 1u16);
        // D s_17_12: cmp-eq s_17_9 s_17_11
        let s_17_12: bool = ((s_17_9) == (s_17_11));
        // D s_17_13: not s_17_12
        let s_17_13: bool = !s_17_12;
        // N s_17_14: branch s_17_13 b19 b18
        if s_17_13 {
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
        // C s_18_0: const #32s : i64
        let s_18_0: i64 = 32;
        // D s_18_1: write-var esize <= s_18_0
        fn_state.esize = s_18_0;
        // C s_18_2: const #2s : i64
        let s_18_2: i64 = 2;
        // D s_18_3: write-var elements <= s_18_2
        fn_state.elements = s_18_2;
        // D s_18_4: read-var imm6:u8
        let s_18_4: u8 = fn_state.imm6;
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 6u16);
        // D s_18_6: cast zx s_18_5 -> i
        let s_18_6: i128 = (s_18_5.value() as i128);
        // D s_18_7: cast reint s_18_6 -> i64
        let s_18_7: i64 = (s_18_6 as i64);
        // C s_18_8: const #64s : i
        let s_18_8: i128 = 64;
        // D s_18_9: cast zx s_18_7 -> i
        let s_18_9: i128 = (i128::try_from(s_18_7).unwrap());
        // D s_18_10: sub s_18_8 s_18_9
        let s_18_10: i128 = ((s_18_8) - (s_18_9));
        // D s_18_11: write-var shift_amount <= s_18_10
        fn_state.shift_amount = s_18_10;
        // N s_18_12: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: panic
        panic!("{:?}", ());
        // N s_20_1: return
        return;
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
        // D s_22_0: read-var op:u8
        let s_22_0: bool = fn_state.op;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #0u : u8
        let s_22_2: bool = false;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: write-var gs#316912 <= s_22_4
        fn_state.gs_316912 = s_22_4;
        // N s_22_6: jump b7
        return block_7(state, tracer, fn_state);
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
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#316907 <= s_24_0
        fn_state.gs_316907 = s_24_0;
        // N s_24_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
