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
use execute_aarch32_instrs_VDIV_Op_A_txt::*;
use HaveFP16Ext::*;
use common::*;
pub fn decode_aarch32_instrs_VDIV_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    D: bool,
    Vn: u8,
    Vd: u8,
    size: u8,
    N: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        gs_308482: bool,
        n: i64,
        gs_308483: bool,
        d: i64,
        gs_308484: bool,
        gs_308485: bool,
        cond: u8,
        D: bool,
        Vn: u8,
        Vd: u8,
        size: u8,
        N: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        cond,
        D,
        Vn,
        Vd,
        size,
        N,
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
        // C s_2_6: const #() : ()
        let s_2_6: () = ();
        // S s_2_7: call FPSCR_read__1(s_2_6)
        let s_2_7: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_2_6);
        // S s_2_8: call _get_FPSCR_Type_length(s_2_7)
        let s_2_8: u8 = u_get_FPSCR_Type_length(state, tracer, s_2_7);
        // S s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 3u16);
        // C s_2_10: const #0u : u8
        let s_2_10: u8 = 0;
        // C s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 3u16);
        // S s_2_12: cmp-ne s_2_9 s_2_11
        let s_2_12: bool = ((s_2_9) != (s_2_11));
        // N s_2_13: branch s_2_12 b27 b3
        if s_2_12 {
            return block_27(state, tracer, fn_state);
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
        // D s_3_7: write-var gs#308482 <= s_3_6
        fn_state.gs_308482 = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#308482:u8
        let s_4_0: bool = fn_state.gs_308482;
        // N s_4_1: branch s_4_0 b26 b5
        if s_4_0 {
            return block_26(state, tracer, fn_state);
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
        // N s_5_5: branch s_5_4 b25 b6
        if s_5_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var size:u8
        let s_6_0: u8 = fn_state.size;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #1u : u8
        let s_6_2: u8 = 1;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b24 b7
        if s_6_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#308483 <= s_7_0
        fn_state.gs_308483 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#308483:u8
        let s_8_0: bool = fn_state.gs_308483;
        // D s_8_1: write-var gs#308484 <= s_8_0
        fn_state.gs_308484 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#308484:u8
        let s_9_0: bool = fn_state.gs_308484;
        // N s_9_1: branch s_9_0 b23 b10
        if s_9_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
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
        // C s_10_2: const #1u : u8
        let s_10_2: u8 = 1;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b22 b11
        if s_10_4 {
            return block_22(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#308485 <= s_11_0
        fn_state.gs_308485 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#308485:u8
        let s_12_0: bool = fn_state.gs_308485;
        // N s_12_1: branch s_12_0 b21 b13
        if s_12_0 {
            return block_21(state, tracer, fn_state);
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
        // D s_13_2: read-var size:u8
        let s_13_2: u8 = fn_state.size;
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // C s_13_4: const #1u : u8
        let s_13_4: u8 = 1;
        // C s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 2u16);
        // D s_13_6: cmp-eq s_13_3 s_13_5
        let s_13_6: bool = ((s_13_3) == (s_13_5));
        // D s_13_7: not s_13_6
        let s_13_7: bool = !s_13_6;
        // N s_13_8: branch s_13_7 b16 b14
        if s_13_7 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #16s : i64
        let s_14_0: i64 = 16;
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
        // D s_14_19: read-var Vn:u8
        let s_14_19: u8 = fn_state.Vn;
        // D s_14_20: cast zx s_14_19 -> bv
        let s_14_20: Bits = Bits::new(s_14_19 as u128, 4u16);
        // D s_14_21: read-var N:u8
        let s_14_21: bool = fn_state.N;
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
        // D s_14_35: write-var n <= s_14_34
        fn_state.n = s_14_34;
        // D s_14_36: read-var Vm:u8
        let s_14_36: u8 = fn_state.Vm;
        // D s_14_37: cast zx s_14_36 -> bv
        let s_14_37: Bits = Bits::new(s_14_36 as u128, 4u16);
        // D s_14_38: read-var M:u8
        let s_14_38: bool = fn_state.M;
        // D s_14_39: cast zx s_14_38 -> bv
        let s_14_39: Bits = Bits::new(s_14_38 as u128, 1u16);
        // D s_14_40: cast reint s_14_37 -> u128
        let s_14_40: u128 = (s_14_37.value() as u128);
        // D s_14_41: size-of s_14_37
        let s_14_41: u16 = s_14_37.length();
        // D s_14_42: cast reint s_14_39 -> u128
        let s_14_42: u128 = (s_14_39.value() as u128);
        // D s_14_43: size-of s_14_39
        let s_14_43: u16 = s_14_39.length();
        // D s_14_44: lsl s_14_40 s_14_43
        let s_14_44: u128 = s_14_40 << s_14_43;
        // D s_14_45: or s_14_44 s_14_42
        let s_14_45: u128 = ((s_14_44) | (s_14_42));
        // D s_14_46: add s_14_41 s_14_43
        let s_14_46: u16 = (s_14_41 + s_14_43);
        // D s_14_47: create-bits s_14_45 s_14_46
        let s_14_47: Bits = Bits::new(s_14_45, s_14_46);
        // D s_14_48: cast reint s_14_47 -> u8
        let s_14_48: u8 = (s_14_47.value() as u8);
        // D s_14_49: cast zx s_14_48 -> bv
        let s_14_49: Bits = Bits::new(s_14_48 as u128, 5u16);
        // D s_14_50: cast zx s_14_49 -> i
        let s_14_50: i128 = (s_14_49.value() as i128);
        // D s_14_51: cast reint s_14_50 -> i64
        let s_14_51: i64 = (s_14_50 as i64);
        // D s_14_52: write-var m <= s_14_51
        fn_state.m = s_14_51;
        // N s_14_53: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var n:i64
        let s_15_0: i64 = fn_state.n;
        // D s_15_1: read-var m:i64
        let s_15_1: i64 = fn_state.m;
        // D s_15_2: read-var esize:i64
        let s_15_2: i64 = fn_state.esize;
        // D s_15_3: read-var d:i64
        let s_15_3: i64 = fn_state.d;
        // D s_15_4: call execute_aarch32_instrs_VDIV_Op_A_txt(s_15_3, s_15_2, s_15_1, s_15_0)
        let s_15_4: () = execute_aarch32_instrs_VDIV_Op_A_txt(
            state,
            tracer,
            s_15_3,
            s_15_2,
            s_15_1,
            s_15_0,
        );
        // N s_15_5: return
        return;
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
        // C s_16_2: const #2u : u8
        let s_16_2: u8 = 2;
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
        // C s_17_0: const #32s : i64
        let s_17_0: i64 = 32;
        // D s_17_1: write-var esize <= s_17_0
        fn_state.esize = s_17_0;
        // D s_17_2: read-var Vd:u8
        let s_17_2: u8 = fn_state.Vd;
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 4u16);
        // D s_17_4: read-var D:u8
        let s_17_4: bool = fn_state.D;
        // D s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 1u16);
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
        // D s_17_19: read-var Vn:u8
        let s_17_19: u8 = fn_state.Vn;
        // D s_17_20: cast zx s_17_19 -> bv
        let s_17_20: Bits = Bits::new(s_17_19 as u128, 4u16);
        // D s_17_21: read-var N:u8
        let s_17_21: bool = fn_state.N;
        // D s_17_22: cast zx s_17_21 -> bv
        let s_17_22: Bits = Bits::new(s_17_21 as u128, 1u16);
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
        // D s_17_36: read-var Vm:u8
        let s_17_36: u8 = fn_state.Vm;
        // D s_17_37: cast zx s_17_36 -> bv
        let s_17_37: Bits = Bits::new(s_17_36 as u128, 4u16);
        // D s_17_38: read-var M:u8
        let s_17_38: bool = fn_state.M;
        // D s_17_39: cast zx s_17_38 -> bv
        let s_17_39: Bits = Bits::new(s_17_38 as u128, 1u16);
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
        // N s_17_53: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var size:u8
        let s_18_0: u8 = fn_state.size;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 2u16);
        // C s_18_2: const #3u : u8
        let s_18_2: u8 = 3;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 2u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: not s_18_4
        let s_18_5: bool = !s_18_4;
        // N s_18_6: branch s_18_5 b20 b19
        if s_18_5 {
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
        // C s_19_0: const #64s : i64
        let s_19_0: i64 = 64;
        // D s_19_1: write-var esize <= s_19_0
        fn_state.esize = s_19_0;
        // D s_19_2: read-var D:u8
        let s_19_2: bool = fn_state.D;
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: read-var Vd:u8
        let s_19_4: u8 = fn_state.Vd;
        // D s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 4u16);
        // D s_19_6: cast reint s_19_3 -> u128
        let s_19_6: u128 = (s_19_3.value() as u128);
        // D s_19_7: size-of s_19_3
        let s_19_7: u16 = s_19_3.length();
        // D s_19_8: cast reint s_19_5 -> u128
        let s_19_8: u128 = (s_19_5.value() as u128);
        // D s_19_9: size-of s_19_5
        let s_19_9: u16 = s_19_5.length();
        // D s_19_10: lsl s_19_6 s_19_9
        let s_19_10: u128 = s_19_6 << s_19_9;
        // D s_19_11: or s_19_10 s_19_8
        let s_19_11: u128 = ((s_19_10) | (s_19_8));
        // D s_19_12: add s_19_7 s_19_9
        let s_19_12: u16 = (s_19_7 + s_19_9);
        // D s_19_13: create-bits s_19_11 s_19_12
        let s_19_13: Bits = Bits::new(s_19_11, s_19_12);
        // D s_19_14: cast reint s_19_13 -> u8
        let s_19_14: u8 = (s_19_13.value() as u8);
        // D s_19_15: cast zx s_19_14 -> bv
        let s_19_15: Bits = Bits::new(s_19_14 as u128, 5u16);
        // D s_19_16: cast zx s_19_15 -> i
        let s_19_16: i128 = (s_19_15.value() as i128);
        // D s_19_17: cast reint s_19_16 -> i64
        let s_19_17: i64 = (s_19_16 as i64);
        // D s_19_18: write-var d <= s_19_17
        fn_state.d = s_19_17;
        // D s_19_19: read-var N:u8
        let s_19_19: bool = fn_state.N;
        // D s_19_20: cast zx s_19_19 -> bv
        let s_19_20: Bits = Bits::new(s_19_19 as u128, 1u16);
        // D s_19_21: read-var Vn:u8
        let s_19_21: u8 = fn_state.Vn;
        // D s_19_22: cast zx s_19_21 -> bv
        let s_19_22: Bits = Bits::new(s_19_21 as u128, 4u16);
        // D s_19_23: cast reint s_19_20 -> u128
        let s_19_23: u128 = (s_19_20.value() as u128);
        // D s_19_24: size-of s_19_20
        let s_19_24: u16 = s_19_20.length();
        // D s_19_25: cast reint s_19_22 -> u128
        let s_19_25: u128 = (s_19_22.value() as u128);
        // D s_19_26: size-of s_19_22
        let s_19_26: u16 = s_19_22.length();
        // D s_19_27: lsl s_19_23 s_19_26
        let s_19_27: u128 = s_19_23 << s_19_26;
        // D s_19_28: or s_19_27 s_19_25
        let s_19_28: u128 = ((s_19_27) | (s_19_25));
        // D s_19_29: add s_19_24 s_19_26
        let s_19_29: u16 = (s_19_24 + s_19_26);
        // D s_19_30: create-bits s_19_28 s_19_29
        let s_19_30: Bits = Bits::new(s_19_28, s_19_29);
        // D s_19_31: cast reint s_19_30 -> u8
        let s_19_31: u8 = (s_19_30.value() as u8);
        // D s_19_32: cast zx s_19_31 -> bv
        let s_19_32: Bits = Bits::new(s_19_31 as u128, 5u16);
        // D s_19_33: cast zx s_19_32 -> i
        let s_19_33: i128 = (s_19_32.value() as i128);
        // D s_19_34: cast reint s_19_33 -> i64
        let s_19_34: i64 = (s_19_33 as i64);
        // D s_19_35: write-var n <= s_19_34
        fn_state.n = s_19_34;
        // D s_19_36: read-var M:u8
        let s_19_36: bool = fn_state.M;
        // D s_19_37: cast zx s_19_36 -> bv
        let s_19_37: Bits = Bits::new(s_19_36 as u128, 1u16);
        // D s_19_38: read-var Vm:u8
        let s_19_38: u8 = fn_state.Vm;
        // D s_19_39: cast zx s_19_38 -> bv
        let s_19_39: Bits = Bits::new(s_19_38 as u128, 4u16);
        // D s_19_40: cast reint s_19_37 -> u128
        let s_19_40: u128 = (s_19_37.value() as u128);
        // D s_19_41: size-of s_19_37
        let s_19_41: u16 = s_19_37.length();
        // D s_19_42: cast reint s_19_39 -> u128
        let s_19_42: u128 = (s_19_39.value() as u128);
        // D s_19_43: size-of s_19_39
        let s_19_43: u16 = s_19_39.length();
        // D s_19_44: lsl s_19_40 s_19_43
        let s_19_44: u128 = s_19_40 << s_19_43;
        // D s_19_45: or s_19_44 s_19_42
        let s_19_45: u128 = ((s_19_44) | (s_19_42));
        // D s_19_46: add s_19_41 s_19_43
        let s_19_46: u16 = (s_19_41 + s_19_43);
        // D s_19_47: create-bits s_19_45 s_19_46
        let s_19_47: Bits = Bits::new(s_19_45, s_19_46);
        // D s_19_48: cast reint s_19_47 -> u8
        let s_19_48: u8 = (s_19_47.value() as u8);
        // D s_19_49: cast zx s_19_48 -> bv
        let s_19_49: Bits = Bits::new(s_19_48 as u128, 5u16);
        // D s_19_50: cast zx s_19_49 -> i
        let s_19_50: i128 = (s_19_49.value() as i128);
        // D s_19_51: cast reint s_19_50 -> i64
        let s_19_51: i64 = (s_19_50 as i64);
        // D s_19_52: write-var m <= s_19_51
        fn_state.m = s_19_51;
        // N s_19_53: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b15
        return block_15(state, tracer, fn_state);
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
        // D s_22_0: read-var cond:u8
        let s_22_0: u8 = fn_state.cond;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 4u16);
        // C s_22_2: const #14u : u8
        let s_22_2: u8 = 14;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 4u16);
        // D s_22_4: cmp-ne s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) != (s_22_3));
        // D s_22_5: write-var gs#308485 <= s_22_4
        fn_state.gs_308485 = s_22_4;
        // N s_22_6: jump b12
        return block_12(state, tracer, fn_state);
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
        // D s_24_3: write-var gs#308483 <= s_24_2
        fn_state.gs_308483 = s_24_2;
        // N s_24_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#308484 <= s_25_0
        fn_state.gs_308484 = s_25_0;
        // N s_25_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#308482 <= s_27_0
        fn_state.gs_308482 = s_27_0;
        // N s_27_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
