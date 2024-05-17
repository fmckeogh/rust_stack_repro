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
use u__id::*;
use ConditionPassed::*;
use common::*;
pub fn decode_aarch32_instrs_VST4_1_A3enc_A_txt<T: Tracer>(
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
        ga_362579: i128,
        gs_321773: bool,
        gs_321770: bool,
        gs_321761: bool,
        gs_321727: bool,
        gs_321766: bool,
        gs_321754: bool,
        gs_321771: bool,
        gs_321772: bool,
        gs_321741: bool,
        gs_321745: bool,
        inc_name: i64,
        gs_321730: bool,
        gs_321767: bool,
        gs_321747: bool,
        d4: i64,
        gs_321749: bool,
        gs_321739: bool,
        gs_321737: bool,
        m: i64,
        gs_321743: bool,
        gs_321765: bool,
        n: i64,
        index: i64,
        gs_321769: bool,
        d2: i64,
        gs_321763: bool,
        d: i64,
        gs_321759: bool,
        register_index: bool,
        alignment: i128,
        wback: bool,
        d3: i64,
        gs_321752: bool,
        gs_321768: bool,
        ga_362574: i64,
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
        // N s_2_5: branch s_2_4 b85 b3
        if s_2_4 {
            return block_85(state, tracer, fn_state);
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
        // C s_3_2: const #2u : u8
        let s_3_2: u8 = 2;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-ne s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) != (s_3_3));
        // N s_3_5: branch s_3_4 b84 b4
        if s_3_4 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: read-var index_align:u8
        let s_4_1: u8 = fn_state.index_align;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #1s : i
        let s_4_5: i128 = 1;
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
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 2u16);
        // C s_4_10: const #3u : u8
        let s_4_10: u8 = 3;
        // C s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 2u16);
        // D s_4_12: cmp-eq s_4_9 s_4_11
        let s_4_12: bool = ((s_4_9) == (s_4_11));
        // N s_4_13: branch s_4_12 b83 b5
        if s_4_12 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #3s : i
        let s_5_0: i128 = 3;
        // D s_5_1: read-var index_align:u8
        let s_5_1: u8 = fn_state.index_align;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 4u16);
        // C s_5_3: const #1u : u64
        let s_5_3: u64 = 1;
        // D s_5_4: bit-extract s_5_2 s_5_0 s_5_3
        let s_5_4: Bits = (Bits::new(
            ((s_5_2) >> (s_5_0)).value(),
            u16::try_from(s_5_3).unwrap(),
        ));
        // D s_5_5: cast reint s_5_4 -> u8
        let s_5_5: bool = ((s_5_4.value()) != 0);
        // C s_5_6: const #0s : i
        let s_5_6: i128 = 0;
        // C s_5_7: const #0u : u64
        let s_5_7: u64 = 0;
        // D s_5_8: cast zx s_5_5 -> u64
        let s_5_8: u64 = (s_5_5 as u64);
        // C s_5_9: const #1u : u64
        let s_5_9: u64 = 1;
        // D s_5_10: and s_5_8 s_5_9
        let s_5_10: u64 = ((s_5_8) & (s_5_9));
        // D s_5_11: cmp-eq s_5_10 s_5_9
        let s_5_11: bool = ((s_5_10) == (s_5_9));
        // D s_5_12: lsl s_5_8 s_5_6
        let s_5_12: u64 = s_5_8 << s_5_6;
        // D s_5_13: or s_5_7 s_5_12
        let s_5_13: u64 = ((s_5_7) | (s_5_12));
        // D s_5_14: cmpl s_5_12
        let s_5_14: u64 = !s_5_12;
        // D s_5_15: and s_5_7 s_5_14
        let s_5_15: u64 = ((s_5_7) & (s_5_14));
        // D s_5_16: select s_5_11 s_5_13 s_5_15
        let s_5_16: u64 = if s_5_11 { s_5_13 } else { s_5_15 };
        // D s_5_17: cast trunc s_5_16 -> u8
        let s_5_17: bool = ((s_5_16) != 0);
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 1u16);
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (s_5_18.value() as i128);
        // D s_5_20: cast reint s_5_19 -> i64
        let s_5_20: i64 = (s_5_19 as i64);
        // D s_5_21: write-var index <= s_5_20
        fn_state.index = s_5_20;
        // C s_5_22: const #2s : i
        let s_5_22: i128 = 2;
        // D s_5_23: read-var index_align:u8
        let s_5_23: u8 = fn_state.index_align;
        // D s_5_24: cast zx s_5_23 -> bv
        let s_5_24: Bits = Bits::new(s_5_23 as u128, 4u16);
        // C s_5_25: const #1u : u64
        let s_5_25: u64 = 1;
        // D s_5_26: bit-extract s_5_24 s_5_22 s_5_25
        let s_5_26: Bits = (Bits::new(
            ((s_5_24) >> (s_5_22)).value(),
            u16::try_from(s_5_25).unwrap(),
        ));
        // D s_5_27: cast reint s_5_26 -> u8
        let s_5_27: bool = ((s_5_26.value()) != 0);
        // C s_5_28: const #0s : i
        let s_5_28: i128 = 0;
        // C s_5_29: const #0u : u64
        let s_5_29: u64 = 0;
        // D s_5_30: cast zx s_5_27 -> u64
        let s_5_30: u64 = (s_5_27 as u64);
        // C s_5_31: const #1u : u64
        let s_5_31: u64 = 1;
        // D s_5_32: and s_5_30 s_5_31
        let s_5_32: u64 = ((s_5_30) & (s_5_31));
        // D s_5_33: cmp-eq s_5_32 s_5_31
        let s_5_33: bool = ((s_5_32) == (s_5_31));
        // D s_5_34: lsl s_5_30 s_5_28
        let s_5_34: u64 = s_5_30 << s_5_28;
        // D s_5_35: or s_5_29 s_5_34
        let s_5_35: u64 = ((s_5_29) | (s_5_34));
        // D s_5_36: cmpl s_5_34
        let s_5_36: u64 = !s_5_34;
        // D s_5_37: and s_5_29 s_5_36
        let s_5_37: u64 = ((s_5_29) & (s_5_36));
        // D s_5_38: select s_5_33 s_5_35 s_5_37
        let s_5_38: u64 = if s_5_33 { s_5_35 } else { s_5_37 };
        // D s_5_39: cast trunc s_5_38 -> u8
        let s_5_39: bool = ((s_5_38) != 0);
        // D s_5_40: cast zx s_5_39 -> bv
        let s_5_40: Bits = Bits::new(s_5_39 as u128, 1u16);
        // C s_5_41: const #0u : u8
        let s_5_41: bool = false;
        // C s_5_42: cast zx s_5_41 -> bv
        let s_5_42: Bits = Bits::new(s_5_41 as u128, 1u16);
        // D s_5_43: cmp-eq s_5_40 s_5_42
        let s_5_43: bool = ((s_5_40) == (s_5_42));
        // N s_5_44: branch s_5_43 b82 b6
        if s_5_43 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #2s : i64
        let s_6_0: i64 = 2;
        // D s_6_1: write-var ga#362574 <= s_6_0
        fn_state.ga_362574 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#362574:i64
        let s_7_0: i64 = fn_state.ga_362574;
        // D s_7_1: write-var inc_name <= s_7_0
        fn_state.inc_name = s_7_0;
        // C s_7_2: const #0s : i
        let s_7_2: i128 = 0;
        // D s_7_3: read-var index_align:u8
        let s_7_3: u8 = fn_state.index_align;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 4u16);
        // C s_7_5: const #1s : i64
        let s_7_5: i64 = 1;
        // C s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // C s_7_7: const #1s : i
        let s_7_7: i128 = 1;
        // C s_7_8: add s_7_7 s_7_6
        let s_7_8: i128 = (s_7_7 + s_7_6);
        // D s_7_9: bit-extract s_7_4 s_7_2 s_7_8
        let s_7_9: Bits = (Bits::new(
            ((s_7_4) >> (s_7_2)).value(),
            u16::try_from(s_7_8).unwrap(),
        ));
        // D s_7_10: cast reint s_7_9 -> u8
        let s_7_10: u8 = (s_7_9.value() as u8);
        // D s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 2u16);
        // C s_7_12: const #0u : u8
        let s_7_12: u8 = 0;
        // C s_7_13: cast zx s_7_12 -> bv
        let s_7_13: Bits = Bits::new(s_7_12 as u128, 2u16);
        // D s_7_14: cmp-eq s_7_11 s_7_13
        let s_7_14: bool = ((s_7_11) == (s_7_13));
        // N s_7_15: branch s_7_14 b81 b8
        if s_7_14 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0s : i
        let s_8_0: i128 = 0;
        // D s_8_1: read-var index_align:u8
        let s_8_1: u8 = fn_state.index_align;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 4u16);
        // C s_8_3: const #1s : i64
        let s_8_3: i64 = 1;
        // C s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // C s_8_5: const #1s : i
        let s_8_5: i128 = 1;
        // C s_8_6: add s_8_5 s_8_4
        let s_8_6: i128 = (s_8_5 + s_8_4);
        // D s_8_7: bit-extract s_8_2 s_8_0 s_8_6
        let s_8_7: Bits = (Bits::new(
            ((s_8_2) >> (s_8_0)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D s_8_8: cast reint s_8_7 -> u8
        let s_8_8: u8 = (s_8_7.value() as u8);
        // D s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 2u16);
        // D s_8_10: cast zx s_8_9 -> i
        let s_8_10: i128 = (s_8_9.value() as i128);
        // D s_8_11: cast reint s_8_10 -> i64
        let s_8_11: i64 = (s_8_10 as i64);
        // C s_8_12: const #4s : i
        let s_8_12: i128 = 4;
        // D s_8_13: cast zx s_8_11 -> i
        let s_8_13: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_14: lsl s_8_12 s_8_13
        let s_8_14: i128 = s_8_12 << s_8_13;
        // D s_8_15: write-var ga#362579 <= s_8_14
        fn_state.ga_362579 = s_8_14;
        // N s_8_16: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#362579:i
        let s_9_0: i128 = fn_state.ga_362579;
        // D s_9_1: write-var alignment <= s_9_0
        fn_state.alignment = s_9_0;
        // D s_9_2: read-var D:u8
        let s_9_2: bool = fn_state.D;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: read-var Vd:u8
        let s_9_4: u8 = fn_state.Vd;
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 4u16);
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
        // D s_9_19: read-var d:i64
        let s_9_19: i64 = fn_state.d;
        // D s_9_20: cast zx s_9_19 -> i
        let s_9_20: i128 = (i128::try_from(s_9_19).unwrap());
        // D s_9_21: read-var inc_name:i64
        let s_9_21: i64 = fn_state.inc_name;
        // D s_9_22: cast zx s_9_21 -> i
        let s_9_22: i128 = (i128::try_from(s_9_21).unwrap());
        // D s_9_23: add s_9_20 s_9_22
        let s_9_23: i128 = (s_9_20 + s_9_22);
        // D s_9_24: cast reint s_9_23 -> i64
        let s_9_24: i64 = (s_9_23 as i64);
        // D s_9_25: write-var d2 <= s_9_24
        fn_state.d2 = s_9_24;
        // D s_9_26: read-var d2:i64
        let s_9_26: i64 = fn_state.d2;
        // D s_9_27: cast zx s_9_26 -> i
        let s_9_27: i128 = (i128::try_from(s_9_26).unwrap());
        // D s_9_28: read-var inc_name:i64
        let s_9_28: i64 = fn_state.inc_name;
        // D s_9_29: cast zx s_9_28 -> i
        let s_9_29: i128 = (i128::try_from(s_9_28).unwrap());
        // D s_9_30: add s_9_27 s_9_29
        let s_9_30: i128 = (s_9_27 + s_9_29);
        // D s_9_31: cast reint s_9_30 -> i64
        let s_9_31: i64 = (s_9_30 as i64);
        // D s_9_32: write-var d3 <= s_9_31
        fn_state.d3 = s_9_31;
        // D s_9_33: read-var d3:i64
        let s_9_33: i64 = fn_state.d3;
        // D s_9_34: cast zx s_9_33 -> i
        let s_9_34: i128 = (i128::try_from(s_9_33).unwrap());
        // D s_9_35: read-var inc_name:i64
        let s_9_35: i64 = fn_state.inc_name;
        // D s_9_36: cast zx s_9_35 -> i
        let s_9_36: i128 = (i128::try_from(s_9_35).unwrap());
        // D s_9_37: add s_9_34 s_9_36
        let s_9_37: i128 = (s_9_34 + s_9_36);
        // D s_9_38: cast reint s_9_37 -> i64
        let s_9_38: i64 = (s_9_37 as i64);
        // D s_9_39: write-var d4 <= s_9_38
        fn_state.d4 = s_9_38;
        // D s_9_40: read-var Rn:u8
        let s_9_40: u8 = fn_state.Rn;
        // D s_9_41: cast zx s_9_40 -> bv
        let s_9_41: Bits = Bits::new(s_9_40 as u128, 4u16);
        // D s_9_42: cast zx s_9_41 -> i
        let s_9_42: i128 = (s_9_41.value() as i128);
        // D s_9_43: cast reint s_9_42 -> i64
        let s_9_43: i64 = (s_9_42 as i64);
        // D s_9_44: write-var n <= s_9_43
        fn_state.n = s_9_43;
        // D s_9_45: read-var Rm:u8
        let s_9_45: u8 = fn_state.Rm;
        // D s_9_46: cast zx s_9_45 -> bv
        let s_9_46: Bits = Bits::new(s_9_45 as u128, 4u16);
        // D s_9_47: cast zx s_9_46 -> i
        let s_9_47: i128 = (s_9_46.value() as i128);
        // D s_9_48: cast reint s_9_47 -> i64
        let s_9_48: i64 = (s_9_47 as i64);
        // D s_9_49: write-var m <= s_9_48
        fn_state.m = s_9_48;
        // C s_9_50: const #15s : i
        let s_9_50: i128 = 15;
        // D s_9_51: read-var m:i64
        let s_9_51: i64 = fn_state.m;
        // D s_9_52: cast zx s_9_51 -> i
        let s_9_52: i128 = (i128::try_from(s_9_51).unwrap());
        // D s_9_53: call neq_int(s_9_52, s_9_50)
        let s_9_53: bool = neq_int(state, tracer, s_9_52, s_9_50);
        // D s_9_54: write-var wback <= s_9_53
        fn_state.wback = s_9_53;
        // C s_9_55: const #15s : i
        let s_9_55: i128 = 15;
        // D s_9_56: read-var m:i64
        let s_9_56: i64 = fn_state.m;
        // D s_9_57: cast zx s_9_56 -> i
        let s_9_57: i128 = (i128::try_from(s_9_56).unwrap());
        // D s_9_58: call neq_int(s_9_57, s_9_55)
        let s_9_58: bool = neq_int(state, tracer, s_9_57, s_9_55);
        // N s_9_59: branch s_9_58 b80 b10
        if s_9_58 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#321727 <= s_10_0
        fn_state.gs_321727 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#321727:u8
        let s_11_0: bool = fn_state.gs_321727;
        // D s_11_1: write-var register_index <= s_11_0
        fn_state.register_index = s_11_0;
        // C s_11_2: const #15s : i
        let s_11_2: i128 = 15;
        // D s_11_3: read-var n:i64
        let s_11_3: i64 = fn_state.n;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: cmp-eq s_11_4 s_11_2
        let s_11_5: bool = ((s_11_4) == (s_11_2));
        // N s_11_6: branch s_11_5 b79 b12
        if s_11_5 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #31s : i
        let s_12_0: i128 = 31;
        // D s_12_1: read-var d4:i64
        let s_12_1: i64 = fn_state.d4;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cmp-gt s_12_2 s_12_0
        let s_12_3: bool = ((s_12_2) > (s_12_0));
        // D s_12_4: write-var gs#321730 <= s_12_3
        fn_state.gs_321730 = s_12_3;
        // N s_12_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#321730:u8
        let s_13_0: bool = fn_state.gs_321730;
        // N s_13_1: branch s_13_0 b78 b14
        if s_13_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var n:i64
        let s_14_0: i64 = fn_state.n;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call __id(s_14_1)
        let s_14_2: i128 = u__id(state, tracer, s_14_1);
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: const #0s : i
        let s_14_4: i128 = 0;
        // D s_14_5: cast zx s_14_3 -> i
        let s_14_5: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_6: cmp-le s_14_4 s_14_5
        let s_14_6: bool = ((s_14_4) <= (s_14_5));
        // N s_14_7: branch s_14_6 b17 b15
        if s_14_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#321773 <= s_15_0
        fn_state.gs_321773 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#321773:u8
        let s_16_0: bool = fn_state.gs_321773;
        // N s_16_1: assert s_16_0
        let s_16_1: () = assert!(s_16_0);
        // D s_16_2: read-var alignment:i
        let s_16_2: i128 = fn_state.alignment;
        // D s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // D s_16_4: read-var d2:i64
        let s_16_4: i64 = fn_state.d2;
        // D s_16_5: cast zx s_16_4 -> i
        let s_16_5: i128 = (i128::try_from(s_16_4).unwrap());
        // D s_16_6: read-var d3:i64
        let s_16_6: i64 = fn_state.d3;
        // D s_16_7: cast zx s_16_6 -> i
        let s_16_7: i128 = (i128::try_from(s_16_6).unwrap());
        // D s_16_8: read-var d4:i64
        let s_16_8: i64 = fn_state.d4;
        // D s_16_9: cast zx s_16_8 -> i
        let s_16_9: i128 = (i128::try_from(s_16_8).unwrap());
        // D s_16_10: read-var d:i64
        let s_16_10: i64 = fn_state.d;
        // C s_16_11: const #4s : i64
        let s_16_11: i64 = 4;
        // D s_16_12: read-var index:i64
        let s_16_12: i64 = fn_state.index;
        // D s_16_13: read-var m:i64
        let s_16_13: i64 = fn_state.m;
        // D s_16_14: read-var n:i64
        let s_16_14: i64 = fn_state.n;
        // D s_16_15: read-var register_index:u8
        let s_16_15: bool = fn_state.register_index;
        // D s_16_16: read-var wback:u8
        let s_16_16: bool = fn_state.wback;
        // D s_16_17: call execute_aarch32_instrs_VST4_1_Op_A_txt(s_16_3, s_16_10, s_16_5, s_16_7, s_16_9, s_16_11, s_16_12, s_16_13, s_16_14, s_16_15, s_16_16)
        let s_16_17: () = execute_aarch32_instrs_VST4_1_Op_A_txt(
            state,
            tracer,
            s_16_3,
            s_16_10,
            s_16_5,
            s_16_7,
            s_16_9,
            s_16_11,
            s_16_12,
            s_16_13,
            s_16_14,
            s_16_15,
            s_16_16,
        );
        // N s_16_18: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var n:i64
        let s_17_0: i64 = fn_state.n;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: call __id(s_17_1)
        let s_17_2: i128 = u__id(state, tracer, s_17_1);
        // D s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // C s_17_4: const #15s : i
        let s_17_4: i128 = 15;
        // D s_17_5: cast zx s_17_3 -> i
        let s_17_5: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_6: cmp-le s_17_5 s_17_4
        let s_17_6: bool = ((s_17_5) <= (s_17_4));
        // N s_17_7: branch s_17_6 b20 b18
        if s_17_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#321772 <= s_18_0
        fn_state.gs_321772 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#321772:u8
        let s_19_0: bool = fn_state.gs_321772;
        // D s_19_1: write-var gs#321773 <= s_19_0
        fn_state.gs_321773 = s_19_0;
        // N s_19_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var m:i64
        let s_20_0: i64 = fn_state.m;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: call __id(s_20_1)
        let s_20_2: i128 = u__id(state, tracer, s_20_1);
        // D s_20_3: cast reint s_20_2 -> i64
        let s_20_3: i64 = (s_20_2 as i64);
        // C s_20_4: const #0s : i
        let s_20_4: i128 = 0;
        // D s_20_5: cast zx s_20_3 -> i
        let s_20_5: i128 = (i128::try_from(s_20_3).unwrap());
        // D s_20_6: cmp-le s_20_4 s_20_5
        let s_20_6: bool = ((s_20_4) <= (s_20_5));
        // N s_20_7: branch s_20_6 b23 b21
        if s_20_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#321771 <= s_21_0
        fn_state.gs_321771 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#321771:u8
        let s_22_0: bool = fn_state.gs_321771;
        // D s_22_1: write-var gs#321772 <= s_22_0
        fn_state.gs_321772 = s_22_0;
        // N s_22_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var m:i64
        let s_23_0: i64 = fn_state.m;
        // D s_23_1: cast zx s_23_0 -> i
        let s_23_1: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_2: call __id(s_23_1)
        let s_23_2: i128 = u__id(state, tracer, s_23_1);
        // D s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // C s_23_4: const #15s : i
        let s_23_4: i128 = 15;
        // D s_23_5: cast zx s_23_3 -> i
        let s_23_5: i128 = (i128::try_from(s_23_3).unwrap());
        // D s_23_6: cmp-le s_23_5 s_23_4
        let s_23_6: bool = ((s_23_5) <= (s_23_4));
        // N s_23_7: branch s_23_6 b26 b24
        if s_23_6 {
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
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#321770 <= s_24_0
        fn_state.gs_321770 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#321770:u8
        let s_25_0: bool = fn_state.gs_321770;
        // D s_25_1: write-var gs#321771 <= s_25_0
        fn_state.gs_321771 = s_25_0;
        // N s_25_2: jump b22
        return block_22(state, tracer, fn_state);
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
        // C s_26_4: const #0s : i
        let s_26_4: i128 = 0;
        // D s_26_5: cast zx s_26_3 -> i
        let s_26_5: i128 = (i128::try_from(s_26_3).unwrap());
        // D s_26_6: cmp-eq s_26_5 s_26_4
        let s_26_6: bool = ((s_26_5) == (s_26_4));
        // N s_26_7: branch s_26_6 b77 b27
        if s_26_6 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var index:i64
        let s_27_0: i64 = fn_state.index;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: call __id(s_27_1)
        let s_27_2: i128 = u__id(state, tracer, s_27_1);
        // D s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: const #1s : i
        let s_27_4: i128 = 1;
        // D s_27_5: cast zx s_27_3 -> i
        let s_27_5: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_6: cmp-eq s_27_5 s_27_4
        let s_27_6: bool = ((s_27_5) == (s_27_4));
        // D s_27_7: write-var gs#321737 <= s_27_6
        fn_state.gs_321737 = s_27_6;
        // N s_27_8: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#321737:u8
        let s_28_0: bool = fn_state.gs_321737;
        // N s_28_1: branch s_28_0 b76 b29
        if s_28_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var index:i64
        let s_29_0: i64 = fn_state.index;
        // D s_29_1: cast zx s_29_0 -> i
        let s_29_1: i128 = (i128::try_from(s_29_0).unwrap());
        // D s_29_2: call __id(s_29_1)
        let s_29_2: i128 = u__id(state, tracer, s_29_1);
        // D s_29_3: cast reint s_29_2 -> i64
        let s_29_3: i64 = (s_29_2 as i64);
        // C s_29_4: const #2s : i
        let s_29_4: i128 = 2;
        // D s_29_5: cast zx s_29_3 -> i
        let s_29_5: i128 = (i128::try_from(s_29_3).unwrap());
        // D s_29_6: cmp-eq s_29_5 s_29_4
        let s_29_6: bool = ((s_29_5) == (s_29_4));
        // D s_29_7: write-var gs#321739 <= s_29_6
        fn_state.gs_321739 = s_29_6;
        // N s_29_8: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#321739:u8
        let s_30_0: bool = fn_state.gs_321739;
        // N s_30_1: branch s_30_0 b75 b31
        if s_30_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var index:i64
        let s_31_0: i64 = fn_state.index;
        // D s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: call __id(s_31_1)
        let s_31_2: i128 = u__id(state, tracer, s_31_1);
        // D s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: const #3s : i
        let s_31_4: i128 = 3;
        // D s_31_5: cast zx s_31_3 -> i
        let s_31_5: i128 = (i128::try_from(s_31_3).unwrap());
        // D s_31_6: cmp-eq s_31_5 s_31_4
        let s_31_6: bool = ((s_31_5) == (s_31_4));
        // D s_31_7: write-var gs#321741 <= s_31_6
        fn_state.gs_321741 = s_31_6;
        // N s_31_8: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#321741:u8
        let s_32_0: bool = fn_state.gs_321741;
        // N s_32_1: branch s_32_0 b74 b33
        if s_32_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var index:i64
        let s_33_0: i64 = fn_state.index;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: call __id(s_33_1)
        let s_33_2: i128 = u__id(state, tracer, s_33_1);
        // D s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: const #4s : i
        let s_33_4: i128 = 4;
        // D s_33_5: cast zx s_33_3 -> i
        let s_33_5: i128 = (i128::try_from(s_33_3).unwrap());
        // D s_33_6: cmp-eq s_33_5 s_33_4
        let s_33_6: bool = ((s_33_5) == (s_33_4));
        // D s_33_7: write-var gs#321743 <= s_33_6
        fn_state.gs_321743 = s_33_6;
        // N s_33_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#321743:u8
        let s_34_0: bool = fn_state.gs_321743;
        // N s_34_1: branch s_34_0 b73 b35
        if s_34_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var index:i64
        let s_35_0: i64 = fn_state.index;
        // D s_35_1: cast zx s_35_0 -> i
        let s_35_1: i128 = (i128::try_from(s_35_0).unwrap());
        // D s_35_2: call __id(s_35_1)
        let s_35_2: i128 = u__id(state, tracer, s_35_1);
        // D s_35_3: cast reint s_35_2 -> i64
        let s_35_3: i64 = (s_35_2 as i64);
        // C s_35_4: const #5s : i
        let s_35_4: i128 = 5;
        // D s_35_5: cast zx s_35_3 -> i
        let s_35_5: i128 = (i128::try_from(s_35_3).unwrap());
        // D s_35_6: cmp-eq s_35_5 s_35_4
        let s_35_6: bool = ((s_35_5) == (s_35_4));
        // D s_35_7: write-var gs#321745 <= s_35_6
        fn_state.gs_321745 = s_35_6;
        // N s_35_8: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#321745:u8
        let s_36_0: bool = fn_state.gs_321745;
        // N s_36_1: branch s_36_0 b72 b37
        if s_36_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var index:i64
        let s_37_0: i64 = fn_state.index;
        // D s_37_1: cast zx s_37_0 -> i
        let s_37_1: i128 = (i128::try_from(s_37_0).unwrap());
        // D s_37_2: call __id(s_37_1)
        let s_37_2: i128 = u__id(state, tracer, s_37_1);
        // D s_37_3: cast reint s_37_2 -> i64
        let s_37_3: i64 = (s_37_2 as i64);
        // C s_37_4: const #6s : i
        let s_37_4: i128 = 6;
        // D s_37_5: cast zx s_37_3 -> i
        let s_37_5: i128 = (i128::try_from(s_37_3).unwrap());
        // D s_37_6: cmp-eq s_37_5 s_37_4
        let s_37_6: bool = ((s_37_5) == (s_37_4));
        // D s_37_7: write-var gs#321747 <= s_37_6
        fn_state.gs_321747 = s_37_6;
        // N s_37_8: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#321747:u8
        let s_38_0: bool = fn_state.gs_321747;
        // N s_38_1: branch s_38_0 b71 b39
        if s_38_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var index:i64
        let s_39_0: i64 = fn_state.index;
        // D s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_2: call __id(s_39_1)
        let s_39_2: i128 = u__id(state, tracer, s_39_1);
        // D s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: const #7s : i
        let s_39_4: i128 = 7;
        // D s_39_5: cast zx s_39_3 -> i
        let s_39_5: i128 = (i128::try_from(s_39_3).unwrap());
        // D s_39_6: cmp-eq s_39_5 s_39_4
        let s_39_6: bool = ((s_39_5) == (s_39_4));
        // D s_39_7: write-var gs#321749 <= s_39_6
        fn_state.gs_321749 = s_39_6;
        // N s_39_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#321749:u8
        let s_40_0: bool = fn_state.gs_321749;
        // N s_40_1: branch s_40_0 b43 b41
        if s_40_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#321769 <= s_41_0
        fn_state.gs_321769 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#321769:u8
        let s_42_0: bool = fn_state.gs_321769;
        // D s_42_1: write-var gs#321770 <= s_42_0
        fn_state.gs_321770 = s_42_0;
        // N s_42_2: jump b25
        return block_25(state, tracer, fn_state);
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
        // C s_43_4: const #1s : i
        let s_43_4: i128 = 1;
        // S s_43_5: cast zx s_43_3 -> i
        let s_43_5: i128 = (i128::try_from(s_43_3).unwrap());
        // S s_43_6: cmp-eq s_43_5 s_43_4
        let s_43_6: bool = ((s_43_5) == (s_43_4));
        // N s_43_7: branch s_43_6 b70 b44
        if s_43_6 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #4s : i64
        let s_44_0: i64 = 4;
        // C s_44_1: cast zx s_44_0 -> i
        let s_44_1: i128 = (i128::try_from(s_44_0).unwrap());
        // S s_44_2: call __id(s_44_1)
        let s_44_2: i128 = u__id(state, tracer, s_44_1);
        // S s_44_3: cast reint s_44_2 -> i64
        let s_44_3: i64 = (s_44_2 as i64);
        // C s_44_4: const #2s : i
        let s_44_4: i128 = 2;
        // S s_44_5: cast zx s_44_3 -> i
        let s_44_5: i128 = (i128::try_from(s_44_3).unwrap());
        // S s_44_6: cmp-eq s_44_5 s_44_4
        let s_44_6: bool = ((s_44_5) == (s_44_4));
        // D s_44_7: write-var gs#321752 <= s_44_6
        fn_state.gs_321752 = s_44_6;
        // N s_44_8: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#321752:u8
        let s_45_0: bool = fn_state.gs_321752;
        // N s_45_1: branch s_45_0 b69 b46
        if s_45_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #4s : i64
        let s_46_0: i64 = 4;
        // C s_46_1: cast zx s_46_0 -> i
        let s_46_1: i128 = (i128::try_from(s_46_0).unwrap());
        // S s_46_2: call __id(s_46_1)
        let s_46_2: i128 = u__id(state, tracer, s_46_1);
        // S s_46_3: cast reint s_46_2 -> i64
        let s_46_3: i64 = (s_46_2 as i64);
        // C s_46_4: const #4s : i
        let s_46_4: i128 = 4;
        // S s_46_5: cast zx s_46_3 -> i
        let s_46_5: i128 = (i128::try_from(s_46_3).unwrap());
        // S s_46_6: cmp-eq s_46_5 s_46_4
        let s_46_6: bool = ((s_46_5) == (s_46_4));
        // D s_46_7: write-var gs#321754 <= s_46_6
        fn_state.gs_321754 = s_46_6;
        // N s_46_8: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#321754:u8
        let s_47_0: bool = fn_state.gs_321754;
        // N s_47_1: branch s_47_0 b50 b48
        if s_47_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#321768 <= s_48_0
        fn_state.gs_321768 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#321768:u8
        let s_49_0: bool = fn_state.gs_321768;
        // D s_49_1: write-var gs#321769 <= s_49_0
        fn_state.gs_321769 = s_49_0;
        // N s_49_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var d:i64
        let s_50_0: i64 = fn_state.d;
        // D s_50_1: cast zx s_50_0 -> i
        let s_50_1: i128 = (i128::try_from(s_50_0).unwrap());
        // D s_50_2: call __id(s_50_1)
        let s_50_2: i128 = u__id(state, tracer, s_50_1);
        // D s_50_3: cast reint s_50_2 -> i64
        let s_50_3: i64 = (s_50_2 as i64);
        // C s_50_4: const #0s : i
        let s_50_4: i128 = 0;
        // D s_50_5: cast zx s_50_3 -> i
        let s_50_5: i128 = (i128::try_from(s_50_3).unwrap());
        // D s_50_6: cmp-le s_50_4 s_50_5
        let s_50_6: bool = ((s_50_4) <= (s_50_5));
        // N s_50_7: branch s_50_6 b53 b51
        if s_50_6 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#321767 <= s_51_0
        fn_state.gs_321767 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#321767:u8
        let s_52_0: bool = fn_state.gs_321767;
        // D s_52_1: write-var gs#321768 <= s_52_0
        fn_state.gs_321768 = s_52_0;
        // N s_52_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var d:i64
        let s_53_0: i64 = fn_state.d;
        // D s_53_1: cast zx s_53_0 -> i
        let s_53_1: i128 = (i128::try_from(s_53_0).unwrap());
        // D s_53_2: call __id(s_53_1)
        let s_53_2: i128 = u__id(state, tracer, s_53_1);
        // D s_53_3: cast reint s_53_2 -> i64
        let s_53_3: i64 = (s_53_2 as i64);
        // C s_53_4: const #31s : i
        let s_53_4: i128 = 31;
        // D s_53_5: cast zx s_53_3 -> i
        let s_53_5: i128 = (i128::try_from(s_53_3).unwrap());
        // D s_53_6: cmp-le s_53_5 s_53_4
        let s_53_6: bool = ((s_53_5) <= (s_53_4));
        // N s_53_7: branch s_53_6 b56 b54
        if s_53_6 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#321766 <= s_54_0
        fn_state.gs_321766 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#321766:u8
        let s_55_0: bool = fn_state.gs_321766;
        // D s_55_1: write-var gs#321767 <= s_55_0
        fn_state.gs_321767 = s_55_0;
        // N s_55_2: jump b52
        return block_52(state, tracer, fn_state);
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
        // C s_56_2: const #1s : i
        let s_56_2: i128 = 1;
        // D s_56_3: cmp-eq s_56_1 s_56_2
        let s_56_3: bool = ((s_56_1) == (s_56_2));
        // N s_56_4: branch s_56_3 b68 b57
        if s_56_3 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var alignment:i
        let s_57_0: i128 = fn_state.alignment;
        // D s_57_1: call __id(s_57_0)
        let s_57_1: i128 = u__id(state, tracer, s_57_0);
        // C s_57_2: const #4s : i
        let s_57_2: i128 = 4;
        // D s_57_3: cmp-eq s_57_1 s_57_2
        let s_57_3: bool = ((s_57_1) == (s_57_2));
        // D s_57_4: write-var gs#321759 <= s_57_3
        fn_state.gs_321759 = s_57_3;
        // N s_57_5: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#321759:u8
        let s_58_0: bool = fn_state.gs_321759;
        // N s_58_1: branch s_58_0 b67 b59
        if s_58_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var alignment:i
        let s_59_0: i128 = fn_state.alignment;
        // D s_59_1: call __id(s_59_0)
        let s_59_1: i128 = u__id(state, tracer, s_59_0);
        // C s_59_2: const #8s : i
        let s_59_2: i128 = 8;
        // D s_59_3: cmp-eq s_59_1 s_59_2
        let s_59_3: bool = ((s_59_1) == (s_59_2));
        // D s_59_4: write-var gs#321761 <= s_59_3
        fn_state.gs_321761 = s_59_3;
        // N s_59_5: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#321761:u8
        let s_60_0: bool = fn_state.gs_321761;
        // N s_60_1: branch s_60_0 b66 b61
        if s_60_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var alignment:i
        let s_61_0: i128 = fn_state.alignment;
        // D s_61_1: call __id(s_61_0)
        let s_61_1: i128 = u__id(state, tracer, s_61_0);
        // C s_61_2: const #16s : i
        let s_61_2: i128 = 16;
        // D s_61_3: cmp-eq s_61_1 s_61_2
        let s_61_3: bool = ((s_61_1) == (s_61_2));
        // D s_61_4: write-var gs#321763 <= s_61_3
        fn_state.gs_321763 = s_61_3;
        // N s_61_5: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#321763:u8
        let s_62_0: bool = fn_state.gs_321763;
        // N s_62_1: branch s_62_0 b65 b63
        if s_62_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var alignment:i
        let s_63_0: i128 = fn_state.alignment;
        // D s_63_1: call __id(s_63_0)
        let s_63_1: i128 = u__id(state, tracer, s_63_0);
        // C s_63_2: const #32s : i
        let s_63_2: i128 = 32;
        // D s_63_3: cmp-eq s_63_1 s_63_2
        let s_63_3: bool = ((s_63_1) == (s_63_2));
        // D s_63_4: write-var gs#321765 <= s_63_3
        fn_state.gs_321765 = s_63_3;
        // N s_63_5: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#321765:u8
        let s_64_0: bool = fn_state.gs_321765;
        // D s_64_1: write-var gs#321766 <= s_64_0
        fn_state.gs_321766 = s_64_0;
        // N s_64_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#321765 <= s_65_0
        fn_state.gs_321765 = s_65_0;
        // N s_65_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // D s_66_1: write-var gs#321763 <= s_66_0
        fn_state.gs_321763 = s_66_0;
        // N s_66_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#321761 <= s_67_0
        fn_state.gs_321761 = s_67_0;
        // N s_67_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#321759 <= s_68_0
        fn_state.gs_321759 = s_68_0;
        // N s_68_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // D s_69_1: write-var gs#321754 <= s_69_0
        fn_state.gs_321754 = s_69_0;
        // N s_69_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var gs#321752 <= s_70_0
        fn_state.gs_321752 = s_70_0;
        // N s_70_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var gs#321749 <= s_71_0
        fn_state.gs_321749 = s_71_0;
        // N s_71_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#321747 <= s_72_0
        fn_state.gs_321747 = s_72_0;
        // N s_72_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #1u : u8
        let s_73_0: bool = true;
        // D s_73_1: write-var gs#321745 <= s_73_0
        fn_state.gs_321745 = s_73_0;
        // N s_73_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #1u : u8
        let s_74_0: bool = true;
        // D s_74_1: write-var gs#321743 <= s_74_0
        fn_state.gs_321743 = s_74_0;
        // N s_74_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#321741 <= s_75_0
        fn_state.gs_321741 = s_75_0;
        // N s_75_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #1u : u8
        let s_76_0: bool = true;
        // D s_76_1: write-var gs#321739 <= s_76_0
        fn_state.gs_321739 = s_76_0;
        // N s_76_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #1u : u8
        let s_77_0: bool = true;
        // D s_77_1: write-var gs#321737 <= s_77_0
        fn_state.gs_321737 = s_77_0;
        // N s_77_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_78_0: panic
        panic!("{:?}", ());
        // N s_78_1: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #1u : u8
        let s_79_0: bool = true;
        // D s_79_1: write-var gs#321730 <= s_79_0
        fn_state.gs_321730 = s_79_0;
        // N s_79_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #13s : i
        let s_80_0: i128 = 13;
        // D s_80_1: read-var m:i64
        let s_80_1: i64 = fn_state.m;
        // D s_80_2: cast zx s_80_1 -> i
        let s_80_2: i128 = (i128::try_from(s_80_1).unwrap());
        // D s_80_3: call neq_int(s_80_2, s_80_0)
        let s_80_3: bool = neq_int(state, tracer, s_80_2, s_80_0);
        // D s_80_4: write-var gs#321727 <= s_80_3
        fn_state.gs_321727 = s_80_3;
        // N s_80_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #1s : i
        let s_81_0: i128 = 1;
        // D s_81_1: write-var ga#362579 <= s_81_0
        fn_state.ga_362579 = s_81_0;
        // N s_81_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #1s : i64
        let s_82_0: i64 = 1;
        // D s_82_1: write-var ga#362574 <= s_82_0
        fn_state.ga_362574 = s_82_0;
        // N s_82_2: jump b7
        return block_7(state, tracer, fn_state);
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
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_84_0: panic
        panic!("{:?}", ());
        // N s_84_1: return
        return;
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_85_0: panic
        panic!("{:?}", ());
        // N s_85_1: return
        return;
    }
}
