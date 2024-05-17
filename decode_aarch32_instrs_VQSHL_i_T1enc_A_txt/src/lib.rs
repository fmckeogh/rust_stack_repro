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
use execute_aarch32_instrs_VQSHL_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VQSHL_i_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    D: bool,
    imm6: u8,
    Vd: u8,
    op: bool,
    L: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_7730: i64,
        m: i64,
        esize: i64,
        gs_317114: bool,
        shift_amountshadow_7729: i128,
        gs_317119: bool,
        ga_359636: i64,
        gs_317124: bool,
        regs: i64,
        dest_unsigned: bool,
        ga_359623: u8,
        gs_317168: bool,
        d: i64,
        src_unsigned: bool,
        elementsshadow_7731: i64,
        elements: i64,
        shift_amount: i128,
        gs_317125: bool,
        U: bool,
        D: bool,
        imm6: u8,
        Vd: u8,
        op: bool,
        L: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        U,
        D,
        imm6,
        Vd,
        op,
        L,
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
        // D s_2_0: read-var L:u8
        let s_2_0: bool = fn_state.L;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // D s_2_2: read-var imm6:u8
        let s_2_2: u8 = fn_state.imm6;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 6u16);
        // D s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // D s_2_12: cast reint s_2_11 -> u8
        let s_2_12: u8 = (s_2_11.value() as u8);
        // C s_2_13: const #3s : i
        let s_2_13: i128 = 3;
        // D s_2_14: cast zx s_2_12 -> bv
        let s_2_14: Bits = Bits::new(s_2_12 as u128, 7u16);
        // C s_2_15: const #1s : i64
        let s_2_15: i64 = 1;
        // C s_2_16: cast zx s_2_15 -> i
        let s_2_16: i128 = (i128::try_from(s_2_15).unwrap());
        // C s_2_17: const #3s : i
        let s_2_17: i128 = 3;
        // C s_2_18: add s_2_17 s_2_16
        let s_2_18: i128 = (s_2_17 + s_2_16);
        // D s_2_19: bit-extract s_2_14 s_2_13 s_2_18
        let s_2_19: Bits = (Bits::new(
            ((s_2_14) >> (s_2_13)).value(),
            u16::try_from(s_2_18).unwrap(),
        ));
        // D s_2_20: cast reint s_2_19 -> u8
        let s_2_20: u8 = (s_2_19.value() as u8);
        // D s_2_21: cast zx s_2_20 -> bv
        let s_2_21: Bits = Bits::new(s_2_20 as u128, 4u16);
        // C s_2_22: const #0u : u8
        let s_2_22: u8 = 0;
        // C s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 4u16);
        // D s_2_24: cmp-eq s_2_21 s_2_23
        let s_2_24: bool = ((s_2_21) == (s_2_23));
        // D s_2_25: not s_2_24
        let s_2_25: bool = !s_2_24;
        // N s_2_26: branch s_2_25 b43 b3
        if s_2_25 {
            return block_43(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#317114 <= s_3_0
        fn_state.gs_317114 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#317114:u8
        let s_4_0: bool = fn_state.gs_317114;
        // N s_4_1: branch s_4_0 b42 b5
        if s_4_0 {
            return block_42(state, tracer, fn_state);
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
        // N s_5_5: branch s_5_4 b41 b6
        if s_5_4 {
            return block_41(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#317119 <= s_6_0
        fn_state.gs_317119 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#317119:u8
        let s_7_0: bool = fn_state.gs_317119;
        // N s_7_1: branch s_7_0 b40 b8
        if s_7_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var Q:u8
        let s_8_0: bool = fn_state.Q;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #1u : u8
        let s_8_2: bool = true;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b36 b9
        if s_8_4 {
            return block_36(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#317125 <= s_9_0
        fn_state.gs_317125 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#317125:u8
        let s_10_0: bool = fn_state.gs_317125;
        // N s_10_1: branch s_10_0 b35 b11
        if s_10_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #8s : i64
        let s_11_0: i64 = 8;
        // D s_11_1: write-var esize <= s_11_0
        fn_state.esize = s_11_0;
        // C s_11_2: const #1s : i64
        let s_11_2: i64 = 1;
        // D s_11_3: write-var elements <= s_11_2
        fn_state.elements = s_11_2;
        // D s_11_4: read-var L:u8
        let s_11_4: bool = fn_state.L;
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // D s_11_6: read-var imm6:u8
        let s_11_6: u8 = fn_state.imm6;
        // D s_11_7: cast zx s_11_6 -> bv
        let s_11_7: Bits = Bits::new(s_11_6 as u128, 6u16);
        // D s_11_8: cast reint s_11_5 -> u128
        let s_11_8: u128 = (s_11_5.value() as u128);
        // D s_11_9: size-of s_11_5
        let s_11_9: u16 = s_11_5.length();
        // D s_11_10: cast reint s_11_7 -> u128
        let s_11_10: u128 = (s_11_7.value() as u128);
        // D s_11_11: size-of s_11_7
        let s_11_11: u16 = s_11_7.length();
        // D s_11_12: lsl s_11_8 s_11_11
        let s_11_12: u128 = s_11_8 << s_11_11;
        // D s_11_13: or s_11_12 s_11_10
        let s_11_13: u128 = ((s_11_12) | (s_11_10));
        // D s_11_14: add s_11_9 s_11_11
        let s_11_14: u16 = (s_11_9 + s_11_11);
        // D s_11_15: create-bits s_11_13 s_11_14
        let s_11_15: Bits = Bits::new(s_11_13, s_11_14);
        // D s_11_16: cast reint s_11_15 -> u8
        let s_11_16: u8 = (s_11_15.value() as u8);
        // D s_11_17: write-var ga#359623 <= s_11_16
        fn_state.ga_359623 = s_11_16;
        // D s_11_18: read-var ga#359623:u8
        let s_11_18: u8 = fn_state.ga_359623;
        // C s_11_19: const #3s : i
        let s_11_19: i128 = 3;
        // D s_11_20: cast zx s_11_18 -> bv
        let s_11_20: Bits = Bits::new(s_11_18 as u128, 7u16);
        // C s_11_21: const #1s : i64
        let s_11_21: i64 = 1;
        // C s_11_22: cast zx s_11_21 -> i
        let s_11_22: i128 = (i128::try_from(s_11_21).unwrap());
        // C s_11_23: const #3s : i
        let s_11_23: i128 = 3;
        // C s_11_24: add s_11_23 s_11_22
        let s_11_24: i128 = (s_11_23 + s_11_22);
        // D s_11_25: bit-extract s_11_20 s_11_19 s_11_24
        let s_11_25: Bits = (Bits::new(
            ((s_11_20) >> (s_11_19)).value(),
            u16::try_from(s_11_24).unwrap(),
        ));
        // D s_11_26: cast reint s_11_25 -> u8
        let s_11_26: u8 = (s_11_25.value() as u8);
        // D s_11_27: cast zx s_11_26 -> bv
        let s_11_27: Bits = Bits::new(s_11_26 as u128, 4u16);
        // C s_11_28: const #1u : u8
        let s_11_28: u8 = 1;
        // C s_11_29: cast zx s_11_28 -> bv
        let s_11_29: Bits = Bits::new(s_11_28 as u128, 4u16);
        // D s_11_30: cmp-eq s_11_27 s_11_29
        let s_11_30: bool = ((s_11_27) == (s_11_29));
        // D s_11_31: not s_11_30
        let s_11_31: bool = !s_11_30;
        // N s_11_32: branch s_11_31 b28 b12
        if s_11_31 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #8s : i64
        let s_12_0: i64 = 8;
        // D s_12_1: write-var esize <= s_12_0
        fn_state.esize = s_12_0;
        // C s_12_2: const #8s : i64
        let s_12_2: i64 = 8;
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
        // C s_12_8: const #8s : i
        let s_12_8: i128 = 8;
        // D s_12_9: cast zx s_12_7 -> i
        let s_12_9: i128 = (i128::try_from(s_12_7).unwrap());
        // D s_12_10: sub s_12_9 s_12_8
        let s_12_10: i128 = ((s_12_9) - (s_12_8));
        // D s_12_11: write-var shift_amount <= s_12_10
        fn_state.shift_amount = s_12_10;
        // N s_12_12: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var shift_amount:i
        let s_13_0: i128 = fn_state.shift_amount;
        // D s_13_1: write-var shift_amountshadow#7729 <= s_13_0
        fn_state.shift_amountshadow_7729 = s_13_0;
        // D s_13_2: read-var esize:i64
        let s_13_2: i64 = fn_state.esize;
        // D s_13_3: write-var esizeshadow#7730 <= s_13_2
        fn_state.esizeshadow_7730 = s_13_2;
        // D s_13_4: read-var elements:i64
        let s_13_4: i64 = fn_state.elements;
        // D s_13_5: write-var elementsshadow#7731 <= s_13_4
        fn_state.elementsshadow_7731 = s_13_4;
        // D s_13_6: read-var U:u8
        let s_13_6: bool = fn_state.U;
        // D s_13_7: cast zx s_13_6 -> bv
        let s_13_7: Bits = Bits::new(s_13_6 as u128, 1u16);
        // C s_13_8: const #1u : u8
        let s_13_8: bool = true;
        // C s_13_9: cast zx s_13_8 -> bv
        let s_13_9: Bits = Bits::new(s_13_8 as u128, 1u16);
        // D s_13_10: cmp-eq s_13_7 s_13_9
        let s_13_10: bool = ((s_13_7) == (s_13_9));
        // N s_13_11: branch s_13_10 b27 b14
        if s_13_10 {
            return block_27(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#317168 <= s_14_0
        fn_state.gs_317168 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#317168:u8
        let s_15_0: bool = fn_state.gs_317168;
        // D s_15_1: write-var src_unsigned <= s_15_0
        fn_state.src_unsigned = s_15_0;
        // D s_15_2: read-var U:u8
        let s_15_2: bool = fn_state.U;
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // C s_15_4: const #1u : u8
        let s_15_4: bool = true;
        // C s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 1u16);
        // D s_15_6: cmp-eq s_15_3 s_15_5
        let s_15_6: bool = ((s_15_3) == (s_15_5));
        // D s_15_7: write-var dest_unsigned <= s_15_6
        fn_state.dest_unsigned = s_15_6;
        // D s_15_8: read-var D:u8
        let s_15_8: bool = fn_state.D;
        // D s_15_9: cast zx s_15_8 -> bv
        let s_15_9: Bits = Bits::new(s_15_8 as u128, 1u16);
        // D s_15_10: read-var Vd:u8
        let s_15_10: u8 = fn_state.Vd;
        // D s_15_11: cast zx s_15_10 -> bv
        let s_15_11: Bits = Bits::new(s_15_10 as u128, 4u16);
        // D s_15_12: cast reint s_15_9 -> u128
        let s_15_12: u128 = (s_15_9.value() as u128);
        // D s_15_13: size-of s_15_9
        let s_15_13: u16 = s_15_9.length();
        // D s_15_14: cast reint s_15_11 -> u128
        let s_15_14: u128 = (s_15_11.value() as u128);
        // D s_15_15: size-of s_15_11
        let s_15_15: u16 = s_15_11.length();
        // D s_15_16: lsl s_15_12 s_15_15
        let s_15_16: u128 = s_15_12 << s_15_15;
        // D s_15_17: or s_15_16 s_15_14
        let s_15_17: u128 = ((s_15_16) | (s_15_14));
        // D s_15_18: add s_15_13 s_15_15
        let s_15_18: u16 = (s_15_13 + s_15_15);
        // D s_15_19: create-bits s_15_17 s_15_18
        let s_15_19: Bits = Bits::new(s_15_17, s_15_18);
        // D s_15_20: cast reint s_15_19 -> u8
        let s_15_20: u8 = (s_15_19.value() as u8);
        // D s_15_21: cast zx s_15_20 -> bv
        let s_15_21: Bits = Bits::new(s_15_20 as u128, 5u16);
        // D s_15_22: cast zx s_15_21 -> i
        let s_15_22: i128 = (s_15_21.value() as i128);
        // D s_15_23: cast reint s_15_22 -> i64
        let s_15_23: i64 = (s_15_22 as i64);
        // D s_15_24: write-var d <= s_15_23
        fn_state.d = s_15_23;
        // D s_15_25: read-var M:u8
        let s_15_25: bool = fn_state.M;
        // D s_15_26: cast zx s_15_25 -> bv
        let s_15_26: Bits = Bits::new(s_15_25 as u128, 1u16);
        // D s_15_27: read-var Vm:u8
        let s_15_27: u8 = fn_state.Vm;
        // D s_15_28: cast zx s_15_27 -> bv
        let s_15_28: Bits = Bits::new(s_15_27 as u128, 4u16);
        // D s_15_29: cast reint s_15_26 -> u128
        let s_15_29: u128 = (s_15_26.value() as u128);
        // D s_15_30: size-of s_15_26
        let s_15_30: u16 = s_15_26.length();
        // D s_15_31: cast reint s_15_28 -> u128
        let s_15_31: u128 = (s_15_28.value() as u128);
        // D s_15_32: size-of s_15_28
        let s_15_32: u16 = s_15_28.length();
        // D s_15_33: lsl s_15_29 s_15_32
        let s_15_33: u128 = s_15_29 << s_15_32;
        // D s_15_34: or s_15_33 s_15_31
        let s_15_34: u128 = ((s_15_33) | (s_15_31));
        // D s_15_35: add s_15_30 s_15_32
        let s_15_35: u16 = (s_15_30 + s_15_32);
        // D s_15_36: create-bits s_15_34 s_15_35
        let s_15_36: Bits = Bits::new(s_15_34, s_15_35);
        // D s_15_37: cast reint s_15_36 -> u8
        let s_15_37: u8 = (s_15_36.value() as u8);
        // D s_15_38: cast zx s_15_37 -> bv
        let s_15_38: Bits = Bits::new(s_15_37 as u128, 5u16);
        // D s_15_39: cast zx s_15_38 -> i
        let s_15_39: i128 = (s_15_38.value() as i128);
        // D s_15_40: cast reint s_15_39 -> i64
        let s_15_40: i64 = (s_15_39 as i64);
        // D s_15_41: write-var m <= s_15_40
        fn_state.m = s_15_40;
        // D s_15_42: read-var Q:u8
        let s_15_42: bool = fn_state.Q;
        // D s_15_43: cast zx s_15_42 -> bv
        let s_15_43: Bits = Bits::new(s_15_42 as u128, 1u16);
        // C s_15_44: const #0u : u8
        let s_15_44: bool = false;
        // C s_15_45: cast zx s_15_44 -> bv
        let s_15_45: Bits = Bits::new(s_15_44 as u128, 1u16);
        // D s_15_46: cmp-eq s_15_43 s_15_45
        let s_15_46: bool = ((s_15_43) == (s_15_45));
        // N s_15_47: branch s_15_46 b26 b16
        if s_15_46 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #2s : i64
        let s_16_0: i64 = 2;
        // D s_16_1: write-var ga#359636 <= s_16_0
        fn_state.ga_359636 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var ga#359636:i64
        let s_17_0: i64 = fn_state.ga_359636;
        // D s_17_1: write-var regs <= s_17_0
        fn_state.regs = s_17_0;
        // D s_17_2: read-var esizeshadow#7730:i64
        let s_17_2: i64 = fn_state.esizeshadow_7730;
        // C s_17_3: const #8s : i
        let s_17_3: i128 = 8;
        // D s_17_4: cast zx s_17_2 -> i
        let s_17_4: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_5: cmp-eq s_17_4 s_17_3
        let s_17_5: bool = ((s_17_4) == (s_17_3));
        // D s_17_6: not s_17_5
        let s_17_6: bool = !s_17_5;
        // N s_17_7: branch s_17_6 b19 b18
        if s_17_6 {
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
        // C s_18_0: const #8s : i64
        let s_18_0: i64 = 8;
        // D s_18_1: read-var d:i64
        let s_18_1: i64 = fn_state.d;
        // D s_18_2: read-var dest_unsigned:u8
        let s_18_2: bool = fn_state.dest_unsigned;
        // D s_18_3: read-var elementsshadow#7731:i64
        let s_18_3: i64 = fn_state.elementsshadow_7731;
        // D s_18_4: read-var m:i64
        let s_18_4: i64 = fn_state.m;
        // D s_18_5: read-var regs:i64
        let s_18_5: i64 = fn_state.regs;
        // D s_18_6: read-var shift_amountshadow#7729:i
        let s_18_6: i128 = fn_state.shift_amountshadow_7729;
        // D s_18_7: read-var src_unsigned:u8
        let s_18_7: bool = fn_state.src_unsigned;
        // D s_18_8: call execute_aarch32_instrs_VQSHL_i_Op_A_txt(s_18_1, s_18_2, s_18_3, s_18_0, s_18_4, s_18_5, s_18_6, s_18_7)
        let s_18_8: () = execute_aarch32_instrs_VQSHL_i_Op_A_txt(
            state,
            tracer,
            s_18_1,
            s_18_2,
            s_18_3,
            s_18_0,
            s_18_4,
            s_18_5,
            s_18_6,
            s_18_7,
        );
        // N s_18_9: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var esizeshadow#7730:i64
        let s_19_0: i64 = fn_state.esizeshadow_7730;
        // C s_19_1: const #16s : i
        let s_19_1: i128 = 16;
        // D s_19_2: cast zx s_19_0 -> i
        let s_19_2: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_3: cmp-eq s_19_2 s_19_1
        let s_19_3: bool = ((s_19_2) == (s_19_1));
        // D s_19_4: not s_19_3
        let s_19_4: bool = !s_19_3;
        // N s_19_5: branch s_19_4 b21 b20
        if s_19_4 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #16s : i64
        let s_20_0: i64 = 16;
        // D s_20_1: read-var d:i64
        let s_20_1: i64 = fn_state.d;
        // D s_20_2: read-var dest_unsigned:u8
        let s_20_2: bool = fn_state.dest_unsigned;
        // D s_20_3: read-var elementsshadow#7731:i64
        let s_20_3: i64 = fn_state.elementsshadow_7731;
        // D s_20_4: read-var m:i64
        let s_20_4: i64 = fn_state.m;
        // D s_20_5: read-var regs:i64
        let s_20_5: i64 = fn_state.regs;
        // D s_20_6: read-var shift_amountshadow#7729:i
        let s_20_6: i128 = fn_state.shift_amountshadow_7729;
        // D s_20_7: read-var src_unsigned:u8
        let s_20_7: bool = fn_state.src_unsigned;
        // D s_20_8: call execute_aarch32_instrs_VQSHL_i_Op_A_txt(s_20_1, s_20_2, s_20_3, s_20_0, s_20_4, s_20_5, s_20_6, s_20_7)
        let s_20_8: () = execute_aarch32_instrs_VQSHL_i_Op_A_txt(
            state,
            tracer,
            s_20_1,
            s_20_2,
            s_20_3,
            s_20_0,
            s_20_4,
            s_20_5,
            s_20_6,
            s_20_7,
        );
        // N s_20_9: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var esizeshadow#7730:i64
        let s_21_0: i64 = fn_state.esizeshadow_7730;
        // C s_21_1: const #32s : i
        let s_21_1: i128 = 32;
        // D s_21_2: cast zx s_21_0 -> i
        let s_21_2: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_3: cmp-eq s_21_2 s_21_1
        let s_21_3: bool = ((s_21_2) == (s_21_1));
        // D s_21_4: not s_21_3
        let s_21_4: bool = !s_21_3;
        // N s_21_5: branch s_21_4 b23 b22
        if s_21_4 {
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
        // D s_22_1: read-var d:i64
        let s_22_1: i64 = fn_state.d;
        // D s_22_2: read-var dest_unsigned:u8
        let s_22_2: bool = fn_state.dest_unsigned;
        // D s_22_3: read-var elementsshadow#7731:i64
        let s_22_3: i64 = fn_state.elementsshadow_7731;
        // D s_22_4: read-var m:i64
        let s_22_4: i64 = fn_state.m;
        // D s_22_5: read-var regs:i64
        let s_22_5: i64 = fn_state.regs;
        // D s_22_6: read-var shift_amountshadow#7729:i
        let s_22_6: i128 = fn_state.shift_amountshadow_7729;
        // D s_22_7: read-var src_unsigned:u8
        let s_22_7: bool = fn_state.src_unsigned;
        // D s_22_8: call execute_aarch32_instrs_VQSHL_i_Op_A_txt(s_22_1, s_22_2, s_22_3, s_22_0, s_22_4, s_22_5, s_22_6, s_22_7)
        let s_22_8: () = execute_aarch32_instrs_VQSHL_i_Op_A_txt(
            state,
            tracer,
            s_22_1,
            s_22_2,
            s_22_3,
            s_22_0,
            s_22_4,
            s_22_5,
            s_22_6,
            s_22_7,
        );
        // N s_22_9: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var esizeshadow#7730:i64
        let s_23_0: i64 = fn_state.esizeshadow_7730;
        // C s_23_1: const #64s : i
        let s_23_1: i128 = 64;
        // D s_23_2: cast zx s_23_0 -> i
        let s_23_2: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_3: cmp-eq s_23_2 s_23_1
        let s_23_3: bool = ((s_23_2) == (s_23_1));
        // D s_23_4: not s_23_3
        let s_23_4: bool = !s_23_3;
        // N s_23_5: branch s_23_4 b25 b24
        if s_23_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #64s : i64
        let s_24_0: i64 = 64;
        // D s_24_1: read-var d:i64
        let s_24_1: i64 = fn_state.d;
        // D s_24_2: read-var dest_unsigned:u8
        let s_24_2: bool = fn_state.dest_unsigned;
        // D s_24_3: read-var elementsshadow#7731:i64
        let s_24_3: i64 = fn_state.elementsshadow_7731;
        // D s_24_4: read-var m:i64
        let s_24_4: i64 = fn_state.m;
        // D s_24_5: read-var regs:i64
        let s_24_5: i64 = fn_state.regs;
        // D s_24_6: read-var shift_amountshadow#7729:i
        let s_24_6: i128 = fn_state.shift_amountshadow_7729;
        // D s_24_7: read-var src_unsigned:u8
        let s_24_7: bool = fn_state.src_unsigned;
        // D s_24_8: call execute_aarch32_instrs_VQSHL_i_Op_A_txt(s_24_1, s_24_2, s_24_3, s_24_0, s_24_4, s_24_5, s_24_6, s_24_7)
        let s_24_8: () = execute_aarch32_instrs_VQSHL_i_Op_A_txt(
            state,
            tracer,
            s_24_1,
            s_24_2,
            s_24_3,
            s_24_0,
            s_24_4,
            s_24_5,
            s_24_6,
            s_24_7,
        );
        // N s_24_9: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // N s_25_1: assert s_25_0
        let s_25_1: () = assert!(s_25_0);
        // N s_25_2: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1s : i64
        let s_26_0: i64 = 1;
        // D s_26_1: write-var ga#359636 <= s_26_0
        fn_state.ga_359636 = s_26_0;
        // N s_26_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var op:u8
        let s_27_0: bool = fn_state.op;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #1u : u8
        let s_27_2: bool = true;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#317168 <= s_27_4
        fn_state.gs_317168 = s_27_4;
        // N s_27_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var ga#359623:u8
        let s_28_0: u8 = fn_state.ga_359623;
        // C s_28_1: const #4s : i
        let s_28_1: i128 = 4;
        // D s_28_2: cast zx s_28_0 -> bv
        let s_28_2: Bits = Bits::new(s_28_0 as u128, 7u16);
        // C s_28_3: const #1s : i64
        let s_28_3: i64 = 1;
        // C s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // C s_28_5: const #2s : i
        let s_28_5: i128 = 2;
        // C s_28_6: add s_28_5 s_28_4
        let s_28_6: i128 = (s_28_5 + s_28_4);
        // D s_28_7: bit-extract s_28_2 s_28_1 s_28_6
        let s_28_7: Bits = (Bits::new(
            ((s_28_2) >> (s_28_1)).value(),
            u16::try_from(s_28_6).unwrap(),
        ));
        // D s_28_8: cast reint s_28_7 -> u8
        let s_28_8: u8 = (s_28_7.value() as u8);
        // D s_28_9: cast zx s_28_8 -> bv
        let s_28_9: Bits = Bits::new(s_28_8 as u128, 3u16);
        // C s_28_10: const #1u : u8
        let s_28_10: u8 = 1;
        // C s_28_11: cast zx s_28_10 -> bv
        let s_28_11: Bits = Bits::new(s_28_10 as u128, 3u16);
        // D s_28_12: cmp-eq s_28_9 s_28_11
        let s_28_12: bool = ((s_28_9) == (s_28_11));
        // D s_28_13: not s_28_12
        let s_28_13: bool = !s_28_12;
        // N s_28_14: branch s_28_13 b30 b29
        if s_28_13 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #16s : i64
        let s_29_0: i64 = 16;
        // D s_29_1: write-var esize <= s_29_0
        fn_state.esize = s_29_0;
        // C s_29_2: const #4s : i64
        let s_29_2: i64 = 4;
        // D s_29_3: write-var elements <= s_29_2
        fn_state.elements = s_29_2;
        // D s_29_4: read-var imm6:u8
        let s_29_4: u8 = fn_state.imm6;
        // D s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 6u16);
        // D s_29_6: cast zx s_29_5 -> i
        let s_29_6: i128 = (s_29_5.value() as i128);
        // D s_29_7: cast reint s_29_6 -> i64
        let s_29_7: i64 = (s_29_6 as i64);
        // C s_29_8: const #16s : i
        let s_29_8: i128 = 16;
        // D s_29_9: cast zx s_29_7 -> i
        let s_29_9: i128 = (i128::try_from(s_29_7).unwrap());
        // D s_29_10: sub s_29_9 s_29_8
        let s_29_10: i128 = ((s_29_9) - (s_29_8));
        // D s_29_11: write-var shift_amount <= s_29_10
        fn_state.shift_amount = s_29_10;
        // N s_29_12: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var ga#359623:u8
        let s_30_0: u8 = fn_state.ga_359623;
        // C s_30_1: const #5s : i
        let s_30_1: i128 = 5;
        // D s_30_2: cast zx s_30_0 -> bv
        let s_30_2: Bits = Bits::new(s_30_0 as u128, 7u16);
        // C s_30_3: const #1s : i64
        let s_30_3: i64 = 1;
        // C s_30_4: cast zx s_30_3 -> i
        let s_30_4: i128 = (i128::try_from(s_30_3).unwrap());
        // C s_30_5: const #1s : i
        let s_30_5: i128 = 1;
        // C s_30_6: add s_30_5 s_30_4
        let s_30_6: i128 = (s_30_5 + s_30_4);
        // D s_30_7: bit-extract s_30_2 s_30_1 s_30_6
        let s_30_7: Bits = (Bits::new(
            ((s_30_2) >> (s_30_1)).value(),
            u16::try_from(s_30_6).unwrap(),
        ));
        // D s_30_8: cast reint s_30_7 -> u8
        let s_30_8: u8 = (s_30_7.value() as u8);
        // D s_30_9: cast zx s_30_8 -> bv
        let s_30_9: Bits = Bits::new(s_30_8 as u128, 2u16);
        // C s_30_10: const #1u : u8
        let s_30_10: u8 = 1;
        // C s_30_11: cast zx s_30_10 -> bv
        let s_30_11: Bits = Bits::new(s_30_10 as u128, 2u16);
        // D s_30_12: cmp-eq s_30_9 s_30_11
        let s_30_12: bool = ((s_30_9) == (s_30_11));
        // D s_30_13: not s_30_12
        let s_30_13: bool = !s_30_12;
        // N s_30_14: branch s_30_13 b32 b31
        if s_30_13 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #32s : i64
        let s_31_0: i64 = 32;
        // D s_31_1: write-var esize <= s_31_0
        fn_state.esize = s_31_0;
        // C s_31_2: const #2s : i64
        let s_31_2: i64 = 2;
        // D s_31_3: write-var elements <= s_31_2
        fn_state.elements = s_31_2;
        // D s_31_4: read-var imm6:u8
        let s_31_4: u8 = fn_state.imm6;
        // D s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 6u16);
        // D s_31_6: cast zx s_31_5 -> i
        let s_31_6: i128 = (s_31_5.value() as i128);
        // D s_31_7: cast reint s_31_6 -> i64
        let s_31_7: i64 = (s_31_6 as i64);
        // C s_31_8: const #32s : i
        let s_31_8: i128 = 32;
        // D s_31_9: cast zx s_31_7 -> i
        let s_31_9: i128 = (i128::try_from(s_31_7).unwrap());
        // D s_31_10: sub s_31_9 s_31_8
        let s_31_10: i128 = ((s_31_9) - (s_31_8));
        // D s_31_11: write-var shift_amount <= s_31_10
        fn_state.shift_amount = s_31_10;
        // N s_31_12: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var ga#359623:u8
        let s_32_0: u8 = fn_state.ga_359623;
        // C s_32_1: const #6s : i
        let s_32_1: i128 = 6;
        // D s_32_2: cast zx s_32_0 -> bv
        let s_32_2: Bits = Bits::new(s_32_0 as u128, 7u16);
        // C s_32_3: const #1s : i64
        let s_32_3: i64 = 1;
        // C s_32_4: cast zx s_32_3 -> i
        let s_32_4: i128 = (i128::try_from(s_32_3).unwrap());
        // C s_32_5: const #0s : i
        let s_32_5: i128 = 0;
        // C s_32_6: add s_32_5 s_32_4
        let s_32_6: i128 = (s_32_5 + s_32_4);
        // D s_32_7: bit-extract s_32_2 s_32_1 s_32_6
        let s_32_7: Bits = (Bits::new(
            ((s_32_2) >> (s_32_1)).value(),
            u16::try_from(s_32_6).unwrap(),
        ));
        // D s_32_8: cast reint s_32_7 -> u8
        let s_32_8: bool = ((s_32_7.value()) != 0);
        // D s_32_9: cast zx s_32_8 -> bv
        let s_32_9: Bits = Bits::new(s_32_8 as u128, 1u16);
        // C s_32_10: const #1u : u8
        let s_32_10: bool = true;
        // C s_32_11: cast zx s_32_10 -> bv
        let s_32_11: Bits = Bits::new(s_32_10 as u128, 1u16);
        // D s_32_12: cmp-eq s_32_9 s_32_11
        let s_32_12: bool = ((s_32_9) == (s_32_11));
        // D s_32_13: not s_32_12
        let s_32_13: bool = !s_32_12;
        // N s_32_14: branch s_32_13 b34 b33
        if s_32_13 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #64s : i64
        let s_33_0: i64 = 64;
        // D s_33_1: write-var esize <= s_33_0
        fn_state.esize = s_33_0;
        // C s_33_2: const #1s : i64
        let s_33_2: i64 = 1;
        // D s_33_3: write-var elements <= s_33_2
        fn_state.elements = s_33_2;
        // D s_33_4: read-var imm6:u8
        let s_33_4: u8 = fn_state.imm6;
        // D s_33_5: cast zx s_33_4 -> bv
        let s_33_5: Bits = Bits::new(s_33_4 as u128, 6u16);
        // D s_33_6: cast zx s_33_5 -> i
        let s_33_6: i128 = (s_33_5.value() as i128);
        // D s_33_7: write-var shift_amount <= s_33_6
        fn_state.shift_amount = s_33_6;
        // N s_33_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: panic
        panic!("{:?}", ());
        // N s_35_1: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0s : i
        let s_36_0: i128 = 0;
        // D s_36_1: read-var Vd:u8
        let s_36_1: u8 = fn_state.Vd;
        // D s_36_2: cast zx s_36_1 -> bv
        let s_36_2: Bits = Bits::new(s_36_1 as u128, 4u16);
        // C s_36_3: const #1u : u64
        let s_36_3: u64 = 1;
        // D s_36_4: bit-extract s_36_2 s_36_0 s_36_3
        let s_36_4: Bits = (Bits::new(
            ((s_36_2) >> (s_36_0)).value(),
            u16::try_from(s_36_3).unwrap(),
        ));
        // D s_36_5: cast reint s_36_4 -> u8
        let s_36_5: bool = ((s_36_4.value()) != 0);
        // C s_36_6: const #0s : i
        let s_36_6: i128 = 0;
        // C s_36_7: const #0u : u64
        let s_36_7: u64 = 0;
        // D s_36_8: cast zx s_36_5 -> u64
        let s_36_8: u64 = (s_36_5 as u64);
        // C s_36_9: const #1u : u64
        let s_36_9: u64 = 1;
        // D s_36_10: and s_36_8 s_36_9
        let s_36_10: u64 = ((s_36_8) & (s_36_9));
        // D s_36_11: cmp-eq s_36_10 s_36_9
        let s_36_11: bool = ((s_36_10) == (s_36_9));
        // D s_36_12: lsl s_36_8 s_36_6
        let s_36_12: u64 = s_36_8 << s_36_6;
        // D s_36_13: or s_36_7 s_36_12
        let s_36_13: u64 = ((s_36_7) | (s_36_12));
        // D s_36_14: cmpl s_36_12
        let s_36_14: u64 = !s_36_12;
        // D s_36_15: and s_36_7 s_36_14
        let s_36_15: u64 = ((s_36_7) & (s_36_14));
        // D s_36_16: select s_36_11 s_36_13 s_36_15
        let s_36_16: u64 = if s_36_11 { s_36_13 } else { s_36_15 };
        // D s_36_17: cast trunc s_36_16 -> u8
        let s_36_17: bool = ((s_36_16) != 0);
        // D s_36_18: cast zx s_36_17 -> bv
        let s_36_18: Bits = Bits::new(s_36_17 as u128, 1u16);
        // C s_36_19: const #1u : u8
        let s_36_19: bool = true;
        // C s_36_20: cast zx s_36_19 -> bv
        let s_36_20: Bits = Bits::new(s_36_19 as u128, 1u16);
        // D s_36_21: cmp-eq s_36_18 s_36_20
        let s_36_21: bool = ((s_36_18) == (s_36_20));
        // N s_36_22: branch s_36_21 b39 b37
        if s_36_21 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0s : i
        let s_37_0: i128 = 0;
        // D s_37_1: read-var Vm:u8
        let s_37_1: u8 = fn_state.Vm;
        // D s_37_2: cast zx s_37_1 -> bv
        let s_37_2: Bits = Bits::new(s_37_1 as u128, 4u16);
        // C s_37_3: const #1u : u64
        let s_37_3: u64 = 1;
        // D s_37_4: bit-extract s_37_2 s_37_0 s_37_3
        let s_37_4: Bits = (Bits::new(
            ((s_37_2) >> (s_37_0)).value(),
            u16::try_from(s_37_3).unwrap(),
        ));
        // D s_37_5: cast reint s_37_4 -> u8
        let s_37_5: bool = ((s_37_4.value()) != 0);
        // C s_37_6: const #0s : i
        let s_37_6: i128 = 0;
        // C s_37_7: const #0u : u64
        let s_37_7: u64 = 0;
        // D s_37_8: cast zx s_37_5 -> u64
        let s_37_8: u64 = (s_37_5 as u64);
        // C s_37_9: const #1u : u64
        let s_37_9: u64 = 1;
        // D s_37_10: and s_37_8 s_37_9
        let s_37_10: u64 = ((s_37_8) & (s_37_9));
        // D s_37_11: cmp-eq s_37_10 s_37_9
        let s_37_11: bool = ((s_37_10) == (s_37_9));
        // D s_37_12: lsl s_37_8 s_37_6
        let s_37_12: u64 = s_37_8 << s_37_6;
        // D s_37_13: or s_37_7 s_37_12
        let s_37_13: u64 = ((s_37_7) | (s_37_12));
        // D s_37_14: cmpl s_37_12
        let s_37_14: u64 = !s_37_12;
        // D s_37_15: and s_37_7 s_37_14
        let s_37_15: u64 = ((s_37_7) & (s_37_14));
        // D s_37_16: select s_37_11 s_37_13 s_37_15
        let s_37_16: u64 = if s_37_11 { s_37_13 } else { s_37_15 };
        // D s_37_17: cast trunc s_37_16 -> u8
        let s_37_17: bool = ((s_37_16) != 0);
        // D s_37_18: cast zx s_37_17 -> bv
        let s_37_18: Bits = Bits::new(s_37_17 as u128, 1u16);
        // C s_37_19: const #1u : u8
        let s_37_19: bool = true;
        // C s_37_20: cast zx s_37_19 -> bv
        let s_37_20: Bits = Bits::new(s_37_19 as u128, 1u16);
        // D s_37_21: cmp-eq s_37_18 s_37_20
        let s_37_21: bool = ((s_37_18) == (s_37_20));
        // D s_37_22: write-var gs#317124 <= s_37_21
        fn_state.gs_317124 = s_37_21;
        // N s_37_23: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#317124:u8
        let s_38_0: bool = fn_state.gs_317124;
        // D s_38_1: write-var gs#317125 <= s_38_0
        fn_state.gs_317125 = s_38_0;
        // N s_38_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#317124 <= s_39_0
        fn_state.gs_317124 = s_39_0;
        // N s_39_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_40_0: panic
        panic!("{:?}", ());
        // N s_40_1: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var op:u8
        let s_41_0: bool = fn_state.op;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 1u16);
        // C s_41_2: const #0u : u8
        let s_41_2: bool = false;
        // C s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // D s_41_4: cmp-eq s_41_1 s_41_3
        let s_41_4: bool = ((s_41_1) == (s_41_3));
        // D s_41_5: write-var gs#317119 <= s_41_4
        fn_state.gs_317119 = s_41_4;
        // N s_41_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_42_0: panic
        panic!("{:?}", ());
        // N s_42_1: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#317114 <= s_43_0
        fn_state.gs_317114 = s_43_0;
        // N s_43_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
