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
use HaveFP16Ext::*;
use execute_aarch32_instrs_VRINTA_vfp_Op_A_txt::*;
use FPDecodeRM::*;
use common::*;
pub fn decode_aarch32_instrs_VRINTA_vfp_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    RM: u8,
    Vd: u8,
    size: u8,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_325508: bool,
        gs_325509: bool,
        esize: i64,
        d: i64,
        rounding: u32,
        D: bool,
        RM: u8,
        Vd: u8,
        size: u8,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        RM,
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
        // D s_0_0: read-var size:u8
        let s_0_0: u8 = fn_state.size;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 2u16);
        // C s_0_2: const #0u : u8
        let s_0_2: u8 = 0;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 2u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b15 b1
        if s_0_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var size:u8
        let s_1_0: u8 = fn_state.size;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #1u : u8
        let s_1_2: u8 = 1;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b14 b2
        if s_1_4 {
            return block_14(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#325508 <= s_2_0
        fn_state.gs_325508 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#325508:u8
        let s_3_0: bool = fn_state.gs_325508;
        // D s_3_1: write-var gs#325509 <= s_3_0
        fn_state.gs_325509 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#325509:u8
        let s_4_0: bool = fn_state.gs_325509;
        // N s_4_1: branch s_4_0 b13 b5
        if s_4_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var RM:u8
        let s_5_0: u8 = fn_state.RM;
        // D s_5_1: call FPDecodeRM(s_5_0)
        let s_5_1: u32 = FPDecodeRM(state, tracer, s_5_0);
        // D s_5_2: write-var rounding <= s_5_1
        fn_state.rounding = s_5_1;
        // C s_5_3: const #16s : i64
        let s_5_3: i64 = 16;
        // D s_5_4: write-var esize <= s_5_3
        fn_state.esize = s_5_3;
        // D s_5_5: read-var size:u8
        let s_5_5: u8 = fn_state.size;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 2u16);
        // C s_5_7: const #1u : u8
        let s_5_7: u8 = 1;
        // C s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 2u16);
        // D s_5_9: cmp-eq s_5_6 s_5_8
        let s_5_9: bool = ((s_5_6) == (s_5_8));
        // D s_5_10: not s_5_9
        let s_5_10: bool = !s_5_9;
        // N s_5_11: branch s_5_10 b8 b6
        if s_5_10 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #16s : i64
        let s_6_0: i64 = 16;
        // D s_6_1: write-var esize <= s_6_0
        fn_state.esize = s_6_0;
        // D s_6_2: read-var Vd:u8
        let s_6_2: u8 = fn_state.Vd;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 4u16);
        // D s_6_4: read-var D:u8
        let s_6_4: bool = fn_state.D;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // D s_6_6: cast reint s_6_3 -> u128
        let s_6_6: u128 = (s_6_3.value() as u128);
        // D s_6_7: size-of s_6_3
        let s_6_7: u16 = s_6_3.length();
        // D s_6_8: cast reint s_6_5 -> u128
        let s_6_8: u128 = (s_6_5.value() as u128);
        // D s_6_9: size-of s_6_5
        let s_6_9: u16 = s_6_5.length();
        // D s_6_10: lsl s_6_6 s_6_9
        let s_6_10: u128 = s_6_6 << s_6_9;
        // D s_6_11: or s_6_10 s_6_8
        let s_6_11: u128 = ((s_6_10) | (s_6_8));
        // D s_6_12: add s_6_7 s_6_9
        let s_6_12: u16 = (s_6_7 + s_6_9);
        // D s_6_13: create-bits s_6_11 s_6_12
        let s_6_13: Bits = Bits::new(s_6_11, s_6_12);
        // D s_6_14: cast reint s_6_13 -> u8
        let s_6_14: u8 = (s_6_13.value() as u8);
        // D s_6_15: cast zx s_6_14 -> bv
        let s_6_15: Bits = Bits::new(s_6_14 as u128, 5u16);
        // D s_6_16: cast zx s_6_15 -> i
        let s_6_16: i128 = (s_6_15.value() as i128);
        // D s_6_17: cast reint s_6_16 -> i64
        let s_6_17: i64 = (s_6_16 as i64);
        // D s_6_18: write-var d <= s_6_17
        fn_state.d = s_6_17;
        // D s_6_19: read-var Vm:u8
        let s_6_19: u8 = fn_state.Vm;
        // D s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 4u16);
        // D s_6_21: read-var M:u8
        let s_6_21: bool = fn_state.M;
        // D s_6_22: cast zx s_6_21 -> bv
        let s_6_22: Bits = Bits::new(s_6_21 as u128, 1u16);
        // D s_6_23: cast reint s_6_20 -> u128
        let s_6_23: u128 = (s_6_20.value() as u128);
        // D s_6_24: size-of s_6_20
        let s_6_24: u16 = s_6_20.length();
        // D s_6_25: cast reint s_6_22 -> u128
        let s_6_25: u128 = (s_6_22.value() as u128);
        // D s_6_26: size-of s_6_22
        let s_6_26: u16 = s_6_22.length();
        // D s_6_27: lsl s_6_23 s_6_26
        let s_6_27: u128 = s_6_23 << s_6_26;
        // D s_6_28: or s_6_27 s_6_25
        let s_6_28: u128 = ((s_6_27) | (s_6_25));
        // D s_6_29: add s_6_24 s_6_26
        let s_6_29: u16 = (s_6_24 + s_6_26);
        // D s_6_30: create-bits s_6_28 s_6_29
        let s_6_30: Bits = Bits::new(s_6_28, s_6_29);
        // D s_6_31: cast reint s_6_30 -> u8
        let s_6_31: u8 = (s_6_30.value() as u8);
        // D s_6_32: cast zx s_6_31 -> bv
        let s_6_32: Bits = Bits::new(s_6_31 as u128, 5u16);
        // D s_6_33: cast zx s_6_32 -> i
        let s_6_33: i128 = (s_6_32.value() as i128);
        // D s_6_34: cast reint s_6_33 -> i64
        let s_6_34: i64 = (s_6_33 as i64);
        // D s_6_35: write-var m <= s_6_34
        fn_state.m = s_6_34;
        // N s_6_36: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var m:i64
        let s_7_0: i64 = fn_state.m;
        // D s_7_1: read-var esize:i64
        let s_7_1: i64 = fn_state.esize;
        // D s_7_2: read-var d:i64
        let s_7_2: i64 = fn_state.d;
        // C s_7_3: const #0u : u8
        let s_7_3: bool = false;
        // D s_7_4: read-var rounding:u32
        let s_7_4: u32 = fn_state.rounding;
        // D s_7_5: call execute_aarch32_instrs_VRINTA_vfp_Op_A_txt(s_7_2, s_7_1, s_7_3, s_7_0, s_7_4)
        let s_7_5: () = execute_aarch32_instrs_VRINTA_vfp_Op_A_txt(
            state,
            tracer,
            s_7_2,
            s_7_1,
            s_7_3,
            s_7_0,
            s_7_4,
        );
        // N s_7_6: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var size:u8
        let s_8_0: u8 = fn_state.size;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b10 b9
        if s_8_5 {
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
        // C s_9_0: const #32s : i64
        let s_9_0: i64 = 32;
        // D s_9_1: write-var esize <= s_9_0
        fn_state.esize = s_9_0;
        // D s_9_2: read-var Vd:u8
        let s_9_2: u8 = fn_state.Vd;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 4u16);
        // D s_9_4: read-var D:u8
        let s_9_4: bool = fn_state.D;
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // D s_9_6: cast reint s_9_3 -> u128
        let s_9_6: u128 = (s_9_3.value() as u128);
        // D s_9_7: size-of s_9_3
        let s_9_7: u16 = s_9_3.length();
        // D s_9_8: cast reint s_9_5 -> u128
        let s_9_8: u128 = (s_9_5.value() as u128);
        // D s_9_9: size-of s_9_5
        let s_9_9: u16 = s_9_5.length();
        // D s_9_10: lsl s_9_6 s_9_9
        let s_9_10: u128 = s_9_6 << s_9_9;
        // D s_9_11: or s_9_10 s_9_8
        let s_9_11: u128 = ((s_9_10) | (s_9_8));
        // D s_9_12: add s_9_7 s_9_9
        let s_9_12: u16 = (s_9_7 + s_9_9);
        // D s_9_13: create-bits s_9_11 s_9_12
        let s_9_13: Bits = Bits::new(s_9_11, s_9_12);
        // D s_9_14: cast reint s_9_13 -> u8
        let s_9_14: u8 = (s_9_13.value() as u8);
        // D s_9_15: cast zx s_9_14 -> bv
        let s_9_15: Bits = Bits::new(s_9_14 as u128, 5u16);
        // D s_9_16: cast zx s_9_15 -> i
        let s_9_16: i128 = (s_9_15.value() as i128);
        // D s_9_17: cast reint s_9_16 -> i64
        let s_9_17: i64 = (s_9_16 as i64);
        // D s_9_18: write-var d <= s_9_17
        fn_state.d = s_9_17;
        // D s_9_19: read-var Vm:u8
        let s_9_19: u8 = fn_state.Vm;
        // D s_9_20: cast zx s_9_19 -> bv
        let s_9_20: Bits = Bits::new(s_9_19 as u128, 4u16);
        // D s_9_21: read-var M:u8
        let s_9_21: bool = fn_state.M;
        // D s_9_22: cast zx s_9_21 -> bv
        let s_9_22: Bits = Bits::new(s_9_21 as u128, 1u16);
        // D s_9_23: cast reint s_9_20 -> u128
        let s_9_23: u128 = (s_9_20.value() as u128);
        // D s_9_24: size-of s_9_20
        let s_9_24: u16 = s_9_20.length();
        // D s_9_25: cast reint s_9_22 -> u128
        let s_9_25: u128 = (s_9_22.value() as u128);
        // D s_9_26: size-of s_9_22
        let s_9_26: u16 = s_9_22.length();
        // D s_9_27: lsl s_9_23 s_9_26
        let s_9_27: u128 = s_9_23 << s_9_26;
        // D s_9_28: or s_9_27 s_9_25
        let s_9_28: u128 = ((s_9_27) | (s_9_25));
        // D s_9_29: add s_9_24 s_9_26
        let s_9_29: u16 = (s_9_24 + s_9_26);
        // D s_9_30: create-bits s_9_28 s_9_29
        let s_9_30: Bits = Bits::new(s_9_28, s_9_29);
        // D s_9_31: cast reint s_9_30 -> u8
        let s_9_31: u8 = (s_9_30.value() as u8);
        // D s_9_32: cast zx s_9_31 -> bv
        let s_9_32: Bits = Bits::new(s_9_31 as u128, 5u16);
        // D s_9_33: cast zx s_9_32 -> i
        let s_9_33: i128 = (s_9_32.value() as i128);
        // D s_9_34: cast reint s_9_33 -> i64
        let s_9_34: i64 = (s_9_33 as i64);
        // D s_9_35: write-var m <= s_9_34
        fn_state.m = s_9_34;
        // N s_9_36: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var size:u8
        let s_10_0: u8 = fn_state.size;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #3u : u8
        let s_10_2: u8 = 3;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: not s_10_4
        let s_10_5: bool = !s_10_4;
        // N s_10_6: branch s_10_5 b12 b11
        if s_10_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // D s_11_1: write-var esize <= s_11_0
        fn_state.esize = s_11_0;
        // D s_11_2: read-var D:u8
        let s_11_2: bool = fn_state.D;
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: read-var Vd:u8
        let s_11_4: u8 = fn_state.Vd;
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 4u16);
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
        // D s_11_19: read-var M:u8
        let s_11_19: bool = fn_state.M;
        // D s_11_20: cast zx s_11_19 -> bv
        let s_11_20: Bits = Bits::new(s_11_19 as u128, 1u16);
        // D s_11_21: read-var Vm:u8
        let s_11_21: u8 = fn_state.Vm;
        // D s_11_22: cast zx s_11_21 -> bv
        let s_11_22: Bits = Bits::new(s_11_21 as u128, 4u16);
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
        // N s_11_36: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call HaveFP16Ext(s_14_0)
        let s_14_1: bool = HaveFP16Ext(state, tracer, s_14_0);
        // S s_14_2: not s_14_1
        let s_14_2: bool = !s_14_1;
        // D s_14_3: write-var gs#325508 <= s_14_2
        fn_state.gs_325508 = s_14_2;
        // N s_14_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#325509 <= s_15_0
        fn_state.gs_325509 = s_15_0;
        // N s_15_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
