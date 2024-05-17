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
pub fn decode_aarch32_instrs_VLDM_A2enc_A_txt<T: Tracer>(
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
        gs_311803: bool,
        regs: i64,
        gs_311819: bool,
        n: i64,
        gs_311806: bool,
        gs_311804: bool,
        d: i64,
        add: bool,
        gs_311823: bool,
        gs_311820: bool,
        wback: bool,
        gs_311805: bool,
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
        // N s_2_11: branch s_2_10 b33 b3
        if s_2_10 {
            return block_33(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#311803 <= s_3_0
        fn_state.gs_311803 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#311803:u8
        let s_4_0: bool = fn_state.gs_311803;
        // N s_4_1: branch s_4_0 b32 b5
        if s_4_0 {
            return block_32(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#311804 <= s_5_0
        fn_state.gs_311804 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#311804:u8
        let s_6_0: bool = fn_state.gs_311804;
        // N s_6_1: branch s_6_0 b31 b7
        if s_6_0 {
            return block_31(state, tracer, fn_state);
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
        // N s_7_5: branch s_7_4 b30 b8
        if s_7_4 {
            return block_30(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#311805 <= s_8_0
        fn_state.gs_311805 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#311805:u8
        let s_9_0: bool = fn_state.gs_311805;
        // N s_9_1: branch s_9_0 b29 b10
        if s_9_0 {
            return block_29(state, tracer, fn_state);
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
        // N s_10_5: branch s_10_4 b28 b11
        if s_10_4 {
            return block_28(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#311806 <= s_11_0
        fn_state.gs_311806 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#311806:u8
        let s_12_0: bool = fn_state.gs_311806;
        // N s_12_1: branch s_12_0 b27 b13
        if s_12_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_13_12: read-var Vd:u8
        let s_13_12: u8 = fn_state.Vd;
        // D s_13_13: cast zx s_13_12 -> bv
        let s_13_13: Bits = Bits::new(s_13_12 as u128, 4u16);
        // D s_13_14: read-var D:u8
        let s_13_14: bool = fn_state.D;
        // D s_13_15: cast zx s_13_14 -> bv
        let s_13_15: Bits = Bits::new(s_13_14 as u128, 1u16);
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
        // D s_13_47: write-var regs <= s_13_46
        fn_state.regs = s_13_46;
        // C s_13_48: const #15s : i
        let s_13_48: i128 = 15;
        // D s_13_49: read-var n:i64
        let s_13_49: i64 = fn_state.n;
        // D s_13_50: cast zx s_13_49 -> i
        let s_13_50: i128 = (i128::try_from(s_13_49).unwrap());
        // D s_13_51: cmp-eq s_13_50 s_13_48
        let s_13_51: bool = ((s_13_50) == (s_13_48));
        // N s_13_52: branch s_13_51 b23 b14
        if s_13_51 {
            return block_23(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#311820 <= s_14_0
        fn_state.gs_311820 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#311820:u8
        let s_15_0: bool = fn_state.gs_311820;
        // N s_15_1: branch s_15_0 b22 b16
        if s_15_0 {
            return block_22(state, tracer, fn_state);
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
        // N s_16_4: branch s_16_3 b21 b17
        if s_16_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var d:i64
        let s_17_0: i64 = fn_state.d;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: read-var regs:i64
        let s_17_2: i64 = fn_state.regs;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: add s_17_1 s_17_3
        let s_17_4: i128 = (s_17_1 + s_17_3);
        // D s_17_5: cast reint s_17_4 -> i64
        let s_17_5: i64 = (s_17_4 as i64);
        // C s_17_6: const #32s : i
        let s_17_6: i128 = 32;
        // D s_17_7: cast zx s_17_5 -> i
        let s_17_7: i128 = (i128::try_from(s_17_5).unwrap());
        // D s_17_8: cmp-gt s_17_7 s_17_6
        let s_17_8: bool = ((s_17_7) > (s_17_6));
        // D s_17_9: write-var gs#311823 <= s_17_8
        fn_state.gs_311823 = s_17_8;
        // N s_17_10: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#311823:u8
        let s_18_0: bool = fn_state.gs_311823;
        // N s_18_1: branch s_18_0 b20 b19
        if s_18_0 {
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
        // D s_19_0: read-var regs:i64
        let s_19_0: i64 = fn_state.regs;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: read-var add:u8
        let s_19_2: bool = fn_state.add;
        // D s_19_3: read-var d:i64
        let s_19_3: i64 = fn_state.d;
        // D s_19_4: read-var imm32:u32
        let s_19_4: u32 = fn_state.imm32;
        // D s_19_5: read-var n:i64
        let s_19_5: i64 = fn_state.n;
        // C s_19_6: const #1u : u8
        let s_19_6: bool = true;
        // D s_19_7: read-var wback:u8
        let s_19_7: bool = fn_state.wback;
        // D s_19_8: call execute_aarch32_instrs_VLDM_Op_A_txt(s_19_2, s_19_3, s_19_4, s_19_5, s_19_1, s_19_6, s_19_7)
        let s_19_8: () = execute_aarch32_instrs_VLDM_Op_A_txt(
            state,
            tracer,
            s_19_2,
            s_19_3,
            s_19_4,
            s_19_5,
            s_19_1,
            s_19_6,
            s_19_7,
        );
        // N s_19_9: return
        return;
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
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#311823 <= s_21_0
        fn_state.gs_311823 = s_21_0;
        // N s_21_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var wback:u8
        let s_23_0: bool = fn_state.wback;
        // N s_23_1: branch s_23_0 b26 b24
        if s_23_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call CurrentInstrSet(s_24_0)
        let s_24_1: u32 = CurrentInstrSet(state, tracer, s_24_0);
        // C s_24_2: const #1u : u32
        let s_24_2: u32 = 1;
        // S s_24_3: cmp-eq s_24_1 s_24_2
        let s_24_3: bool = ((s_24_1) == (s_24_2));
        // D s_24_4: write-var gs#311819 <= s_24_3
        fn_state.gs_311819 = s_24_3;
        // N s_24_5: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#311819:u8
        let s_25_0: bool = fn_state.gs_311819;
        // D s_25_1: write-var gs#311820 <= s_25_0
        fn_state.gs_311820 = s_25_0;
        // N s_25_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#311819 <= s_26_0
        fn_state.gs_311819 = s_26_0;
        // N s_26_2: jump b25
        return block_25(state, tracer, fn_state);
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
        // D s_28_0: read-var W:u8
        let s_28_0: bool = fn_state.W;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 1u16);
        // C s_28_2: const #1u : u8
        let s_28_2: bool = true;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: write-var gs#311806 <= s_28_4
        fn_state.gs_311806 = s_28_4;
        // N s_28_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: panic
        panic!("{:?}", ());
        // N s_29_1: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var W:u8
        let s_30_0: bool = fn_state.W;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #0u : u8
        let s_30_2: bool = false;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var gs#311805 <= s_30_4
        fn_state.gs_311805 = s_30_4;
        // N s_30_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: panic
        panic!("{:?}", ());
        // N s_31_1: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var W:u8
        let s_32_0: bool = fn_state.W;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #0u : u8
        let s_32_2: bool = false;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#311804 <= s_32_4
        fn_state.gs_311804 = s_32_4;
        // N s_32_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var U:u8
        let s_33_0: bool = fn_state.U;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #0u : u8
        let s_33_2: bool = false;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#311803 <= s_33_4
        fn_state.gs_311803 = s_33_4;
        // N s_33_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
