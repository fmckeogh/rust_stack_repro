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
use execute_aarch32_instrs_VST4_m_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VST4_m_T1enc_A_txt<T: Tracer>(
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
        gs_322099: bool,
        m: i64,
        ebytes: i64,
        gs_322109: bool,
        gs_322110: bool,
        gs_322108: bool,
        gs_322080: bool,
        n: i64,
        inc_nameshadow_7854: i64,
        gs_322090: bool,
        gs_322094: bool,
        gs_322105: bool,
        d2: i64,
        gs_322101: bool,
        inc_name: i64,
        gs_322112: bool,
        d: i64,
        ga_362845: i128,
        gs_322092: bool,
        elements: i64,
        d4: i64,
        register_index: bool,
        alignment: i128,
        gs_322106: bool,
        wback: bool,
        d3: i64,
        gs_322103: bool,
        gs_322083: bool,
        gs_322111: bool,
        gs_322107: bool,
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
        // N s_2_5: branch s_2_4 b62 b3
        if s_2_4 {
            return block_62(state, tracer, fn_state);
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
        // D s_3_2: read-var itype:u8
        let s_3_2: u8 = fn_state.itype;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 4u16);
        // C s_3_4: const #0u : u8
        let s_3_4: u8 = 0;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 4u16);
        // D s_3_6: cmp-eq s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) == (s_3_5));
        // D s_3_7: not s_3_6
        let s_3_7: bool = !s_3_6;
        // N s_3_8: branch s_3_7 b59 b4
        if s_3_7 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1s : i64
        let s_4_0: i64 = 1;
        // D s_4_1: write-var inc_name <= s_4_0
        fn_state.inc_name = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var inc_name:i64
        let s_5_0: i64 = fn_state.inc_name;
        // D s_5_1: write-var inc_nameshadow#7854 <= s_5_0
        fn_state.inc_nameshadow_7854 = s_5_0;
        // D s_5_2: read-var align:u8
        let s_5_2: u8 = fn_state.align;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // C s_5_4: const #0u : u8
        let s_5_4: u8 = 0;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 2u16);
        // D s_5_6: cmp-eq s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) == (s_5_5));
        // N s_5_7: branch s_5_6 b58 b6
        if s_5_6 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var align:u8
        let s_6_0: u8 = fn_state.align;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (s_6_1.value() as i128);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #4s : i
        let s_6_4: i128 = 4;
        // D s_6_5: cast zx s_6_3 -> i
        let s_6_5: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_6: lsl s_6_4 s_6_5
        let s_6_6: i128 = s_6_4 << s_6_5;
        // D s_6_7: write-var ga#362845 <= s_6_6
        fn_state.ga_362845 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#362845:i
        let s_7_0: i128 = fn_state.ga_362845;
        // D s_7_1: write-var alignment <= s_7_0
        fn_state.alignment = s_7_0;
        // D s_7_2: read-var size:u8
        let s_7_2: u8 = fn_state.size;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 2u16);
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (s_7_3.value() as i128);
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // C s_7_6: const #1s : i64
        let s_7_6: i64 = 1;
        // D s_7_7: lsl s_7_6 s_7_5
        let s_7_7: i64 = s_7_6 << s_7_5;
        // D s_7_8: write-var ebytes <= s_7_7
        fn_state.ebytes = s_7_7;
        // C s_7_9: const #8s : i
        let s_7_9: i128 = 8;
        // D s_7_10: read-var ebytes:i64
        let s_7_10: i64 = fn_state.ebytes;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: div s_7_9 s_7_11
        let s_7_12: i128 = ((s_7_9) / (s_7_11));
        // D s_7_13: cast reint s_7_12 -> i64
        let s_7_13: i64 = (s_7_12 as i64);
        // D s_7_14: write-var elements <= s_7_13
        fn_state.elements = s_7_13;
        // D s_7_15: read-var D:u8
        let s_7_15: bool = fn_state.D;
        // D s_7_16: cast zx s_7_15 -> bv
        let s_7_16: Bits = Bits::new(s_7_15 as u128, 1u16);
        // D s_7_17: read-var Vd:u8
        let s_7_17: u8 = fn_state.Vd;
        // D s_7_18: cast zx s_7_17 -> bv
        let s_7_18: Bits = Bits::new(s_7_17 as u128, 4u16);
        // D s_7_19: cast reint s_7_16 -> u128
        let s_7_19: u128 = (s_7_16.value() as u128);
        // D s_7_20: size-of s_7_16
        let s_7_20: u16 = s_7_16.length();
        // D s_7_21: cast reint s_7_18 -> u128
        let s_7_21: u128 = (s_7_18.value() as u128);
        // D s_7_22: size-of s_7_18
        let s_7_22: u16 = s_7_18.length();
        // D s_7_23: lsl s_7_19 s_7_22
        let s_7_23: u128 = s_7_19 << s_7_22;
        // D s_7_24: or s_7_23 s_7_21
        let s_7_24: u128 = ((s_7_23) | (s_7_21));
        // D s_7_25: add s_7_20 s_7_22
        let s_7_25: u16 = (s_7_20 + s_7_22);
        // D s_7_26: create-bits s_7_24 s_7_25
        let s_7_26: Bits = Bits::new(s_7_24, s_7_25);
        // D s_7_27: cast reint s_7_26 -> u8
        let s_7_27: u8 = (s_7_26.value() as u8);
        // D s_7_28: cast zx s_7_27 -> bv
        let s_7_28: Bits = Bits::new(s_7_27 as u128, 5u16);
        // D s_7_29: cast zx s_7_28 -> i
        let s_7_29: i128 = (s_7_28.value() as i128);
        // D s_7_30: cast reint s_7_29 -> i64
        let s_7_30: i64 = (s_7_29 as i64);
        // D s_7_31: write-var d <= s_7_30
        fn_state.d = s_7_30;
        // D s_7_32: read-var d:i64
        let s_7_32: i64 = fn_state.d;
        // D s_7_33: cast zx s_7_32 -> i
        let s_7_33: i128 = (i128::try_from(s_7_32).unwrap());
        // D s_7_34: read-var inc_nameshadow#7854:i64
        let s_7_34: i64 = fn_state.inc_nameshadow_7854;
        // D s_7_35: cast zx s_7_34 -> i
        let s_7_35: i128 = (i128::try_from(s_7_34).unwrap());
        // D s_7_36: add s_7_33 s_7_35
        let s_7_36: i128 = (s_7_33 + s_7_35);
        // D s_7_37: cast reint s_7_36 -> i64
        let s_7_37: i64 = (s_7_36 as i64);
        // D s_7_38: write-var d2 <= s_7_37
        fn_state.d2 = s_7_37;
        // D s_7_39: read-var d2:i64
        let s_7_39: i64 = fn_state.d2;
        // D s_7_40: cast zx s_7_39 -> i
        let s_7_40: i128 = (i128::try_from(s_7_39).unwrap());
        // D s_7_41: read-var inc_nameshadow#7854:i64
        let s_7_41: i64 = fn_state.inc_nameshadow_7854;
        // D s_7_42: cast zx s_7_41 -> i
        let s_7_42: i128 = (i128::try_from(s_7_41).unwrap());
        // D s_7_43: add s_7_40 s_7_42
        let s_7_43: i128 = (s_7_40 + s_7_42);
        // D s_7_44: cast reint s_7_43 -> i64
        let s_7_44: i64 = (s_7_43 as i64);
        // D s_7_45: write-var d3 <= s_7_44
        fn_state.d3 = s_7_44;
        // D s_7_46: read-var d3:i64
        let s_7_46: i64 = fn_state.d3;
        // D s_7_47: cast zx s_7_46 -> i
        let s_7_47: i128 = (i128::try_from(s_7_46).unwrap());
        // D s_7_48: read-var inc_nameshadow#7854:i64
        let s_7_48: i64 = fn_state.inc_nameshadow_7854;
        // D s_7_49: cast zx s_7_48 -> i
        let s_7_49: i128 = (i128::try_from(s_7_48).unwrap());
        // D s_7_50: add s_7_47 s_7_49
        let s_7_50: i128 = (s_7_47 + s_7_49);
        // D s_7_51: cast reint s_7_50 -> i64
        let s_7_51: i64 = (s_7_50 as i64);
        // D s_7_52: write-var d4 <= s_7_51
        fn_state.d4 = s_7_51;
        // D s_7_53: read-var Rn:u8
        let s_7_53: u8 = fn_state.Rn;
        // D s_7_54: cast zx s_7_53 -> bv
        let s_7_54: Bits = Bits::new(s_7_53 as u128, 4u16);
        // D s_7_55: cast zx s_7_54 -> i
        let s_7_55: i128 = (s_7_54.value() as i128);
        // D s_7_56: cast reint s_7_55 -> i64
        let s_7_56: i64 = (s_7_55 as i64);
        // D s_7_57: write-var n <= s_7_56
        fn_state.n = s_7_56;
        // D s_7_58: read-var Rm:u8
        let s_7_58: u8 = fn_state.Rm;
        // D s_7_59: cast zx s_7_58 -> bv
        let s_7_59: Bits = Bits::new(s_7_58 as u128, 4u16);
        // D s_7_60: cast zx s_7_59 -> i
        let s_7_60: i128 = (s_7_59.value() as i128);
        // D s_7_61: cast reint s_7_60 -> i64
        let s_7_61: i64 = (s_7_60 as i64);
        // D s_7_62: write-var m <= s_7_61
        fn_state.m = s_7_61;
        // C s_7_63: const #15s : i
        let s_7_63: i128 = 15;
        // D s_7_64: read-var m:i64
        let s_7_64: i64 = fn_state.m;
        // D s_7_65: cast zx s_7_64 -> i
        let s_7_65: i128 = (i128::try_from(s_7_64).unwrap());
        // D s_7_66: call neq_int(s_7_65, s_7_63)
        let s_7_66: bool = neq_int(state, tracer, s_7_65, s_7_63);
        // D s_7_67: write-var wback <= s_7_66
        fn_state.wback = s_7_66;
        // C s_7_68: const #15s : i
        let s_7_68: i128 = 15;
        // D s_7_69: read-var m:i64
        let s_7_69: i64 = fn_state.m;
        // D s_7_70: cast zx s_7_69 -> i
        let s_7_70: i128 = (i128::try_from(s_7_69).unwrap());
        // D s_7_71: call neq_int(s_7_70, s_7_68)
        let s_7_71: bool = neq_int(state, tracer, s_7_70, s_7_68);
        // N s_7_72: branch s_7_71 b57 b8
        if s_7_71 {
            return block_57(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#322080 <= s_8_0
        fn_state.gs_322080 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#322080:u8
        let s_9_0: bool = fn_state.gs_322080;
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
        // N s_9_6: branch s_9_5 b56 b10
        if s_9_5 {
            return block_56(state, tracer, fn_state);
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
        // D s_10_1: read-var d4:i64
        let s_10_1: i64 = fn_state.d4;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cmp-gt s_10_2 s_10_0
        let s_10_3: bool = ((s_10_2) > (s_10_0));
        // D s_10_4: write-var gs#322083 <= s_10_3
        fn_state.gs_322083 = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#322083:u8
        let s_11_0: bool = fn_state.gs_322083;
        // N s_11_1: branch s_11_0 b55 b12
        if s_11_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var n:i64
        let s_12_0: i64 = fn_state.n;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: call __id(s_12_1)
        let s_12_2: i128 = u__id(state, tracer, s_12_1);
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // C s_12_4: const #0s : i
        let s_12_4: i128 = 0;
        // D s_12_5: cast zx s_12_3 -> i
        let s_12_5: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_6: cmp-le s_12_4 s_12_5
        let s_12_6: bool = ((s_12_4) <= (s_12_5));
        // N s_12_7: branch s_12_6 b15 b13
        if s_12_6 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#322112 <= s_13_0
        fn_state.gs_322112 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#322112:u8
        let s_14_0: bool = fn_state.gs_322112;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // D s_14_2: read-var alignment:i
        let s_14_2: i128 = fn_state.alignment;
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // D s_14_4: read-var d2:i64
        let s_14_4: i64 = fn_state.d2;
        // D s_14_5: cast zx s_14_4 -> i
        let s_14_5: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_6: read-var d3:i64
        let s_14_6: i64 = fn_state.d3;
        // D s_14_7: cast zx s_14_6 -> i
        let s_14_7: i128 = (i128::try_from(s_14_6).unwrap());
        // D s_14_8: read-var d4:i64
        let s_14_8: i64 = fn_state.d4;
        // D s_14_9: cast zx s_14_8 -> i
        let s_14_9: i128 = (i128::try_from(s_14_8).unwrap());
        // D s_14_10: read-var elements:i64
        let s_14_10: i64 = fn_state.elements;
        // D s_14_11: cast zx s_14_10 -> i
        let s_14_11: i128 = (i128::try_from(s_14_10).unwrap());
        // D s_14_12: read-var d:i64
        let s_14_12: i64 = fn_state.d;
        // D s_14_13: read-var ebytes:i64
        let s_14_13: i64 = fn_state.ebytes;
        // D s_14_14: read-var m:i64
        let s_14_14: i64 = fn_state.m;
        // D s_14_15: read-var n:i64
        let s_14_15: i64 = fn_state.n;
        // D s_14_16: read-var register_index:u8
        let s_14_16: bool = fn_state.register_index;
        // D s_14_17: read-var wback:u8
        let s_14_17: bool = fn_state.wback;
        // D s_14_18: call execute_aarch32_instrs_VST4_m_Op_A_txt(s_14_3, s_14_12, s_14_5, s_14_7, s_14_9, s_14_13, s_14_11, s_14_14, s_14_15, s_14_16, s_14_17)
        let s_14_18: () = execute_aarch32_instrs_VST4_m_Op_A_txt(
            state,
            tracer,
            s_14_3,
            s_14_12,
            s_14_5,
            s_14_7,
            s_14_9,
            s_14_13,
            s_14_11,
            s_14_14,
            s_14_15,
            s_14_16,
            s_14_17,
        );
        // N s_14_19: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var n:i64
        let s_15_0: i64 = fn_state.n;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call __id(s_15_1)
        let s_15_2: i128 = u__id(state, tracer, s_15_1);
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: const #15s : i
        let s_15_4: i128 = 15;
        // D s_15_5: cast zx s_15_3 -> i
        let s_15_5: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_6: cmp-le s_15_5 s_15_4
        let s_15_6: bool = ((s_15_5) <= (s_15_4));
        // N s_15_7: branch s_15_6 b18 b16
        if s_15_6 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#322111 <= s_16_0
        fn_state.gs_322111 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#322111:u8
        let s_17_0: bool = fn_state.gs_322111;
        // D s_17_1: write-var gs#322112 <= s_17_0
        fn_state.gs_322112 = s_17_0;
        // N s_17_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var m:i64
        let s_18_0: i64 = fn_state.m;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: call __id(s_18_1)
        let s_18_2: i128 = u__id(state, tracer, s_18_1);
        // D s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: const #0s : i
        let s_18_4: i128 = 0;
        // D s_18_5: cast zx s_18_3 -> i
        let s_18_5: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_6: cmp-le s_18_4 s_18_5
        let s_18_6: bool = ((s_18_4) <= (s_18_5));
        // N s_18_7: branch s_18_6 b21 b19
        if s_18_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#322110 <= s_19_0
        fn_state.gs_322110 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#322110:u8
        let s_20_0: bool = fn_state.gs_322110;
        // D s_20_1: write-var gs#322111 <= s_20_0
        fn_state.gs_322111 = s_20_0;
        // N s_20_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var m:i64
        let s_21_0: i64 = fn_state.m;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: call __id(s_21_1)
        let s_21_2: i128 = u__id(state, tracer, s_21_1);
        // D s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // C s_21_4: const #15s : i
        let s_21_4: i128 = 15;
        // D s_21_5: cast zx s_21_3 -> i
        let s_21_5: i128 = (i128::try_from(s_21_3).unwrap());
        // D s_21_6: cmp-le s_21_5 s_21_4
        let s_21_6: bool = ((s_21_5) <= (s_21_4));
        // N s_21_7: branch s_21_6 b24 b22
        if s_21_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#322109 <= s_22_0
        fn_state.gs_322109 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#322109:u8
        let s_23_0: bool = fn_state.gs_322109;
        // D s_23_1: write-var gs#322110 <= s_23_0
        fn_state.gs_322110 = s_23_0;
        // N s_23_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var ebytes:i64
        let s_24_0: i64 = fn_state.ebytes;
        // D s_24_1: cast zx s_24_0 -> i
        let s_24_1: i128 = (i128::try_from(s_24_0).unwrap());
        // D s_24_2: call __id(s_24_1)
        let s_24_2: i128 = u__id(state, tracer, s_24_1);
        // D s_24_3: cast reint s_24_2 -> i64
        let s_24_3: i64 = (s_24_2 as i64);
        // C s_24_4: const #1s : i
        let s_24_4: i128 = 1;
        // D s_24_5: cast zx s_24_3 -> i
        let s_24_5: i128 = (i128::try_from(s_24_3).unwrap());
        // D s_24_6: cmp-eq s_24_5 s_24_4
        let s_24_6: bool = ((s_24_5) == (s_24_4));
        // N s_24_7: branch s_24_6 b54 b25
        if s_24_6 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var ebytes:i64
        let s_25_0: i64 = fn_state.ebytes;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: call __id(s_25_1)
        let s_25_2: i128 = u__id(state, tracer, s_25_1);
        // D s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: const #2s : i
        let s_25_4: i128 = 2;
        // D s_25_5: cast zx s_25_3 -> i
        let s_25_5: i128 = (i128::try_from(s_25_3).unwrap());
        // D s_25_6: cmp-eq s_25_5 s_25_4
        let s_25_6: bool = ((s_25_5) == (s_25_4));
        // D s_25_7: write-var gs#322090 <= s_25_6
        fn_state.gs_322090 = s_25_6;
        // N s_25_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#322090:u8
        let s_26_0: bool = fn_state.gs_322090;
        // N s_26_1: branch s_26_0 b53 b27
        if s_26_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var ebytes:i64
        let s_27_0: i64 = fn_state.ebytes;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: call __id(s_27_1)
        let s_27_2: i128 = u__id(state, tracer, s_27_1);
        // D s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: const #4s : i
        let s_27_4: i128 = 4;
        // D s_27_5: cast zx s_27_3 -> i
        let s_27_5: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_6: cmp-eq s_27_5 s_27_4
        let s_27_6: bool = ((s_27_5) == (s_27_4));
        // D s_27_7: write-var gs#322092 <= s_27_6
        fn_state.gs_322092 = s_27_6;
        // N s_27_8: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#322092:u8
        let s_28_0: bool = fn_state.gs_322092;
        // N s_28_1: branch s_28_0 b52 b29
        if s_28_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var ebytes:i64
        let s_29_0: i64 = fn_state.ebytes;
        // D s_29_1: cast zx s_29_0 -> i
        let s_29_1: i128 = (i128::try_from(s_29_0).unwrap());
        // D s_29_2: call __id(s_29_1)
        let s_29_2: i128 = u__id(state, tracer, s_29_1);
        // D s_29_3: cast reint s_29_2 -> i64
        let s_29_3: i64 = (s_29_2 as i64);
        // C s_29_4: const #8s : i
        let s_29_4: i128 = 8;
        // D s_29_5: cast zx s_29_3 -> i
        let s_29_5: i128 = (i128::try_from(s_29_3).unwrap());
        // D s_29_6: cmp-eq s_29_5 s_29_4
        let s_29_6: bool = ((s_29_5) == (s_29_4));
        // D s_29_7: write-var gs#322094 <= s_29_6
        fn_state.gs_322094 = s_29_6;
        // N s_29_8: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#322094:u8
        let s_30_0: bool = fn_state.gs_322094;
        // N s_30_1: branch s_30_0 b33 b31
        if s_30_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#322108 <= s_31_0
        fn_state.gs_322108 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#322108:u8
        let s_32_0: bool = fn_state.gs_322108;
        // D s_32_1: write-var gs#322109 <= s_32_0
        fn_state.gs_322109 = s_32_0;
        // N s_32_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var d:i64
        let s_33_0: i64 = fn_state.d;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: call __id(s_33_1)
        let s_33_2: i128 = u__id(state, tracer, s_33_1);
        // D s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: const #0s : i
        let s_33_4: i128 = 0;
        // D s_33_5: cast zx s_33_3 -> i
        let s_33_5: i128 = (i128::try_from(s_33_3).unwrap());
        // D s_33_6: cmp-le s_33_4 s_33_5
        let s_33_6: bool = ((s_33_4) <= (s_33_5));
        // N s_33_7: branch s_33_6 b36 b34
        if s_33_6 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#322107 <= s_34_0
        fn_state.gs_322107 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#322107:u8
        let s_35_0: bool = fn_state.gs_322107;
        // D s_35_1: write-var gs#322108 <= s_35_0
        fn_state.gs_322108 = s_35_0;
        // N s_35_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var d:i64
        let s_36_0: i64 = fn_state.d;
        // D s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_2: call __id(s_36_1)
        let s_36_2: i128 = u__id(state, tracer, s_36_1);
        // D s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: const #31s : i
        let s_36_4: i128 = 31;
        // D s_36_5: cast zx s_36_3 -> i
        let s_36_5: i128 = (i128::try_from(s_36_3).unwrap());
        // D s_36_6: cmp-le s_36_5 s_36_4
        let s_36_6: bool = ((s_36_5) <= (s_36_4));
        // N s_36_7: branch s_36_6 b39 b37
        if s_36_6 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#322106 <= s_37_0
        fn_state.gs_322106 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#322106:u8
        let s_38_0: bool = fn_state.gs_322106;
        // D s_38_1: write-var gs#322107 <= s_38_0
        fn_state.gs_322107 = s_38_0;
        // N s_38_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var alignment:i
        let s_39_0: i128 = fn_state.alignment;
        // D s_39_1: call __id(s_39_0)
        let s_39_1: i128 = u__id(state, tracer, s_39_0);
        // C s_39_2: const #1s : i
        let s_39_2: i128 = 1;
        // D s_39_3: cmp-eq s_39_1 s_39_2
        let s_39_3: bool = ((s_39_1) == (s_39_2));
        // N s_39_4: branch s_39_3 b51 b40
        if s_39_3 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var alignment:i
        let s_40_0: i128 = fn_state.alignment;
        // D s_40_1: call __id(s_40_0)
        let s_40_1: i128 = u__id(state, tracer, s_40_0);
        // C s_40_2: const #4s : i
        let s_40_2: i128 = 4;
        // D s_40_3: cmp-eq s_40_1 s_40_2
        let s_40_3: bool = ((s_40_1) == (s_40_2));
        // D s_40_4: write-var gs#322099 <= s_40_3
        fn_state.gs_322099 = s_40_3;
        // N s_40_5: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#322099:u8
        let s_41_0: bool = fn_state.gs_322099;
        // N s_41_1: branch s_41_0 b50 b42
        if s_41_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var alignment:i
        let s_42_0: i128 = fn_state.alignment;
        // D s_42_1: call __id(s_42_0)
        let s_42_1: i128 = u__id(state, tracer, s_42_0);
        // C s_42_2: const #8s : i
        let s_42_2: i128 = 8;
        // D s_42_3: cmp-eq s_42_1 s_42_2
        let s_42_3: bool = ((s_42_1) == (s_42_2));
        // D s_42_4: write-var gs#322101 <= s_42_3
        fn_state.gs_322101 = s_42_3;
        // N s_42_5: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#322101:u8
        let s_43_0: bool = fn_state.gs_322101;
        // N s_43_1: branch s_43_0 b49 b44
        if s_43_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var alignment:i
        let s_44_0: i128 = fn_state.alignment;
        // D s_44_1: call __id(s_44_0)
        let s_44_1: i128 = u__id(state, tracer, s_44_0);
        // C s_44_2: const #16s : i
        let s_44_2: i128 = 16;
        // D s_44_3: cmp-eq s_44_1 s_44_2
        let s_44_3: bool = ((s_44_1) == (s_44_2));
        // D s_44_4: write-var gs#322103 <= s_44_3
        fn_state.gs_322103 = s_44_3;
        // N s_44_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#322103:u8
        let s_45_0: bool = fn_state.gs_322103;
        // N s_45_1: branch s_45_0 b48 b46
        if s_45_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var alignment:i
        let s_46_0: i128 = fn_state.alignment;
        // D s_46_1: call __id(s_46_0)
        let s_46_1: i128 = u__id(state, tracer, s_46_0);
        // C s_46_2: const #32s : i
        let s_46_2: i128 = 32;
        // D s_46_3: cmp-eq s_46_1 s_46_2
        let s_46_3: bool = ((s_46_1) == (s_46_2));
        // D s_46_4: write-var gs#322105 <= s_46_3
        fn_state.gs_322105 = s_46_3;
        // N s_46_5: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#322105:u8
        let s_47_0: bool = fn_state.gs_322105;
        // D s_47_1: write-var gs#322106 <= s_47_0
        fn_state.gs_322106 = s_47_0;
        // N s_47_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#322105 <= s_48_0
        fn_state.gs_322105 = s_48_0;
        // N s_48_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var gs#322103 <= s_49_0
        fn_state.gs_322103 = s_49_0;
        // N s_49_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #1u : u8
        let s_50_0: bool = true;
        // D s_50_1: write-var gs#322101 <= s_50_0
        fn_state.gs_322101 = s_50_0;
        // N s_50_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #1u : u8
        let s_51_0: bool = true;
        // D s_51_1: write-var gs#322099 <= s_51_0
        fn_state.gs_322099 = s_51_0;
        // N s_51_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // D s_52_1: write-var gs#322094 <= s_52_0
        fn_state.gs_322094 = s_52_0;
        // N s_52_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var gs#322092 <= s_53_0
        fn_state.gs_322092 = s_53_0;
        // N s_53_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#322090 <= s_54_0
        fn_state.gs_322090 = s_54_0;
        // N s_54_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_55_0: panic
        panic!("{:?}", ());
        // N s_55_1: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#322083 <= s_56_0
        fn_state.gs_322083 = s_56_0;
        // N s_56_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #13s : i
        let s_57_0: i128 = 13;
        // D s_57_1: read-var m:i64
        let s_57_1: i64 = fn_state.m;
        // D s_57_2: cast zx s_57_1 -> i
        let s_57_2: i128 = (i128::try_from(s_57_1).unwrap());
        // D s_57_3: call neq_int(s_57_2, s_57_0)
        let s_57_3: bool = neq_int(state, tracer, s_57_2, s_57_0);
        // D s_57_4: write-var gs#322080 <= s_57_3
        fn_state.gs_322080 = s_57_3;
        // N s_57_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #1s : i
        let s_58_0: i128 = 1;
        // D s_58_1: write-var ga#362845 <= s_58_0
        fn_state.ga_362845 = s_58_0;
        // N s_58_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var itype:u8
        let s_59_0: u8 = fn_state.itype;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 4u16);
        // C s_59_2: const #1u : u8
        let s_59_2: u8 = 1;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 4u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: not s_59_4
        let s_59_5: bool = !s_59_4;
        // N s_59_6: branch s_59_5 b61 b60
        if s_59_5 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #2s : i64
        let s_60_0: i64 = 2;
        // D s_60_1: write-var inc_name <= s_60_0
        fn_state.inc_name = s_60_0;
        // N s_60_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_61_0: panic
        panic!("{:?}", ());
        // N s_61_1: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_62_0: panic
        panic!("{:?}", ());
        // N s_62_1: return
        return;
    }
}
