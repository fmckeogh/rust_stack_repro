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
use InITBlock::*;
use execute_aarch32_instrs_MMLA_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_MMLA_T1enc_A_txt<T: Tracer>(
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
        gs_327251: bool,
        ga_367396: u8,
        op1_unsignedshadow_7997: bool,
        op1_unsigned: bool,
        gs_327254: bool,
        op2_unsignedshadow_7996: bool,
        op2_unsigned: bool,
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
        // S s_0_1: call InITBlock(s_0_0)
        let s_0_1: bool = InITBlock(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b19 b1
        if s_0_1 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveAArch32Int8MatMulExt(s_1_0)
        let s_1_1: bool = HaveAArch32Int8MatMulExt(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // N s_1_3: branch s_1_2 b18 b2
        if s_1_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var B:u8
        let s_2_0: bool = fn_state.B;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // D s_2_2: read-var U:u8
        let s_2_2: bool = fn_state.U;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
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
        // D s_2_13: write-var ga#367396 <= s_2_12
        fn_state.ga_367396 = s_2_12;
        // D s_2_14: read-var ga#367396:u8
        let s_2_14: u8 = fn_state.ga_367396;
        // D s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 2u16);
        // C s_2_16: const #0u : u8
        let s_2_16: u8 = 0;
        // C s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 2u16);
        // D s_2_18: cmp-eq s_2_15 s_2_17
        let s_2_18: bool = ((s_2_15) == (s_2_17));
        // D s_2_19: not s_2_18
        let s_2_19: bool = !s_2_18;
        // N s_2_20: branch s_2_19 b13 b3
        if s_2_19 {
            return block_13(state, tracer, fn_state);
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
        // D s_3_1: write-var op1_unsigned <= s_3_0
        fn_state.op1_unsigned = s_3_0;
        // C s_3_2: const #0u : u8
        let s_3_2: bool = false;
        // D s_3_3: write-var op2_unsigned <= s_3_2
        fn_state.op2_unsigned = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var op2_unsigned:u8
        let s_4_0: bool = fn_state.op2_unsigned;
        // D s_4_1: write-var op2_unsignedshadow#7996 <= s_4_0
        fn_state.op2_unsignedshadow_7996 = s_4_0;
        // D s_4_2: read-var op1_unsigned:u8
        let s_4_2: bool = fn_state.op1_unsigned;
        // D s_4_3: write-var op1_unsignedshadow#7997 <= s_4_2
        fn_state.op1_unsignedshadow_7997 = s_4_2;
        // C s_4_4: const #0s : i
        let s_4_4: i128 = 0;
        // D s_4_5: read-var Vd:u8
        let s_4_5: u8 = fn_state.Vd;
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 4u16);
        // C s_4_7: const #1u : u64
        let s_4_7: u64 = 1;
        // D s_4_8: bit-extract s_4_6 s_4_4 s_4_7
        let s_4_8: Bits = (Bits::new(
            ((s_4_6) >> (s_4_4)).value(),
            u16::try_from(s_4_7).unwrap(),
        ));
        // D s_4_9: cast reint s_4_8 -> u8
        let s_4_9: bool = ((s_4_8.value()) != 0);
        // C s_4_10: const #0s : i
        let s_4_10: i128 = 0;
        // C s_4_11: const #0u : u64
        let s_4_11: u64 = 0;
        // D s_4_12: cast zx s_4_9 -> u64
        let s_4_12: u64 = (s_4_9 as u64);
        // C s_4_13: const #1u : u64
        let s_4_13: u64 = 1;
        // D s_4_14: and s_4_12 s_4_13
        let s_4_14: u64 = ((s_4_12) & (s_4_13));
        // D s_4_15: cmp-eq s_4_14 s_4_13
        let s_4_15: bool = ((s_4_14) == (s_4_13));
        // D s_4_16: lsl s_4_12 s_4_10
        let s_4_16: u64 = s_4_12 << s_4_10;
        // D s_4_17: or s_4_11 s_4_16
        let s_4_17: u64 = ((s_4_11) | (s_4_16));
        // D s_4_18: cmpl s_4_16
        let s_4_18: u64 = !s_4_16;
        // D s_4_19: and s_4_11 s_4_18
        let s_4_19: u64 = ((s_4_11) & (s_4_18));
        // D s_4_20: select s_4_15 s_4_17 s_4_19
        let s_4_20: u64 = if s_4_15 { s_4_17 } else { s_4_19 };
        // D s_4_21: cast trunc s_4_20 -> u8
        let s_4_21: bool = ((s_4_20) != 0);
        // D s_4_22: cast zx s_4_21 -> bv
        let s_4_22: Bits = Bits::new(s_4_21 as u128, 1u16);
        // C s_4_23: const #1u : u8
        let s_4_23: bool = true;
        // C s_4_24: cast zx s_4_23 -> bv
        let s_4_24: Bits = Bits::new(s_4_23 as u128, 1u16);
        // D s_4_25: cmp-eq s_4_22 s_4_24
        let s_4_25: bool = ((s_4_22) == (s_4_24));
        // N s_4_26: branch s_4_25 b12 b5
        if s_4_25 {
            return block_12(state, tracer, fn_state);
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
        // D s_5_1: read-var Vn:u8
        let s_5_1: u8 = fn_state.Vn;
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
        // D s_5_22: write-var gs#327251 <= s_5_21
        fn_state.gs_327251 = s_5_21;
        // N s_5_23: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#327251:u8
        let s_6_0: bool = fn_state.gs_327251;
        // N s_6_1: branch s_6_0 b11 b7
        if s_6_0 {
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
        // C s_7_0: const #0s : i
        let s_7_0: i128 = 0;
        // D s_7_1: read-var Vm:u8
        let s_7_1: u8 = fn_state.Vm;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 4u16);
        // C s_7_3: const #1u : u64
        let s_7_3: u64 = 1;
        // D s_7_4: bit-extract s_7_2 s_7_0 s_7_3
        let s_7_4: Bits = (Bits::new(
            ((s_7_2) >> (s_7_0)).value(),
            u16::try_from(s_7_3).unwrap(),
        ));
        // D s_7_5: cast reint s_7_4 -> u8
        let s_7_5: bool = ((s_7_4.value()) != 0);
        // C s_7_6: const #0s : i
        let s_7_6: i128 = 0;
        // C s_7_7: const #0u : u64
        let s_7_7: u64 = 0;
        // D s_7_8: cast zx s_7_5 -> u64
        let s_7_8: u64 = (s_7_5 as u64);
        // C s_7_9: const #1u : u64
        let s_7_9: u64 = 1;
        // D s_7_10: and s_7_8 s_7_9
        let s_7_10: u64 = ((s_7_8) & (s_7_9));
        // D s_7_11: cmp-eq s_7_10 s_7_9
        let s_7_11: bool = ((s_7_10) == (s_7_9));
        // D s_7_12: lsl s_7_8 s_7_6
        let s_7_12: u64 = s_7_8 << s_7_6;
        // D s_7_13: or s_7_7 s_7_12
        let s_7_13: u64 = ((s_7_7) | (s_7_12));
        // D s_7_14: cmpl s_7_12
        let s_7_14: u64 = !s_7_12;
        // D s_7_15: and s_7_7 s_7_14
        let s_7_15: u64 = ((s_7_7) & (s_7_14));
        // D s_7_16: select s_7_11 s_7_13 s_7_15
        let s_7_16: u64 = if s_7_11 { s_7_13 } else { s_7_15 };
        // D s_7_17: cast trunc s_7_16 -> u8
        let s_7_17: bool = ((s_7_16) != 0);
        // D s_7_18: cast zx s_7_17 -> bv
        let s_7_18: Bits = Bits::new(s_7_17 as u128, 1u16);
        // C s_7_19: const #1u : u8
        let s_7_19: bool = true;
        // C s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 1u16);
        // D s_7_21: cmp-eq s_7_18 s_7_20
        let s_7_21: bool = ((s_7_18) == (s_7_20));
        // D s_7_22: write-var gs#327254 <= s_7_21
        fn_state.gs_327254 = s_7_21;
        // N s_7_23: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#327254:u8
        let s_8_0: bool = fn_state.gs_327254;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
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
        // D s_9_0: read-var D:u8
        let s_9_0: bool = fn_state.D;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // D s_9_2: read-var Vd:u8
        let s_9_2: u8 = fn_state.Vd;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 4u16);
        // D s_9_4: cast reint s_9_1 -> u128
        let s_9_4: u128 = (s_9_1.value() as u128);
        // D s_9_5: size-of s_9_1
        let s_9_5: u16 = s_9_1.length();
        // D s_9_6: cast reint s_9_3 -> u128
        let s_9_6: u128 = (s_9_3.value() as u128);
        // D s_9_7: size-of s_9_3
        let s_9_7: u16 = s_9_3.length();
        // D s_9_8: lsl s_9_4 s_9_7
        let s_9_8: u128 = s_9_4 << s_9_7;
        // D s_9_9: or s_9_8 s_9_6
        let s_9_9: u128 = ((s_9_8) | (s_9_6));
        // D s_9_10: add s_9_5 s_9_7
        let s_9_10: u16 = (s_9_5 + s_9_7);
        // D s_9_11: create-bits s_9_9 s_9_10
        let s_9_11: Bits = Bits::new(s_9_9, s_9_10);
        // D s_9_12: cast reint s_9_11 -> u8
        let s_9_12: u8 = (s_9_11.value() as u8);
        // D s_9_13: cast zx s_9_12 -> bv
        let s_9_13: Bits = Bits::new(s_9_12 as u128, 5u16);
        // D s_9_14: cast zx s_9_13 -> i
        let s_9_14: i128 = (s_9_13.value() as i128);
        // D s_9_15: cast reint s_9_14 -> i64
        let s_9_15: i64 = (s_9_14 as i64);
        // D s_9_16: read-var N:u8
        let s_9_16: bool = fn_state.N;
        // D s_9_17: cast zx s_9_16 -> bv
        let s_9_17: Bits = Bits::new(s_9_16 as u128, 1u16);
        // D s_9_18: read-var Vn:u8
        let s_9_18: u8 = fn_state.Vn;
        // D s_9_19: cast zx s_9_18 -> bv
        let s_9_19: Bits = Bits::new(s_9_18 as u128, 4u16);
        // D s_9_20: cast reint s_9_17 -> u128
        let s_9_20: u128 = (s_9_17.value() as u128);
        // D s_9_21: size-of s_9_17
        let s_9_21: u16 = s_9_17.length();
        // D s_9_22: cast reint s_9_19 -> u128
        let s_9_22: u128 = (s_9_19.value() as u128);
        // D s_9_23: size-of s_9_19
        let s_9_23: u16 = s_9_19.length();
        // D s_9_24: lsl s_9_20 s_9_23
        let s_9_24: u128 = s_9_20 << s_9_23;
        // D s_9_25: or s_9_24 s_9_22
        let s_9_25: u128 = ((s_9_24) | (s_9_22));
        // D s_9_26: add s_9_21 s_9_23
        let s_9_26: u16 = (s_9_21 + s_9_23);
        // D s_9_27: create-bits s_9_25 s_9_26
        let s_9_27: Bits = Bits::new(s_9_25, s_9_26);
        // D s_9_28: cast reint s_9_27 -> u8
        let s_9_28: u8 = (s_9_27.value() as u8);
        // D s_9_29: cast zx s_9_28 -> bv
        let s_9_29: Bits = Bits::new(s_9_28 as u128, 5u16);
        // D s_9_30: cast zx s_9_29 -> i
        let s_9_30: i128 = (s_9_29.value() as i128);
        // D s_9_31: cast reint s_9_30 -> i64
        let s_9_31: i64 = (s_9_30 as i64);
        // D s_9_32: read-var M:u8
        let s_9_32: bool = fn_state.M;
        // D s_9_33: cast zx s_9_32 -> bv
        let s_9_33: Bits = Bits::new(s_9_32 as u128, 1u16);
        // D s_9_34: read-var Vm:u8
        let s_9_34: u8 = fn_state.Vm;
        // D s_9_35: cast zx s_9_34 -> bv
        let s_9_35: Bits = Bits::new(s_9_34 as u128, 4u16);
        // D s_9_36: cast reint s_9_33 -> u128
        let s_9_36: u128 = (s_9_33.value() as u128);
        // D s_9_37: size-of s_9_33
        let s_9_37: u16 = s_9_33.length();
        // D s_9_38: cast reint s_9_35 -> u128
        let s_9_38: u128 = (s_9_35.value() as u128);
        // D s_9_39: size-of s_9_35
        let s_9_39: u16 = s_9_35.length();
        // D s_9_40: lsl s_9_36 s_9_39
        let s_9_40: u128 = s_9_36 << s_9_39;
        // D s_9_41: or s_9_40 s_9_38
        let s_9_41: u128 = ((s_9_40) | (s_9_38));
        // D s_9_42: add s_9_37 s_9_39
        let s_9_42: u16 = (s_9_37 + s_9_39);
        // D s_9_43: create-bits s_9_41 s_9_42
        let s_9_43: Bits = Bits::new(s_9_41, s_9_42);
        // D s_9_44: cast reint s_9_43 -> u8
        let s_9_44: u8 = (s_9_43.value() as u8);
        // D s_9_45: cast zx s_9_44 -> bv
        let s_9_45: Bits = Bits::new(s_9_44 as u128, 5u16);
        // D s_9_46: cast zx s_9_45 -> i
        let s_9_46: i128 = (s_9_45.value() as i128);
        // D s_9_47: cast reint s_9_46 -> i64
        let s_9_47: i64 = (s_9_46 as i64);
        // D s_9_48: read-var op1_unsignedshadow#7997:u8
        let s_9_48: bool = fn_state.op1_unsignedshadow_7997;
        // D s_9_49: read-var op2_unsignedshadow#7996:u8
        let s_9_49: bool = fn_state.op2_unsignedshadow_7996;
        // D s_9_50: call execute_aarch32_instrs_MMLA_Op_A_txt(s_9_15, s_9_47, s_9_31, s_9_48, s_9_49)
        let s_9_50: () = execute_aarch32_instrs_MMLA_Op_A_txt(
            state,
            tracer,
            s_9_15,
            s_9_47,
            s_9_31,
            s_9_48,
            s_9_49,
        );
        // N s_9_51: return
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
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#327254 <= s_11_0
        fn_state.gs_327254 = s_11_0;
        // N s_11_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#327251 <= s_12_0
        fn_state.gs_327251 = s_12_0;
        // N s_12_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var ga#367396:u8
        let s_13_0: u8 = fn_state.ga_367396;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #1u : u8
        let s_13_2: u8 = 1;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b15 b14
        if s_13_5 {
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
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var op1_unsigned <= s_14_0
        fn_state.op1_unsigned = s_14_0;
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // D s_14_3: write-var op2_unsigned <= s_14_2
        fn_state.op2_unsigned = s_14_2;
        // N s_14_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var ga#367396:u8
        let s_15_0: u8 = fn_state.ga_367396;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 2u16);
        // C s_15_2: const #2u : u8
        let s_15_2: u8 = 2;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 2u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: not s_15_4
        let s_15_5: bool = !s_15_4;
        // N s_15_6: branch s_15_5 b17 b16
        if s_15_5 {
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
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var op1_unsigned <= s_16_0
        fn_state.op1_unsigned = s_16_0;
        // C s_16_2: const #0u : u8
        let s_16_2: bool = false;
        // D s_16_3: write-var op2_unsigned <= s_16_2
        fn_state.op2_unsigned = s_16_2;
        // N s_16_4: jump b4
        return block_4(state, tracer, fn_state);
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
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
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
}
