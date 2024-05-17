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
use FPSCR_read__1::*;
use u_get_FPSCR_Type_length::*;
use ConditionPassed::*;
use u_get_FPSCR_Type_Stride::*;
use execute_aarch32_instrs_VFMA_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_VFMA_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Vn: u8,
    Vd: u8,
    size: u8,
    N: bool,
    op: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        n: i64,
        gs_309028: bool,
        ga_353052: i128,
        ga_353054: i128,
        gs_309030: bool,
        gs_309029: bool,
        d: i64,
        op1_neg: bool,
        D: bool,
        Vn: u8,
        Vd: u8,
        size: u8,
        N: bool,
        op: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        Vn,
        Vd,
        size,
        N,
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
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call FPSCR_read__1(s_2_0)
        let s_2_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_2_0);
        // S s_2_2: call _get_FPSCR_Type_length(s_2_1)
        let s_2_2: u8 = u_get_FPSCR_Type_length(state, tracer, s_2_1);
        // S s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 3u16);
        // C s_2_4: const #0u : u8
        let s_2_4: u8 = 0;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 3u16);
        // S s_2_6: cmp-ne s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) != (s_2_5));
        // N s_2_7: branch s_2_6 b24 b3
        if s_2_6 {
            return block_24(state, tracer, fn_state);
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
        // S s_3_1: call FPSCR_read__1(s_3_0)
        let s_3_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_3_0);
        // S s_3_2: call _get_FPSCR_Type_Stride(s_3_1)
        let s_3_2: u8 = u_get_FPSCR_Type_Stride(state, tracer, s_3_1);
        // S s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // C s_3_4: const #0u : u8
        let s_3_4: u8 = 0;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // S s_3_6: cmp-ne s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) != (s_3_5));
        // D s_3_7: write-var gs#309028 <= s_3_6
        fn_state.gs_309028 = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#309028:u8
        let s_4_0: bool = fn_state.gs_309028;
        // N s_4_1: branch s_4_0 b23 b5
        if s_4_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var size:u8
        let s_5_0: u8 = fn_state.size;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #0u : u8
        let s_5_2: u8 = 0;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b22 b6
        if s_5_4 {
            return block_22(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#309029 <= s_6_0
        fn_state.gs_309029 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#309029:u8
        let s_7_0: bool = fn_state.gs_309029;
        // N s_7_1: branch s_7_0 b21 b8
        if s_7_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
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
        // C s_8_2: const #1u : u8
        let s_8_2: u8 = 1;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b20 b9
        if s_8_4 {
            return block_20(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#309030 <= s_9_0
        fn_state.gs_309030 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#309030:u8
        let s_10_0: bool = fn_state.gs_309030;
        // N s_10_1: branch s_10_0 b19 b11
        if s_10_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var op:u8
        let s_11_0: bool = fn_state.op;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #1u : u8
        let s_11_2: bool = true;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: write-var op1_neg <= s_11_4
        fn_state.op1_neg = s_11_4;
        // C s_11_6: const #16s : i64
        let s_11_6: i64 = 16;
        // D s_11_7: write-var esize <= s_11_6
        fn_state.esize = s_11_6;
        // D s_11_8: read-var size:u8
        let s_11_8: u8 = fn_state.size;
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
        // N s_11_14: branch s_11_13 b14 b12
        if s_11_13 {
            return block_14(state, tracer, fn_state);
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
        // D s_12_2: read-var Vd:u8
        let s_12_2: u8 = fn_state.Vd;
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 4u16);
        // D s_12_4: read-var D:u8
        let s_12_4: bool = fn_state.D;
        // D s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 1u16);
        // D s_12_6: cast reint s_12_3 -> u128
        let s_12_6: u128 = (s_12_3.value() as u128);
        // D s_12_7: size-of s_12_3
        let s_12_7: u16 = s_12_3.length();
        // D s_12_8: cast reint s_12_5 -> u128
        let s_12_8: u128 = (s_12_5.value() as u128);
        // D s_12_9: size-of s_12_5
        let s_12_9: u16 = s_12_5.length();
        // D s_12_10: lsl s_12_6 s_12_9
        let s_12_10: u128 = s_12_6 << s_12_9;
        // D s_12_11: or s_12_10 s_12_8
        let s_12_11: u128 = ((s_12_10) | (s_12_8));
        // D s_12_12: add s_12_7 s_12_9
        let s_12_12: u16 = (s_12_7 + s_12_9);
        // D s_12_13: create-bits s_12_11 s_12_12
        let s_12_13: Bits = Bits::new(s_12_11, s_12_12);
        // D s_12_14: cast reint s_12_13 -> u8
        let s_12_14: u8 = (s_12_13.value() as u8);
        // D s_12_15: cast zx s_12_14 -> bv
        let s_12_15: Bits = Bits::new(s_12_14 as u128, 5u16);
        // D s_12_16: cast zx s_12_15 -> i
        let s_12_16: i128 = (s_12_15.value() as i128);
        // D s_12_17: cast reint s_12_16 -> i64
        let s_12_17: i64 = (s_12_16 as i64);
        // D s_12_18: write-var d <= s_12_17
        fn_state.d = s_12_17;
        // D s_12_19: read-var Vn:u8
        let s_12_19: u8 = fn_state.Vn;
        // D s_12_20: cast zx s_12_19 -> bv
        let s_12_20: Bits = Bits::new(s_12_19 as u128, 4u16);
        // D s_12_21: read-var N:u8
        let s_12_21: bool = fn_state.N;
        // D s_12_22: cast zx s_12_21 -> bv
        let s_12_22: Bits = Bits::new(s_12_21 as u128, 1u16);
        // D s_12_23: cast reint s_12_20 -> u128
        let s_12_23: u128 = (s_12_20.value() as u128);
        // D s_12_24: size-of s_12_20
        let s_12_24: u16 = s_12_20.length();
        // D s_12_25: cast reint s_12_22 -> u128
        let s_12_25: u128 = (s_12_22.value() as u128);
        // D s_12_26: size-of s_12_22
        let s_12_26: u16 = s_12_22.length();
        // D s_12_27: lsl s_12_23 s_12_26
        let s_12_27: u128 = s_12_23 << s_12_26;
        // D s_12_28: or s_12_27 s_12_25
        let s_12_28: u128 = ((s_12_27) | (s_12_25));
        // D s_12_29: add s_12_24 s_12_26
        let s_12_29: u16 = (s_12_24 + s_12_26);
        // D s_12_30: create-bits s_12_28 s_12_29
        let s_12_30: Bits = Bits::new(s_12_28, s_12_29);
        // D s_12_31: cast reint s_12_30 -> u8
        let s_12_31: u8 = (s_12_30.value() as u8);
        // D s_12_32: cast zx s_12_31 -> bv
        let s_12_32: Bits = Bits::new(s_12_31 as u128, 5u16);
        // D s_12_33: cast zx s_12_32 -> i
        let s_12_33: i128 = (s_12_32.value() as i128);
        // D s_12_34: cast reint s_12_33 -> i64
        let s_12_34: i64 = (s_12_33 as i64);
        // D s_12_35: write-var n <= s_12_34
        fn_state.n = s_12_34;
        // D s_12_36: read-var Vm:u8
        let s_12_36: u8 = fn_state.Vm;
        // D s_12_37: cast zx s_12_36 -> bv
        let s_12_37: Bits = Bits::new(s_12_36 as u128, 4u16);
        // D s_12_38: read-var M:u8
        let s_12_38: bool = fn_state.M;
        // D s_12_39: cast zx s_12_38 -> bv
        let s_12_39: Bits = Bits::new(s_12_38 as u128, 1u16);
        // D s_12_40: cast reint s_12_37 -> u128
        let s_12_40: u128 = (s_12_37.value() as u128);
        // D s_12_41: size-of s_12_37
        let s_12_41: u16 = s_12_37.length();
        // D s_12_42: cast reint s_12_39 -> u128
        let s_12_42: u128 = (s_12_39.value() as u128);
        // D s_12_43: size-of s_12_39
        let s_12_43: u16 = s_12_39.length();
        // D s_12_44: lsl s_12_40 s_12_43
        let s_12_44: u128 = s_12_40 << s_12_43;
        // D s_12_45: or s_12_44 s_12_42
        let s_12_45: u128 = ((s_12_44) | (s_12_42));
        // D s_12_46: add s_12_41 s_12_43
        let s_12_46: u16 = (s_12_41 + s_12_43);
        // D s_12_47: create-bits s_12_45 s_12_46
        let s_12_47: Bits = Bits::new(s_12_45, s_12_46);
        // D s_12_48: cast reint s_12_47 -> u8
        let s_12_48: u8 = (s_12_47.value() as u8);
        // D s_12_49: cast zx s_12_48 -> bv
        let s_12_49: Bits = Bits::new(s_12_48 as u128, 5u16);
        // D s_12_50: cast zx s_12_49 -> i
        let s_12_50: i128 = (s_12_49.value() as i128);
        // D s_12_51: cast reint s_12_50 -> i64
        let s_12_51: i64 = (s_12_50 as i64);
        // D s_12_52: write-var m <= s_12_51
        fn_state.m = s_12_51;
        // N s_12_53: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var n:i64
        let s_13_0: i64 = fn_state.n;
        // D s_13_1: read-var m:i64
        let s_13_1: i64 = fn_state.m;
        // D s_13_2: read-var esize:i64
        let s_13_2: i64 = fn_state.esize;
        // D s_13_3: read-var d:i64
        let s_13_3: i64 = fn_state.d;
        // D s_13_4: cast zx s_13_2 -> i
        let s_13_4: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_5: cast reint s_13_4 -> i64
        let s_13_5: i64 = (s_13_4 as i64);
        // C s_13_6: const #0u : u8
        let s_13_6: bool = false;
        // D s_13_7: read-var ga#353052:i
        let s_13_7: i128 = fn_state.ga_353052;
        // D s_13_8: read-var op1_neg:u8
        let s_13_8: bool = fn_state.op1_neg;
        // D s_13_9: read-var ga#353054:i
        let s_13_9: i128 = fn_state.ga_353054;
        // D s_13_10: call execute_aarch32_instrs_VFMA_Op_A_txt(s_13_6, s_13_3, s_13_7, s_13_5, s_13_1, s_13_0, s_13_8, s_13_9)
        let s_13_10: () = execute_aarch32_instrs_VFMA_Op_A_txt(
            state,
            tracer,
            s_13_6,
            s_13_3,
            s_13_7,
            s_13_5,
            s_13_1,
            s_13_0,
            s_13_8,
            s_13_9,
        );
        // N s_13_11: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var size:u8
        let s_14_0: u8 = fn_state.size;
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
        // C s_15_0: const #32s : i64
        let s_15_0: i64 = 32;
        // D s_15_1: write-var esize <= s_15_0
        fn_state.esize = s_15_0;
        // D s_15_2: read-var Vd:u8
        let s_15_2: u8 = fn_state.Vd;
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 4u16);
        // D s_15_4: read-var D:u8
        let s_15_4: bool = fn_state.D;
        // D s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 1u16);
        // D s_15_6: cast reint s_15_3 -> u128
        let s_15_6: u128 = (s_15_3.value() as u128);
        // D s_15_7: size-of s_15_3
        let s_15_7: u16 = s_15_3.length();
        // D s_15_8: cast reint s_15_5 -> u128
        let s_15_8: u128 = (s_15_5.value() as u128);
        // D s_15_9: size-of s_15_5
        let s_15_9: u16 = s_15_5.length();
        // D s_15_10: lsl s_15_6 s_15_9
        let s_15_10: u128 = s_15_6 << s_15_9;
        // D s_15_11: or s_15_10 s_15_8
        let s_15_11: u128 = ((s_15_10) | (s_15_8));
        // D s_15_12: add s_15_7 s_15_9
        let s_15_12: u16 = (s_15_7 + s_15_9);
        // D s_15_13: create-bits s_15_11 s_15_12
        let s_15_13: Bits = Bits::new(s_15_11, s_15_12);
        // D s_15_14: cast reint s_15_13 -> u8
        let s_15_14: u8 = (s_15_13.value() as u8);
        // D s_15_15: cast zx s_15_14 -> bv
        let s_15_15: Bits = Bits::new(s_15_14 as u128, 5u16);
        // D s_15_16: cast zx s_15_15 -> i
        let s_15_16: i128 = (s_15_15.value() as i128);
        // D s_15_17: cast reint s_15_16 -> i64
        let s_15_17: i64 = (s_15_16 as i64);
        // D s_15_18: write-var d <= s_15_17
        fn_state.d = s_15_17;
        // D s_15_19: read-var Vn:u8
        let s_15_19: u8 = fn_state.Vn;
        // D s_15_20: cast zx s_15_19 -> bv
        let s_15_20: Bits = Bits::new(s_15_19 as u128, 4u16);
        // D s_15_21: read-var N:u8
        let s_15_21: bool = fn_state.N;
        // D s_15_22: cast zx s_15_21 -> bv
        let s_15_22: Bits = Bits::new(s_15_21 as u128, 1u16);
        // D s_15_23: cast reint s_15_20 -> u128
        let s_15_23: u128 = (s_15_20.value() as u128);
        // D s_15_24: size-of s_15_20
        let s_15_24: u16 = s_15_20.length();
        // D s_15_25: cast reint s_15_22 -> u128
        let s_15_25: u128 = (s_15_22.value() as u128);
        // D s_15_26: size-of s_15_22
        let s_15_26: u16 = s_15_22.length();
        // D s_15_27: lsl s_15_23 s_15_26
        let s_15_27: u128 = s_15_23 << s_15_26;
        // D s_15_28: or s_15_27 s_15_25
        let s_15_28: u128 = ((s_15_27) | (s_15_25));
        // D s_15_29: add s_15_24 s_15_26
        let s_15_29: u16 = (s_15_24 + s_15_26);
        // D s_15_30: create-bits s_15_28 s_15_29
        let s_15_30: Bits = Bits::new(s_15_28, s_15_29);
        // D s_15_31: cast reint s_15_30 -> u8
        let s_15_31: u8 = (s_15_30.value() as u8);
        // D s_15_32: cast zx s_15_31 -> bv
        let s_15_32: Bits = Bits::new(s_15_31 as u128, 5u16);
        // D s_15_33: cast zx s_15_32 -> i
        let s_15_33: i128 = (s_15_32.value() as i128);
        // D s_15_34: cast reint s_15_33 -> i64
        let s_15_34: i64 = (s_15_33 as i64);
        // D s_15_35: write-var n <= s_15_34
        fn_state.n = s_15_34;
        // D s_15_36: read-var Vm:u8
        let s_15_36: u8 = fn_state.Vm;
        // D s_15_37: cast zx s_15_36 -> bv
        let s_15_37: Bits = Bits::new(s_15_36 as u128, 4u16);
        // D s_15_38: read-var M:u8
        let s_15_38: bool = fn_state.M;
        // D s_15_39: cast zx s_15_38 -> bv
        let s_15_39: Bits = Bits::new(s_15_38 as u128, 1u16);
        // D s_15_40: cast reint s_15_37 -> u128
        let s_15_40: u128 = (s_15_37.value() as u128);
        // D s_15_41: size-of s_15_37
        let s_15_41: u16 = s_15_37.length();
        // D s_15_42: cast reint s_15_39 -> u128
        let s_15_42: u128 = (s_15_39.value() as u128);
        // D s_15_43: size-of s_15_39
        let s_15_43: u16 = s_15_39.length();
        // D s_15_44: lsl s_15_40 s_15_43
        let s_15_44: u128 = s_15_40 << s_15_43;
        // D s_15_45: or s_15_44 s_15_42
        let s_15_45: u128 = ((s_15_44) | (s_15_42));
        // D s_15_46: add s_15_41 s_15_43
        let s_15_46: u16 = (s_15_41 + s_15_43);
        // D s_15_47: create-bits s_15_45 s_15_46
        let s_15_47: Bits = Bits::new(s_15_45, s_15_46);
        // D s_15_48: cast reint s_15_47 -> u8
        let s_15_48: u8 = (s_15_47.value() as u8);
        // D s_15_49: cast zx s_15_48 -> bv
        let s_15_49: Bits = Bits::new(s_15_48 as u128, 5u16);
        // D s_15_50: cast zx s_15_49 -> i
        let s_15_50: i128 = (s_15_49.value() as i128);
        // D s_15_51: cast reint s_15_50 -> i64
        let s_15_51: i64 = (s_15_50 as i64);
        // D s_15_52: write-var m <= s_15_51
        fn_state.m = s_15_51;
        // N s_15_53: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var size:u8
        let s_16_0: u8 = fn_state.size;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 2u16);
        // C s_16_2: const #3u : u8
        let s_16_2: u8 = 3;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 2u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: not s_16_4
        let s_16_5: bool = !s_16_4;
        // N s_16_6: branch s_16_5 b18 b17
        if s_16_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #64s : i64
        let s_17_0: i64 = 64;
        // D s_17_1: write-var esize <= s_17_0
        fn_state.esize = s_17_0;
        // D s_17_2: read-var D:u8
        let s_17_2: bool = fn_state.D;
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_4: read-var Vd:u8
        let s_17_4: u8 = fn_state.Vd;
        // D s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 4u16);
        // D s_17_6: cast reint s_17_3 -> u128
        let s_17_6: u128 = (s_17_3.value() as u128);
        // D s_17_7: size-of s_17_3
        let s_17_7: u16 = s_17_3.length();
        // D s_17_8: cast reint s_17_5 -> u128
        let s_17_8: u128 = (s_17_5.value() as u128);
        // D s_17_9: size-of s_17_5
        let s_17_9: u16 = s_17_5.length();
        // D s_17_10: lsl s_17_6 s_17_9
        let s_17_10: u128 = s_17_6 << s_17_9;
        // D s_17_11: or s_17_10 s_17_8
        let s_17_11: u128 = ((s_17_10) | (s_17_8));
        // D s_17_12: add s_17_7 s_17_9
        let s_17_12: u16 = (s_17_7 + s_17_9);
        // D s_17_13: create-bits s_17_11 s_17_12
        let s_17_13: Bits = Bits::new(s_17_11, s_17_12);
        // D s_17_14: cast reint s_17_13 -> u8
        let s_17_14: u8 = (s_17_13.value() as u8);
        // D s_17_15: cast zx s_17_14 -> bv
        let s_17_15: Bits = Bits::new(s_17_14 as u128, 5u16);
        // D s_17_16: cast zx s_17_15 -> i
        let s_17_16: i128 = (s_17_15.value() as i128);
        // D s_17_17: cast reint s_17_16 -> i64
        let s_17_17: i64 = (s_17_16 as i64);
        // D s_17_18: write-var d <= s_17_17
        fn_state.d = s_17_17;
        // D s_17_19: read-var N:u8
        let s_17_19: bool = fn_state.N;
        // D s_17_20: cast zx s_17_19 -> bv
        let s_17_20: Bits = Bits::new(s_17_19 as u128, 1u16);
        // D s_17_21: read-var Vn:u8
        let s_17_21: u8 = fn_state.Vn;
        // D s_17_22: cast zx s_17_21 -> bv
        let s_17_22: Bits = Bits::new(s_17_21 as u128, 4u16);
        // D s_17_23: cast reint s_17_20 -> u128
        let s_17_23: u128 = (s_17_20.value() as u128);
        // D s_17_24: size-of s_17_20
        let s_17_24: u16 = s_17_20.length();
        // D s_17_25: cast reint s_17_22 -> u128
        let s_17_25: u128 = (s_17_22.value() as u128);
        // D s_17_26: size-of s_17_22
        let s_17_26: u16 = s_17_22.length();
        // D s_17_27: lsl s_17_23 s_17_26
        let s_17_27: u128 = s_17_23 << s_17_26;
        // D s_17_28: or s_17_27 s_17_25
        let s_17_28: u128 = ((s_17_27) | (s_17_25));
        // D s_17_29: add s_17_24 s_17_26
        let s_17_29: u16 = (s_17_24 + s_17_26);
        // D s_17_30: create-bits s_17_28 s_17_29
        let s_17_30: Bits = Bits::new(s_17_28, s_17_29);
        // D s_17_31: cast reint s_17_30 -> u8
        let s_17_31: u8 = (s_17_30.value() as u8);
        // D s_17_32: cast zx s_17_31 -> bv
        let s_17_32: Bits = Bits::new(s_17_31 as u128, 5u16);
        // D s_17_33: cast zx s_17_32 -> i
        let s_17_33: i128 = (s_17_32.value() as i128);
        // D s_17_34: cast reint s_17_33 -> i64
        let s_17_34: i64 = (s_17_33 as i64);
        // D s_17_35: write-var n <= s_17_34
        fn_state.n = s_17_34;
        // D s_17_36: read-var M:u8
        let s_17_36: bool = fn_state.M;
        // D s_17_37: cast zx s_17_36 -> bv
        let s_17_37: Bits = Bits::new(s_17_36 as u128, 1u16);
        // D s_17_38: read-var Vm:u8
        let s_17_38: u8 = fn_state.Vm;
        // D s_17_39: cast zx s_17_38 -> bv
        let s_17_39: Bits = Bits::new(s_17_38 as u128, 4u16);
        // D s_17_40: cast reint s_17_37 -> u128
        let s_17_40: u128 = (s_17_37.value() as u128);
        // D s_17_41: size-of s_17_37
        let s_17_41: u16 = s_17_37.length();
        // D s_17_42: cast reint s_17_39 -> u128
        let s_17_42: u128 = (s_17_39.value() as u128);
        // D s_17_43: size-of s_17_39
        let s_17_43: u16 = s_17_39.length();
        // D s_17_44: lsl s_17_40 s_17_43
        let s_17_44: u128 = s_17_40 << s_17_43;
        // D s_17_45: or s_17_44 s_17_42
        let s_17_45: u128 = ((s_17_44) | (s_17_42));
        // D s_17_46: add s_17_41 s_17_43
        let s_17_46: u16 = (s_17_41 + s_17_43);
        // D s_17_47: create-bits s_17_45 s_17_46
        let s_17_47: Bits = Bits::new(s_17_45, s_17_46);
        // D s_17_48: cast reint s_17_47 -> u8
        let s_17_48: u8 = (s_17_47.value() as u8);
        // D s_17_49: cast zx s_17_48 -> bv
        let s_17_49: Bits = Bits::new(s_17_48 as u128, 5u16);
        // D s_17_50: cast zx s_17_49 -> i
        let s_17_50: i128 = (s_17_49.value() as i128);
        // D s_17_51: cast reint s_17_50 -> i64
        let s_17_51: i64 = (s_17_50 as i64);
        // D s_17_52: write-var m <= s_17_51
        fn_state.m = s_17_51;
        // N s_17_53: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b13
        return block_13(state, tracer, fn_state);
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
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call InITBlock(s_20_0)
        let s_20_1: bool = InITBlock(state, tracer, s_20_0);
        // D s_20_2: write-var gs#309030 <= s_20_1
        fn_state.gs_309030 = s_20_1;
        // N s_20_3: jump b10
        return block_10(state, tracer, fn_state);
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
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#309029 <= s_22_0
        fn_state.gs_309029 = s_22_0;
        // N s_22_2: jump b7
        return block_7(state, tracer, fn_state);
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
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#309028 <= s_24_0
        fn_state.gs_309028 = s_24_0;
        // N s_24_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
