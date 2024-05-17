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
use execute_aarch32_instrs_VSHLL_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VSHLL_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    D: bool,
    imm6: u8,
    Vd: u8,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        elementsshadow_7802: i64,
        esizeshadow_7801: i64,
        gs_318877: bool,
        elements: i64,
        shift_amount: i128,
        shift_amountshadow_7800: i128,
        U: bool,
        D: bool,
        imm6: u8,
        Vd: u8,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        U,
        D,
        imm6,
        Vd,
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
        // N s_2_14: branch s_2_13 b18 b3
        if s_2_13 {
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
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var gs#318877 <= s_3_0
        fn_state.gs_318877 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#318877:u8
        let s_4_0: bool = fn_state.gs_318877;
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
        // C s_5_0: const #0s : i
        let s_5_0: i128 = 0;
        // D s_5_1: read-var Vd:u8
        let s_5_1: u8 = fn_state.Vd;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 4u16);
        // C s_5_3: const #1u : u64
        let s_5_3: u64 = 1;
        // D s_5_4: bit-extract s_5_2 s_5_0 s_5_3
        let s_5_4: Bits = (Bits::new(
            ((s_5_2) >> (s_5_0)).value(),
            u16::try_from(s_5_3).unwrap(),
        ));
        // D s_5_5: cast reint s_5_4 -> u8
        let s_5_5: bool = ((s_5_4.value()) != 0);
        // C s_5_6: const #0s : i
        let s_5_6: i128 = 0;
        // C s_5_7: const #0u : u64
        let s_5_7: u64 = 0;
        // D s_5_8: cast zx s_5_5 -> u64
        let s_5_8: u64 = (s_5_5 as u64);
        // C s_5_9: const #1u : u64
        let s_5_9: u64 = 1;
        // D s_5_10: and s_5_8 s_5_9
        let s_5_10: u64 = ((s_5_8) & (s_5_9));
        // D s_5_11: cmp-eq s_5_10 s_5_9
        let s_5_11: bool = ((s_5_10) == (s_5_9));
        // D s_5_12: lsl s_5_8 s_5_6
        let s_5_12: u64 = s_5_8 << s_5_6;
        // D s_5_13: or s_5_7 s_5_12
        let s_5_13: u64 = ((s_5_7) | (s_5_12));
        // D s_5_14: cmpl s_5_12
        let s_5_14: u64 = !s_5_12;
        // D s_5_15: and s_5_7 s_5_14
        let s_5_15: u64 = ((s_5_7) & (s_5_14));
        // D s_5_16: select s_5_11 s_5_13 s_5_15
        let s_5_16: u64 = if s_5_11 { s_5_13 } else { s_5_15 };
        // D s_5_17: cast trunc s_5_16 -> u8
        let s_5_17: bool = ((s_5_16) != 0);
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 1u16);
        // C s_5_19: const #1u : u8
        let s_5_19: bool = true;
        // C s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 1u16);
        // D s_5_21: cmp-eq s_5_18 s_5_20
        let s_5_21: bool = ((s_5_18) == (s_5_20));
        // N s_5_22: branch s_5_21 b16 b6
        if s_5_21 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #8s : i64
        let s_6_0: i64 = 8;
        // D s_6_1: write-var esize <= s_6_0
        fn_state.esize = s_6_0;
        // C s_6_2: const #2s : i64
        let s_6_2: i64 = 2;
        // D s_6_3: write-var elements <= s_6_2
        fn_state.elements = s_6_2;
        // D s_6_4: read-var imm6:u8
        let s_6_4: u8 = fn_state.imm6;
        // C s_6_5: const #3s : i
        let s_6_5: i128 = 3;
        // D s_6_6: cast zx s_6_4 -> bv
        let s_6_6: Bits = Bits::new(s_6_4 as u128, 6u16);
        // C s_6_7: const #1s : i64
        let s_6_7: i64 = 1;
        // C s_6_8: cast zx s_6_7 -> i
        let s_6_8: i128 = (i128::try_from(s_6_7).unwrap());
        // C s_6_9: const #2s : i
        let s_6_9: i128 = 2;
        // C s_6_10: add s_6_9 s_6_8
        let s_6_10: i128 = (s_6_9 + s_6_8);
        // D s_6_11: bit-extract s_6_6 s_6_5 s_6_10
        let s_6_11: Bits = (Bits::new(
            ((s_6_6) >> (s_6_5)).value(),
            u16::try_from(s_6_10).unwrap(),
        ));
        // D s_6_12: cast reint s_6_11 -> u8
        let s_6_12: u8 = (s_6_11.value() as u8);
        // D s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 3u16);
        // C s_6_14: const #1u : u8
        let s_6_14: u8 = 1;
        // C s_6_15: cast zx s_6_14 -> bv
        let s_6_15: Bits = Bits::new(s_6_14 as u128, 3u16);
        // D s_6_16: cmp-eq s_6_13 s_6_15
        let s_6_16: bool = ((s_6_13) == (s_6_15));
        // D s_6_17: not s_6_16
        let s_6_17: bool = !s_6_16;
        // N s_6_18: branch s_6_17 b11 b7
        if s_6_17 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #8s : i64
        let s_7_0: i64 = 8;
        // D s_7_1: write-var esize <= s_7_0
        fn_state.esize = s_7_0;
        // C s_7_2: const #8s : i64
        let s_7_2: i64 = 8;
        // D s_7_3: write-var elements <= s_7_2
        fn_state.elements = s_7_2;
        // D s_7_4: read-var imm6:u8
        let s_7_4: u8 = fn_state.imm6;
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 6u16);
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (s_7_5.value() as i128);
        // D s_7_7: cast reint s_7_6 -> i64
        let s_7_7: i64 = (s_7_6 as i64);
        // C s_7_8: const #8s : i
        let s_7_8: i128 = 8;
        // D s_7_9: cast zx s_7_7 -> i
        let s_7_9: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_10: sub s_7_9 s_7_8
        let s_7_10: i128 = ((s_7_9) - (s_7_8));
        // D s_7_11: write-var shift_amount <= s_7_10
        fn_state.shift_amount = s_7_10;
        // N s_7_12: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var shift_amount:i
        let s_8_0: i128 = fn_state.shift_amount;
        // D s_8_1: write-var shift_amountshadow#7800 <= s_8_0
        fn_state.shift_amountshadow_7800 = s_8_0;
        // D s_8_2: read-var esize:i64
        let s_8_2: i64 = fn_state.esize;
        // D s_8_3: write-var esizeshadow#7801 <= s_8_2
        fn_state.esizeshadow_7801 = s_8_2;
        // D s_8_4: read-var elements:i64
        let s_8_4: i64 = fn_state.elements;
        // D s_8_5: write-var elementsshadow#7802 <= s_8_4
        fn_state.elementsshadow_7802 = s_8_4;
        // C s_8_6: const #0s : i
        let s_8_6: i128 = 0;
        // D s_8_7: read-var shift_amountshadow#7800:i
        let s_8_7: i128 = fn_state.shift_amountshadow_7800;
        // D s_8_8: cmp-eq s_8_7 s_8_6
        let s_8_8: bool = ((s_8_7) == (s_8_6));
        // N s_8_9: branch s_8_8 b10 b9
        if s_8_8 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var shift_amountshadow#7800:i
        let s_9_0: i128 = fn_state.shift_amountshadow_7800;
        // D s_9_1: read-var U:u8
        let s_9_1: bool = fn_state.U;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 1u16);
        // C s_9_3: const #1u : u8
        let s_9_3: bool = true;
        // C s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 1u16);
        // D s_9_5: cmp-eq s_9_2 s_9_4
        let s_9_5: bool = ((s_9_2) == (s_9_4));
        // D s_9_6: read-var D:u8
        let s_9_6: bool = fn_state.D;
        // D s_9_7: cast zx s_9_6 -> bv
        let s_9_7: Bits = Bits::new(s_9_6 as u128, 1u16);
        // D s_9_8: read-var Vd:u8
        let s_9_8: u8 = fn_state.Vd;
        // D s_9_9: cast zx s_9_8 -> bv
        let s_9_9: Bits = Bits::new(s_9_8 as u128, 4u16);
        // D s_9_10: cast reint s_9_7 -> u128
        let s_9_10: u128 = (s_9_7.value() as u128);
        // D s_9_11: size-of s_9_7
        let s_9_11: u16 = s_9_7.length();
        // D s_9_12: cast reint s_9_9 -> u128
        let s_9_12: u128 = (s_9_9.value() as u128);
        // D s_9_13: size-of s_9_9
        let s_9_13: u16 = s_9_9.length();
        // D s_9_14: lsl s_9_10 s_9_13
        let s_9_14: u128 = s_9_10 << s_9_13;
        // D s_9_15: or s_9_14 s_9_12
        let s_9_15: u128 = ((s_9_14) | (s_9_12));
        // D s_9_16: add s_9_11 s_9_13
        let s_9_16: u16 = (s_9_11 + s_9_13);
        // D s_9_17: create-bits s_9_15 s_9_16
        let s_9_17: Bits = Bits::new(s_9_15, s_9_16);
        // D s_9_18: cast reint s_9_17 -> u8
        let s_9_18: u8 = (s_9_17.value() as u8);
        // D s_9_19: cast zx s_9_18 -> bv
        let s_9_19: Bits = Bits::new(s_9_18 as u128, 5u16);
        // D s_9_20: cast zx s_9_19 -> i
        let s_9_20: i128 = (s_9_19.value() as i128);
        // D s_9_21: cast reint s_9_20 -> i64
        let s_9_21: i64 = (s_9_20 as i64);
        // D s_9_22: read-var M:u8
        let s_9_22: bool = fn_state.M;
        // D s_9_23: cast zx s_9_22 -> bv
        let s_9_23: Bits = Bits::new(s_9_22 as u128, 1u16);
        // D s_9_24: read-var Vm:u8
        let s_9_24: u8 = fn_state.Vm;
        // D s_9_25: cast zx s_9_24 -> bv
        let s_9_25: Bits = Bits::new(s_9_24 as u128, 4u16);
        // D s_9_26: cast reint s_9_23 -> u128
        let s_9_26: u128 = (s_9_23.value() as u128);
        // D s_9_27: size-of s_9_23
        let s_9_27: u16 = s_9_23.length();
        // D s_9_28: cast reint s_9_25 -> u128
        let s_9_28: u128 = (s_9_25.value() as u128);
        // D s_9_29: size-of s_9_25
        let s_9_29: u16 = s_9_25.length();
        // D s_9_30: lsl s_9_26 s_9_29
        let s_9_30: u128 = s_9_26 << s_9_29;
        // D s_9_31: or s_9_30 s_9_28
        let s_9_31: u128 = ((s_9_30) | (s_9_28));
        // D s_9_32: add s_9_27 s_9_29
        let s_9_32: u16 = (s_9_27 + s_9_29);
        // D s_9_33: create-bits s_9_31 s_9_32
        let s_9_33: Bits = Bits::new(s_9_31, s_9_32);
        // D s_9_34: cast reint s_9_33 -> u8
        let s_9_34: u8 = (s_9_33.value() as u8);
        // D s_9_35: cast zx s_9_34 -> bv
        let s_9_35: Bits = Bits::new(s_9_34 as u128, 5u16);
        // D s_9_36: cast zx s_9_35 -> i
        let s_9_36: i128 = (s_9_35.value() as i128);
        // D s_9_37: cast reint s_9_36 -> i64
        let s_9_37: i64 = (s_9_36 as i64);
        // D s_9_38: read-var esizeshadow#7801:i64
        let s_9_38: i64 = fn_state.esizeshadow_7801;
        // D s_9_39: cast zx s_9_38 -> i
        let s_9_39: i128 = (i128::try_from(s_9_38).unwrap());
        // D s_9_40: cast reint s_9_39 -> i64
        let s_9_40: i64 = (s_9_39 as i64);
        // D s_9_41: read-var elementsshadow#7802:i64
        let s_9_41: i64 = fn_state.elementsshadow_7802;
        // D s_9_42: cast zx s_9_41 -> i
        let s_9_42: i128 = (i128::try_from(s_9_41).unwrap());
        // D s_9_43: call execute_aarch32_instrs_VSHLL_Op_A_txt(s_9_21, s_9_42, s_9_40, s_9_37, s_9_0, s_9_5)
        let s_9_43: () = execute_aarch32_instrs_VSHLL_Op_A_txt(
            state,
            tracer,
            s_9_21,
            s_9_42,
            s_9_40,
            s_9_37,
            s_9_0,
            s_9_5,
        );
        // N s_9_44: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var imm6:u8
        let s_11_0: u8 = fn_state.imm6;
        // C s_11_1: const #4s : i
        let s_11_1: i128 = 4;
        // D s_11_2: cast zx s_11_0 -> bv
        let s_11_2: Bits = Bits::new(s_11_0 as u128, 6u16);
        // C s_11_3: const #1s : i64
        let s_11_3: i64 = 1;
        // C s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // C s_11_5: const #1s : i
        let s_11_5: i128 = 1;
        // C s_11_6: add s_11_5 s_11_4
        let s_11_6: i128 = (s_11_5 + s_11_4);
        // D s_11_7: bit-extract s_11_2 s_11_1 s_11_6
        let s_11_7: Bits = (Bits::new(
            ((s_11_2) >> (s_11_1)).value(),
            u16::try_from(s_11_6).unwrap(),
        ));
        // D s_11_8: cast reint s_11_7 -> u8
        let s_11_8: u8 = (s_11_7.value() as u8);
        // D s_11_9: cast zx s_11_8 -> bv
        let s_11_9: Bits = Bits::new(s_11_8 as u128, 2u16);
        // C s_11_10: const #1u : u8
        let s_11_10: u8 = 1;
        // C s_11_11: cast zx s_11_10 -> bv
        let s_11_11: Bits = Bits::new(s_11_10 as u128, 2u16);
        // D s_11_12: cmp-eq s_11_9 s_11_11
        let s_11_12: bool = ((s_11_9) == (s_11_11));
        // D s_11_13: not s_11_12
        let s_11_13: bool = !s_11_12;
        // N s_11_14: branch s_11_13 b13 b12
        if s_11_13 {
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
        // C s_12_0: const #16s : i64
        let s_12_0: i64 = 16;
        // D s_12_1: write-var esize <= s_12_0
        fn_state.esize = s_12_0;
        // C s_12_2: const #4s : i64
        let s_12_2: i64 = 4;
        // D s_12_3: write-var elements <= s_12_2
        fn_state.elements = s_12_2;
        // D s_12_4: read-var imm6:u8
        let s_12_4: u8 = fn_state.imm6;
        // D s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 6u16);
        // D s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (s_12_5.value() as i128);
        // D s_12_7: cast reint s_12_6 -> i64
        let s_12_7: i64 = (s_12_6 as i64);
        // C s_12_8: const #16s : i
        let s_12_8: i128 = 16;
        // D s_12_9: cast zx s_12_7 -> i
        let s_12_9: i128 = (i128::try_from(s_12_7).unwrap());
        // D s_12_10: sub s_12_9 s_12_8
        let s_12_10: i128 = ((s_12_9) - (s_12_8));
        // D s_12_11: write-var shift_amount <= s_12_10
        fn_state.shift_amount = s_12_10;
        // N s_12_12: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var imm6:u8
        let s_13_0: u8 = fn_state.imm6;
        // C s_13_1: const #5s : i
        let s_13_1: i128 = 5;
        // D s_13_2: cast zx s_13_0 -> bv
        let s_13_2: Bits = Bits::new(s_13_0 as u128, 6u16);
        // C s_13_3: const #1s : i64
        let s_13_3: i64 = 1;
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #0s : i
        let s_13_5: i128 = 0;
        // C s_13_6: add s_13_5 s_13_4
        let s_13_6: i128 = (s_13_5 + s_13_4);
        // D s_13_7: bit-extract s_13_2 s_13_1 s_13_6
        let s_13_7: Bits = (Bits::new(
            ((s_13_2) >> (s_13_1)).value(),
            u16::try_from(s_13_6).unwrap(),
        ));
        // D s_13_8: cast reint s_13_7 -> u8
        let s_13_8: bool = ((s_13_7.value()) != 0);
        // D s_13_9: cast zx s_13_8 -> bv
        let s_13_9: Bits = Bits::new(s_13_8 as u128, 1u16);
        // C s_13_10: const #1u : u8
        let s_13_10: bool = true;
        // C s_13_11: cast zx s_13_10 -> bv
        let s_13_11: Bits = Bits::new(s_13_10 as u128, 1u16);
        // D s_13_12: cmp-eq s_13_9 s_13_11
        let s_13_12: bool = ((s_13_9) == (s_13_11));
        // D s_13_13: not s_13_12
        let s_13_13: bool = !s_13_12;
        // N s_13_14: branch s_13_13 b15 b14
        if s_13_13 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #32s : i64
        let s_14_0: i64 = 32;
        // D s_14_1: write-var esize <= s_14_0
        fn_state.esize = s_14_0;
        // C s_14_2: const #2s : i64
        let s_14_2: i64 = 2;
        // D s_14_3: write-var elements <= s_14_2
        fn_state.elements = s_14_2;
        // D s_14_4: read-var imm6:u8
        let s_14_4: u8 = fn_state.imm6;
        // D s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 6u16);
        // D s_14_6: cast zx s_14_5 -> i
        let s_14_6: i128 = (s_14_5.value() as i128);
        // D s_14_7: cast reint s_14_6 -> i64
        let s_14_7: i64 = (s_14_6 as i64);
        // C s_14_8: const #32s : i
        let s_14_8: i128 = 32;
        // D s_14_9: cast zx s_14_7 -> i
        let s_14_9: i128 = (i128::try_from(s_14_7).unwrap());
        // D s_14_10: sub s_14_9 s_14_8
        let s_14_10: i128 = ((s_14_9) - (s_14_8));
        // D s_14_11: write-var shift_amount <= s_14_10
        fn_state.shift_amount = s_14_10;
        // N s_14_12: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b8
        return block_8(state, tracer, fn_state);
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
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#318877 <= s_18_0
        fn_state.gs_318877 = s_18_0;
        // N s_18_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
