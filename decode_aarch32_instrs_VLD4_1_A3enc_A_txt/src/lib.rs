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
use u__id::*;
use ConditionPassed::*;
use execute_aarch32_instrs_VLD4_1_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VLD4_1_A3enc_A_txt<T: Tracer>(
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
        gs_311231: bool,
        ga_354681: i64,
        gs_311209: bool,
        gs_311218: bool,
        ga_354686: i128,
        gs_311213: bool,
        gs_311232: bool,
        inc_name: i64,
        gs_311205: bool,
        gs_311233: bool,
        gs_311216: bool,
        d4: i64,
        gs_311237: bool,
        gs_311223: bool,
        m: i64,
        gs_311227: bool,
        gs_311234: bool,
        gs_311236: bool,
        gs_311225: bool,
        gs_311201: bool,
        n: i64,
        index: i64,
        gs_311207: bool,
        gs_311229: bool,
        gs_311203: bool,
        d2: i64,
        gs_311194: bool,
        gs_311235: bool,
        gs_311230: bool,
        d: i64,
        register_index: bool,
        alignment: i128,
        wback: bool,
        d3: i64,
        gs_311211: bool,
        gs_311191: bool,
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
        // N s_2_5: branch s_2_4 b83 b3
        if s_2_4 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
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
        // C s_3_10: const #3u : u8
        let s_3_10: u8 = 3;
        // C s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 2u16);
        // D s_3_12: cmp-eq s_3_9 s_3_11
        let s_3_12: bool = ((s_3_9) == (s_3_11));
        // N s_3_13: branch s_3_12 b82 b4
        if s_3_12 {
            return block_82(state, tracer, fn_state);
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
        // N s_4_44: branch s_4_43 b81 b5
        if s_4_43 {
            return block_81(state, tracer, fn_state);
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
        // D s_5_1: write-var ga#354681 <= s_5_0
        fn_state.ga_354681 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#354681:i64
        let s_6_0: i64 = fn_state.ga_354681;
        // D s_6_1: write-var inc_name <= s_6_0
        fn_state.inc_name = s_6_0;
        // C s_6_2: const #0s : i
        let s_6_2: i128 = 0;
        // D s_6_3: read-var index_align:u8
        let s_6_3: u8 = fn_state.index_align;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 4u16);
        // C s_6_5: const #1s : i64
        let s_6_5: i64 = 1;
        // C s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // C s_6_7: const #1s : i
        let s_6_7: i128 = 1;
        // C s_6_8: add s_6_7 s_6_6
        let s_6_8: i128 = (s_6_7 + s_6_6);
        // D s_6_9: bit-extract s_6_4 s_6_2 s_6_8
        let s_6_9: Bits = (Bits::new(
            ((s_6_4) >> (s_6_2)).value(),
            u16::try_from(s_6_8).unwrap(),
        ));
        // D s_6_10: cast reint s_6_9 -> u8
        let s_6_10: u8 = (s_6_9.value() as u8);
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 2u16);
        // C s_6_12: const #0u : u8
        let s_6_12: u8 = 0;
        // C s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 2u16);
        // D s_6_14: cmp-eq s_6_11 s_6_13
        let s_6_14: bool = ((s_6_11) == (s_6_13));
        // N s_6_15: branch s_6_14 b80 b7
        if s_6_14 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0s : i
        let s_7_0: i128 = 0;
        // D s_7_1: read-var index_align:u8
        let s_7_1: u8 = fn_state.index_align;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 4u16);
        // C s_7_3: const #1s : i64
        let s_7_3: i64 = 1;
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // C s_7_5: const #1s : i
        let s_7_5: i128 = 1;
        // C s_7_6: add s_7_5 s_7_4
        let s_7_6: i128 = (s_7_5 + s_7_4);
        // D s_7_7: bit-extract s_7_2 s_7_0 s_7_6
        let s_7_7: Bits = (Bits::new(
            ((s_7_2) >> (s_7_0)).value(),
            u16::try_from(s_7_6).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u8
        let s_7_8: u8 = (s_7_7.value() as u8);
        // D s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 2u16);
        // D s_7_10: cast zx s_7_9 -> i
        let s_7_10: i128 = (s_7_9.value() as i128);
        // D s_7_11: cast reint s_7_10 -> i64
        let s_7_11: i64 = (s_7_10 as i64);
        // C s_7_12: const #4s : i
        let s_7_12: i128 = 4;
        // D s_7_13: cast zx s_7_11 -> i
        let s_7_13: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_14: lsl s_7_12 s_7_13
        let s_7_14: i128 = s_7_12 << s_7_13;
        // D s_7_15: write-var ga#354686 <= s_7_14
        fn_state.ga_354686 = s_7_14;
        // N s_7_16: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#354686:i
        let s_8_0: i128 = fn_state.ga_354686;
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
        // D s_8_26: read-var d2:i64
        let s_8_26: i64 = fn_state.d2;
        // D s_8_27: cast zx s_8_26 -> i
        let s_8_27: i128 = (i128::try_from(s_8_26).unwrap());
        // D s_8_28: read-var inc_name:i64
        let s_8_28: i64 = fn_state.inc_name;
        // D s_8_29: cast zx s_8_28 -> i
        let s_8_29: i128 = (i128::try_from(s_8_28).unwrap());
        // D s_8_30: add s_8_27 s_8_29
        let s_8_30: i128 = (s_8_27 + s_8_29);
        // D s_8_31: cast reint s_8_30 -> i64
        let s_8_31: i64 = (s_8_30 as i64);
        // D s_8_32: write-var d3 <= s_8_31
        fn_state.d3 = s_8_31;
        // D s_8_33: read-var d3:i64
        let s_8_33: i64 = fn_state.d3;
        // D s_8_34: cast zx s_8_33 -> i
        let s_8_34: i128 = (i128::try_from(s_8_33).unwrap());
        // D s_8_35: read-var inc_name:i64
        let s_8_35: i64 = fn_state.inc_name;
        // D s_8_36: cast zx s_8_35 -> i
        let s_8_36: i128 = (i128::try_from(s_8_35).unwrap());
        // D s_8_37: add s_8_34 s_8_36
        let s_8_37: i128 = (s_8_34 + s_8_36);
        // D s_8_38: cast reint s_8_37 -> i64
        let s_8_38: i64 = (s_8_37 as i64);
        // D s_8_39: write-var d4 <= s_8_38
        fn_state.d4 = s_8_38;
        // D s_8_40: read-var Rn:u8
        let s_8_40: u8 = fn_state.Rn;
        // D s_8_41: cast zx s_8_40 -> bv
        let s_8_41: Bits = Bits::new(s_8_40 as u128, 4u16);
        // D s_8_42: cast zx s_8_41 -> i
        let s_8_42: i128 = (s_8_41.value() as i128);
        // D s_8_43: cast reint s_8_42 -> i64
        let s_8_43: i64 = (s_8_42 as i64);
        // D s_8_44: write-var n <= s_8_43
        fn_state.n = s_8_43;
        // D s_8_45: read-var Rm:u8
        let s_8_45: u8 = fn_state.Rm;
        // D s_8_46: cast zx s_8_45 -> bv
        let s_8_46: Bits = Bits::new(s_8_45 as u128, 4u16);
        // D s_8_47: cast zx s_8_46 -> i
        let s_8_47: i128 = (s_8_46.value() as i128);
        // D s_8_48: cast reint s_8_47 -> i64
        let s_8_48: i64 = (s_8_47 as i64);
        // D s_8_49: write-var m <= s_8_48
        fn_state.m = s_8_48;
        // C s_8_50: const #15s : i
        let s_8_50: i128 = 15;
        // D s_8_51: read-var m:i64
        let s_8_51: i64 = fn_state.m;
        // D s_8_52: cast zx s_8_51 -> i
        let s_8_52: i128 = (i128::try_from(s_8_51).unwrap());
        // D s_8_53: call neq_int(s_8_52, s_8_50)
        let s_8_53: bool = neq_int(state, tracer, s_8_52, s_8_50);
        // D s_8_54: write-var wback <= s_8_53
        fn_state.wback = s_8_53;
        // C s_8_55: const #15s : i
        let s_8_55: i128 = 15;
        // D s_8_56: read-var m:i64
        let s_8_56: i64 = fn_state.m;
        // D s_8_57: cast zx s_8_56 -> i
        let s_8_57: i128 = (i128::try_from(s_8_56).unwrap());
        // D s_8_58: call neq_int(s_8_57, s_8_55)
        let s_8_58: bool = neq_int(state, tracer, s_8_57, s_8_55);
        // N s_8_59: branch s_8_58 b79 b9
        if s_8_58 {
            return block_79(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#311191 <= s_9_0
        fn_state.gs_311191 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#311191:u8
        let s_10_0: bool = fn_state.gs_311191;
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
        // N s_10_6: branch s_10_5 b78 b11
        if s_10_5 {
            return block_78(state, tracer, fn_state);
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
        // D s_11_1: read-var d4:i64
        let s_11_1: i64 = fn_state.d4;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cmp-gt s_11_2 s_11_0
        let s_11_3: bool = ((s_11_2) > (s_11_0));
        // D s_11_4: write-var gs#311194 <= s_11_3
        fn_state.gs_311194 = s_11_3;
        // N s_11_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#311194:u8
        let s_12_0: bool = fn_state.gs_311194;
        // N s_12_1: branch s_12_0 b77 b13
        if s_12_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var n:i64
        let s_13_0: i64 = fn_state.n;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call __id(s_13_1)
        let s_13_2: i128 = u__id(state, tracer, s_13_1);
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: const #0s : i
        let s_13_4: i128 = 0;
        // D s_13_5: cast zx s_13_3 -> i
        let s_13_5: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_6: cmp-le s_13_4 s_13_5
        let s_13_6: bool = ((s_13_4) <= (s_13_5));
        // N s_13_7: branch s_13_6 b16 b14
        if s_13_6 {
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
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#311237 <= s_14_0
        fn_state.gs_311237 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#311237:u8
        let s_15_0: bool = fn_state.gs_311237;
        // N s_15_1: assert s_15_0
        let s_15_1: () = assert!(s_15_0);
        // D s_15_2: read-var alignment:i
        let s_15_2: i128 = fn_state.alignment;
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // D s_15_4: read-var d2:i64
        let s_15_4: i64 = fn_state.d2;
        // D s_15_5: cast zx s_15_4 -> i
        let s_15_5: i128 = (i128::try_from(s_15_4).unwrap());
        // D s_15_6: read-var d3:i64
        let s_15_6: i64 = fn_state.d3;
        // D s_15_7: cast zx s_15_6 -> i
        let s_15_7: i128 = (i128::try_from(s_15_6).unwrap());
        // D s_15_8: read-var d4:i64
        let s_15_8: i64 = fn_state.d4;
        // D s_15_9: cast zx s_15_8 -> i
        let s_15_9: i128 = (i128::try_from(s_15_8).unwrap());
        // D s_15_10: read-var d:i64
        let s_15_10: i64 = fn_state.d;
        // C s_15_11: const #4s : i64
        let s_15_11: i64 = 4;
        // D s_15_12: read-var index:i64
        let s_15_12: i64 = fn_state.index;
        // D s_15_13: read-var m:i64
        let s_15_13: i64 = fn_state.m;
        // D s_15_14: read-var n:i64
        let s_15_14: i64 = fn_state.n;
        // D s_15_15: read-var register_index:u8
        let s_15_15: bool = fn_state.register_index;
        // D s_15_16: read-var wback:u8
        let s_15_16: bool = fn_state.wback;
        // D s_15_17: call execute_aarch32_instrs_VLD4_1_Op_A_txt(s_15_3, s_15_10, s_15_5, s_15_7, s_15_9, s_15_11, s_15_12, s_15_13, s_15_14, s_15_15, s_15_16)
        let s_15_17: () = execute_aarch32_instrs_VLD4_1_Op_A_txt(
            state,
            tracer,
            s_15_3,
            s_15_10,
            s_15_5,
            s_15_7,
            s_15_9,
            s_15_11,
            s_15_12,
            s_15_13,
            s_15_14,
            s_15_15,
            s_15_16,
        );
        // N s_15_18: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var n:i64
        let s_16_0: i64 = fn_state.n;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: call __id(s_16_1)
        let s_16_2: i128 = u__id(state, tracer, s_16_1);
        // D s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // C s_16_4: const #15s : i
        let s_16_4: i128 = 15;
        // D s_16_5: cast zx s_16_3 -> i
        let s_16_5: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_6: cmp-le s_16_5 s_16_4
        let s_16_6: bool = ((s_16_5) <= (s_16_4));
        // N s_16_7: branch s_16_6 b19 b17
        if s_16_6 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#311236 <= s_17_0
        fn_state.gs_311236 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#311236:u8
        let s_18_0: bool = fn_state.gs_311236;
        // D s_18_1: write-var gs#311237 <= s_18_0
        fn_state.gs_311237 = s_18_0;
        // N s_18_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var m:i64
        let s_19_0: i64 = fn_state.m;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: call __id(s_19_1)
        let s_19_2: i128 = u__id(state, tracer, s_19_1);
        // D s_19_3: cast reint s_19_2 -> i64
        let s_19_3: i64 = (s_19_2 as i64);
        // C s_19_4: const #0s : i
        let s_19_4: i128 = 0;
        // D s_19_5: cast zx s_19_3 -> i
        let s_19_5: i128 = (i128::try_from(s_19_3).unwrap());
        // D s_19_6: cmp-le s_19_4 s_19_5
        let s_19_6: bool = ((s_19_4) <= (s_19_5));
        // N s_19_7: branch s_19_6 b22 b20
        if s_19_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#311235 <= s_20_0
        fn_state.gs_311235 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#311235:u8
        let s_21_0: bool = fn_state.gs_311235;
        // D s_21_1: write-var gs#311236 <= s_21_0
        fn_state.gs_311236 = s_21_0;
        // N s_21_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var m:i64
        let s_22_0: i64 = fn_state.m;
        // D s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_2: call __id(s_22_1)
        let s_22_2: i128 = u__id(state, tracer, s_22_1);
        // D s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: const #15s : i
        let s_22_4: i128 = 15;
        // D s_22_5: cast zx s_22_3 -> i
        let s_22_5: i128 = (i128::try_from(s_22_3).unwrap());
        // D s_22_6: cmp-le s_22_5 s_22_4
        let s_22_6: bool = ((s_22_5) <= (s_22_4));
        // N s_22_7: branch s_22_6 b25 b23
        if s_22_6 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#311234 <= s_23_0
        fn_state.gs_311234 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#311234:u8
        let s_24_0: bool = fn_state.gs_311234;
        // D s_24_1: write-var gs#311235 <= s_24_0
        fn_state.gs_311235 = s_24_0;
        // N s_24_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var index:i64
        let s_25_0: i64 = fn_state.index;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: call __id(s_25_1)
        let s_25_2: i128 = u__id(state, tracer, s_25_1);
        // D s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: const #0s : i
        let s_25_4: i128 = 0;
        // D s_25_5: cast zx s_25_3 -> i
        let s_25_5: i128 = (i128::try_from(s_25_3).unwrap());
        // D s_25_6: cmp-eq s_25_5 s_25_4
        let s_25_6: bool = ((s_25_5) == (s_25_4));
        // N s_25_7: branch s_25_6 b76 b26
        if s_25_6 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var index:i64
        let s_26_0: i64 = fn_state.index;
        // D s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_2: call __id(s_26_1)
        let s_26_2: i128 = u__id(state, tracer, s_26_1);
        // D s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: const #1s : i
        let s_26_4: i128 = 1;
        // D s_26_5: cast zx s_26_3 -> i
        let s_26_5: i128 = (i128::try_from(s_26_3).unwrap());
        // D s_26_6: cmp-eq s_26_5 s_26_4
        let s_26_6: bool = ((s_26_5) == (s_26_4));
        // D s_26_7: write-var gs#311201 <= s_26_6
        fn_state.gs_311201 = s_26_6;
        // N s_26_8: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#311201:u8
        let s_27_0: bool = fn_state.gs_311201;
        // N s_27_1: branch s_27_0 b75 b28
        if s_27_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var index:i64
        let s_28_0: i64 = fn_state.index;
        // D s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: call __id(s_28_1)
        let s_28_2: i128 = u__id(state, tracer, s_28_1);
        // D s_28_3: cast reint s_28_2 -> i64
        let s_28_3: i64 = (s_28_2 as i64);
        // C s_28_4: const #2s : i
        let s_28_4: i128 = 2;
        // D s_28_5: cast zx s_28_3 -> i
        let s_28_5: i128 = (i128::try_from(s_28_3).unwrap());
        // D s_28_6: cmp-eq s_28_5 s_28_4
        let s_28_6: bool = ((s_28_5) == (s_28_4));
        // D s_28_7: write-var gs#311203 <= s_28_6
        fn_state.gs_311203 = s_28_6;
        // N s_28_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#311203:u8
        let s_29_0: bool = fn_state.gs_311203;
        // N s_29_1: branch s_29_0 b74 b30
        if s_29_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var index:i64
        let s_30_0: i64 = fn_state.index;
        // D s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // D s_30_2: call __id(s_30_1)
        let s_30_2: i128 = u__id(state, tracer, s_30_1);
        // D s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: const #3s : i
        let s_30_4: i128 = 3;
        // D s_30_5: cast zx s_30_3 -> i
        let s_30_5: i128 = (i128::try_from(s_30_3).unwrap());
        // D s_30_6: cmp-eq s_30_5 s_30_4
        let s_30_6: bool = ((s_30_5) == (s_30_4));
        // D s_30_7: write-var gs#311205 <= s_30_6
        fn_state.gs_311205 = s_30_6;
        // N s_30_8: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#311205:u8
        let s_31_0: bool = fn_state.gs_311205;
        // N s_31_1: branch s_31_0 b73 b32
        if s_31_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var index:i64
        let s_32_0: i64 = fn_state.index;
        // D s_32_1: cast zx s_32_0 -> i
        let s_32_1: i128 = (i128::try_from(s_32_0).unwrap());
        // D s_32_2: call __id(s_32_1)
        let s_32_2: i128 = u__id(state, tracer, s_32_1);
        // D s_32_3: cast reint s_32_2 -> i64
        let s_32_3: i64 = (s_32_2 as i64);
        // C s_32_4: const #4s : i
        let s_32_4: i128 = 4;
        // D s_32_5: cast zx s_32_3 -> i
        let s_32_5: i128 = (i128::try_from(s_32_3).unwrap());
        // D s_32_6: cmp-eq s_32_5 s_32_4
        let s_32_6: bool = ((s_32_5) == (s_32_4));
        // D s_32_7: write-var gs#311207 <= s_32_6
        fn_state.gs_311207 = s_32_6;
        // N s_32_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#311207:u8
        let s_33_0: bool = fn_state.gs_311207;
        // N s_33_1: branch s_33_0 b72 b34
        if s_33_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var index:i64
        let s_34_0: i64 = fn_state.index;
        // D s_34_1: cast zx s_34_0 -> i
        let s_34_1: i128 = (i128::try_from(s_34_0).unwrap());
        // D s_34_2: call __id(s_34_1)
        let s_34_2: i128 = u__id(state, tracer, s_34_1);
        // D s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: const #5s : i
        let s_34_4: i128 = 5;
        // D s_34_5: cast zx s_34_3 -> i
        let s_34_5: i128 = (i128::try_from(s_34_3).unwrap());
        // D s_34_6: cmp-eq s_34_5 s_34_4
        let s_34_6: bool = ((s_34_5) == (s_34_4));
        // D s_34_7: write-var gs#311209 <= s_34_6
        fn_state.gs_311209 = s_34_6;
        // N s_34_8: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#311209:u8
        let s_35_0: bool = fn_state.gs_311209;
        // N s_35_1: branch s_35_0 b71 b36
        if s_35_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var index:i64
        let s_36_0: i64 = fn_state.index;
        // D s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_2: call __id(s_36_1)
        let s_36_2: i128 = u__id(state, tracer, s_36_1);
        // D s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: const #6s : i
        let s_36_4: i128 = 6;
        // D s_36_5: cast zx s_36_3 -> i
        let s_36_5: i128 = (i128::try_from(s_36_3).unwrap());
        // D s_36_6: cmp-eq s_36_5 s_36_4
        let s_36_6: bool = ((s_36_5) == (s_36_4));
        // D s_36_7: write-var gs#311211 <= s_36_6
        fn_state.gs_311211 = s_36_6;
        // N s_36_8: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#311211:u8
        let s_37_0: bool = fn_state.gs_311211;
        // N s_37_1: branch s_37_0 b70 b38
        if s_37_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var index:i64
        let s_38_0: i64 = fn_state.index;
        // D s_38_1: cast zx s_38_0 -> i
        let s_38_1: i128 = (i128::try_from(s_38_0).unwrap());
        // D s_38_2: call __id(s_38_1)
        let s_38_2: i128 = u__id(state, tracer, s_38_1);
        // D s_38_3: cast reint s_38_2 -> i64
        let s_38_3: i64 = (s_38_2 as i64);
        // C s_38_4: const #7s : i
        let s_38_4: i128 = 7;
        // D s_38_5: cast zx s_38_3 -> i
        let s_38_5: i128 = (i128::try_from(s_38_3).unwrap());
        // D s_38_6: cmp-eq s_38_5 s_38_4
        let s_38_6: bool = ((s_38_5) == (s_38_4));
        // D s_38_7: write-var gs#311213 <= s_38_6
        fn_state.gs_311213 = s_38_6;
        // N s_38_8: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#311213:u8
        let s_39_0: bool = fn_state.gs_311213;
        // N s_39_1: branch s_39_0 b42 b40
        if s_39_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#311233 <= s_40_0
        fn_state.gs_311233 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#311233:u8
        let s_41_0: bool = fn_state.gs_311233;
        // D s_41_1: write-var gs#311234 <= s_41_0
        fn_state.gs_311234 = s_41_0;
        // N s_41_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #4s : i64
        let s_42_0: i64 = 4;
        // C s_42_1: cast zx s_42_0 -> i
        let s_42_1: i128 = (i128::try_from(s_42_0).unwrap());
        // S s_42_2: call __id(s_42_1)
        let s_42_2: i128 = u__id(state, tracer, s_42_1);
        // S s_42_3: cast reint s_42_2 -> i64
        let s_42_3: i64 = (s_42_2 as i64);
        // C s_42_4: const #1s : i
        let s_42_4: i128 = 1;
        // S s_42_5: cast zx s_42_3 -> i
        let s_42_5: i128 = (i128::try_from(s_42_3).unwrap());
        // S s_42_6: cmp-eq s_42_5 s_42_4
        let s_42_6: bool = ((s_42_5) == (s_42_4));
        // N s_42_7: branch s_42_6 b69 b43
        if s_42_6 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #4s : i64
        let s_43_0: i64 = 4;
        // C s_43_1: cast zx s_43_0 -> i
        let s_43_1: i128 = (i128::try_from(s_43_0).unwrap());
        // S s_43_2: call __id(s_43_1)
        let s_43_2: i128 = u__id(state, tracer, s_43_1);
        // S s_43_3: cast reint s_43_2 -> i64
        let s_43_3: i64 = (s_43_2 as i64);
        // C s_43_4: const #2s : i
        let s_43_4: i128 = 2;
        // S s_43_5: cast zx s_43_3 -> i
        let s_43_5: i128 = (i128::try_from(s_43_3).unwrap());
        // S s_43_6: cmp-eq s_43_5 s_43_4
        let s_43_6: bool = ((s_43_5) == (s_43_4));
        // D s_43_7: write-var gs#311216 <= s_43_6
        fn_state.gs_311216 = s_43_6;
        // N s_43_8: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#311216:u8
        let s_44_0: bool = fn_state.gs_311216;
        // N s_44_1: branch s_44_0 b68 b45
        if s_44_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #4s : i64
        let s_45_0: i64 = 4;
        // C s_45_1: cast zx s_45_0 -> i
        let s_45_1: i128 = (i128::try_from(s_45_0).unwrap());
        // S s_45_2: call __id(s_45_1)
        let s_45_2: i128 = u__id(state, tracer, s_45_1);
        // S s_45_3: cast reint s_45_2 -> i64
        let s_45_3: i64 = (s_45_2 as i64);
        // C s_45_4: const #4s : i
        let s_45_4: i128 = 4;
        // S s_45_5: cast zx s_45_3 -> i
        let s_45_5: i128 = (i128::try_from(s_45_3).unwrap());
        // S s_45_6: cmp-eq s_45_5 s_45_4
        let s_45_6: bool = ((s_45_5) == (s_45_4));
        // D s_45_7: write-var gs#311218 <= s_45_6
        fn_state.gs_311218 = s_45_6;
        // N s_45_8: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#311218:u8
        let s_46_0: bool = fn_state.gs_311218;
        // N s_46_1: branch s_46_0 b49 b47
        if s_46_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#311232 <= s_47_0
        fn_state.gs_311232 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#311232:u8
        let s_48_0: bool = fn_state.gs_311232;
        // D s_48_1: write-var gs#311233 <= s_48_0
        fn_state.gs_311233 = s_48_0;
        // N s_48_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var d:i64
        let s_49_0: i64 = fn_state.d;
        // D s_49_1: cast zx s_49_0 -> i
        let s_49_1: i128 = (i128::try_from(s_49_0).unwrap());
        // D s_49_2: call __id(s_49_1)
        let s_49_2: i128 = u__id(state, tracer, s_49_1);
        // D s_49_3: cast reint s_49_2 -> i64
        let s_49_3: i64 = (s_49_2 as i64);
        // C s_49_4: const #0s : i
        let s_49_4: i128 = 0;
        // D s_49_5: cast zx s_49_3 -> i
        let s_49_5: i128 = (i128::try_from(s_49_3).unwrap());
        // D s_49_6: cmp-le s_49_4 s_49_5
        let s_49_6: bool = ((s_49_4) <= (s_49_5));
        // N s_49_7: branch s_49_6 b52 b50
        if s_49_6 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#311231 <= s_50_0
        fn_state.gs_311231 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#311231:u8
        let s_51_0: bool = fn_state.gs_311231;
        // D s_51_1: write-var gs#311232 <= s_51_0
        fn_state.gs_311232 = s_51_0;
        // N s_51_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var d:i64
        let s_52_0: i64 = fn_state.d;
        // D s_52_1: cast zx s_52_0 -> i
        let s_52_1: i128 = (i128::try_from(s_52_0).unwrap());
        // D s_52_2: call __id(s_52_1)
        let s_52_2: i128 = u__id(state, tracer, s_52_1);
        // D s_52_3: cast reint s_52_2 -> i64
        let s_52_3: i64 = (s_52_2 as i64);
        // C s_52_4: const #31s : i
        let s_52_4: i128 = 31;
        // D s_52_5: cast zx s_52_3 -> i
        let s_52_5: i128 = (i128::try_from(s_52_3).unwrap());
        // D s_52_6: cmp-le s_52_5 s_52_4
        let s_52_6: bool = ((s_52_5) <= (s_52_4));
        // N s_52_7: branch s_52_6 b55 b53
        if s_52_6 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#311230 <= s_53_0
        fn_state.gs_311230 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#311230:u8
        let s_54_0: bool = fn_state.gs_311230;
        // D s_54_1: write-var gs#311231 <= s_54_0
        fn_state.gs_311231 = s_54_0;
        // N s_54_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var alignment:i
        let s_55_0: i128 = fn_state.alignment;
        // D s_55_1: call __id(s_55_0)
        let s_55_1: i128 = u__id(state, tracer, s_55_0);
        // C s_55_2: const #1s : i
        let s_55_2: i128 = 1;
        // D s_55_3: cmp-eq s_55_1 s_55_2
        let s_55_3: bool = ((s_55_1) == (s_55_2));
        // N s_55_4: branch s_55_3 b67 b56
        if s_55_3 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var alignment:i
        let s_56_0: i128 = fn_state.alignment;
        // D s_56_1: call __id(s_56_0)
        let s_56_1: i128 = u__id(state, tracer, s_56_0);
        // C s_56_2: const #4s : i
        let s_56_2: i128 = 4;
        // D s_56_3: cmp-eq s_56_1 s_56_2
        let s_56_3: bool = ((s_56_1) == (s_56_2));
        // D s_56_4: write-var gs#311223 <= s_56_3
        fn_state.gs_311223 = s_56_3;
        // N s_56_5: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#311223:u8
        let s_57_0: bool = fn_state.gs_311223;
        // N s_57_1: branch s_57_0 b66 b58
        if s_57_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var alignment:i
        let s_58_0: i128 = fn_state.alignment;
        // D s_58_1: call __id(s_58_0)
        let s_58_1: i128 = u__id(state, tracer, s_58_0);
        // C s_58_2: const #8s : i
        let s_58_2: i128 = 8;
        // D s_58_3: cmp-eq s_58_1 s_58_2
        let s_58_3: bool = ((s_58_1) == (s_58_2));
        // D s_58_4: write-var gs#311225 <= s_58_3
        fn_state.gs_311225 = s_58_3;
        // N s_58_5: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#311225:u8
        let s_59_0: bool = fn_state.gs_311225;
        // N s_59_1: branch s_59_0 b65 b60
        if s_59_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var alignment:i
        let s_60_0: i128 = fn_state.alignment;
        // D s_60_1: call __id(s_60_0)
        let s_60_1: i128 = u__id(state, tracer, s_60_0);
        // C s_60_2: const #16s : i
        let s_60_2: i128 = 16;
        // D s_60_3: cmp-eq s_60_1 s_60_2
        let s_60_3: bool = ((s_60_1) == (s_60_2));
        // D s_60_4: write-var gs#311227 <= s_60_3
        fn_state.gs_311227 = s_60_3;
        // N s_60_5: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#311227:u8
        let s_61_0: bool = fn_state.gs_311227;
        // N s_61_1: branch s_61_0 b64 b62
        if s_61_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var alignment:i
        let s_62_0: i128 = fn_state.alignment;
        // D s_62_1: call __id(s_62_0)
        let s_62_1: i128 = u__id(state, tracer, s_62_0);
        // C s_62_2: const #32s : i
        let s_62_2: i128 = 32;
        // D s_62_3: cmp-eq s_62_1 s_62_2
        let s_62_3: bool = ((s_62_1) == (s_62_2));
        // D s_62_4: write-var gs#311229 <= s_62_3
        fn_state.gs_311229 = s_62_3;
        // N s_62_5: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#311229:u8
        let s_63_0: bool = fn_state.gs_311229;
        // D s_63_1: write-var gs#311230 <= s_63_0
        fn_state.gs_311230 = s_63_0;
        // N s_63_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #1u : u8
        let s_64_0: bool = true;
        // D s_64_1: write-var gs#311229 <= s_64_0
        fn_state.gs_311229 = s_64_0;
        // N s_64_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#311227 <= s_65_0
        fn_state.gs_311227 = s_65_0;
        // N s_65_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // D s_66_1: write-var gs#311225 <= s_66_0
        fn_state.gs_311225 = s_66_0;
        // N s_66_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#311223 <= s_67_0
        fn_state.gs_311223 = s_67_0;
        // N s_67_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#311218 <= s_68_0
        fn_state.gs_311218 = s_68_0;
        // N s_68_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // D s_69_1: write-var gs#311216 <= s_69_0
        fn_state.gs_311216 = s_69_0;
        // N s_69_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var gs#311213 <= s_70_0
        fn_state.gs_311213 = s_70_0;
        // N s_70_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var gs#311211 <= s_71_0
        fn_state.gs_311211 = s_71_0;
        // N s_71_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#311209 <= s_72_0
        fn_state.gs_311209 = s_72_0;
        // N s_72_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #1u : u8
        let s_73_0: bool = true;
        // D s_73_1: write-var gs#311207 <= s_73_0
        fn_state.gs_311207 = s_73_0;
        // N s_73_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #1u : u8
        let s_74_0: bool = true;
        // D s_74_1: write-var gs#311205 <= s_74_0
        fn_state.gs_311205 = s_74_0;
        // N s_74_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#311203 <= s_75_0
        fn_state.gs_311203 = s_75_0;
        // N s_75_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #1u : u8
        let s_76_0: bool = true;
        // D s_76_1: write-var gs#311201 <= s_76_0
        fn_state.gs_311201 = s_76_0;
        // N s_76_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_77_0: panic
        panic!("{:?}", ());
        // N s_77_1: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #1u : u8
        let s_78_0: bool = true;
        // D s_78_1: write-var gs#311194 <= s_78_0
        fn_state.gs_311194 = s_78_0;
        // N s_78_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #13s : i
        let s_79_0: i128 = 13;
        // D s_79_1: read-var m:i64
        let s_79_1: i64 = fn_state.m;
        // D s_79_2: cast zx s_79_1 -> i
        let s_79_2: i128 = (i128::try_from(s_79_1).unwrap());
        // D s_79_3: call neq_int(s_79_2, s_79_0)
        let s_79_3: bool = neq_int(state, tracer, s_79_2, s_79_0);
        // D s_79_4: write-var gs#311191 <= s_79_3
        fn_state.gs_311191 = s_79_3;
        // N s_79_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #1s : i
        let s_80_0: i128 = 1;
        // D s_80_1: write-var ga#354686 <= s_80_0
        fn_state.ga_354686 = s_80_0;
        // N s_80_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #1s : i64
        let s_81_0: i64 = 1;
        // D s_81_1: write-var ga#354681 <= s_81_0
        fn_state.ga_354681 = s_81_0;
        // N s_81_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_82_0: panic
        panic!("{:?}", ());
        // N s_82_1: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_83_0: panic
        panic!("{:?}", ());
        // N s_83_1: return
        return;
    }
}
