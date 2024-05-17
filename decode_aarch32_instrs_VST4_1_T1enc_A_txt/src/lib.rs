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
use neq_int::*;
use execute_aarch32_instrs_VST4_1_Op_A_txt::*;
use ConditionPassed::*;
use common::*;
pub fn decode_aarch32_instrs_VST4_1_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Rn: u8,
    Vd: u8,
    size: u8,
    index_align: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        d2: i64,
        index: i64,
        n: i64,
        d: i64,
        d4: i64,
        register_index: bool,
        ga_362645: i64,
        alignment: i64,
        wback: bool,
        gs_321810: bool,
        d3: i64,
        gs_321813: bool,
        D: bool,
        Rn: u8,
        Vd: u8,
        size: u8,
        index_align: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        D,
        Rn,
        Vd,
        size,
        index_align,
        Rm,
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
        // C s_2_2: const #3u : u8
        let s_2_2: u8 = 3;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b17 b3
        if s_2_4 {
            return block_17(state, tracer, fn_state);
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
        // C s_3_2: const #0u : u8
        let s_3_2: u8 = 0;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-ne s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) != (s_3_3));
        // N s_3_5: branch s_3_4 b16 b4
        if s_3_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1s : i
        let s_4_0: i128 = 1;
        // D s_4_1: read-var index_align:u8
        let s_4_1: u8 = fn_state.index_align;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #2s : i
        let s_4_5: i128 = 2;
        // C s_4_6: add s_4_5 s_4_4
        let s_4_6: i128 = (s_4_5 + s_4_4);
        // D s_4_7: bit-extract s_4_2 s_4_0 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u8
        let s_4_8: u8 = (s_4_7.value() as u8);
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 3u16);
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (s_4_9.value() as i128);
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // D s_4_12: write-var index <= s_4_11
        fn_state.index = s_4_11;
        // C s_4_13: const #0s : i
        let s_4_13: i128 = 0;
        // D s_4_14: read-var index_align:u8
        let s_4_14: u8 = fn_state.index_align;
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 4u16);
        // C s_4_16: const #1u : u64
        let s_4_16: u64 = 1;
        // D s_4_17: bit-extract s_4_15 s_4_13 s_4_16
        let s_4_17: Bits = (Bits::new(
            ((s_4_15) >> (s_4_13)).value(),
            u16::try_from(s_4_16).unwrap(),
        ));
        // D s_4_18: cast reint s_4_17 -> u8
        let s_4_18: bool = ((s_4_17.value()) != 0);
        // C s_4_19: const #0s : i
        let s_4_19: i128 = 0;
        // C s_4_20: const #0u : u64
        let s_4_20: u64 = 0;
        // D s_4_21: cast zx s_4_18 -> u64
        let s_4_21: u64 = (s_4_18 as u64);
        // C s_4_22: const #1u : u64
        let s_4_22: u64 = 1;
        // D s_4_23: and s_4_21 s_4_22
        let s_4_23: u64 = ((s_4_21) & (s_4_22));
        // D s_4_24: cmp-eq s_4_23 s_4_22
        let s_4_24: bool = ((s_4_23) == (s_4_22));
        // D s_4_25: lsl s_4_21 s_4_19
        let s_4_25: u64 = s_4_21 << s_4_19;
        // D s_4_26: or s_4_20 s_4_25
        let s_4_26: u64 = ((s_4_20) | (s_4_25));
        // D s_4_27: cmpl s_4_25
        let s_4_27: u64 = !s_4_25;
        // D s_4_28: and s_4_20 s_4_27
        let s_4_28: u64 = ((s_4_20) & (s_4_27));
        // D s_4_29: select s_4_24 s_4_26 s_4_28
        let s_4_29: u64 = if s_4_24 { s_4_26 } else { s_4_28 };
        // D s_4_30: cast trunc s_4_29 -> u8
        let s_4_30: bool = ((s_4_29) != 0);
        // D s_4_31: cast zx s_4_30 -> bv
        let s_4_31: Bits = Bits::new(s_4_30 as u128, 1u16);
        // C s_4_32: const #0u : u8
        let s_4_32: bool = false;
        // C s_4_33: cast zx s_4_32 -> bv
        let s_4_33: Bits = Bits::new(s_4_32 as u128, 1u16);
        // D s_4_34: cmp-eq s_4_31 s_4_33
        let s_4_34: bool = ((s_4_31) == (s_4_33));
        // N s_4_35: branch s_4_34 b15 b5
        if s_4_34 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #4s : i64
        let s_5_0: i64 = 4;
        // D s_5_1: write-var ga#362645 <= s_5_0
        fn_state.ga_362645 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#362645:i64
        let s_6_0: i64 = fn_state.ga_362645;
        // D s_6_1: write-var alignment <= s_6_0
        fn_state.alignment = s_6_0;
        // D s_6_2: read-var D:u8
        let s_6_2: bool = fn_state.D;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: read-var Vd:u8
        let s_6_4: u8 = fn_state.Vd;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 4u16);
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
        // D s_6_19: read-var d:i64
        let s_6_19: i64 = fn_state.d;
        // D s_6_20: cast zx s_6_19 -> i
        let s_6_20: i128 = (i128::try_from(s_6_19).unwrap());
        // C s_6_21: const #1s : i64
        let s_6_21: i64 = 1;
        // C s_6_22: cast zx s_6_21 -> i
        let s_6_22: i128 = (i128::try_from(s_6_21).unwrap());
        // D s_6_23: add s_6_20 s_6_22
        let s_6_23: i128 = (s_6_20 + s_6_22);
        // D s_6_24: cast reint s_6_23 -> i64
        let s_6_24: i64 = (s_6_23 as i64);
        // D s_6_25: write-var d2 <= s_6_24
        fn_state.d2 = s_6_24;
        // D s_6_26: read-var d2:i64
        let s_6_26: i64 = fn_state.d2;
        // D s_6_27: cast zx s_6_26 -> i
        let s_6_27: i128 = (i128::try_from(s_6_26).unwrap());
        // C s_6_28: const #1s : i64
        let s_6_28: i64 = 1;
        // C s_6_29: cast zx s_6_28 -> i
        let s_6_29: i128 = (i128::try_from(s_6_28).unwrap());
        // D s_6_30: add s_6_27 s_6_29
        let s_6_30: i128 = (s_6_27 + s_6_29);
        // D s_6_31: cast reint s_6_30 -> i64
        let s_6_31: i64 = (s_6_30 as i64);
        // D s_6_32: write-var d3 <= s_6_31
        fn_state.d3 = s_6_31;
        // D s_6_33: read-var d3:i64
        let s_6_33: i64 = fn_state.d3;
        // D s_6_34: cast zx s_6_33 -> i
        let s_6_34: i128 = (i128::try_from(s_6_33).unwrap());
        // C s_6_35: const #1s : i64
        let s_6_35: i64 = 1;
        // C s_6_36: cast zx s_6_35 -> i
        let s_6_36: i128 = (i128::try_from(s_6_35).unwrap());
        // D s_6_37: add s_6_34 s_6_36
        let s_6_37: i128 = (s_6_34 + s_6_36);
        // D s_6_38: cast reint s_6_37 -> i64
        let s_6_38: i64 = (s_6_37 as i64);
        // D s_6_39: write-var d4 <= s_6_38
        fn_state.d4 = s_6_38;
        // D s_6_40: read-var Rn:u8
        let s_6_40: u8 = fn_state.Rn;
        // D s_6_41: cast zx s_6_40 -> bv
        let s_6_41: Bits = Bits::new(s_6_40 as u128, 4u16);
        // D s_6_42: cast zx s_6_41 -> i
        let s_6_42: i128 = (s_6_41.value() as i128);
        // D s_6_43: cast reint s_6_42 -> i64
        let s_6_43: i64 = (s_6_42 as i64);
        // D s_6_44: write-var n <= s_6_43
        fn_state.n = s_6_43;
        // D s_6_45: read-var Rm:u8
        let s_6_45: u8 = fn_state.Rm;
        // D s_6_46: cast zx s_6_45 -> bv
        let s_6_46: Bits = Bits::new(s_6_45 as u128, 4u16);
        // D s_6_47: cast zx s_6_46 -> i
        let s_6_47: i128 = (s_6_46.value() as i128);
        // D s_6_48: cast reint s_6_47 -> i64
        let s_6_48: i64 = (s_6_47 as i64);
        // D s_6_49: write-var m <= s_6_48
        fn_state.m = s_6_48;
        // C s_6_50: const #15s : i
        let s_6_50: i128 = 15;
        // D s_6_51: read-var m:i64
        let s_6_51: i64 = fn_state.m;
        // D s_6_52: cast zx s_6_51 -> i
        let s_6_52: i128 = (i128::try_from(s_6_51).unwrap());
        // D s_6_53: call neq_int(s_6_52, s_6_50)
        let s_6_53: bool = neq_int(state, tracer, s_6_52, s_6_50);
        // D s_6_54: write-var wback <= s_6_53
        fn_state.wback = s_6_53;
        // C s_6_55: const #15s : i
        let s_6_55: i128 = 15;
        // D s_6_56: read-var m:i64
        let s_6_56: i64 = fn_state.m;
        // D s_6_57: cast zx s_6_56 -> i
        let s_6_57: i128 = (i128::try_from(s_6_56).unwrap());
        // D s_6_58: call neq_int(s_6_57, s_6_55)
        let s_6_58: bool = neq_int(state, tracer, s_6_57, s_6_55);
        // N s_6_59: branch s_6_58 b14 b7
        if s_6_58 {
            return block_14(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#321810 <= s_7_0
        fn_state.gs_321810 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#321810:u8
        let s_8_0: bool = fn_state.gs_321810;
        // D s_8_1: write-var register_index <= s_8_0
        fn_state.register_index = s_8_0;
        // C s_8_2: const #15s : i
        let s_8_2: i128 = 15;
        // D s_8_3: read-var n:i64
        let s_8_3: i64 = fn_state.n;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cmp-eq s_8_4 s_8_2
        let s_8_5: bool = ((s_8_4) == (s_8_2));
        // N s_8_6: branch s_8_5 b13 b9
        if s_8_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #31s : i
        let s_9_0: i128 = 31;
        // D s_9_1: read-var d4:i64
        let s_9_1: i64 = fn_state.d4;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cmp-gt s_9_2 s_9_0
        let s_9_3: bool = ((s_9_2) > (s_9_0));
        // D s_9_4: write-var gs#321813 <= s_9_3
        fn_state.gs_321813 = s_9_3;
        // N s_9_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#321813:u8
        let s_10_0: bool = fn_state.gs_321813;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
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
        // D s_11_0: read-var d2:i64
        let s_11_0: i64 = fn_state.d2;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: read-var d3:i64
        let s_11_2: i64 = fn_state.d3;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: read-var d4:i64
        let s_11_4: i64 = fn_state.d4;
        // D s_11_5: cast zx s_11_4 -> i
        let s_11_5: i128 = (i128::try_from(s_11_4).unwrap());
        // D s_11_6: read-var alignment:i64
        let s_11_6: i64 = fn_state.alignment;
        // D s_11_7: read-var d:i64
        let s_11_7: i64 = fn_state.d;
        // C s_11_8: const #1s : i64
        let s_11_8: i64 = 1;
        // D s_11_9: read-var index:i64
        let s_11_9: i64 = fn_state.index;
        // D s_11_10: read-var m:i64
        let s_11_10: i64 = fn_state.m;
        // D s_11_11: read-var n:i64
        let s_11_11: i64 = fn_state.n;
        // D s_11_12: read-var register_index:u8
        let s_11_12: bool = fn_state.register_index;
        // D s_11_13: read-var wback:u8
        let s_11_13: bool = fn_state.wback;
        // D s_11_14: call execute_aarch32_instrs_VST4_1_Op_A_txt(s_11_6, s_11_7, s_11_1, s_11_3, s_11_5, s_11_8, s_11_9, s_11_10, s_11_11, s_11_12, s_11_13)
        let s_11_14: () = execute_aarch32_instrs_VST4_1_Op_A_txt(
            state,
            tracer,
            s_11_6,
            s_11_7,
            s_11_1,
            s_11_3,
            s_11_5,
            s_11_8,
            s_11_9,
            s_11_10,
            s_11_11,
            s_11_12,
            s_11_13,
        );
        // N s_11_15: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#321813 <= s_13_0
        fn_state.gs_321813 = s_13_0;
        // N s_13_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #13s : i
        let s_14_0: i128 = 13;
        // D s_14_1: read-var m:i64
        let s_14_1: i64 = fn_state.m;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: call neq_int(s_14_2, s_14_0)
        let s_14_3: bool = neq_int(state, tracer, s_14_2, s_14_0);
        // D s_14_4: write-var gs#321810 <= s_14_3
        fn_state.gs_321810 = s_14_3;
        // N s_14_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1s : i64
        let s_15_0: i64 = 1;
        // D s_15_1: write-var ga#362645 <= s_15_0
        fn_state.ga_362645 = s_15_0;
        // N s_15_2: jump b6
        return block_6(state, tracer, fn_state);
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
