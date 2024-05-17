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
use execute_aarch32_instrs_VST2_1_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VST2_1_A3enc_A_txt<T: Tracer>(
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
        gs_320849: bool,
        inc_name: i64,
        n: i64,
        index: i64,
        d: i64,
        gs_320846: bool,
        register_index: bool,
        alignment: i64,
        ga_361941: i64,
        wback: bool,
        ga_361937: i64,
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
        // N s_2_5: branch s_2_4 b20 b3
        if s_2_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1s : i
        let s_3_0: i128 = 1;
        // D s_3_1: read-var index_align:u8
        let s_3_1: u8 = fn_state.index_align;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 4u16);
        // C s_3_3: const #1u : u64
        let s_3_3: u64 = 1;
        // D s_3_4: bit-extract s_3_2 s_3_0 s_3_3
        let s_3_4: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_3).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: bool = ((s_3_4.value()) != 0);
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // C s_3_7: const #0u : u64
        let s_3_7: u64 = 0;
        // D s_3_8: cast zx s_3_5 -> u64
        let s_3_8: u64 = (s_3_5 as u64);
        // C s_3_9: const #1u : u64
        let s_3_9: u64 = 1;
        // D s_3_10: and s_3_8 s_3_9
        let s_3_10: u64 = ((s_3_8) & (s_3_9));
        // D s_3_11: cmp-eq s_3_10 s_3_9
        let s_3_11: bool = ((s_3_10) == (s_3_9));
        // D s_3_12: lsl s_3_8 s_3_6
        let s_3_12: u64 = s_3_8 << s_3_6;
        // D s_3_13: or s_3_7 s_3_12
        let s_3_13: u64 = ((s_3_7) | (s_3_12));
        // D s_3_14: cmpl s_3_12
        let s_3_14: u64 = !s_3_12;
        // D s_3_15: and s_3_7 s_3_14
        let s_3_15: u64 = ((s_3_7) & (s_3_14));
        // D s_3_16: select s_3_11 s_3_13 s_3_15
        let s_3_16: u64 = if s_3_11 { s_3_13 } else { s_3_15 };
        // D s_3_17: cast trunc s_3_16 -> u8
        let s_3_17: bool = ((s_3_16) != 0);
        // D s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 1u16);
        // C s_3_19: const #0u : u8
        let s_3_19: bool = false;
        // C s_3_20: cast zx s_3_19 -> bv
        let s_3_20: Bits = Bits::new(s_3_19 as u128, 1u16);
        // D s_3_21: cmp-ne s_3_18 s_3_20
        let s_3_21: bool = ((s_3_18) != (s_3_20));
        // N s_3_22: branch s_3_21 b19 b4
        if s_3_21 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #3s : i
        let s_4_0: i128 = 3;
        // D s_4_1: read-var index_align:u8
        let s_4_1: u8 = fn_state.index_align;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1u : u64
        let s_4_3: u64 = 1;
        // D s_4_4: bit-extract s_4_2 s_4_0 s_4_3
        let s_4_4: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_3).unwrap(),
        ));
        // D s_4_5: cast reint s_4_4 -> u8
        let s_4_5: bool = ((s_4_4.value()) != 0);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // C s_4_7: const #0u : u64
        let s_4_7: u64 = 0;
        // D s_4_8: cast zx s_4_5 -> u64
        let s_4_8: u64 = (s_4_5 as u64);
        // C s_4_9: const #1u : u64
        let s_4_9: u64 = 1;
        // D s_4_10: and s_4_8 s_4_9
        let s_4_10: u64 = ((s_4_8) & (s_4_9));
        // D s_4_11: cmp-eq s_4_10 s_4_9
        let s_4_11: bool = ((s_4_10) == (s_4_9));
        // D s_4_12: lsl s_4_8 s_4_6
        let s_4_12: u64 = s_4_8 << s_4_6;
        // D s_4_13: or s_4_7 s_4_12
        let s_4_13: u64 = ((s_4_7) | (s_4_12));
        // D s_4_14: cmpl s_4_12
        let s_4_14: u64 = !s_4_12;
        // D s_4_15: and s_4_7 s_4_14
        let s_4_15: u64 = ((s_4_7) & (s_4_14));
        // D s_4_16: select s_4_11 s_4_13 s_4_15
        let s_4_16: u64 = if s_4_11 { s_4_13 } else { s_4_15 };
        // D s_4_17: cast trunc s_4_16 -> u8
        let s_4_17: bool = ((s_4_16) != 0);
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 1u16);
        // D s_4_19: cast zx s_4_18 -> i
        let s_4_19: i128 = (s_4_18.value() as i128);
        // D s_4_20: cast reint s_4_19 -> i64
        let s_4_20: i64 = (s_4_19 as i64);
        // D s_4_21: write-var index <= s_4_20
        fn_state.index = s_4_20;
        // C s_4_22: const #2s : i
        let s_4_22: i128 = 2;
        // D s_4_23: read-var index_align:u8
        let s_4_23: u8 = fn_state.index_align;
        // D s_4_24: cast zx s_4_23 -> bv
        let s_4_24: Bits = Bits::new(s_4_23 as u128, 4u16);
        // C s_4_25: const #1u : u64
        let s_4_25: u64 = 1;
        // D s_4_26: bit-extract s_4_24 s_4_22 s_4_25
        let s_4_26: Bits = (Bits::new(
            ((s_4_24) >> (s_4_22)).value(),
            u16::try_from(s_4_25).unwrap(),
        ));
        // D s_4_27: cast reint s_4_26 -> u8
        let s_4_27: bool = ((s_4_26.value()) != 0);
        // C s_4_28: const #0s : i
        let s_4_28: i128 = 0;
        // C s_4_29: const #0u : u64
        let s_4_29: u64 = 0;
        // D s_4_30: cast zx s_4_27 -> u64
        let s_4_30: u64 = (s_4_27 as u64);
        // C s_4_31: const #1u : u64
        let s_4_31: u64 = 1;
        // D s_4_32: and s_4_30 s_4_31
        let s_4_32: u64 = ((s_4_30) & (s_4_31));
        // D s_4_33: cmp-eq s_4_32 s_4_31
        let s_4_33: bool = ((s_4_32) == (s_4_31));
        // D s_4_34: lsl s_4_30 s_4_28
        let s_4_34: u64 = s_4_30 << s_4_28;
        // D s_4_35: or s_4_29 s_4_34
        let s_4_35: u64 = ((s_4_29) | (s_4_34));
        // D s_4_36: cmpl s_4_34
        let s_4_36: u64 = !s_4_34;
        // D s_4_37: and s_4_29 s_4_36
        let s_4_37: u64 = ((s_4_29) & (s_4_36));
        // D s_4_38: select s_4_33 s_4_35 s_4_37
        let s_4_38: u64 = if s_4_33 { s_4_35 } else { s_4_37 };
        // D s_4_39: cast trunc s_4_38 -> u8
        let s_4_39: bool = ((s_4_38) != 0);
        // D s_4_40: cast zx s_4_39 -> bv
        let s_4_40: Bits = Bits::new(s_4_39 as u128, 1u16);
        // C s_4_41: const #0u : u8
        let s_4_41: bool = false;
        // C s_4_42: cast zx s_4_41 -> bv
        let s_4_42: Bits = Bits::new(s_4_41 as u128, 1u16);
        // D s_4_43: cmp-eq s_4_40 s_4_42
        let s_4_43: bool = ((s_4_40) == (s_4_42));
        // N s_4_44: branch s_4_43 b18 b5
        if s_4_43 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #2s : i64
        let s_5_0: i64 = 2;
        // D s_5_1: write-var ga#361937 <= s_5_0
        fn_state.ga_361937 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#361937:i64
        let s_6_0: i64 = fn_state.ga_361937;
        // D s_6_1: write-var inc_name <= s_6_0
        fn_state.inc_name = s_6_0;
        // C s_6_2: const #0s : i
        let s_6_2: i128 = 0;
        // D s_6_3: read-var index_align:u8
        let s_6_3: u8 = fn_state.index_align;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 4u16);
        // C s_6_5: const #1u : u64
        let s_6_5: u64 = 1;
        // D s_6_6: bit-extract s_6_4 s_6_2 s_6_5
        let s_6_6: Bits = (Bits::new(
            ((s_6_4) >> (s_6_2)).value(),
            u16::try_from(s_6_5).unwrap(),
        ));
        // D s_6_7: cast reint s_6_6 -> u8
        let s_6_7: bool = ((s_6_6.value()) != 0);
        // C s_6_8: const #0s : i
        let s_6_8: i128 = 0;
        // C s_6_9: const #0u : u64
        let s_6_9: u64 = 0;
        // D s_6_10: cast zx s_6_7 -> u64
        let s_6_10: u64 = (s_6_7 as u64);
        // C s_6_11: const #1u : u64
        let s_6_11: u64 = 1;
        // D s_6_12: and s_6_10 s_6_11
        let s_6_12: u64 = ((s_6_10) & (s_6_11));
        // D s_6_13: cmp-eq s_6_12 s_6_11
        let s_6_13: bool = ((s_6_12) == (s_6_11));
        // D s_6_14: lsl s_6_10 s_6_8
        let s_6_14: u64 = s_6_10 << s_6_8;
        // D s_6_15: or s_6_9 s_6_14
        let s_6_15: u64 = ((s_6_9) | (s_6_14));
        // D s_6_16: cmpl s_6_14
        let s_6_16: u64 = !s_6_14;
        // D s_6_17: and s_6_9 s_6_16
        let s_6_17: u64 = ((s_6_9) & (s_6_16));
        // D s_6_18: select s_6_13 s_6_15 s_6_17
        let s_6_18: u64 = if s_6_13 { s_6_15 } else { s_6_17 };
        // D s_6_19: cast trunc s_6_18 -> u8
        let s_6_19: bool = ((s_6_18) != 0);
        // D s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 1u16);
        // C s_6_21: const #0u : u8
        let s_6_21: bool = false;
        // C s_6_22: cast zx s_6_21 -> bv
        let s_6_22: Bits = Bits::new(s_6_21 as u128, 1u16);
        // D s_6_23: cmp-eq s_6_20 s_6_22
        let s_6_23: bool = ((s_6_20) == (s_6_22));
        // N s_6_24: branch s_6_23 b17 b7
        if s_6_23 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #8s : i64
        let s_7_0: i64 = 8;
        // D s_7_1: write-var ga#361941 <= s_7_0
        fn_state.ga_361941 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#361941:i64
        let s_8_0: i64 = fn_state.ga_361941;
        // D s_8_1: write-var alignment <= s_8_0
        fn_state.alignment = s_8_0;
        // D s_8_2: read-var D:u8
        let s_8_2: bool = fn_state.D;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: read-var Vd:u8
        let s_8_4: u8 = fn_state.Vd;
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 4u16);
        // D s_8_6: cast reint s_8_3 -> u128
        let s_8_6: u128 = (s_8_3.value() as u128);
        // D s_8_7: size-of s_8_3
        let s_8_7: u16 = s_8_3.length();
        // D s_8_8: cast reint s_8_5 -> u128
        let s_8_8: u128 = (s_8_5.value() as u128);
        // D s_8_9: size-of s_8_5
        let s_8_9: u16 = s_8_5.length();
        // D s_8_10: lsl s_8_6 s_8_9
        let s_8_10: u128 = s_8_6 << s_8_9;
        // D s_8_11: or s_8_10 s_8_8
        let s_8_11: u128 = ((s_8_10) | (s_8_8));
        // D s_8_12: add s_8_7 s_8_9
        let s_8_12: u16 = (s_8_7 + s_8_9);
        // D s_8_13: create-bits s_8_11 s_8_12
        let s_8_13: Bits = Bits::new(s_8_11, s_8_12);
        // D s_8_14: cast reint s_8_13 -> u8
        let s_8_14: u8 = (s_8_13.value() as u8);
        // D s_8_15: cast zx s_8_14 -> bv
        let s_8_15: Bits = Bits::new(s_8_14 as u128, 5u16);
        // D s_8_16: cast zx s_8_15 -> i
        let s_8_16: i128 = (s_8_15.value() as i128);
        // D s_8_17: cast reint s_8_16 -> i64
        let s_8_17: i64 = (s_8_16 as i64);
        // D s_8_18: write-var d <= s_8_17
        fn_state.d = s_8_17;
        // D s_8_19: read-var d:i64
        let s_8_19: i64 = fn_state.d;
        // D s_8_20: cast zx s_8_19 -> i
        let s_8_20: i128 = (i128::try_from(s_8_19).unwrap());
        // D s_8_21: read-var inc_name:i64
        let s_8_21: i64 = fn_state.inc_name;
        // D s_8_22: cast zx s_8_21 -> i
        let s_8_22: i128 = (i128::try_from(s_8_21).unwrap());
        // D s_8_23: add s_8_20 s_8_22
        let s_8_23: i128 = (s_8_20 + s_8_22);
        // D s_8_24: cast reint s_8_23 -> i64
        let s_8_24: i64 = (s_8_23 as i64);
        // D s_8_25: write-var d2 <= s_8_24
        fn_state.d2 = s_8_24;
        // D s_8_26: read-var Rn:u8
        let s_8_26: u8 = fn_state.Rn;
        // D s_8_27: cast zx s_8_26 -> bv
        let s_8_27: Bits = Bits::new(s_8_26 as u128, 4u16);
        // D s_8_28: cast zx s_8_27 -> i
        let s_8_28: i128 = (s_8_27.value() as i128);
        // D s_8_29: cast reint s_8_28 -> i64
        let s_8_29: i64 = (s_8_28 as i64);
        // D s_8_30: write-var n <= s_8_29
        fn_state.n = s_8_29;
        // D s_8_31: read-var Rm:u8
        let s_8_31: u8 = fn_state.Rm;
        // D s_8_32: cast zx s_8_31 -> bv
        let s_8_32: Bits = Bits::new(s_8_31 as u128, 4u16);
        // D s_8_33: cast zx s_8_32 -> i
        let s_8_33: i128 = (s_8_32.value() as i128);
        // D s_8_34: cast reint s_8_33 -> i64
        let s_8_34: i64 = (s_8_33 as i64);
        // D s_8_35: write-var m <= s_8_34
        fn_state.m = s_8_34;
        // C s_8_36: const #15s : i
        let s_8_36: i128 = 15;
        // D s_8_37: read-var m:i64
        let s_8_37: i64 = fn_state.m;
        // D s_8_38: cast zx s_8_37 -> i
        let s_8_38: i128 = (i128::try_from(s_8_37).unwrap());
        // D s_8_39: call neq_int(s_8_38, s_8_36)
        let s_8_39: bool = neq_int(state, tracer, s_8_38, s_8_36);
        // D s_8_40: write-var wback <= s_8_39
        fn_state.wback = s_8_39;
        // C s_8_41: const #15s : i
        let s_8_41: i128 = 15;
        // D s_8_42: read-var m:i64
        let s_8_42: i64 = fn_state.m;
        // D s_8_43: cast zx s_8_42 -> i
        let s_8_43: i128 = (i128::try_from(s_8_42).unwrap());
        // D s_8_44: call neq_int(s_8_43, s_8_41)
        let s_8_44: bool = neq_int(state, tracer, s_8_43, s_8_41);
        // N s_8_45: branch s_8_44 b16 b9
        if s_8_44 {
            return block_16(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#320846 <= s_9_0
        fn_state.gs_320846 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#320846:u8
        let s_10_0: bool = fn_state.gs_320846;
        // D s_10_1: write-var register_index <= s_10_0
        fn_state.register_index = s_10_0;
        // C s_10_2: const #15s : i
        let s_10_2: i128 = 15;
        // D s_10_3: read-var n:i64
        let s_10_3: i64 = fn_state.n;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cmp-eq s_10_4 s_10_2
        let s_10_5: bool = ((s_10_4) == (s_10_2));
        // N s_10_6: branch s_10_5 b15 b11
        if s_10_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #31s : i
        let s_11_0: i128 = 31;
        // D s_11_1: read-var d2:i64
        let s_11_1: i64 = fn_state.d2;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cmp-gt s_11_2 s_11_0
        let s_11_3: bool = ((s_11_2) > (s_11_0));
        // D s_11_4: write-var gs#320849 <= s_11_3
        fn_state.gs_320849 = s_11_3;
        // N s_11_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#320849:u8
        let s_12_0: bool = fn_state.gs_320849;
        // N s_12_1: branch s_12_0 b14 b13
        if s_12_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var d2:i64
        let s_13_0: i64 = fn_state.d2;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var alignment:i64
        let s_13_2: i64 = fn_state.alignment;
        // D s_13_3: read-var d:i64
        let s_13_3: i64 = fn_state.d;
        // C s_13_4: const #4s : i64
        let s_13_4: i64 = 4;
        // D s_13_5: read-var index:i64
        let s_13_5: i64 = fn_state.index;
        // D s_13_6: read-var m:i64
        let s_13_6: i64 = fn_state.m;
        // D s_13_7: read-var n:i64
        let s_13_7: i64 = fn_state.n;
        // D s_13_8: read-var register_index:u8
        let s_13_8: bool = fn_state.register_index;
        // D s_13_9: read-var wback:u8
        let s_13_9: bool = fn_state.wback;
        // D s_13_10: call execute_aarch32_instrs_VST2_1_Op_A_txt(s_13_2, s_13_3, s_13_1, s_13_4, s_13_5, s_13_6, s_13_7, s_13_8, s_13_9)
        let s_13_10: () = execute_aarch32_instrs_VST2_1_Op_A_txt(
            state,
            tracer,
            s_13_2,
            s_13_3,
            s_13_1,
            s_13_4,
            s_13_5,
            s_13_6,
            s_13_7,
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
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#320849 <= s_15_0
        fn_state.gs_320849 = s_15_0;
        // N s_15_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #13s : i
        let s_16_0: i128 = 13;
        // D s_16_1: read-var m:i64
        let s_16_1: i64 = fn_state.m;
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // D s_16_3: call neq_int(s_16_2, s_16_0)
        let s_16_3: bool = neq_int(state, tracer, s_16_2, s_16_0);
        // D s_16_4: write-var gs#320846 <= s_16_3
        fn_state.gs_320846 = s_16_3;
        // N s_16_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1s : i64
        let s_17_0: i64 = 1;
        // D s_17_1: write-var ga#361941 <= s_17_0
        fn_state.ga_361941 = s_17_0;
        // N s_17_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1s : i64
        let s_18_0: i64 = 1;
        // D s_18_1: write-var ga#361937 <= s_18_0
        fn_state.ga_361937 = s_18_0;
        // N s_18_2: jump b6
        return block_6(state, tracer, fn_state);
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
        // N s_20_0: panic
        panic!("{:?}", ());
        // N s_20_1: return
        return;
    }
}
