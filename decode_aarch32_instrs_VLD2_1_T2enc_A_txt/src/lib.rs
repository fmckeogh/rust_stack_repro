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
use ConditionPassed::*;
use execute_aarch32_instrs_VLD2_1_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VLD2_1_T2enc_A_txt<T: Tracer>(
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
        gs_310249: bool,
        m: i64,
        d2: i64,
        ga_353977: i64,
        ga_353973: i64,
        gs_310246: bool,
        inc_name: i64,
        n: i64,
        index: i64,
        d: i64,
        register_index: bool,
        alignment: i64,
        wback: bool,
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
        // N s_2_5: branch s_2_4 b18 b3
        if s_2_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var index_align:u8
        let s_3_1: u8 = fn_state.index_align;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 4u16);
        // C s_3_3: const #1s : i64
        let s_3_3: i64 = 1;
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #1s : i
        let s_3_5: i128 = 1;
        // C s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: bit-extract s_3_2 s_3_0 s_3_6
        let s_3_7: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: u8 = (s_3_7.value() as u8);
        // D s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 2u16);
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (s_3_9.value() as i128);
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: write-var index <= s_3_11
        fn_state.index = s_3_11;
        // C s_3_13: const #1s : i
        let s_3_13: i128 = 1;
        // D s_3_14: read-var index_align:u8
        let s_3_14: u8 = fn_state.index_align;
        // D s_3_15: cast zx s_3_14 -> bv
        let s_3_15: Bits = Bits::new(s_3_14 as u128, 4u16);
        // C s_3_16: const #1u : u64
        let s_3_16: u64 = 1;
        // D s_3_17: bit-extract s_3_15 s_3_13 s_3_16
        let s_3_17: Bits = (Bits::new(
            ((s_3_15) >> (s_3_13)).value(),
            u16::try_from(s_3_16).unwrap(),
        ));
        // D s_3_18: cast reint s_3_17 -> u8
        let s_3_18: bool = ((s_3_17.value()) != 0);
        // C s_3_19: const #0s : i
        let s_3_19: i128 = 0;
        // C s_3_20: const #0u : u64
        let s_3_20: u64 = 0;
        // D s_3_21: cast zx s_3_18 -> u64
        let s_3_21: u64 = (s_3_18 as u64);
        // C s_3_22: const #1u : u64
        let s_3_22: u64 = 1;
        // D s_3_23: and s_3_21 s_3_22
        let s_3_23: u64 = ((s_3_21) & (s_3_22));
        // D s_3_24: cmp-eq s_3_23 s_3_22
        let s_3_24: bool = ((s_3_23) == (s_3_22));
        // D s_3_25: lsl s_3_21 s_3_19
        let s_3_25: u64 = s_3_21 << s_3_19;
        // D s_3_26: or s_3_20 s_3_25
        let s_3_26: u64 = ((s_3_20) | (s_3_25));
        // D s_3_27: cmpl s_3_25
        let s_3_27: u64 = !s_3_25;
        // D s_3_28: and s_3_20 s_3_27
        let s_3_28: u64 = ((s_3_20) & (s_3_27));
        // D s_3_29: select s_3_24 s_3_26 s_3_28
        let s_3_29: u64 = if s_3_24 { s_3_26 } else { s_3_28 };
        // D s_3_30: cast trunc s_3_29 -> u8
        let s_3_30: bool = ((s_3_29) != 0);
        // D s_3_31: cast zx s_3_30 -> bv
        let s_3_31: Bits = Bits::new(s_3_30 as u128, 1u16);
        // C s_3_32: const #0u : u8
        let s_3_32: bool = false;
        // C s_3_33: cast zx s_3_32 -> bv
        let s_3_33: Bits = Bits::new(s_3_32 as u128, 1u16);
        // D s_3_34: cmp-eq s_3_31 s_3_33
        let s_3_34: bool = ((s_3_31) == (s_3_33));
        // N s_3_35: branch s_3_34 b17 b4
        if s_3_34 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2s : i64
        let s_4_0: i64 = 2;
        // D s_4_1: write-var ga#353973 <= s_4_0
        fn_state.ga_353973 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#353973:i64
        let s_5_0: i64 = fn_state.ga_353973;
        // D s_5_1: write-var inc_name <= s_5_0
        fn_state.inc_name = s_5_0;
        // C s_5_2: const #0s : i
        let s_5_2: i128 = 0;
        // D s_5_3: read-var index_align:u8
        let s_5_3: u8 = fn_state.index_align;
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 4u16);
        // C s_5_5: const #1u : u64
        let s_5_5: u64 = 1;
        // D s_5_6: bit-extract s_5_4 s_5_2 s_5_5
        let s_5_6: Bits = (Bits::new(
            ((s_5_4) >> (s_5_2)).value(),
            u16::try_from(s_5_5).unwrap(),
        ));
        // D s_5_7: cast reint s_5_6 -> u8
        let s_5_7: bool = ((s_5_6.value()) != 0);
        // C s_5_8: const #0s : i
        let s_5_8: i128 = 0;
        // C s_5_9: const #0u : u64
        let s_5_9: u64 = 0;
        // D s_5_10: cast zx s_5_7 -> u64
        let s_5_10: u64 = (s_5_7 as u64);
        // C s_5_11: const #1u : u64
        let s_5_11: u64 = 1;
        // D s_5_12: and s_5_10 s_5_11
        let s_5_12: u64 = ((s_5_10) & (s_5_11));
        // D s_5_13: cmp-eq s_5_12 s_5_11
        let s_5_13: bool = ((s_5_12) == (s_5_11));
        // D s_5_14: lsl s_5_10 s_5_8
        let s_5_14: u64 = s_5_10 << s_5_8;
        // D s_5_15: or s_5_9 s_5_14
        let s_5_15: u64 = ((s_5_9) | (s_5_14));
        // D s_5_16: cmpl s_5_14
        let s_5_16: u64 = !s_5_14;
        // D s_5_17: and s_5_9 s_5_16
        let s_5_17: u64 = ((s_5_9) & (s_5_16));
        // D s_5_18: select s_5_13 s_5_15 s_5_17
        let s_5_18: u64 = if s_5_13 { s_5_15 } else { s_5_17 };
        // D s_5_19: cast trunc s_5_18 -> u8
        let s_5_19: bool = ((s_5_18) != 0);
        // D s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 1u16);
        // C s_5_21: const #0u : u8
        let s_5_21: bool = false;
        // C s_5_22: cast zx s_5_21 -> bv
        let s_5_22: Bits = Bits::new(s_5_21 as u128, 1u16);
        // D s_5_23: cmp-eq s_5_20 s_5_22
        let s_5_23: bool = ((s_5_20) == (s_5_22));
        // N s_5_24: branch s_5_23 b16 b6
        if s_5_23 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #4s : i64
        let s_6_0: i64 = 4;
        // D s_6_1: write-var ga#353977 <= s_6_0
        fn_state.ga_353977 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#353977:i64
        let s_7_0: i64 = fn_state.ga_353977;
        // D s_7_1: write-var alignment <= s_7_0
        fn_state.alignment = s_7_0;
        // D s_7_2: read-var D:u8
        let s_7_2: bool = fn_state.D;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: read-var Vd:u8
        let s_7_4: u8 = fn_state.Vd;
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 4u16);
        // D s_7_6: cast reint s_7_3 -> u128
        let s_7_6: u128 = (s_7_3.value() as u128);
        // D s_7_7: size-of s_7_3
        let s_7_7: u16 = s_7_3.length();
        // D s_7_8: cast reint s_7_5 -> u128
        let s_7_8: u128 = (s_7_5.value() as u128);
        // D s_7_9: size-of s_7_5
        let s_7_9: u16 = s_7_5.length();
        // D s_7_10: lsl s_7_6 s_7_9
        let s_7_10: u128 = s_7_6 << s_7_9;
        // D s_7_11: or s_7_10 s_7_8
        let s_7_11: u128 = ((s_7_10) | (s_7_8));
        // D s_7_12: add s_7_7 s_7_9
        let s_7_12: u16 = (s_7_7 + s_7_9);
        // D s_7_13: create-bits s_7_11 s_7_12
        let s_7_13: Bits = Bits::new(s_7_11, s_7_12);
        // D s_7_14: cast reint s_7_13 -> u8
        let s_7_14: u8 = (s_7_13.value() as u8);
        // D s_7_15: cast zx s_7_14 -> bv
        let s_7_15: Bits = Bits::new(s_7_14 as u128, 5u16);
        // D s_7_16: cast zx s_7_15 -> i
        let s_7_16: i128 = (s_7_15.value() as i128);
        // D s_7_17: cast reint s_7_16 -> i64
        let s_7_17: i64 = (s_7_16 as i64);
        // D s_7_18: write-var d <= s_7_17
        fn_state.d = s_7_17;
        // D s_7_19: read-var d:i64
        let s_7_19: i64 = fn_state.d;
        // D s_7_20: cast zx s_7_19 -> i
        let s_7_20: i128 = (i128::try_from(s_7_19).unwrap());
        // D s_7_21: read-var inc_name:i64
        let s_7_21: i64 = fn_state.inc_name;
        // D s_7_22: cast zx s_7_21 -> i
        let s_7_22: i128 = (i128::try_from(s_7_21).unwrap());
        // D s_7_23: add s_7_20 s_7_22
        let s_7_23: i128 = (s_7_20 + s_7_22);
        // D s_7_24: cast reint s_7_23 -> i64
        let s_7_24: i64 = (s_7_23 as i64);
        // D s_7_25: write-var d2 <= s_7_24
        fn_state.d2 = s_7_24;
        // D s_7_26: read-var Rn:u8
        let s_7_26: u8 = fn_state.Rn;
        // D s_7_27: cast zx s_7_26 -> bv
        let s_7_27: Bits = Bits::new(s_7_26 as u128, 4u16);
        // D s_7_28: cast zx s_7_27 -> i
        let s_7_28: i128 = (s_7_27.value() as i128);
        // D s_7_29: cast reint s_7_28 -> i64
        let s_7_29: i64 = (s_7_28 as i64);
        // D s_7_30: write-var n <= s_7_29
        fn_state.n = s_7_29;
        // D s_7_31: read-var Rm:u8
        let s_7_31: u8 = fn_state.Rm;
        // D s_7_32: cast zx s_7_31 -> bv
        let s_7_32: Bits = Bits::new(s_7_31 as u128, 4u16);
        // D s_7_33: cast zx s_7_32 -> i
        let s_7_33: i128 = (s_7_32.value() as i128);
        // D s_7_34: cast reint s_7_33 -> i64
        let s_7_34: i64 = (s_7_33 as i64);
        // D s_7_35: write-var m <= s_7_34
        fn_state.m = s_7_34;
        // C s_7_36: const #15s : i
        let s_7_36: i128 = 15;
        // D s_7_37: read-var m:i64
        let s_7_37: i64 = fn_state.m;
        // D s_7_38: cast zx s_7_37 -> i
        let s_7_38: i128 = (i128::try_from(s_7_37).unwrap());
        // D s_7_39: call neq_int(s_7_38, s_7_36)
        let s_7_39: bool = neq_int(state, tracer, s_7_38, s_7_36);
        // D s_7_40: write-var wback <= s_7_39
        fn_state.wback = s_7_39;
        // C s_7_41: const #15s : i
        let s_7_41: i128 = 15;
        // D s_7_42: read-var m:i64
        let s_7_42: i64 = fn_state.m;
        // D s_7_43: cast zx s_7_42 -> i
        let s_7_43: i128 = (i128::try_from(s_7_42).unwrap());
        // D s_7_44: call neq_int(s_7_43, s_7_41)
        let s_7_44: bool = neq_int(state, tracer, s_7_43, s_7_41);
        // N s_7_45: branch s_7_44 b15 b8
        if s_7_44 {
            return block_15(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#310246 <= s_8_0
        fn_state.gs_310246 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#310246:u8
        let s_9_0: bool = fn_state.gs_310246;
        // D s_9_1: write-var register_index <= s_9_0
        fn_state.register_index = s_9_0;
        // C s_9_2: const #15s : i
        let s_9_2: i128 = 15;
        // D s_9_3: read-var n:i64
        let s_9_3: i64 = fn_state.n;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: cmp-eq s_9_4 s_9_2
        let s_9_5: bool = ((s_9_4) == (s_9_2));
        // N s_9_6: branch s_9_5 b14 b10
        if s_9_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #31s : i
        let s_10_0: i128 = 31;
        // D s_10_1: read-var d2:i64
        let s_10_1: i64 = fn_state.d2;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cmp-gt s_10_2 s_10_0
        let s_10_3: bool = ((s_10_2) > (s_10_0));
        // D s_10_4: write-var gs#310249 <= s_10_3
        fn_state.gs_310249 = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#310249:u8
        let s_11_0: bool = fn_state.gs_310249;
        // N s_11_1: branch s_11_0 b13 b12
        if s_11_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var d2:i64
        let s_12_0: i64 = fn_state.d2;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var alignment:i64
        let s_12_2: i64 = fn_state.alignment;
        // D s_12_3: read-var d:i64
        let s_12_3: i64 = fn_state.d;
        // C s_12_4: const #2s : i64
        let s_12_4: i64 = 2;
        // D s_12_5: read-var index:i64
        let s_12_5: i64 = fn_state.index;
        // D s_12_6: read-var m:i64
        let s_12_6: i64 = fn_state.m;
        // D s_12_7: read-var n:i64
        let s_12_7: i64 = fn_state.n;
        // D s_12_8: read-var register_index:u8
        let s_12_8: bool = fn_state.register_index;
        // D s_12_9: read-var wback:u8
        let s_12_9: bool = fn_state.wback;
        // D s_12_10: call execute_aarch32_instrs_VLD2_1_Op_A_txt(s_12_2, s_12_3, s_12_1, s_12_4, s_12_5, s_12_6, s_12_7, s_12_8, s_12_9)
        let s_12_10: () = execute_aarch32_instrs_VLD2_1_Op_A_txt(
            state,
            tracer,
            s_12_2,
            s_12_3,
            s_12_1,
            s_12_4,
            s_12_5,
            s_12_6,
            s_12_7,
            s_12_8,
            s_12_9,
        );
        // N s_12_11: return
        return;
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
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#310249 <= s_14_0
        fn_state.gs_310249 = s_14_0;
        // N s_14_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #13s : i
        let s_15_0: i128 = 13;
        // D s_15_1: read-var m:i64
        let s_15_1: i64 = fn_state.m;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: call neq_int(s_15_2, s_15_0)
        let s_15_3: bool = neq_int(state, tracer, s_15_2, s_15_0);
        // D s_15_4: write-var gs#310246 <= s_15_3
        fn_state.gs_310246 = s_15_3;
        // N s_15_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1s : i64
        let s_16_0: i64 = 1;
        // D s_16_1: write-var ga#353977 <= s_16_0
        fn_state.ga_353977 = s_16_0;
        // N s_16_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1s : i64
        let s_17_0: i64 = 1;
        // D s_17_1: write-var ga#353973 <= s_17_0
        fn_state.ga_353973 = s_17_0;
        // N s_17_2: jump b5
        return block_5(state, tracer, fn_state);
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
}
