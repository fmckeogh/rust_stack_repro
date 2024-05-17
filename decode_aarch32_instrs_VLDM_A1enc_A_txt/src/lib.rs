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
use CurrentInstrSet::*;
use ConditionPassed::*;
use place_slice::*;
use execute_aarch32_instrs_VLDM_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VLDM_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    P: bool,
    U: bool,
    D: bool,
    W: bool,
    Rn: u8,
    Vd: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
        gs_311770: bool,
        gs_311786: bool,
        n: i64,
        gs_311795: bool,
        gs_311785: bool,
        gs_311771: bool,
        gs_311789: bool,
        gs_311769: bool,
        regs: i64,
        gs_311791: bool,
        gs_311768: bool,
        d: i64,
        add: bool,
        wback: bool,
        cond: u8,
        P: bool,
        U: bool,
        D: bool,
        W: bool,
        Rn: u8,
        Vd: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        cond,
        P,
        U,
        D,
        W,
        Rn,
        Vd,
        imm8,
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
        // D s_2_0: read-var cond:u8
        let s_2_0: u8 = fn_state.cond;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-ne s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) != (s_2_3));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // D s_2_6: read-var P:u8
        let s_2_6: bool = fn_state.P;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 1u16);
        // C s_2_8: const #0u : u8
        let s_2_8: bool = false;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 1u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // N s_2_11: branch s_2_10 b41 b3
        if s_2_10 {
            return block_41(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#311768 <= s_3_0
        fn_state.gs_311768 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#311768:u8
        let s_4_0: bool = fn_state.gs_311768;
        // N s_4_1: branch s_4_0 b40 b5
        if s_4_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#311769 <= s_5_0
        fn_state.gs_311769 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#311769:u8
        let s_6_0: bool = fn_state.gs_311769;
        // N s_6_1: branch s_6_0 b39 b7
        if s_6_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var P:u8
        let s_7_0: bool = fn_state.P;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b38 b8
        if s_7_4 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#311770 <= s_8_0
        fn_state.gs_311770 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#311770:u8
        let s_9_0: bool = fn_state.gs_311770;
        // N s_9_1: branch s_9_0 b37 b10
        if s_9_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var P:u8
        let s_10_0: bool = fn_state.P;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // D s_10_2: read-var U:u8
        let s_10_2: bool = fn_state.U;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b36 b11
        if s_10_4 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#311771 <= s_11_0
        fn_state.gs_311771 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#311771:u8
        let s_12_0: bool = fn_state.gs_311771;
        // N s_12_1: branch s_12_0 b35 b13
        if s_12_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var U:u8
        let s_13_0: bool = fn_state.U;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 1u16);
        // C s_13_2: const #1u : u8
        let s_13_2: bool = true;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: write-var add <= s_13_4
        fn_state.add = s_13_4;
        // D s_13_6: read-var W:u8
        let s_13_6: bool = fn_state.W;
        // D s_13_7: cast zx s_13_6 -> bv
        let s_13_7: Bits = Bits::new(s_13_6 as u128, 1u16);
        // C s_13_8: const #1u : u8
        let s_13_8: bool = true;
        // C s_13_9: cast zx s_13_8 -> bv
        let s_13_9: Bits = Bits::new(s_13_8 as u128, 1u16);
        // D s_13_10: cmp-eq s_13_7 s_13_9
        let s_13_10: bool = ((s_13_7) == (s_13_9));
        // D s_13_11: write-var wback <= s_13_10
        fn_state.wback = s_13_10;
        // D s_13_12: read-var D:u8
        let s_13_12: bool = fn_state.D;
        // D s_13_13: cast zx s_13_12 -> bv
        let s_13_13: Bits = Bits::new(s_13_12 as u128, 1u16);
        // D s_13_14: read-var Vd:u8
        let s_13_14: u8 = fn_state.Vd;
        // D s_13_15: cast zx s_13_14 -> bv
        let s_13_15: Bits = Bits::new(s_13_14 as u128, 4u16);
        // D s_13_16: cast reint s_13_13 -> u128
        let s_13_16: u128 = (s_13_13.value() as u128);
        // D s_13_17: size-of s_13_13
        let s_13_17: u16 = s_13_13.length();
        // D s_13_18: cast reint s_13_15 -> u128
        let s_13_18: u128 = (s_13_15.value() as u128);
        // D s_13_19: size-of s_13_15
        let s_13_19: u16 = s_13_15.length();
        // D s_13_20: lsl s_13_16 s_13_19
        let s_13_20: u128 = s_13_16 << s_13_19;
        // D s_13_21: or s_13_20 s_13_18
        let s_13_21: u128 = ((s_13_20) | (s_13_18));
        // D s_13_22: add s_13_17 s_13_19
        let s_13_22: u16 = (s_13_17 + s_13_19);
        // D s_13_23: create-bits s_13_21 s_13_22
        let s_13_23: Bits = Bits::new(s_13_21, s_13_22);
        // D s_13_24: cast reint s_13_23 -> u8
        let s_13_24: u8 = (s_13_23.value() as u8);
        // D s_13_25: cast zx s_13_24 -> bv
        let s_13_25: Bits = Bits::new(s_13_24 as u128, 5u16);
        // D s_13_26: cast zx s_13_25 -> i
        let s_13_26: i128 = (s_13_25.value() as i128);
        // D s_13_27: cast reint s_13_26 -> i64
        let s_13_27: i64 = (s_13_26 as i64);
        // D s_13_28: write-var d <= s_13_27
        fn_state.d = s_13_27;
        // D s_13_29: read-var Rn:u8
        let s_13_29: u8 = fn_state.Rn;
        // D s_13_30: cast zx s_13_29 -> bv
        let s_13_30: Bits = Bits::new(s_13_29 as u128, 4u16);
        // D s_13_31: cast zx s_13_30 -> i
        let s_13_31: i128 = (s_13_30.value() as i128);
        // D s_13_32: cast reint s_13_31 -> i64
        let s_13_32: i64 = (s_13_31 as i64);
        // D s_13_33: write-var n <= s_13_32
        fn_state.n = s_13_32;
        // C s_13_34: const #32s : i
        let s_13_34: i128 = 32;
        // C s_13_35: const #0s : i
        let s_13_35: i128 = 0;
        // C s_13_36: const #8s : i
        let s_13_36: i128 = 8;
        // C s_13_37: const #2s : i
        let s_13_37: i128 = 2;
        // D s_13_38: read-var imm8:u8
        let s_13_38: u8 = fn_state.imm8;
        // D s_13_39: cast zx s_13_38 -> bv
        let s_13_39: Bits = Bits::new(s_13_38 as u128, 8u16);
        // D s_13_40: call place_slice(s_13_34, s_13_39, s_13_35, s_13_36, s_13_37)
        let s_13_40: Bits = place_slice(
            state,
            tracer,
            s_13_34,
            s_13_39,
            s_13_35,
            s_13_36,
            s_13_37,
        );
        // D s_13_41: cast reint s_13_40 -> u32
        let s_13_41: u32 = (s_13_40.value() as u32);
        // D s_13_42: write-var imm32 <= s_13_41
        fn_state.imm32 = s_13_41;
        // D s_13_43: read-var imm8:u8
        let s_13_43: u8 = fn_state.imm8;
        // D s_13_44: cast zx s_13_43 -> bv
        let s_13_44: Bits = Bits::new(s_13_43 as u128, 8u16);
        // D s_13_45: cast zx s_13_44 -> i
        let s_13_45: i128 = (s_13_44.value() as i128);
        // D s_13_46: cast reint s_13_45 -> i64
        let s_13_46: i64 = (s_13_45 as i64);
        // C s_13_47: const #2s : i
        let s_13_47: i128 = 2;
        // D s_13_48: cast zx s_13_46 -> i
        let s_13_48: i128 = (i128::try_from(s_13_46).unwrap());
        // D s_13_49: div s_13_48 s_13_47
        let s_13_49: i128 = ((s_13_48) / (s_13_47));
        // D s_13_50: cast reint s_13_49 -> i64
        let s_13_50: i64 = (s_13_49 as i64);
        // D s_13_51: write-var regs <= s_13_50
        fn_state.regs = s_13_50;
        // C s_13_52: const #15s : i
        let s_13_52: i128 = 15;
        // D s_13_53: read-var n:i64
        let s_13_53: i64 = fn_state.n;
        // D s_13_54: cast zx s_13_53 -> i
        let s_13_54: i128 = (i128::try_from(s_13_53).unwrap());
        // D s_13_55: cmp-eq s_13_54 s_13_52
        let s_13_55: bool = ((s_13_54) == (s_13_52));
        // N s_13_56: branch s_13_55 b31 b14
        if s_13_55 {
            return block_31(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#311786 <= s_14_0
        fn_state.gs_311786 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#311786:u8
        let s_15_0: bool = fn_state.gs_311786;
        // N s_15_1: branch s_15_0 b30 b16
        if s_15_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0s : i
        let s_16_0: i128 = 0;
        // D s_16_1: read-var regs:i64
        let s_16_1: i64 = fn_state.regs;
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // D s_16_3: cmp-eq s_16_2 s_16_0
        let s_16_3: bool = ((s_16_2) == (s_16_0));
        // N s_16_4: branch s_16_3 b29 b17
        if s_16_3 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #16s : i
        let s_17_0: i128 = 16;
        // D s_17_1: read-var regs:i64
        let s_17_1: i64 = fn_state.regs;
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // D s_17_3: cmp-gt s_17_2 s_17_0
        let s_17_3: bool = ((s_17_2) > (s_17_0));
        // D s_17_4: write-var gs#311789 <= s_17_3
        fn_state.gs_311789 = s_17_3;
        // N s_17_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#311789:u8
        let s_18_0: bool = fn_state.gs_311789;
        // N s_18_1: branch s_18_0 b28 b19
        if s_18_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var d:i64
        let s_19_0: i64 = fn_state.d;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: read-var regs:i64
        let s_19_2: i64 = fn_state.regs;
        // D s_19_3: cast zx s_19_2 -> i
        let s_19_3: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_4: add s_19_1 s_19_3
        let s_19_4: i128 = (s_19_1 + s_19_3);
        // D s_19_5: cast reint s_19_4 -> i64
        let s_19_5: i64 = (s_19_4 as i64);
        // C s_19_6: const #32s : i
        let s_19_6: i128 = 32;
        // D s_19_7: cast zx s_19_5 -> i
        let s_19_7: i128 = (i128::try_from(s_19_5).unwrap());
        // D s_19_8: cmp-gt s_19_7 s_19_6
        let s_19_8: bool = ((s_19_7) > (s_19_6));
        // D s_19_9: write-var gs#311791 <= s_19_8
        fn_state.gs_311791 = s_19_8;
        // N s_19_10: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#311791:u8
        let s_20_0: bool = fn_state.gs_311791;
        // N s_20_1: branch s_20_0 b27 b21
        if s_20_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0s : i
        let s_21_0: i128 = 0;
        // D s_21_1: read-var imm8:u8
        let s_21_1: u8 = fn_state.imm8;
        // D s_21_2: cast zx s_21_1 -> bv
        let s_21_2: Bits = Bits::new(s_21_1 as u128, 8u16);
        // C s_21_3: const #1u : u64
        let s_21_3: u64 = 1;
        // D s_21_4: bit-extract s_21_2 s_21_0 s_21_3
        let s_21_4: Bits = (Bits::new(
            ((s_21_2) >> (s_21_0)).value(),
            u16::try_from(s_21_3).unwrap(),
        ));
        // D s_21_5: cast reint s_21_4 -> u8
        let s_21_5: bool = ((s_21_4.value()) != 0);
        // C s_21_6: const #0s : i
        let s_21_6: i128 = 0;
        // C s_21_7: const #0u : u64
        let s_21_7: u64 = 0;
        // D s_21_8: cast zx s_21_5 -> u64
        let s_21_8: u64 = (s_21_5 as u64);
        // C s_21_9: const #1u : u64
        let s_21_9: u64 = 1;
        // D s_21_10: and s_21_8 s_21_9
        let s_21_10: u64 = ((s_21_8) & (s_21_9));
        // D s_21_11: cmp-eq s_21_10 s_21_9
        let s_21_11: bool = ((s_21_10) == (s_21_9));
        // D s_21_12: lsl s_21_8 s_21_6
        let s_21_12: u64 = s_21_8 << s_21_6;
        // D s_21_13: or s_21_7 s_21_12
        let s_21_13: u64 = ((s_21_7) | (s_21_12));
        // D s_21_14: cmpl s_21_12
        let s_21_14: u64 = !s_21_12;
        // D s_21_15: and s_21_7 s_21_14
        let s_21_15: u64 = ((s_21_7) & (s_21_14));
        // D s_21_16: select s_21_11 s_21_13 s_21_15
        let s_21_16: u64 = if s_21_11 { s_21_13 } else { s_21_15 };
        // D s_21_17: cast trunc s_21_16 -> u8
        let s_21_17: bool = ((s_21_16) != 0);
        // D s_21_18: cast zx s_21_17 -> bv
        let s_21_18: Bits = Bits::new(s_21_17 as u128, 1u16);
        // C s_21_19: const #1u : u8
        let s_21_19: bool = true;
        // C s_21_20: cast zx s_21_19 -> bv
        let s_21_20: Bits = Bits::new(s_21_19 as u128, 1u16);
        // D s_21_21: cmp-eq s_21_18 s_21_20
        let s_21_21: bool = ((s_21_18) == (s_21_20));
        // N s_21_22: branch s_21_21 b26 b22
        if s_21_21 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#311795 <= s_22_0
        fn_state.gs_311795 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#311795:u8
        let s_23_0: bool = fn_state.gs_311795;
        // N s_23_1: branch s_23_0 b25 b24
        if s_23_0 {
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
        // D s_24_0: read-var regs:i64
        let s_24_0: i64 = fn_state.regs;
        // D s_24_1: cast zx s_24_0 -> i
        let s_24_1: i128 = (i128::try_from(s_24_0).unwrap());
        // D s_24_2: read-var add:u8
        let s_24_2: bool = fn_state.add;
        // D s_24_3: read-var d:i64
        let s_24_3: i64 = fn_state.d;
        // D s_24_4: read-var imm32:u32
        let s_24_4: u32 = fn_state.imm32;
        // D s_24_5: read-var n:i64
        let s_24_5: i64 = fn_state.n;
        // C s_24_6: const #0u : u8
        let s_24_6: bool = false;
        // D s_24_7: read-var wback:u8
        let s_24_7: bool = fn_state.wback;
        // D s_24_8: call execute_aarch32_instrs_VLDM_Op_A_txt(s_24_2, s_24_3, s_24_4, s_24_5, s_24_1, s_24_6, s_24_7)
        let s_24_8: () = execute_aarch32_instrs_VLDM_Op_A_txt(
            state,
            tracer,
            s_24_2,
            s_24_3,
            s_24_4,
            s_24_5,
            s_24_1,
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
        // D s_26_0: read-var d:i64
        let s_26_0: i64 = fn_state.d;
        // D s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_2: read-var regs:i64
        let s_26_2: i64 = fn_state.regs;
        // D s_26_3: cast zx s_26_2 -> i
        let s_26_3: i128 = (i128::try_from(s_26_2).unwrap());
        // D s_26_4: add s_26_1 s_26_3
        let s_26_4: i128 = (s_26_1 + s_26_3);
        // D s_26_5: cast reint s_26_4 -> i64
        let s_26_5: i64 = (s_26_4 as i64);
        // C s_26_6: const #16s : i
        let s_26_6: i128 = 16;
        // D s_26_7: cast zx s_26_5 -> i
        let s_26_7: i128 = (i128::try_from(s_26_5).unwrap());
        // D s_26_8: cmp-gt s_26_7 s_26_6
        let s_26_8: bool = ((s_26_7) > (s_26_6));
        // D s_26_9: write-var gs#311795 <= s_26_8
        fn_state.gs_311795 = s_26_8;
        // N s_26_10: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: panic
        panic!("{:?}", ());
        // N s_27_1: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#311791 <= s_28_0
        fn_state.gs_311791 = s_28_0;
        // N s_28_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#311789 <= s_29_0
        fn_state.gs_311789 = s_29_0;
        // N s_29_2: jump b18
        return block_18(state, tracer, fn_state);
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
        // D s_31_0: read-var wback:u8
        let s_31_0: bool = fn_state.wback;
        // N s_31_1: branch s_31_0 b34 b32
        if s_31_0 {
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
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call CurrentInstrSet(s_32_0)
        let s_32_1: u32 = CurrentInstrSet(state, tracer, s_32_0);
        // C s_32_2: const #1u : u32
        let s_32_2: u32 = 1;
        // S s_32_3: cmp-eq s_32_1 s_32_2
        let s_32_3: bool = ((s_32_1) == (s_32_2));
        // D s_32_4: write-var gs#311785 <= s_32_3
        fn_state.gs_311785 = s_32_3;
        // N s_32_5: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#311785:u8
        let s_33_0: bool = fn_state.gs_311785;
        // D s_33_1: write-var gs#311786 <= s_33_0
        fn_state.gs_311786 = s_33_0;
        // N s_33_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#311785 <= s_34_0
        fn_state.gs_311785 = s_34_0;
        // N s_34_2: jump b33
        return block_33(state, tracer, fn_state);
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
        // D s_36_0: read-var W:u8
        let s_36_0: bool = fn_state.W;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 1u16);
        // C s_36_2: const #1u : u8
        let s_36_2: bool = true;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: write-var gs#311771 <= s_36_4
        fn_state.gs_311771 = s_36_4;
        // N s_36_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: panic
        panic!("{:?}", ());
        // N s_37_1: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var W:u8
        let s_38_0: bool = fn_state.W;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 1u16);
        // C s_38_2: const #0u : u8
        let s_38_2: bool = false;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // D s_38_5: write-var gs#311770 <= s_38_4
        fn_state.gs_311770 = s_38_4;
        // N s_38_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: panic
        panic!("{:?}", ());
        // N s_39_1: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var W:u8
        let s_40_0: bool = fn_state.W;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 1u16);
        // C s_40_2: const #0u : u8
        let s_40_2: bool = false;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // D s_40_4: cmp-eq s_40_1 s_40_3
        let s_40_4: bool = ((s_40_1) == (s_40_3));
        // D s_40_5: write-var gs#311769 <= s_40_4
        fn_state.gs_311769 = s_40_4;
        // N s_40_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var U:u8
        let s_41_0: bool = fn_state.U;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 1u16);
        // C s_41_2: const #0u : u8
        let s_41_2: bool = false;
        // C s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // D s_41_4: cmp-eq s_41_1 s_41_3
        let s_41_4: bool = ((s_41_1) == (s_41_3));
        // D s_41_5: write-var gs#311768 <= s_41_4
        fn_state.gs_311768 = s_41_4;
        // N s_41_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
