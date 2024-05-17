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
use HaveAArch32Int8MatMulExt::*;
use execute_aarch32_instrs_MMLA_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_MMLA_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    B: bool,
    D: bool,
    Vn: u8,
    Vd: u8,
    N: bool,
    M: bool,
    U: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_327225: bool,
        op1_unsignedshadow_7995: bool,
        gs_327228: bool,
        op1_unsigned: bool,
        ga_367373: u8,
        op2_unsigned: bool,
        op2_unsignedshadow_7994: bool,
        B: bool,
        D: bool,
        Vn: u8,
        Vd: u8,
        N: bool,
        M: bool,
        U: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        B,
        D,
        Vn,
        Vd,
        N,
        M,
        U,
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
        // S s_0_1: call HaveAArch32Int8MatMulExt(s_0_0)
        let s_0_1: bool = HaveAArch32Int8MatMulExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b17 b1
        if s_0_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var B:u8
        let s_1_0: bool = fn_state.B;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // D s_1_2: read-var U:u8
        let s_1_2: bool = fn_state.U;
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cast reint s_1_1 -> u128
        let s_1_4: u128 = (s_1_1.value() as u128);
        // D s_1_5: size-of s_1_1
        let s_1_5: u16 = s_1_1.length();
        // D s_1_6: cast reint s_1_3 -> u128
        let s_1_6: u128 = (s_1_3.value() as u128);
        // D s_1_7: size-of s_1_3
        let s_1_7: u16 = s_1_3.length();
        // D s_1_8: lsl s_1_4 s_1_7
        let s_1_8: u128 = s_1_4 << s_1_7;
        // D s_1_9: or s_1_8 s_1_6
        let s_1_9: u128 = ((s_1_8) | (s_1_6));
        // D s_1_10: add s_1_5 s_1_7
        let s_1_10: u16 = (s_1_5 + s_1_7);
        // D s_1_11: create-bits s_1_9 s_1_10
        let s_1_11: Bits = Bits::new(s_1_9, s_1_10);
        // D s_1_12: cast reint s_1_11 -> u8
        let s_1_12: u8 = (s_1_11.value() as u8);
        // D s_1_13: write-var ga#367373 <= s_1_12
        fn_state.ga_367373 = s_1_12;
        // D s_1_14: read-var ga#367373:u8
        let s_1_14: u8 = fn_state.ga_367373;
        // D s_1_15: cast zx s_1_14 -> bv
        let s_1_15: Bits = Bits::new(s_1_14 as u128, 2u16);
        // C s_1_16: const #0u : u8
        let s_1_16: u8 = 0;
        // C s_1_17: cast zx s_1_16 -> bv
        let s_1_17: Bits = Bits::new(s_1_16 as u128, 2u16);
        // D s_1_18: cmp-eq s_1_15 s_1_17
        let s_1_18: bool = ((s_1_15) == (s_1_17));
        // D s_1_19: not s_1_18
        let s_1_19: bool = !s_1_18;
        // N s_1_20: branch s_1_19 b12 b2
        if s_1_19 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var op1_unsigned <= s_2_0
        fn_state.op1_unsigned = s_2_0;
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // D s_2_3: write-var op2_unsigned <= s_2_2
        fn_state.op2_unsigned = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var op2_unsigned:u8
        let s_3_0: bool = fn_state.op2_unsigned;
        // D s_3_1: write-var op2_unsignedshadow#7994 <= s_3_0
        fn_state.op2_unsignedshadow_7994 = s_3_0;
        // D s_3_2: read-var op1_unsigned:u8
        let s_3_2: bool = fn_state.op1_unsigned;
        // D s_3_3: write-var op1_unsignedshadow#7995 <= s_3_2
        fn_state.op1_unsignedshadow_7995 = s_3_2;
        // C s_3_4: const #0s : i
        let s_3_4: i128 = 0;
        // D s_3_5: read-var Vd:u8
        let s_3_5: u8 = fn_state.Vd;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 4u16);
        // C s_3_7: const #1u : u64
        let s_3_7: u64 = 1;
        // D s_3_8: bit-extract s_3_6 s_3_4 s_3_7
        let s_3_8: Bits = (Bits::new(
            ((s_3_6) >> (s_3_4)).value(),
            u16::try_from(s_3_7).unwrap(),
        ));
        // D s_3_9: cast reint s_3_8 -> u8
        let s_3_9: bool = ((s_3_8.value()) != 0);
        // C s_3_10: const #0s : i
        let s_3_10: i128 = 0;
        // C s_3_11: const #0u : u64
        let s_3_11: u64 = 0;
        // D s_3_12: cast zx s_3_9 -> u64
        let s_3_12: u64 = (s_3_9 as u64);
        // C s_3_13: const #1u : u64
        let s_3_13: u64 = 1;
        // D s_3_14: and s_3_12 s_3_13
        let s_3_14: u64 = ((s_3_12) & (s_3_13));
        // D s_3_15: cmp-eq s_3_14 s_3_13
        let s_3_15: bool = ((s_3_14) == (s_3_13));
        // D s_3_16: lsl s_3_12 s_3_10
        let s_3_16: u64 = s_3_12 << s_3_10;
        // D s_3_17: or s_3_11 s_3_16
        let s_3_17: u64 = ((s_3_11) | (s_3_16));
        // D s_3_18: cmpl s_3_16
        let s_3_18: u64 = !s_3_16;
        // D s_3_19: and s_3_11 s_3_18
        let s_3_19: u64 = ((s_3_11) & (s_3_18));
        // D s_3_20: select s_3_15 s_3_17 s_3_19
        let s_3_20: u64 = if s_3_15 { s_3_17 } else { s_3_19 };
        // D s_3_21: cast trunc s_3_20 -> u8
        let s_3_21: bool = ((s_3_20) != 0);
        // D s_3_22: cast zx s_3_21 -> bv
        let s_3_22: Bits = Bits::new(s_3_21 as u128, 1u16);
        // C s_3_23: const #1u : u8
        let s_3_23: bool = true;
        // C s_3_24: cast zx s_3_23 -> bv
        let s_3_24: Bits = Bits::new(s_3_23 as u128, 1u16);
        // D s_3_25: cmp-eq s_3_22 s_3_24
        let s_3_25: bool = ((s_3_22) == (s_3_24));
        // N s_3_26: branch s_3_25 b11 b4
        if s_3_25 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: read-var Vn:u8
        let s_4_1: u8 = fn_state.Vn;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1u : u64
        let s_4_3: u64 = 1;
        // D s_4_4: bit-extract s_4_2 s_4_0 s_4_3
        let s_4_4: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_3).unwrap(),
        ));
        // D s_4_5: cast reint s_4_4 -> u8
        let s_4_5: bool = ((s_4_4.value()) != 0);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // C s_4_7: const #0u : u64
        let s_4_7: u64 = 0;
        // D s_4_8: cast zx s_4_5 -> u64
        let s_4_8: u64 = (s_4_5 as u64);
        // C s_4_9: const #1u : u64
        let s_4_9: u64 = 1;
        // D s_4_10: and s_4_8 s_4_9
        let s_4_10: u64 = ((s_4_8) & (s_4_9));
        // D s_4_11: cmp-eq s_4_10 s_4_9
        let s_4_11: bool = ((s_4_10) == (s_4_9));
        // D s_4_12: lsl s_4_8 s_4_6
        let s_4_12: u64 = s_4_8 << s_4_6;
        // D s_4_13: or s_4_7 s_4_12
        let s_4_13: u64 = ((s_4_7) | (s_4_12));
        // D s_4_14: cmpl s_4_12
        let s_4_14: u64 = !s_4_12;
        // D s_4_15: and s_4_7 s_4_14
        let s_4_15: u64 = ((s_4_7) & (s_4_14));
        // D s_4_16: select s_4_11 s_4_13 s_4_15
        let s_4_16: u64 = if s_4_11 { s_4_13 } else { s_4_15 };
        // D s_4_17: cast trunc s_4_16 -> u8
        let s_4_17: bool = ((s_4_16) != 0);
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 1u16);
        // C s_4_19: const #1u : u8
        let s_4_19: bool = true;
        // C s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 1u16);
        // D s_4_21: cmp-eq s_4_18 s_4_20
        let s_4_21: bool = ((s_4_18) == (s_4_20));
        // D s_4_22: write-var gs#327225 <= s_4_21
        fn_state.gs_327225 = s_4_21;
        // N s_4_23: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#327225:u8
        let s_5_0: bool = fn_state.gs_327225;
        // N s_5_1: branch s_5_0 b10 b6
        if s_5_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0s : i
        let s_6_0: i128 = 0;
        // D s_6_1: read-var Vm:u8
        let s_6_1: u8 = fn_state.Vm;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 4u16);
        // C s_6_3: const #1u : u64
        let s_6_3: u64 = 1;
        // D s_6_4: bit-extract s_6_2 s_6_0 s_6_3
        let s_6_4: Bits = (Bits::new(
            ((s_6_2) >> (s_6_0)).value(),
            u16::try_from(s_6_3).unwrap(),
        ));
        // D s_6_5: cast reint s_6_4 -> u8
        let s_6_5: bool = ((s_6_4.value()) != 0);
        // C s_6_6: const #0s : i
        let s_6_6: i128 = 0;
        // C s_6_7: const #0u : u64
        let s_6_7: u64 = 0;
        // D s_6_8: cast zx s_6_5 -> u64
        let s_6_8: u64 = (s_6_5 as u64);
        // C s_6_9: const #1u : u64
        let s_6_9: u64 = 1;
        // D s_6_10: and s_6_8 s_6_9
        let s_6_10: u64 = ((s_6_8) & (s_6_9));
        // D s_6_11: cmp-eq s_6_10 s_6_9
        let s_6_11: bool = ((s_6_10) == (s_6_9));
        // D s_6_12: lsl s_6_8 s_6_6
        let s_6_12: u64 = s_6_8 << s_6_6;
        // D s_6_13: or s_6_7 s_6_12
        let s_6_13: u64 = ((s_6_7) | (s_6_12));
        // D s_6_14: cmpl s_6_12
        let s_6_14: u64 = !s_6_12;
        // D s_6_15: and s_6_7 s_6_14
        let s_6_15: u64 = ((s_6_7) & (s_6_14));
        // D s_6_16: select s_6_11 s_6_13 s_6_15
        let s_6_16: u64 = if s_6_11 { s_6_13 } else { s_6_15 };
        // D s_6_17: cast trunc s_6_16 -> u8
        let s_6_17: bool = ((s_6_16) != 0);
        // D s_6_18: cast zx s_6_17 -> bv
        let s_6_18: Bits = Bits::new(s_6_17 as u128, 1u16);
        // C s_6_19: const #1u : u8
        let s_6_19: bool = true;
        // C s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 1u16);
        // D s_6_21: cmp-eq s_6_18 s_6_20
        let s_6_21: bool = ((s_6_18) == (s_6_20));
        // D s_6_22: write-var gs#327228 <= s_6_21
        fn_state.gs_327228 = s_6_21;
        // N s_6_23: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#327228:u8
        let s_7_0: bool = fn_state.gs_327228;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var D:u8
        let s_8_0: bool = fn_state.D;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // D s_8_2: read-var Vd:u8
        let s_8_2: u8 = fn_state.Vd;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 4u16);
        // D s_8_4: cast reint s_8_1 -> u128
        let s_8_4: u128 = (s_8_1.value() as u128);
        // D s_8_5: size-of s_8_1
        let s_8_5: u16 = s_8_1.length();
        // D s_8_6: cast reint s_8_3 -> u128
        let s_8_6: u128 = (s_8_3.value() as u128);
        // D s_8_7: size-of s_8_3
        let s_8_7: u16 = s_8_3.length();
        // D s_8_8: lsl s_8_4 s_8_7
        let s_8_8: u128 = s_8_4 << s_8_7;
        // D s_8_9: or s_8_8 s_8_6
        let s_8_9: u128 = ((s_8_8) | (s_8_6));
        // D s_8_10: add s_8_5 s_8_7
        let s_8_10: u16 = (s_8_5 + s_8_7);
        // D s_8_11: create-bits s_8_9 s_8_10
        let s_8_11: Bits = Bits::new(s_8_9, s_8_10);
        // D s_8_12: cast reint s_8_11 -> u8
        let s_8_12: u8 = (s_8_11.value() as u8);
        // D s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 5u16);
        // D s_8_14: cast zx s_8_13 -> i
        let s_8_14: i128 = (s_8_13.value() as i128);
        // D s_8_15: cast reint s_8_14 -> i64
        let s_8_15: i64 = (s_8_14 as i64);
        // D s_8_16: read-var N:u8
        let s_8_16: bool = fn_state.N;
        // D s_8_17: cast zx s_8_16 -> bv
        let s_8_17: Bits = Bits::new(s_8_16 as u128, 1u16);
        // D s_8_18: read-var Vn:u8
        let s_8_18: u8 = fn_state.Vn;
        // D s_8_19: cast zx s_8_18 -> bv
        let s_8_19: Bits = Bits::new(s_8_18 as u128, 4u16);
        // D s_8_20: cast reint s_8_17 -> u128
        let s_8_20: u128 = (s_8_17.value() as u128);
        // D s_8_21: size-of s_8_17
        let s_8_21: u16 = s_8_17.length();
        // D s_8_22: cast reint s_8_19 -> u128
        let s_8_22: u128 = (s_8_19.value() as u128);
        // D s_8_23: size-of s_8_19
        let s_8_23: u16 = s_8_19.length();
        // D s_8_24: lsl s_8_20 s_8_23
        let s_8_24: u128 = s_8_20 << s_8_23;
        // D s_8_25: or s_8_24 s_8_22
        let s_8_25: u128 = ((s_8_24) | (s_8_22));
        // D s_8_26: add s_8_21 s_8_23
        let s_8_26: u16 = (s_8_21 + s_8_23);
        // D s_8_27: create-bits s_8_25 s_8_26
        let s_8_27: Bits = Bits::new(s_8_25, s_8_26);
        // D s_8_28: cast reint s_8_27 -> u8
        let s_8_28: u8 = (s_8_27.value() as u8);
        // D s_8_29: cast zx s_8_28 -> bv
        let s_8_29: Bits = Bits::new(s_8_28 as u128, 5u16);
        // D s_8_30: cast zx s_8_29 -> i
        let s_8_30: i128 = (s_8_29.value() as i128);
        // D s_8_31: cast reint s_8_30 -> i64
        let s_8_31: i64 = (s_8_30 as i64);
        // D s_8_32: read-var M:u8
        let s_8_32: bool = fn_state.M;
        // D s_8_33: cast zx s_8_32 -> bv
        let s_8_33: Bits = Bits::new(s_8_32 as u128, 1u16);
        // D s_8_34: read-var Vm:u8
        let s_8_34: u8 = fn_state.Vm;
        // D s_8_35: cast zx s_8_34 -> bv
        let s_8_35: Bits = Bits::new(s_8_34 as u128, 4u16);
        // D s_8_36: cast reint s_8_33 -> u128
        let s_8_36: u128 = (s_8_33.value() as u128);
        // D s_8_37: size-of s_8_33
        let s_8_37: u16 = s_8_33.length();
        // D s_8_38: cast reint s_8_35 -> u128
        let s_8_38: u128 = (s_8_35.value() as u128);
        // D s_8_39: size-of s_8_35
        let s_8_39: u16 = s_8_35.length();
        // D s_8_40: lsl s_8_36 s_8_39
        let s_8_40: u128 = s_8_36 << s_8_39;
        // D s_8_41: or s_8_40 s_8_38
        let s_8_41: u128 = ((s_8_40) | (s_8_38));
        // D s_8_42: add s_8_37 s_8_39
        let s_8_42: u16 = (s_8_37 + s_8_39);
        // D s_8_43: create-bits s_8_41 s_8_42
        let s_8_43: Bits = Bits::new(s_8_41, s_8_42);
        // D s_8_44: cast reint s_8_43 -> u8
        let s_8_44: u8 = (s_8_43.value() as u8);
        // D s_8_45: cast zx s_8_44 -> bv
        let s_8_45: Bits = Bits::new(s_8_44 as u128, 5u16);
        // D s_8_46: cast zx s_8_45 -> i
        let s_8_46: i128 = (s_8_45.value() as i128);
        // D s_8_47: cast reint s_8_46 -> i64
        let s_8_47: i64 = (s_8_46 as i64);
        // D s_8_48: read-var op1_unsignedshadow#7995:u8
        let s_8_48: bool = fn_state.op1_unsignedshadow_7995;
        // D s_8_49: read-var op2_unsignedshadow#7994:u8
        let s_8_49: bool = fn_state.op2_unsignedshadow_7994;
        // D s_8_50: call execute_aarch32_instrs_MMLA_Op_A_txt(s_8_15, s_8_47, s_8_31, s_8_48, s_8_49)
        let s_8_50: () = execute_aarch32_instrs_MMLA_Op_A_txt(
            state,
            tracer,
            s_8_15,
            s_8_47,
            s_8_31,
            s_8_48,
            s_8_49,
        );
        // N s_8_51: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#327228 <= s_10_0
        fn_state.gs_327228 = s_10_0;
        // N s_10_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#327225 <= s_11_0
        fn_state.gs_327225 = s_11_0;
        // N s_11_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#367373:u8
        let s_12_0: u8 = fn_state.ga_367373;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 2u16);
        // C s_12_2: const #1u : u8
        let s_12_2: u8 = 1;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 2u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: not s_12_4
        let s_12_5: bool = !s_12_4;
        // N s_12_6: branch s_12_5 b14 b13
        if s_12_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var op1_unsigned <= s_13_0
        fn_state.op1_unsigned = s_13_0;
        // C s_13_2: const #1u : u8
        let s_13_2: bool = true;
        // D s_13_3: write-var op2_unsigned <= s_13_2
        fn_state.op2_unsigned = s_13_2;
        // N s_13_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var ga#367373:u8
        let s_14_0: u8 = fn_state.ga_367373;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 2u16);
        // C s_14_2: const #2u : u8
        let s_14_2: u8 = 2;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 2u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: not s_14_4
        let s_14_5: bool = !s_14_4;
        // N s_14_6: branch s_14_5 b16 b15
        if s_14_5 {
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
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var op1_unsigned <= s_15_0
        fn_state.op1_unsigned = s_15_0;
        // C s_15_2: const #0u : u8
        let s_15_2: bool = false;
        // D s_15_3: write-var op2_unsigned <= s_15_2
        fn_state.op2_unsigned = s_15_2;
        // N s_15_4: jump b3
        return block_3(state, tracer, fn_state);
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
}
