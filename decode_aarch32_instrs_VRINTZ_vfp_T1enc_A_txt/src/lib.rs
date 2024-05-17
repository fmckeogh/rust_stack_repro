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
use FPRoundingMode::*;
use FPSCR_read::*;
use ConditionPassed::*;
use HaveFP16Ext::*;
use InITBlock::*;
use execute_aarch32_instrs_VRINTZ_vfp_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VRINTZ_vfp_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Vd: u8,
    size: u8,
    op: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_325847: bool,
        m: i64,
        gs_325848: bool,
        esize: i64,
        gs_325846: bool,
        d: i64,
        rounding: u32,
        D: bool,
        Vd: u8,
        size: u8,
        op: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        Vd,
        size,
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
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #0u : u8
        let s_2_2: u8 = 0;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b25 b3
        if s_2_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var size:u8
        let s_3_0: u8 = fn_state.size;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #1u : u8
        let s_3_2: u8 = 1;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b24 b4
        if s_3_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#325846 <= s_4_0
        fn_state.gs_325846 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#325846:u8
        let s_5_0: bool = fn_state.gs_325846;
        // D s_5_1: write-var gs#325847 <= s_5_0
        fn_state.gs_325847 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#325847:u8
        let s_6_0: bool = fn_state.gs_325847;
        // N s_6_1: branch s_6_0 b23 b7
        if s_6_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var size:u8
        let s_7_0: u8 = fn_state.size;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #1u : u8
        let s_7_2: u8 = 1;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 2u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b22 b8
        if s_7_4 {
            return block_22(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#325848 <= s_8_0
        fn_state.gs_325848 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#325848:u8
        let s_9_0: bool = fn_state.gs_325848;
        // N s_9_1: branch s_9_0 b21 b10
        if s_9_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var op:u8
        let s_10_0: bool = fn_state.op;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b20 b11
        if s_10_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call FPSCR_read(s_11_0)
        let s_11_1: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_11_0);
        // S s_11_2: call FPRoundingMode(s_11_1)
        let s_11_2: u32 = FPRoundingMode(state, tracer, s_11_1);
        // D s_11_3: write-var rounding <= s_11_2
        fn_state.rounding = s_11_2;
        // N s_11_4: jump b12
        return block_12(state, tracer, fn_state);
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
        // D s_12_2: read-var size:u8
        let s_12_2: u8 = fn_state.size;
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 2u16);
        // C s_12_4: const #1u : u8
        let s_12_4: u8 = 1;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 2u16);
        // D s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // D s_12_7: not s_12_6
        let s_12_7: bool = !s_12_6;
        // N s_12_8: branch s_12_7 b15 b13
        if s_12_7 {
            return block_15(state, tracer, fn_state);
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
        // D s_13_2: read-var Vd:u8
        let s_13_2: u8 = fn_state.Vd;
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 4u16);
        // D s_13_4: read-var D:u8
        let s_13_4: bool = fn_state.D;
        // D s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 1u16);
        // D s_13_6: cast reint s_13_3 -> u128
        let s_13_6: u128 = (s_13_3.value() as u128);
        // D s_13_7: size-of s_13_3
        let s_13_7: u16 = s_13_3.length();
        // D s_13_8: cast reint s_13_5 -> u128
        let s_13_8: u128 = (s_13_5.value() as u128);
        // D s_13_9: size-of s_13_5
        let s_13_9: u16 = s_13_5.length();
        // D s_13_10: lsl s_13_6 s_13_9
        let s_13_10: u128 = s_13_6 << s_13_9;
        // D s_13_11: or s_13_10 s_13_8
        let s_13_11: u128 = ((s_13_10) | (s_13_8));
        // D s_13_12: add s_13_7 s_13_9
        let s_13_12: u16 = (s_13_7 + s_13_9);
        // D s_13_13: create-bits s_13_11 s_13_12
        let s_13_13: Bits = Bits::new(s_13_11, s_13_12);
        // D s_13_14: cast reint s_13_13 -> u8
        let s_13_14: u8 = (s_13_13.value() as u8);
        // D s_13_15: cast zx s_13_14 -> bv
        let s_13_15: Bits = Bits::new(s_13_14 as u128, 5u16);
        // D s_13_16: cast zx s_13_15 -> i
        let s_13_16: i128 = (s_13_15.value() as i128);
        // D s_13_17: cast reint s_13_16 -> i64
        let s_13_17: i64 = (s_13_16 as i64);
        // D s_13_18: write-var d <= s_13_17
        fn_state.d = s_13_17;
        // D s_13_19: read-var Vm:u8
        let s_13_19: u8 = fn_state.Vm;
        // D s_13_20: cast zx s_13_19 -> bv
        let s_13_20: Bits = Bits::new(s_13_19 as u128, 4u16);
        // D s_13_21: read-var M:u8
        let s_13_21: bool = fn_state.M;
        // D s_13_22: cast zx s_13_21 -> bv
        let s_13_22: Bits = Bits::new(s_13_21 as u128, 1u16);
        // D s_13_23: cast reint s_13_20 -> u128
        let s_13_23: u128 = (s_13_20.value() as u128);
        // D s_13_24: size-of s_13_20
        let s_13_24: u16 = s_13_20.length();
        // D s_13_25: cast reint s_13_22 -> u128
        let s_13_25: u128 = (s_13_22.value() as u128);
        // D s_13_26: size-of s_13_22
        let s_13_26: u16 = s_13_22.length();
        // D s_13_27: lsl s_13_23 s_13_26
        let s_13_27: u128 = s_13_23 << s_13_26;
        // D s_13_28: or s_13_27 s_13_25
        let s_13_28: u128 = ((s_13_27) | (s_13_25));
        // D s_13_29: add s_13_24 s_13_26
        let s_13_29: u16 = (s_13_24 + s_13_26);
        // D s_13_30: create-bits s_13_28 s_13_29
        let s_13_30: Bits = Bits::new(s_13_28, s_13_29);
        // D s_13_31: cast reint s_13_30 -> u8
        let s_13_31: u8 = (s_13_30.value() as u8);
        // D s_13_32: cast zx s_13_31 -> bv
        let s_13_32: Bits = Bits::new(s_13_31 as u128, 5u16);
        // D s_13_33: cast zx s_13_32 -> i
        let s_13_33: i128 = (s_13_32.value() as i128);
        // D s_13_34: cast reint s_13_33 -> i64
        let s_13_34: i64 = (s_13_33 as i64);
        // D s_13_35: write-var m <= s_13_34
        fn_state.m = s_13_34;
        // N s_13_36: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var m:i64
        let s_14_0: i64 = fn_state.m;
        // D s_14_1: read-var esize:i64
        let s_14_1: i64 = fn_state.esize;
        // D s_14_2: read-var d:i64
        let s_14_2: i64 = fn_state.d;
        // C s_14_3: const #0u : u8
        let s_14_3: bool = false;
        // D s_14_4: read-var rounding:u32
        let s_14_4: u32 = fn_state.rounding;
        // D s_14_5: call execute_aarch32_instrs_VRINTZ_vfp_Op_A_txt(s_14_2, s_14_1, s_14_3, s_14_0, s_14_4)
        let s_14_5: () = execute_aarch32_instrs_VRINTZ_vfp_Op_A_txt(
            state,
            tracer,
            s_14_2,
            s_14_1,
            s_14_3,
            s_14_0,
            s_14_4,
        );
        // N s_14_6: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var size:u8
        let s_15_0: u8 = fn_state.size;
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
        // C s_16_0: const #32s : i64
        let s_16_0: i64 = 32;
        // D s_16_1: write-var esize <= s_16_0
        fn_state.esize = s_16_0;
        // D s_16_2: read-var Vd:u8
        let s_16_2: u8 = fn_state.Vd;
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 4u16);
        // D s_16_4: read-var D:u8
        let s_16_4: bool = fn_state.D;
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // D s_16_6: cast reint s_16_3 -> u128
        let s_16_6: u128 = (s_16_3.value() as u128);
        // D s_16_7: size-of s_16_3
        let s_16_7: u16 = s_16_3.length();
        // D s_16_8: cast reint s_16_5 -> u128
        let s_16_8: u128 = (s_16_5.value() as u128);
        // D s_16_9: size-of s_16_5
        let s_16_9: u16 = s_16_5.length();
        // D s_16_10: lsl s_16_6 s_16_9
        let s_16_10: u128 = s_16_6 << s_16_9;
        // D s_16_11: or s_16_10 s_16_8
        let s_16_11: u128 = ((s_16_10) | (s_16_8));
        // D s_16_12: add s_16_7 s_16_9
        let s_16_12: u16 = (s_16_7 + s_16_9);
        // D s_16_13: create-bits s_16_11 s_16_12
        let s_16_13: Bits = Bits::new(s_16_11, s_16_12);
        // D s_16_14: cast reint s_16_13 -> u8
        let s_16_14: u8 = (s_16_13.value() as u8);
        // D s_16_15: cast zx s_16_14 -> bv
        let s_16_15: Bits = Bits::new(s_16_14 as u128, 5u16);
        // D s_16_16: cast zx s_16_15 -> i
        let s_16_16: i128 = (s_16_15.value() as i128);
        // D s_16_17: cast reint s_16_16 -> i64
        let s_16_17: i64 = (s_16_16 as i64);
        // D s_16_18: write-var d <= s_16_17
        fn_state.d = s_16_17;
        // D s_16_19: read-var Vm:u8
        let s_16_19: u8 = fn_state.Vm;
        // D s_16_20: cast zx s_16_19 -> bv
        let s_16_20: Bits = Bits::new(s_16_19 as u128, 4u16);
        // D s_16_21: read-var M:u8
        let s_16_21: bool = fn_state.M;
        // D s_16_22: cast zx s_16_21 -> bv
        let s_16_22: Bits = Bits::new(s_16_21 as u128, 1u16);
        // D s_16_23: cast reint s_16_20 -> u128
        let s_16_23: u128 = (s_16_20.value() as u128);
        // D s_16_24: size-of s_16_20
        let s_16_24: u16 = s_16_20.length();
        // D s_16_25: cast reint s_16_22 -> u128
        let s_16_25: u128 = (s_16_22.value() as u128);
        // D s_16_26: size-of s_16_22
        let s_16_26: u16 = s_16_22.length();
        // D s_16_27: lsl s_16_23 s_16_26
        let s_16_27: u128 = s_16_23 << s_16_26;
        // D s_16_28: or s_16_27 s_16_25
        let s_16_28: u128 = ((s_16_27) | (s_16_25));
        // D s_16_29: add s_16_24 s_16_26
        let s_16_29: u16 = (s_16_24 + s_16_26);
        // D s_16_30: create-bits s_16_28 s_16_29
        let s_16_30: Bits = Bits::new(s_16_28, s_16_29);
        // D s_16_31: cast reint s_16_30 -> u8
        let s_16_31: u8 = (s_16_30.value() as u8);
        // D s_16_32: cast zx s_16_31 -> bv
        let s_16_32: Bits = Bits::new(s_16_31 as u128, 5u16);
        // D s_16_33: cast zx s_16_32 -> i
        let s_16_33: i128 = (s_16_32.value() as i128);
        // D s_16_34: cast reint s_16_33 -> i64
        let s_16_34: i64 = (s_16_33 as i64);
        // D s_16_35: write-var m <= s_16_34
        fn_state.m = s_16_34;
        // N s_16_36: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var size:u8
        let s_17_0: u8 = fn_state.size;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #3u : u8
        let s_17_2: u8 = 3;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 2u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: not s_17_4
        let s_17_5: bool = !s_17_4;
        // N s_17_6: branch s_17_5 b19 b18
        if s_17_5 {
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
        // C s_18_0: const #64s : i64
        let s_18_0: i64 = 64;
        // D s_18_1: write-var esize <= s_18_0
        fn_state.esize = s_18_0;
        // D s_18_2: read-var D:u8
        let s_18_2: bool = fn_state.D;
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: read-var Vd:u8
        let s_18_4: u8 = fn_state.Vd;
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 4u16);
        // D s_18_6: cast reint s_18_3 -> u128
        let s_18_6: u128 = (s_18_3.value() as u128);
        // D s_18_7: size-of s_18_3
        let s_18_7: u16 = s_18_3.length();
        // D s_18_8: cast reint s_18_5 -> u128
        let s_18_8: u128 = (s_18_5.value() as u128);
        // D s_18_9: size-of s_18_5
        let s_18_9: u16 = s_18_5.length();
        // D s_18_10: lsl s_18_6 s_18_9
        let s_18_10: u128 = s_18_6 << s_18_9;
        // D s_18_11: or s_18_10 s_18_8
        let s_18_11: u128 = ((s_18_10) | (s_18_8));
        // D s_18_12: add s_18_7 s_18_9
        let s_18_12: u16 = (s_18_7 + s_18_9);
        // D s_18_13: create-bits s_18_11 s_18_12
        let s_18_13: Bits = Bits::new(s_18_11, s_18_12);
        // D s_18_14: cast reint s_18_13 -> u8
        let s_18_14: u8 = (s_18_13.value() as u8);
        // D s_18_15: cast zx s_18_14 -> bv
        let s_18_15: Bits = Bits::new(s_18_14 as u128, 5u16);
        // D s_18_16: cast zx s_18_15 -> i
        let s_18_16: i128 = (s_18_15.value() as i128);
        // D s_18_17: cast reint s_18_16 -> i64
        let s_18_17: i64 = (s_18_16 as i64);
        // D s_18_18: write-var d <= s_18_17
        fn_state.d = s_18_17;
        // D s_18_19: read-var M:u8
        let s_18_19: bool = fn_state.M;
        // D s_18_20: cast zx s_18_19 -> bv
        let s_18_20: Bits = Bits::new(s_18_19 as u128, 1u16);
        // D s_18_21: read-var Vm:u8
        let s_18_21: u8 = fn_state.Vm;
        // D s_18_22: cast zx s_18_21 -> bv
        let s_18_22: Bits = Bits::new(s_18_21 as u128, 4u16);
        // D s_18_23: cast reint s_18_20 -> u128
        let s_18_23: u128 = (s_18_20.value() as u128);
        // D s_18_24: size-of s_18_20
        let s_18_24: u16 = s_18_20.length();
        // D s_18_25: cast reint s_18_22 -> u128
        let s_18_25: u128 = (s_18_22.value() as u128);
        // D s_18_26: size-of s_18_22
        let s_18_26: u16 = s_18_22.length();
        // D s_18_27: lsl s_18_23 s_18_26
        let s_18_27: u128 = s_18_23 << s_18_26;
        // D s_18_28: or s_18_27 s_18_25
        let s_18_28: u128 = ((s_18_27) | (s_18_25));
        // D s_18_29: add s_18_24 s_18_26
        let s_18_29: u16 = (s_18_24 + s_18_26);
        // D s_18_30: create-bits s_18_28 s_18_29
        let s_18_30: Bits = Bits::new(s_18_28, s_18_29);
        // D s_18_31: cast reint s_18_30 -> u8
        let s_18_31: u8 = (s_18_30.value() as u8);
        // D s_18_32: cast zx s_18_31 -> bv
        let s_18_32: Bits = Bits::new(s_18_31 as u128, 5u16);
        // D s_18_33: cast zx s_18_32 -> i
        let s_18_33: i128 = (s_18_32.value() as i128);
        // D s_18_34: cast reint s_18_33 -> i64
        let s_18_34: i64 = (s_18_33 as i64);
        // D s_18_35: write-var m <= s_18_34
        fn_state.m = s_18_34;
        // N s_18_36: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #3u : u32
        let s_20_0: u32 = 3;
        // D s_20_1: write-var rounding <= s_20_0
        fn_state.rounding = s_20_0;
        // N s_20_2: jump b12
        return block_12(state, tracer, fn_state);
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
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call InITBlock(s_22_0)
        let s_22_1: bool = InITBlock(state, tracer, s_22_0);
        // D s_22_2: write-var gs#325848 <= s_22_1
        fn_state.gs_325848 = s_22_1;
        // N s_22_3: jump b9
        return block_9(state, tracer, fn_state);
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
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HaveFP16Ext(s_24_0)
        let s_24_1: bool = HaveFP16Ext(state, tracer, s_24_0);
        // S s_24_2: not s_24_1
        let s_24_2: bool = !s_24_1;
        // D s_24_3: write-var gs#325846 <= s_24_2
        fn_state.gs_325846 = s_24_2;
        // N s_24_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#325847 <= s_25_0
        fn_state.gs_325847 = s_25_0;
        // N s_25_2: jump b6
        return block_6(state, tracer, fn_state);
    }
}
