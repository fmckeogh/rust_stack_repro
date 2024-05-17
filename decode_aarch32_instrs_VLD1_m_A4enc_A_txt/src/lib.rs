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
use execute_aarch32_instrs_VLD1_m_Op_A_txt::*;
use u__id::*;
use ConditionPassed::*;
use common::*;
pub fn decode_aarch32_instrs_VLD1_m_A4enc_A_txt<T: Tracer>(
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
        gs_309750: bool,
        gs_309786: bool,
        gs_309783: bool,
        n: i64,
        gs_309788: bool,
        gs_309780: bool,
        gs_309756: bool,
        gs_309789: bool,
        gs_309790: bool,
        gs_309769: bool,
        gs_309785: bool,
        gs_309760: bool,
        gs_309782: bool,
        gs_309758: bool,
        ga_353585: i128,
        gs_309767: bool,
        gs_309753: bool,
        gs_309776: bool,
        d: i64,
        gs_309784: bool,
        gs_309787: bool,
        elements: i64,
        gs_309771: bool,
        register_index: bool,
        alignment: i128,
        gs_309778: bool,
        wback: bool,
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
        // D s_2_0: read-var align:u8
        let s_2_0: u8 = fn_state.align;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #0u : u8
        let s_2_2: u8 = 0;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b67 b3
        if s_2_4 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var align:u8
        let s_3_0: u8 = fn_state.align;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: const #4s : i
        let s_3_4: i128 = 4;
        // D s_3_5: cast zx s_3_3 -> i
        let s_3_5: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_6: lsl s_3_4 s_3_5
        let s_3_6: i128 = s_3_4 << s_3_5;
        // D s_3_7: write-var ga#353585 <= s_3_6
        fn_state.ga_353585 = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#353585:i
        let s_4_0: i128 = fn_state.ga_353585;
        // D s_4_1: write-var alignment <= s_4_0
        fn_state.alignment = s_4_0;
        // D s_4_2: read-var size:u8
        let s_4_2: u8 = fn_state.size;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (s_4_3.value() as i128);
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // C s_4_6: const #1s : i64
        let s_4_6: i64 = 1;
        // D s_4_7: lsl s_4_6 s_4_5
        let s_4_7: i64 = s_4_6 << s_4_5;
        // D s_4_8: write-var ebytes <= s_4_7
        fn_state.ebytes = s_4_7;
        // C s_4_9: const #8s : i
        let s_4_9: i128 = 8;
        // D s_4_10: read-var ebytes:i64
        let s_4_10: i64 = fn_state.ebytes;
        // D s_4_11: cast zx s_4_10 -> i
        let s_4_11: i128 = (i128::try_from(s_4_10).unwrap());
        // D s_4_12: div s_4_9 s_4_11
        let s_4_12: i128 = ((s_4_9) / (s_4_11));
        // D s_4_13: cast reint s_4_12 -> i64
        let s_4_13: i64 = (s_4_12 as i64);
        // D s_4_14: write-var elements <= s_4_13
        fn_state.elements = s_4_13;
        // D s_4_15: read-var D:u8
        let s_4_15: bool = fn_state.D;
        // D s_4_16: cast zx s_4_15 -> bv
        let s_4_16: Bits = Bits::new(s_4_15 as u128, 1u16);
        // D s_4_17: read-var Vd:u8
        let s_4_17: u8 = fn_state.Vd;
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 4u16);
        // D s_4_19: cast reint s_4_16 -> u128
        let s_4_19: u128 = (s_4_16.value() as u128);
        // D s_4_20: size-of s_4_16
        let s_4_20: u16 = s_4_16.length();
        // D s_4_21: cast reint s_4_18 -> u128
        let s_4_21: u128 = (s_4_18.value() as u128);
        // D s_4_22: size-of s_4_18
        let s_4_22: u16 = s_4_18.length();
        // D s_4_23: lsl s_4_19 s_4_22
        let s_4_23: u128 = s_4_19 << s_4_22;
        // D s_4_24: or s_4_23 s_4_21
        let s_4_24: u128 = ((s_4_23) | (s_4_21));
        // D s_4_25: add s_4_20 s_4_22
        let s_4_25: u16 = (s_4_20 + s_4_22);
        // D s_4_26: create-bits s_4_24 s_4_25
        let s_4_26: Bits = Bits::new(s_4_24, s_4_25);
        // D s_4_27: cast reint s_4_26 -> u8
        let s_4_27: u8 = (s_4_26.value() as u8);
        // D s_4_28: cast zx s_4_27 -> bv
        let s_4_28: Bits = Bits::new(s_4_27 as u128, 5u16);
        // D s_4_29: cast zx s_4_28 -> i
        let s_4_29: i128 = (s_4_28.value() as i128);
        // D s_4_30: cast reint s_4_29 -> i64
        let s_4_30: i64 = (s_4_29 as i64);
        // D s_4_31: write-var d <= s_4_30
        fn_state.d = s_4_30;
        // D s_4_32: read-var Rn:u8
        let s_4_32: u8 = fn_state.Rn;
        // D s_4_33: cast zx s_4_32 -> bv
        let s_4_33: Bits = Bits::new(s_4_32 as u128, 4u16);
        // D s_4_34: cast zx s_4_33 -> i
        let s_4_34: i128 = (s_4_33.value() as i128);
        // D s_4_35: cast reint s_4_34 -> i64
        let s_4_35: i64 = (s_4_34 as i64);
        // D s_4_36: write-var n <= s_4_35
        fn_state.n = s_4_35;
        // D s_4_37: read-var Rm:u8
        let s_4_37: u8 = fn_state.Rm;
        // D s_4_38: cast zx s_4_37 -> bv
        let s_4_38: Bits = Bits::new(s_4_37 as u128, 4u16);
        // D s_4_39: cast zx s_4_38 -> i
        let s_4_39: i128 = (s_4_38.value() as i128);
        // D s_4_40: cast reint s_4_39 -> i64
        let s_4_40: i64 = (s_4_39 as i64);
        // D s_4_41: write-var m <= s_4_40
        fn_state.m = s_4_40;
        // C s_4_42: const #15s : i
        let s_4_42: i128 = 15;
        // D s_4_43: read-var m:i64
        let s_4_43: i64 = fn_state.m;
        // D s_4_44: cast zx s_4_43 -> i
        let s_4_44: i128 = (i128::try_from(s_4_43).unwrap());
        // D s_4_45: call neq_int(s_4_44, s_4_42)
        let s_4_45: bool = neq_int(state, tracer, s_4_44, s_4_42);
        // D s_4_46: write-var wback <= s_4_45
        fn_state.wback = s_4_45;
        // C s_4_47: const #15s : i
        let s_4_47: i128 = 15;
        // D s_4_48: read-var m:i64
        let s_4_48: i64 = fn_state.m;
        // D s_4_49: cast zx s_4_48 -> i
        let s_4_49: i128 = (i128::try_from(s_4_48).unwrap());
        // D s_4_50: call neq_int(s_4_49, s_4_47)
        let s_4_50: bool = neq_int(state, tracer, s_4_49, s_4_47);
        // N s_4_51: branch s_4_50 b66 b5
        if s_4_50 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#309750 <= s_5_0
        fn_state.gs_309750 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#309750:u8
        let s_6_0: bool = fn_state.gs_309750;
        // D s_6_1: write-var register_index <= s_6_0
        fn_state.register_index = s_6_0;
        // C s_6_2: const #15s : i
        let s_6_2: i128 = 15;
        // D s_6_3: read-var n:i64
        let s_6_3: i64 = fn_state.n;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cmp-eq s_6_4 s_6_2
        let s_6_5: bool = ((s_6_4) == (s_6_2));
        // N s_6_6: branch s_6_5 b65 b7
        if s_6_5 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var d:i64
        let s_7_0: i64 = fn_state.d;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // C s_7_2: const #4s : i64
        let s_7_2: i64 = 4;
        // C s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: add s_7_1 s_7_3
        let s_7_4: i128 = (s_7_1 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // C s_7_6: const #32s : i
        let s_7_6: i128 = 32;
        // D s_7_7: cast zx s_7_5 -> i
        let s_7_7: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_8: cmp-gt s_7_7 s_7_6
        let s_7_8: bool = ((s_7_7) > (s_7_6));
        // D s_7_9: write-var gs#309753 <= s_7_8
        fn_state.gs_309753 = s_7_8;
        // N s_7_10: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#309753:u8
        let s_8_0: bool = fn_state.gs_309753;
        // N s_8_1: branch s_8_0 b64 b9
        if s_8_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #4s : i64
        let s_9_0: i64 = 4;
        // C s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // S s_9_2: call __id(s_9_1)
        let s_9_2: i128 = u__id(state, tracer, s_9_1);
        // S s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #1s : i
        let s_9_4: i128 = 1;
        // S s_9_5: cast zx s_9_3 -> i
        let s_9_5: i128 = (i128::try_from(s_9_3).unwrap());
        // S s_9_6: cmp-eq s_9_5 s_9_4
        let s_9_6: bool = ((s_9_5) == (s_9_4));
        // N s_9_7: branch s_9_6 b63 b10
        if s_9_6 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #4s : i64
        let s_10_0: i64 = 4;
        // C s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // S s_10_2: call __id(s_10_1)
        let s_10_2: i128 = u__id(state, tracer, s_10_1);
        // S s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // C s_10_4: const #2s : i
        let s_10_4: i128 = 2;
        // S s_10_5: cast zx s_10_3 -> i
        let s_10_5: i128 = (i128::try_from(s_10_3).unwrap());
        // S s_10_6: cmp-eq s_10_5 s_10_4
        let s_10_6: bool = ((s_10_5) == (s_10_4));
        // D s_10_7: write-var gs#309756 <= s_10_6
        fn_state.gs_309756 = s_10_6;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#309756:u8
        let s_11_0: bool = fn_state.gs_309756;
        // N s_11_1: branch s_11_0 b62 b12
        if s_11_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #4s : i64
        let s_12_0: i64 = 4;
        // C s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // S s_12_2: call __id(s_12_1)
        let s_12_2: i128 = u__id(state, tracer, s_12_1);
        // S s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // C s_12_4: const #3s : i
        let s_12_4: i128 = 3;
        // S s_12_5: cast zx s_12_3 -> i
        let s_12_5: i128 = (i128::try_from(s_12_3).unwrap());
        // S s_12_6: cmp-eq s_12_5 s_12_4
        let s_12_6: bool = ((s_12_5) == (s_12_4));
        // D s_12_7: write-var gs#309758 <= s_12_6
        fn_state.gs_309758 = s_12_6;
        // N s_12_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#309758:u8
        let s_13_0: bool = fn_state.gs_309758;
        // N s_13_1: branch s_13_0 b61 b14
        if s_13_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #4s : i64
        let s_14_0: i64 = 4;
        // C s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // S s_14_2: call __id(s_14_1)
        let s_14_2: i128 = u__id(state, tracer, s_14_1);
        // S s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: const #4s : i
        let s_14_4: i128 = 4;
        // S s_14_5: cast zx s_14_3 -> i
        let s_14_5: i128 = (i128::try_from(s_14_3).unwrap());
        // S s_14_6: cmp-eq s_14_5 s_14_4
        let s_14_6: bool = ((s_14_5) == (s_14_4));
        // D s_14_7: write-var gs#309760 <= s_14_6
        fn_state.gs_309760 = s_14_6;
        // N s_14_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#309760:u8
        let s_15_0: bool = fn_state.gs_309760;
        // N s_15_1: branch s_15_0 b18 b16
        if s_15_0 {
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
        // D s_16_1: write-var gs#309790 <= s_16_0
        fn_state.gs_309790 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#309790:u8
        let s_17_0: bool = fn_state.gs_309790;
        // N s_17_1: assert s_17_0
        let s_17_1: () = assert!(s_17_0);
        // D s_17_2: read-var alignment:i
        let s_17_2: i128 = fn_state.alignment;
        // D s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // D s_17_4: read-var elements:i64
        let s_17_4: i64 = fn_state.elements;
        // D s_17_5: cast zx s_17_4 -> i
        let s_17_5: i128 = (i128::try_from(s_17_4).unwrap());
        // D s_17_6: read-var d:i64
        let s_17_6: i64 = fn_state.d;
        // D s_17_7: read-var ebytes:i64
        let s_17_7: i64 = fn_state.ebytes;
        // D s_17_8: read-var m:i64
        let s_17_8: i64 = fn_state.m;
        // D s_17_9: read-var n:i64
        let s_17_9: i64 = fn_state.n;
        // D s_17_10: read-var register_index:u8
        let s_17_10: bool = fn_state.register_index;
        // C s_17_11: const #4s : i64
        let s_17_11: i64 = 4;
        // D s_17_12: read-var wback:u8
        let s_17_12: bool = fn_state.wback;
        // D s_17_13: call execute_aarch32_instrs_VLD1_m_Op_A_txt(s_17_3, s_17_6, s_17_7, s_17_5, s_17_8, s_17_9, s_17_10, s_17_11, s_17_12)
        let s_17_13: () = execute_aarch32_instrs_VLD1_m_Op_A_txt(
            state,
            tracer,
            s_17_3,
            s_17_6,
            s_17_7,
            s_17_5,
            s_17_8,
            s_17_9,
            s_17_10,
            s_17_11,
            s_17_12,
        );
        // N s_17_14: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var n:i64
        let s_18_0: i64 = fn_state.n;
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
        // D s_19_1: write-var gs#309789 <= s_19_0
        fn_state.gs_309789 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#309789:u8
        let s_20_0: bool = fn_state.gs_309789;
        // D s_20_1: write-var gs#309790 <= s_20_0
        fn_state.gs_309790 = s_20_0;
        // N s_20_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var n:i64
        let s_21_0: i64 = fn_state.n;
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
        // D s_22_1: write-var gs#309788 <= s_22_0
        fn_state.gs_309788 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#309788:u8
        let s_23_0: bool = fn_state.gs_309788;
        // D s_23_1: write-var gs#309789 <= s_23_0
        fn_state.gs_309789 = s_23_0;
        // N s_23_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var m:i64
        let s_24_0: i64 = fn_state.m;
        // D s_24_1: cast zx s_24_0 -> i
        let s_24_1: i128 = (i128::try_from(s_24_0).unwrap());
        // D s_24_2: call __id(s_24_1)
        let s_24_2: i128 = u__id(state, tracer, s_24_1);
        // D s_24_3: cast reint s_24_2 -> i64
        let s_24_3: i64 = (s_24_2 as i64);
        // C s_24_4: const #0s : i
        let s_24_4: i128 = 0;
        // D s_24_5: cast zx s_24_3 -> i
        let s_24_5: i128 = (i128::try_from(s_24_3).unwrap());
        // D s_24_6: cmp-le s_24_4 s_24_5
        let s_24_6: bool = ((s_24_4) <= (s_24_5));
        // N s_24_7: branch s_24_6 b27 b25
        if s_24_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#309787 <= s_25_0
        fn_state.gs_309787 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#309787:u8
        let s_26_0: bool = fn_state.gs_309787;
        // D s_26_1: write-var gs#309788 <= s_26_0
        fn_state.gs_309788 = s_26_0;
        // N s_26_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var m:i64
        let s_27_0: i64 = fn_state.m;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: call __id(s_27_1)
        let s_27_2: i128 = u__id(state, tracer, s_27_1);
        // D s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: const #15s : i
        let s_27_4: i128 = 15;
        // D s_27_5: cast zx s_27_3 -> i
        let s_27_5: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_6: cmp-le s_27_5 s_27_4
        let s_27_6: bool = ((s_27_5) <= (s_27_4));
        // N s_27_7: branch s_27_6 b30 b28
        if s_27_6 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#309786 <= s_28_0
        fn_state.gs_309786 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#309786:u8
        let s_29_0: bool = fn_state.gs_309786;
        // D s_29_1: write-var gs#309787 <= s_29_0
        fn_state.gs_309787 = s_29_0;
        // N s_29_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var ebytes:i64
        let s_30_0: i64 = fn_state.ebytes;
        // D s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // D s_30_2: call __id(s_30_1)
        let s_30_2: i128 = u__id(state, tracer, s_30_1);
        // D s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: const #1s : i
        let s_30_4: i128 = 1;
        // D s_30_5: cast zx s_30_3 -> i
        let s_30_5: i128 = (i128::try_from(s_30_3).unwrap());
        // D s_30_6: cmp-eq s_30_5 s_30_4
        let s_30_6: bool = ((s_30_5) == (s_30_4));
        // N s_30_7: branch s_30_6 b60 b31
        if s_30_6 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var ebytes:i64
        let s_31_0: i64 = fn_state.ebytes;
        // D s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: call __id(s_31_1)
        let s_31_2: i128 = u__id(state, tracer, s_31_1);
        // D s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: const #2s : i
        let s_31_4: i128 = 2;
        // D s_31_5: cast zx s_31_3 -> i
        let s_31_5: i128 = (i128::try_from(s_31_3).unwrap());
        // D s_31_6: cmp-eq s_31_5 s_31_4
        let s_31_6: bool = ((s_31_5) == (s_31_4));
        // D s_31_7: write-var gs#309767 <= s_31_6
        fn_state.gs_309767 = s_31_6;
        // N s_31_8: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#309767:u8
        let s_32_0: bool = fn_state.gs_309767;
        // N s_32_1: branch s_32_0 b59 b33
        if s_32_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var ebytes:i64
        let s_33_0: i64 = fn_state.ebytes;
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
        // D s_33_7: write-var gs#309769 <= s_33_6
        fn_state.gs_309769 = s_33_6;
        // N s_33_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#309769:u8
        let s_34_0: bool = fn_state.gs_309769;
        // N s_34_1: branch s_34_0 b58 b35
        if s_34_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var ebytes:i64
        let s_35_0: i64 = fn_state.ebytes;
        // D s_35_1: cast zx s_35_0 -> i
        let s_35_1: i128 = (i128::try_from(s_35_0).unwrap());
        // D s_35_2: call __id(s_35_1)
        let s_35_2: i128 = u__id(state, tracer, s_35_1);
        // D s_35_3: cast reint s_35_2 -> i64
        let s_35_3: i64 = (s_35_2 as i64);
        // C s_35_4: const #8s : i
        let s_35_4: i128 = 8;
        // D s_35_5: cast zx s_35_3 -> i
        let s_35_5: i128 = (i128::try_from(s_35_3).unwrap());
        // D s_35_6: cmp-eq s_35_5 s_35_4
        let s_35_6: bool = ((s_35_5) == (s_35_4));
        // D s_35_7: write-var gs#309771 <= s_35_6
        fn_state.gs_309771 = s_35_6;
        // N s_35_8: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#309771:u8
        let s_36_0: bool = fn_state.gs_309771;
        // N s_36_1: branch s_36_0 b39 b37
        if s_36_0 {
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
        // D s_37_1: write-var gs#309785 <= s_37_0
        fn_state.gs_309785 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#309785:u8
        let s_38_0: bool = fn_state.gs_309785;
        // D s_38_1: write-var gs#309786 <= s_38_0
        fn_state.gs_309786 = s_38_0;
        // N s_38_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var d:i64
        let s_39_0: i64 = fn_state.d;
        // D s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_2: call __id(s_39_1)
        let s_39_2: i128 = u__id(state, tracer, s_39_1);
        // D s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: const #0s : i
        let s_39_4: i128 = 0;
        // D s_39_5: cast zx s_39_3 -> i
        let s_39_5: i128 = (i128::try_from(s_39_3).unwrap());
        // D s_39_6: cmp-le s_39_4 s_39_5
        let s_39_6: bool = ((s_39_4) <= (s_39_5));
        // N s_39_7: branch s_39_6 b42 b40
        if s_39_6 {
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
        // D s_40_1: write-var gs#309784 <= s_40_0
        fn_state.gs_309784 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#309784:u8
        let s_41_0: bool = fn_state.gs_309784;
        // D s_41_1: write-var gs#309785 <= s_41_0
        fn_state.gs_309785 = s_41_0;
        // N s_41_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var d:i64
        let s_42_0: i64 = fn_state.d;
        // D s_42_1: cast zx s_42_0 -> i
        let s_42_1: i128 = (i128::try_from(s_42_0).unwrap());
        // D s_42_2: call __id(s_42_1)
        let s_42_2: i128 = u__id(state, tracer, s_42_1);
        // D s_42_3: cast reint s_42_2 -> i64
        let s_42_3: i64 = (s_42_2 as i64);
        // C s_42_4: const #31s : i
        let s_42_4: i128 = 31;
        // D s_42_5: cast zx s_42_3 -> i
        let s_42_5: i128 = (i128::try_from(s_42_3).unwrap());
        // D s_42_6: cmp-le s_42_5 s_42_4
        let s_42_6: bool = ((s_42_5) <= (s_42_4));
        // N s_42_7: branch s_42_6 b45 b43
        if s_42_6 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#309783 <= s_43_0
        fn_state.gs_309783 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#309783:u8
        let s_44_0: bool = fn_state.gs_309783;
        // D s_44_1: write-var gs#309784 <= s_44_0
        fn_state.gs_309784 = s_44_0;
        // N s_44_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var alignment:i
        let s_45_0: i128 = fn_state.alignment;
        // D s_45_1: call __id(s_45_0)
        let s_45_1: i128 = u__id(state, tracer, s_45_0);
        // C s_45_2: const #1s : i
        let s_45_2: i128 = 1;
        // D s_45_3: cmp-eq s_45_1 s_45_2
        let s_45_3: bool = ((s_45_1) == (s_45_2));
        // N s_45_4: branch s_45_3 b57 b46
        if s_45_3 {
            return block_57(state, tracer, fn_state);
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
        // C s_46_2: const #4s : i
        let s_46_2: i128 = 4;
        // D s_46_3: cmp-eq s_46_1 s_46_2
        let s_46_3: bool = ((s_46_1) == (s_46_2));
        // D s_46_4: write-var gs#309776 <= s_46_3
        fn_state.gs_309776 = s_46_3;
        // N s_46_5: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#309776:u8
        let s_47_0: bool = fn_state.gs_309776;
        // N s_47_1: branch s_47_0 b56 b48
        if s_47_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var alignment:i
        let s_48_0: i128 = fn_state.alignment;
        // D s_48_1: call __id(s_48_0)
        let s_48_1: i128 = u__id(state, tracer, s_48_0);
        // C s_48_2: const #8s : i
        let s_48_2: i128 = 8;
        // D s_48_3: cmp-eq s_48_1 s_48_2
        let s_48_3: bool = ((s_48_1) == (s_48_2));
        // D s_48_4: write-var gs#309778 <= s_48_3
        fn_state.gs_309778 = s_48_3;
        // N s_48_5: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#309778:u8
        let s_49_0: bool = fn_state.gs_309778;
        // N s_49_1: branch s_49_0 b55 b50
        if s_49_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var alignment:i
        let s_50_0: i128 = fn_state.alignment;
        // D s_50_1: call __id(s_50_0)
        let s_50_1: i128 = u__id(state, tracer, s_50_0);
        // C s_50_2: const #16s : i
        let s_50_2: i128 = 16;
        // D s_50_3: cmp-eq s_50_1 s_50_2
        let s_50_3: bool = ((s_50_1) == (s_50_2));
        // D s_50_4: write-var gs#309780 <= s_50_3
        fn_state.gs_309780 = s_50_3;
        // N s_50_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#309780:u8
        let s_51_0: bool = fn_state.gs_309780;
        // N s_51_1: branch s_51_0 b54 b52
        if s_51_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var alignment:i
        let s_52_0: i128 = fn_state.alignment;
        // D s_52_1: call __id(s_52_0)
        let s_52_1: i128 = u__id(state, tracer, s_52_0);
        // C s_52_2: const #32s : i
        let s_52_2: i128 = 32;
        // D s_52_3: cmp-eq s_52_1 s_52_2
        let s_52_3: bool = ((s_52_1) == (s_52_2));
        // D s_52_4: write-var gs#309782 <= s_52_3
        fn_state.gs_309782 = s_52_3;
        // N s_52_5: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#309782:u8
        let s_53_0: bool = fn_state.gs_309782;
        // D s_53_1: write-var gs#309783 <= s_53_0
        fn_state.gs_309783 = s_53_0;
        // N s_53_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#309782 <= s_54_0
        fn_state.gs_309782 = s_54_0;
        // N s_54_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#309780 <= s_55_0
        fn_state.gs_309780 = s_55_0;
        // N s_55_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#309778 <= s_56_0
        fn_state.gs_309778 = s_56_0;
        // N s_56_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var gs#309776 <= s_57_0
        fn_state.gs_309776 = s_57_0;
        // N s_57_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#309771 <= s_58_0
        fn_state.gs_309771 = s_58_0;
        // N s_58_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #1u : u8
        let s_59_0: bool = true;
        // D s_59_1: write-var gs#309769 <= s_59_0
        fn_state.gs_309769 = s_59_0;
        // N s_59_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#309767 <= s_60_0
        fn_state.gs_309767 = s_60_0;
        // N s_60_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#309760 <= s_61_0
        fn_state.gs_309760 = s_61_0;
        // N s_61_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #1u : u8
        let s_62_0: bool = true;
        // D s_62_1: write-var gs#309758 <= s_62_0
        fn_state.gs_309758 = s_62_0;
        // N s_62_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #1u : u8
        let s_63_0: bool = true;
        // D s_63_1: write-var gs#309756 <= s_63_0
        fn_state.gs_309756 = s_63_0;
        // N s_63_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_64_0: panic
        panic!("{:?}", ());
        // N s_64_1: return
        return;
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#309753 <= s_65_0
        fn_state.gs_309753 = s_65_0;
        // N s_65_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #13s : i
        let s_66_0: i128 = 13;
        // D s_66_1: read-var m:i64
        let s_66_1: i64 = fn_state.m;
        // D s_66_2: cast zx s_66_1 -> i
        let s_66_2: i128 = (i128::try_from(s_66_1).unwrap());
        // D s_66_3: call neq_int(s_66_2, s_66_0)
        let s_66_3: bool = neq_int(state, tracer, s_66_2, s_66_0);
        // D s_66_4: write-var gs#309750 <= s_66_3
        fn_state.gs_309750 = s_66_3;
        // N s_66_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1s : i
        let s_67_0: i128 = 1;
        // D s_67_1: write-var ga#353585 <= s_67_0
        fn_state.ga_353585 = s_67_0;
        // N s_67_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
