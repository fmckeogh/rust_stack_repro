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
use execute_aarch32_instrs_VLD3_m_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VLD3_m_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Rn: u8,
    Vd: u8,
    itype: u8,
    size: u8,
    align: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        ebytes: i64,
        n: i64,
        inc_nameshadow_7481: i64,
        gs_311011: bool,
        gs_311008: bool,
        d2: i64,
        inc_name: i64,
        d: i64,
        elements: i64,
        alignment: i64,
        register_index: bool,
        wback: bool,
        d3: i64,
        gs_310989: bool,
        ga_354553: i64,
        D: bool,
        Rn: u8,
        Vd: u8,
        itype: u8,
        size: u8,
        align: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        D,
        Rn,
        Vd,
        itype,
        size,
        align,
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
        // C s_2_0: const #1s : i64
        let s_2_0: i64 = 1;
        // D s_2_1: write-var inc_name <= s_2_0
        fn_state.inc_name = s_2_0;
        // D s_2_2: read-var itype:u8
        let s_2_2: u8 = fn_state.itype;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // C s_2_4: const #4u : u8
        let s_2_4: u8 = 4;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 4u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // D s_2_7: not s_2_6
        let s_2_7: bool = !s_2_6;
        // N s_2_8: branch s_2_7 b21 b3
        if s_2_7 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1s : i64
        let s_3_0: i64 = 1;
        // D s_3_1: write-var inc_name <= s_3_0
        fn_state.inc_name = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var inc_name:i64
        let s_4_0: i64 = fn_state.inc_name;
        // D s_4_1: write-var inc_nameshadow#7481 <= s_4_0
        fn_state.inc_nameshadow_7481 = s_4_0;
        // D s_4_2: read-var size:u8
        let s_4_2: u8 = fn_state.size;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // C s_4_4: const #3u : u8
        let s_4_4: u8 = 3;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 2u16);
        // D s_4_6: cmp-eq s_4_3 s_4_5
        let s_4_6: bool = ((s_4_3) == (s_4_5));
        // N s_4_7: branch s_4_6 b20 b5
        if s_4_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1s : i
        let s_5_0: i128 = 1;
        // D s_5_1: read-var align:u8
        let s_5_1: u8 = fn_state.align;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 2u16);
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
        // C s_5_19: const #1u : u8
        let s_5_19: bool = true;
        // C s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 1u16);
        // D s_5_21: cmp-eq s_5_18 s_5_20
        let s_5_21: bool = ((s_5_18) == (s_5_20));
        // D s_5_22: write-var gs#310989 <= s_5_21
        fn_state.gs_310989 = s_5_21;
        // N s_5_23: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#310989:u8
        let s_6_0: bool = fn_state.gs_310989;
        // N s_6_1: branch s_6_0 b19 b7
        if s_6_0 {
            return block_19(state, tracer, fn_state);
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
        // D s_7_1: read-var align:u8
        let s_7_1: u8 = fn_state.align;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 2u16);
        // C s_7_3: const #1u : u64
        let s_7_3: u64 = 1;
        // D s_7_4: bit-extract s_7_2 s_7_0 s_7_3
        let s_7_4: Bits = (Bits::new(
            ((s_7_2) >> (s_7_0)).value(),
            u16::try_from(s_7_3).unwrap(),
        ));
        // D s_7_5: cast reint s_7_4 -> u8
        let s_7_5: bool = ((s_7_4.value()) != 0);
        // C s_7_6: const #0s : i
        let s_7_6: i128 = 0;
        // C s_7_7: const #0u : u64
        let s_7_7: u64 = 0;
        // D s_7_8: cast zx s_7_5 -> u64
        let s_7_8: u64 = (s_7_5 as u64);
        // C s_7_9: const #1u : u64
        let s_7_9: u64 = 1;
        // D s_7_10: and s_7_8 s_7_9
        let s_7_10: u64 = ((s_7_8) & (s_7_9));
        // D s_7_11: cmp-eq s_7_10 s_7_9
        let s_7_11: bool = ((s_7_10) == (s_7_9));
        // D s_7_12: lsl s_7_8 s_7_6
        let s_7_12: u64 = s_7_8 << s_7_6;
        // D s_7_13: or s_7_7 s_7_12
        let s_7_13: u64 = ((s_7_7) | (s_7_12));
        // D s_7_14: cmpl s_7_12
        let s_7_14: u64 = !s_7_12;
        // D s_7_15: and s_7_7 s_7_14
        let s_7_15: u64 = ((s_7_7) & (s_7_14));
        // D s_7_16: select s_7_11 s_7_13 s_7_15
        let s_7_16: u64 = if s_7_11 { s_7_13 } else { s_7_15 };
        // D s_7_17: cast trunc s_7_16 -> u8
        let s_7_17: bool = ((s_7_16) != 0);
        // D s_7_18: cast zx s_7_17 -> bv
        let s_7_18: Bits = Bits::new(s_7_17 as u128, 1u16);
        // C s_7_19: const #0u : u8
        let s_7_19: bool = false;
        // C s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 1u16);
        // D s_7_21: cmp-eq s_7_18 s_7_20
        let s_7_21: bool = ((s_7_18) == (s_7_20));
        // N s_7_22: branch s_7_21 b18 b8
        if s_7_21 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #8s : i64
        let s_8_0: i64 = 8;
        // D s_8_1: write-var ga#354553 <= s_8_0
        fn_state.ga_354553 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#354553:i64
        let s_9_0: i64 = fn_state.ga_354553;
        // D s_9_1: write-var alignment <= s_9_0
        fn_state.alignment = s_9_0;
        // D s_9_2: read-var size:u8
        let s_9_2: u8 = fn_state.size;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 2u16);
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (s_9_3.value() as i128);
        // D s_9_5: cast reint s_9_4 -> i64
        let s_9_5: i64 = (s_9_4 as i64);
        // C s_9_6: const #1s : i64
        let s_9_6: i64 = 1;
        // D s_9_7: lsl s_9_6 s_9_5
        let s_9_7: i64 = s_9_6 << s_9_5;
        // D s_9_8: write-var ebytes <= s_9_7
        fn_state.ebytes = s_9_7;
        // C s_9_9: const #8s : i
        let s_9_9: i128 = 8;
        // D s_9_10: read-var ebytes:i64
        let s_9_10: i64 = fn_state.ebytes;
        // D s_9_11: cast zx s_9_10 -> i
        let s_9_11: i128 = (i128::try_from(s_9_10).unwrap());
        // D s_9_12: div s_9_9 s_9_11
        let s_9_12: i128 = ((s_9_9) / (s_9_11));
        // D s_9_13: cast reint s_9_12 -> i64
        let s_9_13: i64 = (s_9_12 as i64);
        // D s_9_14: write-var elements <= s_9_13
        fn_state.elements = s_9_13;
        // D s_9_15: read-var D:u8
        let s_9_15: bool = fn_state.D;
        // D s_9_16: cast zx s_9_15 -> bv
        let s_9_16: Bits = Bits::new(s_9_15 as u128, 1u16);
        // D s_9_17: read-var Vd:u8
        let s_9_17: u8 = fn_state.Vd;
        // D s_9_18: cast zx s_9_17 -> bv
        let s_9_18: Bits = Bits::new(s_9_17 as u128, 4u16);
        // D s_9_19: cast reint s_9_16 -> u128
        let s_9_19: u128 = (s_9_16.value() as u128);
        // D s_9_20: size-of s_9_16
        let s_9_20: u16 = s_9_16.length();
        // D s_9_21: cast reint s_9_18 -> u128
        let s_9_21: u128 = (s_9_18.value() as u128);
        // D s_9_22: size-of s_9_18
        let s_9_22: u16 = s_9_18.length();
        // D s_9_23: lsl s_9_19 s_9_22
        let s_9_23: u128 = s_9_19 << s_9_22;
        // D s_9_24: or s_9_23 s_9_21
        let s_9_24: u128 = ((s_9_23) | (s_9_21));
        // D s_9_25: add s_9_20 s_9_22
        let s_9_25: u16 = (s_9_20 + s_9_22);
        // D s_9_26: create-bits s_9_24 s_9_25
        let s_9_26: Bits = Bits::new(s_9_24, s_9_25);
        // D s_9_27: cast reint s_9_26 -> u8
        let s_9_27: u8 = (s_9_26.value() as u8);
        // D s_9_28: cast zx s_9_27 -> bv
        let s_9_28: Bits = Bits::new(s_9_27 as u128, 5u16);
        // D s_9_29: cast zx s_9_28 -> i
        let s_9_29: i128 = (s_9_28.value() as i128);
        // D s_9_30: cast reint s_9_29 -> i64
        let s_9_30: i64 = (s_9_29 as i64);
        // D s_9_31: write-var d <= s_9_30
        fn_state.d = s_9_30;
        // D s_9_32: read-var d:i64
        let s_9_32: i64 = fn_state.d;
        // D s_9_33: cast zx s_9_32 -> i
        let s_9_33: i128 = (i128::try_from(s_9_32).unwrap());
        // D s_9_34: read-var inc_nameshadow#7481:i64
        let s_9_34: i64 = fn_state.inc_nameshadow_7481;
        // D s_9_35: cast zx s_9_34 -> i
        let s_9_35: i128 = (i128::try_from(s_9_34).unwrap());
        // D s_9_36: add s_9_33 s_9_35
        let s_9_36: i128 = (s_9_33 + s_9_35);
        // D s_9_37: cast reint s_9_36 -> i64
        let s_9_37: i64 = (s_9_36 as i64);
        // D s_9_38: write-var d2 <= s_9_37
        fn_state.d2 = s_9_37;
        // D s_9_39: read-var d2:i64
        let s_9_39: i64 = fn_state.d2;
        // D s_9_40: cast zx s_9_39 -> i
        let s_9_40: i128 = (i128::try_from(s_9_39).unwrap());
        // D s_9_41: read-var inc_nameshadow#7481:i64
        let s_9_41: i64 = fn_state.inc_nameshadow_7481;
        // D s_9_42: cast zx s_9_41 -> i
        let s_9_42: i128 = (i128::try_from(s_9_41).unwrap());
        // D s_9_43: add s_9_40 s_9_42
        let s_9_43: i128 = (s_9_40 + s_9_42);
        // D s_9_44: cast reint s_9_43 -> i64
        let s_9_44: i64 = (s_9_43 as i64);
        // D s_9_45: write-var d3 <= s_9_44
        fn_state.d3 = s_9_44;
        // D s_9_46: read-var Rn:u8
        let s_9_46: u8 = fn_state.Rn;
        // D s_9_47: cast zx s_9_46 -> bv
        let s_9_47: Bits = Bits::new(s_9_46 as u128, 4u16);
        // D s_9_48: cast zx s_9_47 -> i
        let s_9_48: i128 = (s_9_47.value() as i128);
        // D s_9_49: cast reint s_9_48 -> i64
        let s_9_49: i64 = (s_9_48 as i64);
        // D s_9_50: write-var n <= s_9_49
        fn_state.n = s_9_49;
        // D s_9_51: read-var Rm:u8
        let s_9_51: u8 = fn_state.Rm;
        // D s_9_52: cast zx s_9_51 -> bv
        let s_9_52: Bits = Bits::new(s_9_51 as u128, 4u16);
        // D s_9_53: cast zx s_9_52 -> i
        let s_9_53: i128 = (s_9_52.value() as i128);
        // D s_9_54: cast reint s_9_53 -> i64
        let s_9_54: i64 = (s_9_53 as i64);
        // D s_9_55: write-var m <= s_9_54
        fn_state.m = s_9_54;
        // C s_9_56: const #15s : i
        let s_9_56: i128 = 15;
        // D s_9_57: read-var m:i64
        let s_9_57: i64 = fn_state.m;
        // D s_9_58: cast zx s_9_57 -> i
        let s_9_58: i128 = (i128::try_from(s_9_57).unwrap());
        // D s_9_59: call neq_int(s_9_58, s_9_56)
        let s_9_59: bool = neq_int(state, tracer, s_9_58, s_9_56);
        // D s_9_60: write-var wback <= s_9_59
        fn_state.wback = s_9_59;
        // C s_9_61: const #15s : i
        let s_9_61: i128 = 15;
        // D s_9_62: read-var m:i64
        let s_9_62: i64 = fn_state.m;
        // D s_9_63: cast zx s_9_62 -> i
        let s_9_63: i128 = (i128::try_from(s_9_62).unwrap());
        // D s_9_64: call neq_int(s_9_63, s_9_61)
        let s_9_64: bool = neq_int(state, tracer, s_9_63, s_9_61);
        // N s_9_65: branch s_9_64 b17 b10
        if s_9_64 {
            return block_17(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#311008 <= s_10_0
        fn_state.gs_311008 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#311008:u8
        let s_11_0: bool = fn_state.gs_311008;
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
        // N s_11_6: branch s_11_5 b16 b12
        if s_11_5 {
            return block_16(state, tracer, fn_state);
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
        // D s_12_1: read-var d3:i64
        let s_12_1: i64 = fn_state.d3;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cmp-gt s_12_2 s_12_0
        let s_12_3: bool = ((s_12_2) > (s_12_0));
        // D s_12_4: write-var gs#311011 <= s_12_3
        fn_state.gs_311011 = s_12_3;
        // N s_12_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#311011:u8
        let s_13_0: bool = fn_state.gs_311011;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
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
        // D s_14_0: read-var d2:i64
        let s_14_0: i64 = fn_state.d2;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: read-var d3:i64
        let s_14_2: i64 = fn_state.d3;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: read-var elements:i64
        let s_14_4: i64 = fn_state.elements;
        // D s_14_5: cast zx s_14_4 -> i
        let s_14_5: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_6: read-var alignment:i64
        let s_14_6: i64 = fn_state.alignment;
        // D s_14_7: read-var d:i64
        let s_14_7: i64 = fn_state.d;
        // D s_14_8: read-var ebytes:i64
        let s_14_8: i64 = fn_state.ebytes;
        // D s_14_9: read-var m:i64
        let s_14_9: i64 = fn_state.m;
        // D s_14_10: read-var n:i64
        let s_14_10: i64 = fn_state.n;
        // D s_14_11: read-var register_index:u8
        let s_14_11: bool = fn_state.register_index;
        // D s_14_12: read-var wback:u8
        let s_14_12: bool = fn_state.wback;
        // D s_14_13: call execute_aarch32_instrs_VLD3_m_Op_A_txt(s_14_6, s_14_7, s_14_1, s_14_3, s_14_8, s_14_5, s_14_9, s_14_10, s_14_11, s_14_12)
        let s_14_13: () = execute_aarch32_instrs_VLD3_m_Op_A_txt(
            state,
            tracer,
            s_14_6,
            s_14_7,
            s_14_1,
            s_14_3,
            s_14_8,
            s_14_5,
            s_14_9,
            s_14_10,
            s_14_11,
            s_14_12,
        );
        // N s_14_14: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#311011 <= s_16_0
        fn_state.gs_311011 = s_16_0;
        // N s_16_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #13s : i
        let s_17_0: i128 = 13;
        // D s_17_1: read-var m:i64
        let s_17_1: i64 = fn_state.m;
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // D s_17_3: call neq_int(s_17_2, s_17_0)
        let s_17_3: bool = neq_int(state, tracer, s_17_2, s_17_0);
        // D s_17_4: write-var gs#311008 <= s_17_3
        fn_state.gs_311008 = s_17_3;
        // N s_17_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1s : i64
        let s_18_0: i64 = 1;
        // D s_18_1: write-var ga#354553 <= s_18_0
        fn_state.ga_354553 = s_18_0;
        // N s_18_2: jump b9
        return block_9(state, tracer, fn_state);
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
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#310989 <= s_20_0
        fn_state.gs_310989 = s_20_0;
        // N s_20_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var itype:u8
        let s_21_0: u8 = fn_state.itype;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 4u16);
        // C s_21_2: const #5u : u8
        let s_21_2: u8 = 5;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 4u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: not s_21_4
        let s_21_5: bool = !s_21_4;
        // N s_21_6: branch s_21_5 b23 b22
        if s_21_5 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #2s : i64
        let s_22_0: i64 = 2;
        // D s_22_1: write-var inc_name <= s_22_0
        fn_state.inc_name = s_22_0;
        // N s_22_2: jump b4
        return block_4(state, tracer, fn_state);
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
}
