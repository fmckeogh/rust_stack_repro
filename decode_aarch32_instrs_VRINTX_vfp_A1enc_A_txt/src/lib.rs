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
use HaveFP16Ext::*;
use execute_aarch32_instrs_VRINTX_vfp_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VRINTX_vfp_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    D: bool,
    Vd: u8,
    size: u8,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        d: i64,
        gs_325661: bool,
        gs_325662: bool,
        gs_325660: bool,
        cond: u8,
        D: bool,
        Vd: u8,
        size: u8,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        cond,
        D,
        Vd,
        size,
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
        // D s_2_6: read-var size:u8
        let s_2_6: u8 = fn_state.size;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 2u16);
        // C s_2_8: const #0u : u8
        let s_2_8: u8 = 0;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 2u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // N s_2_11: branch s_2_10 b22 b3
        if s_2_10 {
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
        // N s_3_5: branch s_3_4 b21 b4
        if s_3_4 {
            return block_21(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#325660 <= s_4_0
        fn_state.gs_325660 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#325660:u8
        let s_5_0: bool = fn_state.gs_325660;
        // D s_5_1: write-var gs#325661 <= s_5_0
        fn_state.gs_325661 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#325661:u8
        let s_6_0: bool = fn_state.gs_325661;
        // N s_6_1: branch s_6_0 b20 b7
        if s_6_0 {
            return block_20(state, tracer, fn_state);
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
        // N s_7_5: branch s_7_4 b19 b8
        if s_7_4 {
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
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#325662 <= s_8_0
        fn_state.gs_325662 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#325662:u8
        let s_9_0: bool = fn_state.gs_325662;
        // N s_9_1: branch s_9_0 b18 b10
        if s_9_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #16s : i64
        let s_10_0: i64 = 16;
        // D s_10_1: write-var esize <= s_10_0
        fn_state.esize = s_10_0;
        // D s_10_2: read-var size:u8
        let s_10_2: u8 = fn_state.size;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // C s_10_4: const #1u : u8
        let s_10_4: u8 = 1;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 2u16);
        // D s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // D s_10_7: not s_10_6
        let s_10_7: bool = !s_10_6;
        // N s_10_8: branch s_10_7 b13 b11
        if s_10_7 {
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
        // C s_11_0: const #16s : i64
        let s_11_0: i64 = 16;
        // D s_11_1: write-var esize <= s_11_0
        fn_state.esize = s_11_0;
        // D s_11_2: read-var Vd:u8
        let s_11_2: u8 = fn_state.Vd;
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 4u16);
        // D s_11_4: read-var D:u8
        let s_11_4: bool = fn_state.D;
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // D s_11_6: cast reint s_11_3 -> u128
        let s_11_6: u128 = (s_11_3.value() as u128);
        // D s_11_7: size-of s_11_3
        let s_11_7: u16 = s_11_3.length();
        // D s_11_8: cast reint s_11_5 -> u128
        let s_11_8: u128 = (s_11_5.value() as u128);
        // D s_11_9: size-of s_11_5
        let s_11_9: u16 = s_11_5.length();
        // D s_11_10: lsl s_11_6 s_11_9
        let s_11_10: u128 = s_11_6 << s_11_9;
        // D s_11_11: or s_11_10 s_11_8
        let s_11_11: u128 = ((s_11_10) | (s_11_8));
        // D s_11_12: add s_11_7 s_11_9
        let s_11_12: u16 = (s_11_7 + s_11_9);
        // D s_11_13: create-bits s_11_11 s_11_12
        let s_11_13: Bits = Bits::new(s_11_11, s_11_12);
        // D s_11_14: cast reint s_11_13 -> u8
        let s_11_14: u8 = (s_11_13.value() as u8);
        // D s_11_15: cast zx s_11_14 -> bv
        let s_11_15: Bits = Bits::new(s_11_14 as u128, 5u16);
        // D s_11_16: cast zx s_11_15 -> i
        let s_11_16: i128 = (s_11_15.value() as i128);
        // D s_11_17: cast reint s_11_16 -> i64
        let s_11_17: i64 = (s_11_16 as i64);
        // D s_11_18: write-var d <= s_11_17
        fn_state.d = s_11_17;
        // D s_11_19: read-var Vm:u8
        let s_11_19: u8 = fn_state.Vm;
        // D s_11_20: cast zx s_11_19 -> bv
        let s_11_20: Bits = Bits::new(s_11_19 as u128, 4u16);
        // D s_11_21: read-var M:u8
        let s_11_21: bool = fn_state.M;
        // D s_11_22: cast zx s_11_21 -> bv
        let s_11_22: Bits = Bits::new(s_11_21 as u128, 1u16);
        // D s_11_23: cast reint s_11_20 -> u128
        let s_11_23: u128 = (s_11_20.value() as u128);
        // D s_11_24: size-of s_11_20
        let s_11_24: u16 = s_11_20.length();
        // D s_11_25: cast reint s_11_22 -> u128
        let s_11_25: u128 = (s_11_22.value() as u128);
        // D s_11_26: size-of s_11_22
        let s_11_26: u16 = s_11_22.length();
        // D s_11_27: lsl s_11_23 s_11_26
        let s_11_27: u128 = s_11_23 << s_11_26;
        // D s_11_28: or s_11_27 s_11_25
        let s_11_28: u128 = ((s_11_27) | (s_11_25));
        // D s_11_29: add s_11_24 s_11_26
        let s_11_29: u16 = (s_11_24 + s_11_26);
        // D s_11_30: create-bits s_11_28 s_11_29
        let s_11_30: Bits = Bits::new(s_11_28, s_11_29);
        // D s_11_31: cast reint s_11_30 -> u8
        let s_11_31: u8 = (s_11_30.value() as u8);
        // D s_11_32: cast zx s_11_31 -> bv
        let s_11_32: Bits = Bits::new(s_11_31 as u128, 5u16);
        // D s_11_33: cast zx s_11_32 -> i
        let s_11_33: i128 = (s_11_32.value() as i128);
        // D s_11_34: cast reint s_11_33 -> i64
        let s_11_34: i64 = (s_11_33 as i64);
        // D s_11_35: write-var m <= s_11_34
        fn_state.m = s_11_34;
        // N s_11_36: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var m:i64
        let s_12_0: i64 = fn_state.m;
        // D s_12_1: read-var esize:i64
        let s_12_1: i64 = fn_state.esize;
        // D s_12_2: read-var d:i64
        let s_12_2: i64 = fn_state.d;
        // C s_12_3: const #1u : u8
        let s_12_3: bool = true;
        // D s_12_4: call execute_aarch32_instrs_VRINTX_vfp_Op_A_txt(s_12_2, s_12_1, s_12_3, s_12_0)
        let s_12_4: () = execute_aarch32_instrs_VRINTX_vfp_Op_A_txt(
            state,
            tracer,
            s_12_2,
            s_12_1,
            s_12_3,
            s_12_0,
        );
        // N s_12_5: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var size:u8
        let s_13_0: u8 = fn_state.size;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
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
        // C s_14_0: const #32s : i64
        let s_14_0: i64 = 32;
        // D s_14_1: write-var esize <= s_14_0
        fn_state.esize = s_14_0;
        // D s_14_2: read-var Vd:u8
        let s_14_2: u8 = fn_state.Vd;
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 4u16);
        // D s_14_4: read-var D:u8
        let s_14_4: bool = fn_state.D;
        // D s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // D s_14_6: cast reint s_14_3 -> u128
        let s_14_6: u128 = (s_14_3.value() as u128);
        // D s_14_7: size-of s_14_3
        let s_14_7: u16 = s_14_3.length();
        // D s_14_8: cast reint s_14_5 -> u128
        let s_14_8: u128 = (s_14_5.value() as u128);
        // D s_14_9: size-of s_14_5
        let s_14_9: u16 = s_14_5.length();
        // D s_14_10: lsl s_14_6 s_14_9
        let s_14_10: u128 = s_14_6 << s_14_9;
        // D s_14_11: or s_14_10 s_14_8
        let s_14_11: u128 = ((s_14_10) | (s_14_8));
        // D s_14_12: add s_14_7 s_14_9
        let s_14_12: u16 = (s_14_7 + s_14_9);
        // D s_14_13: create-bits s_14_11 s_14_12
        let s_14_13: Bits = Bits::new(s_14_11, s_14_12);
        // D s_14_14: cast reint s_14_13 -> u8
        let s_14_14: u8 = (s_14_13.value() as u8);
        // D s_14_15: cast zx s_14_14 -> bv
        let s_14_15: Bits = Bits::new(s_14_14 as u128, 5u16);
        // D s_14_16: cast zx s_14_15 -> i
        let s_14_16: i128 = (s_14_15.value() as i128);
        // D s_14_17: cast reint s_14_16 -> i64
        let s_14_17: i64 = (s_14_16 as i64);
        // D s_14_18: write-var d <= s_14_17
        fn_state.d = s_14_17;
        // D s_14_19: read-var Vm:u8
        let s_14_19: u8 = fn_state.Vm;
        // D s_14_20: cast zx s_14_19 -> bv
        let s_14_20: Bits = Bits::new(s_14_19 as u128, 4u16);
        // D s_14_21: read-var M:u8
        let s_14_21: bool = fn_state.M;
        // D s_14_22: cast zx s_14_21 -> bv
        let s_14_22: Bits = Bits::new(s_14_21 as u128, 1u16);
        // D s_14_23: cast reint s_14_20 -> u128
        let s_14_23: u128 = (s_14_20.value() as u128);
        // D s_14_24: size-of s_14_20
        let s_14_24: u16 = s_14_20.length();
        // D s_14_25: cast reint s_14_22 -> u128
        let s_14_25: u128 = (s_14_22.value() as u128);
        // D s_14_26: size-of s_14_22
        let s_14_26: u16 = s_14_22.length();
        // D s_14_27: lsl s_14_23 s_14_26
        let s_14_27: u128 = s_14_23 << s_14_26;
        // D s_14_28: or s_14_27 s_14_25
        let s_14_28: u128 = ((s_14_27) | (s_14_25));
        // D s_14_29: add s_14_24 s_14_26
        let s_14_29: u16 = (s_14_24 + s_14_26);
        // D s_14_30: create-bits s_14_28 s_14_29
        let s_14_30: Bits = Bits::new(s_14_28, s_14_29);
        // D s_14_31: cast reint s_14_30 -> u8
        let s_14_31: u8 = (s_14_30.value() as u8);
        // D s_14_32: cast zx s_14_31 -> bv
        let s_14_32: Bits = Bits::new(s_14_31 as u128, 5u16);
        // D s_14_33: cast zx s_14_32 -> i
        let s_14_33: i128 = (s_14_32.value() as i128);
        // D s_14_34: cast reint s_14_33 -> i64
        let s_14_34: i64 = (s_14_33 as i64);
        // D s_14_35: write-var m <= s_14_34
        fn_state.m = s_14_34;
        // N s_14_36: jump b12
        return block_12(state, tracer, fn_state);
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
        // C s_15_2: const #3u : u8
        let s_15_2: u8 = 3;
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
        // C s_16_0: const #64s : i64
        let s_16_0: i64 = 64;
        // D s_16_1: write-var esize <= s_16_0
        fn_state.esize = s_16_0;
        // D s_16_2: read-var D:u8
        let s_16_2: bool = fn_state.D;
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_4: read-var Vd:u8
        let s_16_4: u8 = fn_state.Vd;
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 4u16);
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
        // D s_16_19: read-var M:u8
        let s_16_19: bool = fn_state.M;
        // D s_16_20: cast zx s_16_19 -> bv
        let s_16_20: Bits = Bits::new(s_16_19 as u128, 1u16);
        // D s_16_21: read-var Vm:u8
        let s_16_21: u8 = fn_state.Vm;
        // D s_16_22: cast zx s_16_21 -> bv
        let s_16_22: Bits = Bits::new(s_16_21 as u128, 4u16);
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
        // N s_16_36: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b12
        return block_12(state, tracer, fn_state);
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
        // D s_19_0: read-var cond:u8
        let s_19_0: u8 = fn_state.cond;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 4u16);
        // C s_19_2: const #14u : u8
        let s_19_2: u8 = 14;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 4u16);
        // D s_19_4: cmp-ne s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) != (s_19_3));
        // D s_19_5: write-var gs#325662 <= s_19_4
        fn_state.gs_325662 = s_19_4;
        // N s_19_6: jump b9
        return block_9(state, tracer, fn_state);
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
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call HaveFP16Ext(s_21_0)
        let s_21_1: bool = HaveFP16Ext(state, tracer, s_21_0);
        // S s_21_2: not s_21_1
        let s_21_2: bool = !s_21_1;
        // D s_21_3: write-var gs#325660 <= s_21_2
        fn_state.gs_325660 = s_21_2;
        // N s_21_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#325661 <= s_22_0
        fn_state.gs_325661 = s_22_0;
        // N s_22_2: jump b6
        return block_6(state, tracer, fn_state);
    }
}
