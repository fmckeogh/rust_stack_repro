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
use HaveSHA1Ext::*;
use ConditionPassed::*;
use execute_aarch32_instrs_SHA1M_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_SHA1M_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Vn: u8,
    Vd: u8,
    N: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_324431: bool,
        gs_324434: bool,
        D: bool,
        Vn: u8,
        Vd: u8,
        N: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        Vn,
        Vd,
        N,
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
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call InITBlock(s_2_0)
        let s_2_1: bool = InITBlock(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b16 b3
        if s_2_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call HaveSHA1Ext(s_3_0)
        let s_3_1: bool = HaveSHA1Ext(state, tracer, s_3_0);
        // S s_3_2: not s_3_1
        let s_3_2: bool = !s_3_1;
        // N s_3_3: branch s_3_2 b15 b4
        if s_3_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Q:u8
        let s_4_0: bool = fn_state.Q;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-ne s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) != (s_4_3));
        // N s_4_5: branch s_4_4 b14 b5
        if s_4_4 {
            return block_14(state, tracer, fn_state);
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
        // N s_5_22: branch s_5_21 b13 b6
        if s_5_21 {
            return block_13(state, tracer, fn_state);
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
        // D s_6_1: read-var Vn:u8
        let s_6_1: u8 = fn_state.Vn;
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
        // D s_6_22: write-var gs#324431 <= s_6_21
        fn_state.gs_324431 = s_6_21;
        // N s_6_23: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#324431:u8
        let s_7_0: bool = fn_state.gs_324431;
        // N s_7_1: branch s_7_0 b12 b8
        if s_7_0 {
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
        // D s_8_22: write-var gs#324434 <= s_8_21
        fn_state.gs_324434 = s_8_21;
        // N s_8_23: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#324434:u8
        let s_9_0: bool = fn_state.gs_324434;
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
        // D s_10_0: read-var D:u8
        let s_10_0: bool = fn_state.D;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // D s_10_2: read-var Vd:u8
        let s_10_2: u8 = fn_state.Vd;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 4u16);
        // D s_10_4: cast reint s_10_1 -> u128
        let s_10_4: u128 = (s_10_1.value() as u128);
        // D s_10_5: size-of s_10_1
        let s_10_5: u16 = s_10_1.length();
        // D s_10_6: cast reint s_10_3 -> u128
        let s_10_6: u128 = (s_10_3.value() as u128);
        // D s_10_7: size-of s_10_3
        let s_10_7: u16 = s_10_3.length();
        // D s_10_8: lsl s_10_4 s_10_7
        let s_10_8: u128 = s_10_4 << s_10_7;
        // D s_10_9: or s_10_8 s_10_6
        let s_10_9: u128 = ((s_10_8) | (s_10_6));
        // D s_10_10: add s_10_5 s_10_7
        let s_10_10: u16 = (s_10_5 + s_10_7);
        // D s_10_11: create-bits s_10_9 s_10_10
        let s_10_11: Bits = Bits::new(s_10_9, s_10_10);
        // D s_10_12: cast reint s_10_11 -> u8
        let s_10_12: u8 = (s_10_11.value() as u8);
        // D s_10_13: cast zx s_10_12 -> bv
        let s_10_13: Bits = Bits::new(s_10_12 as u128, 5u16);
        // D s_10_14: cast zx s_10_13 -> i
        let s_10_14: i128 = (s_10_13.value() as i128);
        // D s_10_15: cast reint s_10_14 -> i64
        let s_10_15: i64 = (s_10_14 as i64);
        // D s_10_16: read-var N:u8
        let s_10_16: bool = fn_state.N;
        // D s_10_17: cast zx s_10_16 -> bv
        let s_10_17: Bits = Bits::new(s_10_16 as u128, 1u16);
        // D s_10_18: read-var Vn:u8
        let s_10_18: u8 = fn_state.Vn;
        // D s_10_19: cast zx s_10_18 -> bv
        let s_10_19: Bits = Bits::new(s_10_18 as u128, 4u16);
        // D s_10_20: cast reint s_10_17 -> u128
        let s_10_20: u128 = (s_10_17.value() as u128);
        // D s_10_21: size-of s_10_17
        let s_10_21: u16 = s_10_17.length();
        // D s_10_22: cast reint s_10_19 -> u128
        let s_10_22: u128 = (s_10_19.value() as u128);
        // D s_10_23: size-of s_10_19
        let s_10_23: u16 = s_10_19.length();
        // D s_10_24: lsl s_10_20 s_10_23
        let s_10_24: u128 = s_10_20 << s_10_23;
        // D s_10_25: or s_10_24 s_10_22
        let s_10_25: u128 = ((s_10_24) | (s_10_22));
        // D s_10_26: add s_10_21 s_10_23
        let s_10_26: u16 = (s_10_21 + s_10_23);
        // D s_10_27: create-bits s_10_25 s_10_26
        let s_10_27: Bits = Bits::new(s_10_25, s_10_26);
        // D s_10_28: cast reint s_10_27 -> u8
        let s_10_28: u8 = (s_10_27.value() as u8);
        // D s_10_29: cast zx s_10_28 -> bv
        let s_10_29: Bits = Bits::new(s_10_28 as u128, 5u16);
        // D s_10_30: cast zx s_10_29 -> i
        let s_10_30: i128 = (s_10_29.value() as i128);
        // D s_10_31: cast reint s_10_30 -> i64
        let s_10_31: i64 = (s_10_30 as i64);
        // D s_10_32: read-var M:u8
        let s_10_32: bool = fn_state.M;
        // D s_10_33: cast zx s_10_32 -> bv
        let s_10_33: Bits = Bits::new(s_10_32 as u128, 1u16);
        // D s_10_34: read-var Vm:u8
        let s_10_34: u8 = fn_state.Vm;
        // D s_10_35: cast zx s_10_34 -> bv
        let s_10_35: Bits = Bits::new(s_10_34 as u128, 4u16);
        // D s_10_36: cast reint s_10_33 -> u128
        let s_10_36: u128 = (s_10_33.value() as u128);
        // D s_10_37: size-of s_10_33
        let s_10_37: u16 = s_10_33.length();
        // D s_10_38: cast reint s_10_35 -> u128
        let s_10_38: u128 = (s_10_35.value() as u128);
        // D s_10_39: size-of s_10_35
        let s_10_39: u16 = s_10_35.length();
        // D s_10_40: lsl s_10_36 s_10_39
        let s_10_40: u128 = s_10_36 << s_10_39;
        // D s_10_41: or s_10_40 s_10_38
        let s_10_41: u128 = ((s_10_40) | (s_10_38));
        // D s_10_42: add s_10_37 s_10_39
        let s_10_42: u16 = (s_10_37 + s_10_39);
        // D s_10_43: create-bits s_10_41 s_10_42
        let s_10_43: Bits = Bits::new(s_10_41, s_10_42);
        // D s_10_44: cast reint s_10_43 -> u8
        let s_10_44: u8 = (s_10_43.value() as u8);
        // D s_10_45: cast zx s_10_44 -> bv
        let s_10_45: Bits = Bits::new(s_10_44 as u128, 5u16);
        // D s_10_46: cast zx s_10_45 -> i
        let s_10_46: i128 = (s_10_45.value() as i128);
        // D s_10_47: cast reint s_10_46 -> i64
        let s_10_47: i64 = (s_10_46 as i64);
        // D s_10_48: call execute_aarch32_instrs_SHA1M_Op_A_txt(s_10_15, s_10_47, s_10_31)
        let s_10_48: () = execute_aarch32_instrs_SHA1M_Op_A_txt(
            state,
            tracer,
            s_10_15,
            s_10_47,
            s_10_31,
        );
        // N s_10_49: return
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
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#324434 <= s_12_0
        fn_state.gs_324434 = s_12_0;
        // N s_12_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#324431 <= s_13_0
        fn_state.gs_324431 = s_13_0;
        // N s_13_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
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
}
