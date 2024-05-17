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
use execute_aarch32_instrs_VLD4_a_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VLD4_a_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Rn: u8,
    Vd: u8,
    size: u8,
    T: bool,
    a: bool,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_311479: bool,
        ebytes: i64,
        gs_311497: bool,
        n: i64,
        gs_311495: bool,
        alignmentshadow_7484: i128,
        gs_311464: bool,
        gs_311484: bool,
        gs_311486: bool,
        gs_311493: bool,
        ga_354892: i64,
        gs_311492: bool,
        gs_311488: bool,
        gs_311477: bool,
        gs_311475: bool,
        gs_311467: bool,
        gs_311496: bool,
        ebytesshadow_7483: i64,
        d2: i64,
        alignmentshadow_7485: i128,
        gs_311437: bool,
        d: i64,
        d4: i64,
        register_index: bool,
        alignment: i128,
        wback: bool,
        gs_311494: bool,
        d3: i64,
        gs_311491: bool,
        gs_311490: bool,
        D: bool,
        Rn: u8,
        Vd: u8,
        size: u8,
        T: bool,
        a: bool,
        Rm: u8,
    }
    let fn_state = FunctionState {
        D,
        Rn,
        Vd,
        size,
        T,
        a,
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
        // N s_2_5: branch s_2_4 b71 b3
        if s_2_4 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#311437 <= s_3_0
        fn_state.gs_311437 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#311437:u8
        let s_4_0: bool = fn_state.gs_311437;
        // N s_4_1: branch s_4_0 b70 b5
        if s_4_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1s : i64
        let s_5_0: i64 = 1;
        // D s_5_1: write-var ebytes <= s_5_0
        fn_state.ebytes = s_5_0;
        // D s_5_2: read-var size:u8
        let s_5_2: u8 = fn_state.size;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // C s_5_4: const #3u : u8
        let s_5_4: u8 = 3;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 2u16);
        // D s_5_6: cmp-eq s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) == (s_5_5));
        // N s_5_7: branch s_5_6 b69 b6
        if s_5_6 {
            return block_69(state, tracer, fn_state);
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
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (s_6_1.value() as i128);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #1s : i64
        let s_6_4: i64 = 1;
        // D s_6_5: lsl s_6_4 s_6_3
        let s_6_5: i64 = s_6_4 << s_6_3;
        // D s_6_6: write-var ebytes <= s_6_5
        fn_state.ebytes = s_6_5;
        // D s_6_7: read-var size:u8
        let s_6_7: u8 = fn_state.size;
        // D s_6_8: cast zx s_6_7 -> bv
        let s_6_8: Bits = Bits::new(s_6_7 as u128, 2u16);
        // C s_6_9: const #2u : u8
        let s_6_9: u8 = 2;
        // C s_6_10: cast zx s_6_9 -> bv
        let s_6_10: Bits = Bits::new(s_6_9 as u128, 2u16);
        // D s_6_11: cmp-eq s_6_8 s_6_10
        let s_6_11: bool = ((s_6_8) == (s_6_10));
        // N s_6_12: branch s_6_11 b65 b7
        if s_6_11 {
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
        // D s_7_0: read-var a:u8
        let s_7_0: bool = fn_state.a;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #0u : u8
        let s_7_2: bool = false;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b64 b8
        if s_7_4 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #4s : i
        let s_8_0: i128 = 4;
        // D s_8_1: read-var ebytes:i64
        let s_8_1: i64 = fn_state.ebytes;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: mul s_8_0 s_8_2
        let s_8_3: i128 = ((s_8_0) * (s_8_2));
        // D s_8_4: write-var alignment <= s_8_3
        fn_state.alignment = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ebytes:i64
        let s_10_0: i64 = fn_state.ebytes;
        // D s_10_1: write-var ebytesshadow#7483 <= s_10_0
        fn_state.ebytesshadow_7483 = s_10_0;
        // D s_10_2: read-var alignment:i
        let s_10_2: i128 = fn_state.alignment;
        // D s_10_3: write-var alignmentshadow#7484 <= s_10_2
        fn_state.alignmentshadow_7484 = s_10_2;
        // D s_10_4: read-var T:u8
        let s_10_4: bool = fn_state.T;
        // D s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // C s_10_6: const #0u : u8
        let s_10_6: bool = false;
        // C s_10_7: cast zx s_10_6 -> bv
        let s_10_7: Bits = Bits::new(s_10_6 as u128, 1u16);
        // D s_10_8: cmp-eq s_10_5 s_10_7
        let s_10_8: bool = ((s_10_5) == (s_10_7));
        // N s_10_9: branch s_10_8 b63 b11
        if s_10_8 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #2s : i64
        let s_11_0: i64 = 2;
        // D s_11_1: write-var ga#354892 <= s_11_0
        fn_state.ga_354892 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#354892:i64
        let s_12_0: i64 = fn_state.ga_354892;
        // D s_12_1: read-var D:u8
        let s_12_1: bool = fn_state.D;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 1u16);
        // D s_12_3: read-var Vd:u8
        let s_12_3: u8 = fn_state.Vd;
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 4u16);
        // D s_12_5: cast reint s_12_2 -> u128
        let s_12_5: u128 = (s_12_2.value() as u128);
        // D s_12_6: size-of s_12_2
        let s_12_6: u16 = s_12_2.length();
        // D s_12_7: cast reint s_12_4 -> u128
        let s_12_7: u128 = (s_12_4.value() as u128);
        // D s_12_8: size-of s_12_4
        let s_12_8: u16 = s_12_4.length();
        // D s_12_9: lsl s_12_5 s_12_8
        let s_12_9: u128 = s_12_5 << s_12_8;
        // D s_12_10: or s_12_9 s_12_7
        let s_12_10: u128 = ((s_12_9) | (s_12_7));
        // D s_12_11: add s_12_6 s_12_8
        let s_12_11: u16 = (s_12_6 + s_12_8);
        // D s_12_12: create-bits s_12_10 s_12_11
        let s_12_12: Bits = Bits::new(s_12_10, s_12_11);
        // D s_12_13: cast reint s_12_12 -> u8
        let s_12_13: u8 = (s_12_12.value() as u8);
        // D s_12_14: cast zx s_12_13 -> bv
        let s_12_14: Bits = Bits::new(s_12_13 as u128, 5u16);
        // D s_12_15: cast zx s_12_14 -> i
        let s_12_15: i128 = (s_12_14.value() as i128);
        // D s_12_16: cast reint s_12_15 -> i64
        let s_12_16: i64 = (s_12_15 as i64);
        // D s_12_17: write-var d <= s_12_16
        fn_state.d = s_12_16;
        // D s_12_18: read-var d:i64
        let s_12_18: i64 = fn_state.d;
        // D s_12_19: cast zx s_12_18 -> i
        let s_12_19: i128 = (i128::try_from(s_12_18).unwrap());
        // D s_12_20: cast zx s_12_0 -> i
        let s_12_20: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_21: add s_12_19 s_12_20
        let s_12_21: i128 = (s_12_19 + s_12_20);
        // D s_12_22: cast reint s_12_21 -> i64
        let s_12_22: i64 = (s_12_21 as i64);
        // D s_12_23: write-var d2 <= s_12_22
        fn_state.d2 = s_12_22;
        // D s_12_24: read-var d2:i64
        let s_12_24: i64 = fn_state.d2;
        // D s_12_25: cast zx s_12_24 -> i
        let s_12_25: i128 = (i128::try_from(s_12_24).unwrap());
        // D s_12_26: cast zx s_12_0 -> i
        let s_12_26: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_27: add s_12_25 s_12_26
        let s_12_27: i128 = (s_12_25 + s_12_26);
        // D s_12_28: cast reint s_12_27 -> i64
        let s_12_28: i64 = (s_12_27 as i64);
        // D s_12_29: write-var d3 <= s_12_28
        fn_state.d3 = s_12_28;
        // D s_12_30: read-var d3:i64
        let s_12_30: i64 = fn_state.d3;
        // D s_12_31: cast zx s_12_30 -> i
        let s_12_31: i128 = (i128::try_from(s_12_30).unwrap());
        // D s_12_32: cast zx s_12_0 -> i
        let s_12_32: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_33: add s_12_31 s_12_32
        let s_12_33: i128 = (s_12_31 + s_12_32);
        // D s_12_34: cast reint s_12_33 -> i64
        let s_12_34: i64 = (s_12_33 as i64);
        // D s_12_35: write-var d4 <= s_12_34
        fn_state.d4 = s_12_34;
        // D s_12_36: read-var Rn:u8
        let s_12_36: u8 = fn_state.Rn;
        // D s_12_37: cast zx s_12_36 -> bv
        let s_12_37: Bits = Bits::new(s_12_36 as u128, 4u16);
        // D s_12_38: cast zx s_12_37 -> i
        let s_12_38: i128 = (s_12_37.value() as i128);
        // D s_12_39: cast reint s_12_38 -> i64
        let s_12_39: i64 = (s_12_38 as i64);
        // D s_12_40: write-var n <= s_12_39
        fn_state.n = s_12_39;
        // D s_12_41: read-var Rm:u8
        let s_12_41: u8 = fn_state.Rm;
        // D s_12_42: cast zx s_12_41 -> bv
        let s_12_42: Bits = Bits::new(s_12_41 as u128, 4u16);
        // D s_12_43: cast zx s_12_42 -> i
        let s_12_43: i128 = (s_12_42.value() as i128);
        // D s_12_44: cast reint s_12_43 -> i64
        let s_12_44: i64 = (s_12_43 as i64);
        // D s_12_45: write-var m <= s_12_44
        fn_state.m = s_12_44;
        // C s_12_46: const #15s : i
        let s_12_46: i128 = 15;
        // D s_12_47: read-var m:i64
        let s_12_47: i64 = fn_state.m;
        // D s_12_48: cast zx s_12_47 -> i
        let s_12_48: i128 = (i128::try_from(s_12_47).unwrap());
        // D s_12_49: call neq_int(s_12_48, s_12_46)
        let s_12_49: bool = neq_int(state, tracer, s_12_48, s_12_46);
        // D s_12_50: write-var wback <= s_12_49
        fn_state.wback = s_12_49;
        // C s_12_51: const #15s : i
        let s_12_51: i128 = 15;
        // D s_12_52: read-var m:i64
        let s_12_52: i64 = fn_state.m;
        // D s_12_53: cast zx s_12_52 -> i
        let s_12_53: i128 = (i128::try_from(s_12_52).unwrap());
        // D s_12_54: call neq_int(s_12_53, s_12_51)
        let s_12_54: bool = neq_int(state, tracer, s_12_53, s_12_51);
        // N s_12_55: branch s_12_54 b62 b13
        if s_12_54 {
            return block_62(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#311464 <= s_13_0
        fn_state.gs_311464 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#311464:u8
        let s_14_0: bool = fn_state.gs_311464;
        // D s_14_1: write-var register_index <= s_14_0
        fn_state.register_index = s_14_0;
        // C s_14_2: const #15s : i
        let s_14_2: i128 = 15;
        // D s_14_3: read-var n:i64
        let s_14_3: i64 = fn_state.n;
        // D s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_5: cmp-eq s_14_4 s_14_2
        let s_14_5: bool = ((s_14_4) == (s_14_2));
        // N s_14_6: branch s_14_5 b61 b15
        if s_14_5 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #31s : i
        let s_15_0: i128 = 31;
        // D s_15_1: read-var d4:i64
        let s_15_1: i64 = fn_state.d4;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: cmp-gt s_15_2 s_15_0
        let s_15_3: bool = ((s_15_2) > (s_15_0));
        // D s_15_4: write-var gs#311467 <= s_15_3
        fn_state.gs_311467 = s_15_3;
        // N s_15_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#311467:u8
        let s_16_0: bool = fn_state.gs_311467;
        // N s_16_1: branch s_16_0 b60 b17
        if s_16_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var alignmentshadow#7484:i
        let s_17_0: i128 = fn_state.alignmentshadow_7484;
        // D s_17_1: write-var alignmentshadow#7485 <= s_17_0
        fn_state.alignmentshadow_7485 = s_17_0;
        // D s_17_2: read-var n:i64
        let s_17_2: i64 = fn_state.n;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: call __id(s_17_3)
        let s_17_4: i128 = u__id(state, tracer, s_17_3);
        // D s_17_5: cast reint s_17_4 -> i64
        let s_17_5: i64 = (s_17_4 as i64);
        // C s_17_6: const #0s : i
        let s_17_6: i128 = 0;
        // D s_17_7: cast zx s_17_5 -> i
        let s_17_7: i128 = (i128::try_from(s_17_5).unwrap());
        // D s_17_8: cmp-le s_17_6 s_17_7
        let s_17_8: bool = ((s_17_6) <= (s_17_7));
        // N s_17_9: branch s_17_8 b20 b18
        if s_17_8 {
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
        // D s_18_1: write-var gs#311497 <= s_18_0
        fn_state.gs_311497 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#311497:u8
        let s_19_0: bool = fn_state.gs_311497;
        // N s_19_1: assert s_19_0
        let s_19_1: () = assert!(s_19_0);
        // D s_19_2: read-var alignmentshadow#7485:i
        let s_19_2: i128 = fn_state.alignmentshadow_7485;
        // D s_19_3: cast reint s_19_2 -> i64
        let s_19_3: i64 = (s_19_2 as i64);
        // D s_19_4: read-var d2:i64
        let s_19_4: i64 = fn_state.d2;
        // D s_19_5: cast zx s_19_4 -> i
        let s_19_5: i128 = (i128::try_from(s_19_4).unwrap());
        // D s_19_6: read-var d3:i64
        let s_19_6: i64 = fn_state.d3;
        // D s_19_7: cast zx s_19_6 -> i
        let s_19_7: i128 = (i128::try_from(s_19_6).unwrap());
        // D s_19_8: read-var d4:i64
        let s_19_8: i64 = fn_state.d4;
        // D s_19_9: cast zx s_19_8 -> i
        let s_19_9: i128 = (i128::try_from(s_19_8).unwrap());
        // D s_19_10: read-var d:i64
        let s_19_10: i64 = fn_state.d;
        // D s_19_11: read-var ebytesshadow#7483:i64
        let s_19_11: i64 = fn_state.ebytesshadow_7483;
        // D s_19_12: read-var m:i64
        let s_19_12: i64 = fn_state.m;
        // D s_19_13: read-var n:i64
        let s_19_13: i64 = fn_state.n;
        // D s_19_14: read-var register_index:u8
        let s_19_14: bool = fn_state.register_index;
        // D s_19_15: read-var wback:u8
        let s_19_15: bool = fn_state.wback;
        // D s_19_16: call execute_aarch32_instrs_VLD4_a_Op_A_txt(s_19_3, s_19_10, s_19_5, s_19_7, s_19_9, s_19_11, s_19_12, s_19_13, s_19_14, s_19_15)
        let s_19_16: () = execute_aarch32_instrs_VLD4_a_Op_A_txt(
            state,
            tracer,
            s_19_3,
            s_19_10,
            s_19_5,
            s_19_7,
            s_19_9,
            s_19_11,
            s_19_12,
            s_19_13,
            s_19_14,
            s_19_15,
        );
        // N s_19_17: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var n:i64
        let s_20_0: i64 = fn_state.n;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: call __id(s_20_1)
        let s_20_2: i128 = u__id(state, tracer, s_20_1);
        // D s_20_3: cast reint s_20_2 -> i64
        let s_20_3: i64 = (s_20_2 as i64);
        // C s_20_4: const #15s : i
        let s_20_4: i128 = 15;
        // D s_20_5: cast zx s_20_3 -> i
        let s_20_5: i128 = (i128::try_from(s_20_3).unwrap());
        // D s_20_6: cmp-le s_20_5 s_20_4
        let s_20_6: bool = ((s_20_5) <= (s_20_4));
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
        // D s_21_1: write-var gs#311496 <= s_21_0
        fn_state.gs_311496 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#311496:u8
        let s_22_0: bool = fn_state.gs_311496;
        // D s_22_1: write-var gs#311497 <= s_22_0
        fn_state.gs_311497 = s_22_0;
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
        // C s_23_4: const #0s : i
        let s_23_4: i128 = 0;
        // D s_23_5: cast zx s_23_3 -> i
        let s_23_5: i128 = (i128::try_from(s_23_3).unwrap());
        // D s_23_6: cmp-le s_23_4 s_23_5
        let s_23_6: bool = ((s_23_4) <= (s_23_5));
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
        // D s_24_1: write-var gs#311495 <= s_24_0
        fn_state.gs_311495 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#311495:u8
        let s_25_0: bool = fn_state.gs_311495;
        // D s_25_1: write-var gs#311496 <= s_25_0
        fn_state.gs_311496 = s_25_0;
        // N s_25_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var m:i64
        let s_26_0: i64 = fn_state.m;
        // D s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_2: call __id(s_26_1)
        let s_26_2: i128 = u__id(state, tracer, s_26_1);
        // D s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: const #15s : i
        let s_26_4: i128 = 15;
        // D s_26_5: cast zx s_26_3 -> i
        let s_26_5: i128 = (i128::try_from(s_26_3).unwrap());
        // D s_26_6: cmp-le s_26_5 s_26_4
        let s_26_6: bool = ((s_26_5) <= (s_26_4));
        // N s_26_7: branch s_26_6 b29 b27
        if s_26_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#311494 <= s_27_0
        fn_state.gs_311494 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#311494:u8
        let s_28_0: bool = fn_state.gs_311494;
        // D s_28_1: write-var gs#311495 <= s_28_0
        fn_state.gs_311495 = s_28_0;
        // N s_28_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var ebytesshadow#7483:i64
        let s_29_0: i64 = fn_state.ebytesshadow_7483;
        // D s_29_1: cast zx s_29_0 -> i
        let s_29_1: i128 = (i128::try_from(s_29_0).unwrap());
        // D s_29_2: call __id(s_29_1)
        let s_29_2: i128 = u__id(state, tracer, s_29_1);
        // D s_29_3: cast reint s_29_2 -> i64
        let s_29_3: i64 = (s_29_2 as i64);
        // C s_29_4: const #1s : i
        let s_29_4: i128 = 1;
        // D s_29_5: cast zx s_29_3 -> i
        let s_29_5: i128 = (i128::try_from(s_29_3).unwrap());
        // D s_29_6: cmp-eq s_29_5 s_29_4
        let s_29_6: bool = ((s_29_5) == (s_29_4));
        // N s_29_7: branch s_29_6 b59 b30
        if s_29_6 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var ebytesshadow#7483:i64
        let s_30_0: i64 = fn_state.ebytesshadow_7483;
        // D s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // D s_30_2: call __id(s_30_1)
        let s_30_2: i128 = u__id(state, tracer, s_30_1);
        // D s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: const #2s : i
        let s_30_4: i128 = 2;
        // D s_30_5: cast zx s_30_3 -> i
        let s_30_5: i128 = (i128::try_from(s_30_3).unwrap());
        // D s_30_6: cmp-eq s_30_5 s_30_4
        let s_30_6: bool = ((s_30_5) == (s_30_4));
        // D s_30_7: write-var gs#311475 <= s_30_6
        fn_state.gs_311475 = s_30_6;
        // N s_30_8: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#311475:u8
        let s_31_0: bool = fn_state.gs_311475;
        // N s_31_1: branch s_31_0 b58 b32
        if s_31_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var ebytesshadow#7483:i64
        let s_32_0: i64 = fn_state.ebytesshadow_7483;
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
        // D s_32_7: write-var gs#311477 <= s_32_6
        fn_state.gs_311477 = s_32_6;
        // N s_32_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#311477:u8
        let s_33_0: bool = fn_state.gs_311477;
        // N s_33_1: branch s_33_0 b57 b34
        if s_33_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var ebytesshadow#7483:i64
        let s_34_0: i64 = fn_state.ebytesshadow_7483;
        // D s_34_1: cast zx s_34_0 -> i
        let s_34_1: i128 = (i128::try_from(s_34_0).unwrap());
        // D s_34_2: call __id(s_34_1)
        let s_34_2: i128 = u__id(state, tracer, s_34_1);
        // D s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: const #8s : i
        let s_34_4: i128 = 8;
        // D s_34_5: cast zx s_34_3 -> i
        let s_34_5: i128 = (i128::try_from(s_34_3).unwrap());
        // D s_34_6: cmp-eq s_34_5 s_34_4
        let s_34_6: bool = ((s_34_5) == (s_34_4));
        // D s_34_7: write-var gs#311479 <= s_34_6
        fn_state.gs_311479 = s_34_6;
        // N s_34_8: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#311479:u8
        let s_35_0: bool = fn_state.gs_311479;
        // N s_35_1: branch s_35_0 b38 b36
        if s_35_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#311493 <= s_36_0
        fn_state.gs_311493 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#311493:u8
        let s_37_0: bool = fn_state.gs_311493;
        // D s_37_1: write-var gs#311494 <= s_37_0
        fn_state.gs_311494 = s_37_0;
        // N s_37_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var d:i64
        let s_38_0: i64 = fn_state.d;
        // D s_38_1: cast zx s_38_0 -> i
        let s_38_1: i128 = (i128::try_from(s_38_0).unwrap());
        // D s_38_2: call __id(s_38_1)
        let s_38_2: i128 = u__id(state, tracer, s_38_1);
        // D s_38_3: cast reint s_38_2 -> i64
        let s_38_3: i64 = (s_38_2 as i64);
        // C s_38_4: const #0s : i
        let s_38_4: i128 = 0;
        // D s_38_5: cast zx s_38_3 -> i
        let s_38_5: i128 = (i128::try_from(s_38_3).unwrap());
        // D s_38_6: cmp-le s_38_4 s_38_5
        let s_38_6: bool = ((s_38_4) <= (s_38_5));
        // N s_38_7: branch s_38_6 b41 b39
        if s_38_6 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#311492 <= s_39_0
        fn_state.gs_311492 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#311492:u8
        let s_40_0: bool = fn_state.gs_311492;
        // D s_40_1: write-var gs#311493 <= s_40_0
        fn_state.gs_311493 = s_40_0;
        // N s_40_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var d:i64
        let s_41_0: i64 = fn_state.d;
        // D s_41_1: cast zx s_41_0 -> i
        let s_41_1: i128 = (i128::try_from(s_41_0).unwrap());
        // D s_41_2: call __id(s_41_1)
        let s_41_2: i128 = u__id(state, tracer, s_41_1);
        // D s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: const #31s : i
        let s_41_4: i128 = 31;
        // D s_41_5: cast zx s_41_3 -> i
        let s_41_5: i128 = (i128::try_from(s_41_3).unwrap());
        // D s_41_6: cmp-le s_41_5 s_41_4
        let s_41_6: bool = ((s_41_5) <= (s_41_4));
        // N s_41_7: branch s_41_6 b44 b42
        if s_41_6 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#311491 <= s_42_0
        fn_state.gs_311491 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#311491:u8
        let s_43_0: bool = fn_state.gs_311491;
        // D s_43_1: write-var gs#311492 <= s_43_0
        fn_state.gs_311492 = s_43_0;
        // N s_43_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var alignmentshadow#7485:i
        let s_44_0: i128 = fn_state.alignmentshadow_7485;
        // D s_44_1: call __id(s_44_0)
        let s_44_1: i128 = u__id(state, tracer, s_44_0);
        // C s_44_2: const #1s : i
        let s_44_2: i128 = 1;
        // D s_44_3: cmp-eq s_44_1 s_44_2
        let s_44_3: bool = ((s_44_1) == (s_44_2));
        // N s_44_4: branch s_44_3 b56 b45
        if s_44_3 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var alignmentshadow#7485:i
        let s_45_0: i128 = fn_state.alignmentshadow_7485;
        // D s_45_1: call __id(s_45_0)
        let s_45_1: i128 = u__id(state, tracer, s_45_0);
        // C s_45_2: const #4s : i
        let s_45_2: i128 = 4;
        // D s_45_3: cmp-eq s_45_1 s_45_2
        let s_45_3: bool = ((s_45_1) == (s_45_2));
        // D s_45_4: write-var gs#311484 <= s_45_3
        fn_state.gs_311484 = s_45_3;
        // N s_45_5: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#311484:u8
        let s_46_0: bool = fn_state.gs_311484;
        // N s_46_1: branch s_46_0 b55 b47
        if s_46_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var alignmentshadow#7485:i
        let s_47_0: i128 = fn_state.alignmentshadow_7485;
        // D s_47_1: call __id(s_47_0)
        let s_47_1: i128 = u__id(state, tracer, s_47_0);
        // C s_47_2: const #8s : i
        let s_47_2: i128 = 8;
        // D s_47_3: cmp-eq s_47_1 s_47_2
        let s_47_3: bool = ((s_47_1) == (s_47_2));
        // D s_47_4: write-var gs#311486 <= s_47_3
        fn_state.gs_311486 = s_47_3;
        // N s_47_5: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#311486:u8
        let s_48_0: bool = fn_state.gs_311486;
        // N s_48_1: branch s_48_0 b54 b49
        if s_48_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var alignmentshadow#7485:i
        let s_49_0: i128 = fn_state.alignmentshadow_7485;
        // D s_49_1: call __id(s_49_0)
        let s_49_1: i128 = u__id(state, tracer, s_49_0);
        // C s_49_2: const #16s : i
        let s_49_2: i128 = 16;
        // D s_49_3: cmp-eq s_49_1 s_49_2
        let s_49_3: bool = ((s_49_1) == (s_49_2));
        // D s_49_4: write-var gs#311488 <= s_49_3
        fn_state.gs_311488 = s_49_3;
        // N s_49_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#311488:u8
        let s_50_0: bool = fn_state.gs_311488;
        // N s_50_1: branch s_50_0 b53 b51
        if s_50_0 {
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
        // D s_51_0: read-var alignmentshadow#7485:i
        let s_51_0: i128 = fn_state.alignmentshadow_7485;
        // D s_51_1: call __id(s_51_0)
        let s_51_1: i128 = u__id(state, tracer, s_51_0);
        // C s_51_2: const #32s : i
        let s_51_2: i128 = 32;
        // D s_51_3: cmp-eq s_51_1 s_51_2
        let s_51_3: bool = ((s_51_1) == (s_51_2));
        // D s_51_4: write-var gs#311490 <= s_51_3
        fn_state.gs_311490 = s_51_3;
        // N s_51_5: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#311490:u8
        let s_52_0: bool = fn_state.gs_311490;
        // D s_52_1: write-var gs#311491 <= s_52_0
        fn_state.gs_311491 = s_52_0;
        // N s_52_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var gs#311490 <= s_53_0
        fn_state.gs_311490 = s_53_0;
        // N s_53_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#311488 <= s_54_0
        fn_state.gs_311488 = s_54_0;
        // N s_54_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#311486 <= s_55_0
        fn_state.gs_311486 = s_55_0;
        // N s_55_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#311484 <= s_56_0
        fn_state.gs_311484 = s_56_0;
        // N s_56_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var gs#311479 <= s_57_0
        fn_state.gs_311479 = s_57_0;
        // N s_57_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#311477 <= s_58_0
        fn_state.gs_311477 = s_58_0;
        // N s_58_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #1u : u8
        let s_59_0: bool = true;
        // D s_59_1: write-var gs#311475 <= s_59_0
        fn_state.gs_311475 = s_59_0;
        // N s_59_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_60_0: panic
        panic!("{:?}", ());
        // N s_60_1: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#311467 <= s_61_0
        fn_state.gs_311467 = s_61_0;
        // N s_61_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #13s : i
        let s_62_0: i128 = 13;
        // D s_62_1: read-var m:i64
        let s_62_1: i64 = fn_state.m;
        // D s_62_2: cast zx s_62_1 -> i
        let s_62_2: i128 = (i128::try_from(s_62_1).unwrap());
        // D s_62_3: call neq_int(s_62_2, s_62_0)
        let s_62_3: bool = neq_int(state, tracer, s_62_2, s_62_0);
        // D s_62_4: write-var gs#311464 <= s_62_3
        fn_state.gs_311464 = s_62_3;
        // N s_62_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #1s : i64
        let s_63_0: i64 = 1;
        // D s_63_1: write-var ga#354892 <= s_63_0
        fn_state.ga_354892 = s_63_0;
        // N s_63_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #1s : i
        let s_64_0: i128 = 1;
        // D s_64_1: write-var alignment <= s_64_0
        fn_state.alignment = s_64_0;
        // N s_64_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var a:u8
        let s_65_0: bool = fn_state.a;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 1u16);
        // C s_65_2: const #0u : u8
        let s_65_2: bool = false;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // N s_65_5: branch s_65_4 b68 b66
        if s_65_4 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #8s : i
        let s_66_0: i128 = 8;
        // D s_66_1: write-var alignment <= s_66_0
        fn_state.alignment = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_67_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #1s : i
        let s_68_0: i128 = 1;
        // D s_68_1: write-var alignment <= s_68_0
        fn_state.alignment = s_68_0;
        // N s_68_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #4s : i64
        let s_69_0: i64 = 4;
        // D s_69_1: write-var ebytes <= s_69_0
        fn_state.ebytes = s_69_0;
        // C s_69_2: const #16s : i
        let s_69_2: i128 = 16;
        // D s_69_3: write-var alignment <= s_69_2
        fn_state.alignment = s_69_2;
        // N s_69_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_70_0: panic
        panic!("{:?}", ());
        // N s_70_1: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var a:u8
        let s_71_0: bool = fn_state.a;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #0u : u8
        let s_71_2: bool = false;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#311437 <= s_71_4
        fn_state.gs_311437 = s_71_4;
        // N s_71_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
