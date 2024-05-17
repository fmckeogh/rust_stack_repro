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
use CurrentVL_read::*;
use u__id::*;
use HaveSME2::*;
use execute_SQCVTN_Z_MZ4__::*;
use common::*;
pub fn decode_SQCVTN_Z_MZ4__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    sz: bool,
    Zn: u8,
    N: bool,
    U: bool,
    Zd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_287669: bool,
        gs_287637: bool,
        gs_287633: bool,
        gs_287639: bool,
        gs_287656: bool,
        gs_287654: bool,
        gs_287684: bool,
        gs_287653: bool,
        gs_287624: bool,
        gs_287636: bool,
        gs_287640: bool,
        gs_287687: bool,
        gs_287652: bool,
        gs_287649: bool,
        esize: i64,
        gs_287681: bool,
        gs_287672: bool,
        n: i64,
        gs_287670: bool,
        gs_287685: bool,
        gs_287668: bool,
        gs_287671: bool,
        gs_287655: bool,
        VL: i64,
        gs_287665: bool,
        gs_287622: bool,
        d: i64,
        gs_287686: bool,
        gs_287620: bool,
        gs_287688: bool,
        gs_287623: bool,
        gs_287638: bool,
        gs_287617: bool,
        gs_287621: bool,
        sz: bool,
        Zn: u8,
        N: bool,
        U: bool,
        Zd: u8,
    }
    let fn_state = FunctionState {
        sz,
        Zn,
        N,
        U,
        Zd,
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
        // S s_0_1: call CurrentVL_read(s_0_0)
        let s_0_1: i64 = CurrentVL_read(state, tracer, s_0_0);
        // D s_0_2: write-var VL <= s_0_1
        fn_state.VL = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call HaveSME2(s_0_3)
        let s_0_4: bool = HaveSME2(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b102 b1
        if s_0_5 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var sz:u8
        let s_1_0: bool = fn_state.sz;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #8s : i64
        let s_1_4: i64 = 8;
        // D s_1_5: lsl s_1_4 s_1_3
        let s_1_5: i64 = s_1_4 << s_1_3;
        // D s_1_6: write-var esize <= s_1_5
        fn_state.esize = s_1_5;
        // D s_1_7: read-var Zn:u8
        let s_1_7: u8 = fn_state.Zn;
        // D s_1_8: cast zx s_1_7 -> bv
        let s_1_8: Bits = Bits::new(s_1_7 as u128, 3u16);
        // C s_1_9: const #0u : u8
        let s_1_9: u8 = 0;
        // C s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 2u16);
        // D s_1_11: cast reint s_1_8 -> u128
        let s_1_11: u128 = (s_1_8.value() as u128);
        // D s_1_12: size-of s_1_8
        let s_1_12: u16 = s_1_8.length();
        // C s_1_13: cast reint s_1_10 -> u128
        let s_1_13: u128 = (s_1_10.value() as u128);
        // D s_1_14: size-of s_1_10
        let s_1_14: u16 = s_1_10.length();
        // D s_1_15: lsl s_1_11 s_1_14
        let s_1_15: u128 = s_1_11 << s_1_14;
        // D s_1_16: or s_1_15 s_1_13
        let s_1_16: u128 = ((s_1_15) | (s_1_13));
        // D s_1_17: add s_1_12 s_1_14
        let s_1_17: u16 = (s_1_12 + s_1_14);
        // D s_1_18: create-bits s_1_16 s_1_17
        let s_1_18: Bits = Bits::new(s_1_16, s_1_17);
        // D s_1_19: cast reint s_1_18 -> u8
        let s_1_19: u8 = (s_1_18.value() as u8);
        // D s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 5u16);
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (s_1_20.value() as i128);
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: write-var n <= s_1_22
        fn_state.n = s_1_22;
        // D s_1_24: read-var Zd:u8
        let s_1_24: u8 = fn_state.Zd;
        // D s_1_25: cast zx s_1_24 -> bv
        let s_1_25: Bits = Bits::new(s_1_24 as u128, 5u16);
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (s_1_25.value() as i128);
        // D s_1_27: cast reint s_1_26 -> i64
        let s_1_27: i64 = (s_1_26 as i64);
        // D s_1_28: write-var d <= s_1_27
        fn_state.d = s_1_27;
        // D s_1_29: read-var VL:i64
        let s_1_29: i64 = fn_state.VL;
        // C s_1_30: const #128s : i
        let s_1_30: i128 = 128;
        // D s_1_31: cast zx s_1_29 -> i
        let s_1_31: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_32: cmp-eq s_1_31 s_1_30
        let s_1_32: bool = ((s_1_31) == (s_1_30));
        // D s_1_33: not s_1_32
        let s_1_33: bool = !s_1_32;
        // N s_1_34: branch s_1_33 b21 b2
        if s_1_33 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var n:i64
        let s_2_0: i64 = fn_state.n;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call __id(s_2_1)
        let s_2_2: i128 = u__id(state, tracer, s_2_1);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #0s : i
        let s_2_4: i128 = 0;
        // D s_2_5: cast zx s_2_3 -> i
        let s_2_5: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_6: cmp-le s_2_4 s_2_5
        let s_2_6: bool = ((s_2_4) <= (s_2_5));
        // N s_2_7: branch s_2_6 b5 b3
        if s_2_6 {
            return block_5(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#287624 <= s_3_0
        fn_state.gs_287624 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#287624:u8
        let s_4_0: bool = fn_state.gs_287624;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #128s : i64
        let s_4_2: i64 = 128;
        // D s_4_3: read-var esize:i64
        let s_4_3: i64 = fn_state.esize;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: read-var d:i64
        let s_4_6: i64 = fn_state.d;
        // D s_4_7: read-var n:i64
        let s_4_7: i64 = fn_state.n;
        // D s_4_8: call execute_SQCVTN_Z_MZ4__(s_4_2, s_4_6, s_4_5, s_4_7)
        let s_4_8: () = execute_SQCVTN_Z_MZ4__(
            state,
            tracer,
            s_4_2,
            s_4_6,
            s_4_5,
            s_4_7,
        );
        // N s_4_9: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call __id(s_5_1)
        let s_5_2: i128 = u__id(state, tracer, s_5_1);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: const #31s : i
        let s_5_4: i128 = 31;
        // D s_5_5: cast zx s_5_3 -> i
        let s_5_5: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_6: cmp-le s_5_5 s_5_4
        let s_5_6: bool = ((s_5_5) <= (s_5_4));
        // N s_5_7: branch s_5_6 b8 b6
        if s_5_6 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#287623 <= s_6_0
        fn_state.gs_287623 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#287623:u8
        let s_7_0: bool = fn_state.gs_287623;
        // D s_7_1: write-var gs#287624 <= s_7_0
        fn_state.gs_287624 = s_7_0;
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esize:i64
        let s_8_0: i64 = fn_state.esize;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call __id(s_8_1)
        let s_8_2: i128 = u__id(state, tracer, s_8_1);
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // C s_8_4: const #8s : i
        let s_8_4: i128 = 8;
        // D s_8_5: cast zx s_8_3 -> i
        let s_8_5: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_6: cmp-eq s_8_5 s_8_4
        let s_8_6: bool = ((s_8_5) == (s_8_4));
        // N s_8_7: branch s_8_6 b20 b9
        if s_8_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esize:i64
        let s_9_0: i64 = fn_state.esize;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call __id(s_9_1)
        let s_9_2: i128 = u__id(state, tracer, s_9_1);
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #16s : i
        let s_9_4: i128 = 16;
        // D s_9_5: cast zx s_9_3 -> i
        let s_9_5: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_6: cmp-eq s_9_5 s_9_4
        let s_9_6: bool = ((s_9_5) == (s_9_4));
        // D s_9_7: write-var gs#287617 <= s_9_6
        fn_state.gs_287617 = s_9_6;
        // N s_9_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#287617:u8
        let s_10_0: bool = fn_state.gs_287617;
        // N s_10_1: branch s_10_0 b13 b11
        if s_10_0 {
            return block_13(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#287622 <= s_11_0
        fn_state.gs_287622 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#287622:u8
        let s_12_0: bool = fn_state.gs_287622;
        // D s_12_1: write-var gs#287623 <= s_12_0
        fn_state.gs_287623 = s_12_0;
        // N s_12_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var d:i64
        let s_13_0: i64 = fn_state.d;
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
        // D s_14_1: write-var gs#287621 <= s_14_0
        fn_state.gs_287621 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#287621:u8
        let s_15_0: bool = fn_state.gs_287621;
        // D s_15_1: write-var gs#287622 <= s_15_0
        fn_state.gs_287622 = s_15_0;
        // N s_15_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var d:i64
        let s_16_0: i64 = fn_state.d;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: call __id(s_16_1)
        let s_16_2: i128 = u__id(state, tracer, s_16_1);
        // D s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // C s_16_4: const #31s : i
        let s_16_4: i128 = 31;
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
        // D s_17_1: write-var gs#287620 <= s_17_0
        fn_state.gs_287620 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#287620:u8
        let s_18_0: bool = fn_state.gs_287620;
        // D s_18_1: write-var gs#287621 <= s_18_0
        fn_state.gs_287621 = s_18_0;
        // N s_18_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#287620 <= s_19_0
        fn_state.gs_287620 = s_19_0;
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#287617 <= s_20_0
        fn_state.gs_287617 = s_20_0;
        // N s_20_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var VL:i64
        let s_21_0: i64 = fn_state.VL;
        // C s_21_1: const #256s : i
        let s_21_1: i128 = 256;
        // D s_21_2: cast zx s_21_0 -> i
        let s_21_2: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_3: cmp-eq s_21_2 s_21_1
        let s_21_3: bool = ((s_21_2) == (s_21_1));
        // D s_21_4: not s_21_3
        let s_21_4: bool = !s_21_3;
        // N s_21_5: branch s_21_4 b41 b22
        if s_21_4 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var n:i64
        let s_22_0: i64 = fn_state.n;
        // D s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_2: call __id(s_22_1)
        let s_22_2: i128 = u__id(state, tracer, s_22_1);
        // D s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: const #0s : i
        let s_22_4: i128 = 0;
        // D s_22_5: cast zx s_22_3 -> i
        let s_22_5: i128 = (i128::try_from(s_22_3).unwrap());
        // D s_22_6: cmp-le s_22_4 s_22_5
        let s_22_6: bool = ((s_22_4) <= (s_22_5));
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
        // D s_23_1: write-var gs#287640 <= s_23_0
        fn_state.gs_287640 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#287640:u8
        let s_24_0: bool = fn_state.gs_287640;
        // N s_24_1: assert s_24_0
        let s_24_1: () = assert!(s_24_0);
        // C s_24_2: const #256s : i64
        let s_24_2: i64 = 256;
        // D s_24_3: read-var esize:i64
        let s_24_3: i64 = fn_state.esize;
        // D s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // D s_24_5: cast reint s_24_4 -> i64
        let s_24_5: i64 = (s_24_4 as i64);
        // D s_24_6: read-var d:i64
        let s_24_6: i64 = fn_state.d;
        // D s_24_7: read-var n:i64
        let s_24_7: i64 = fn_state.n;
        // D s_24_8: call execute_SQCVTN_Z_MZ4__(s_24_2, s_24_6, s_24_5, s_24_7)
        let s_24_8: () = execute_SQCVTN_Z_MZ4__(
            state,
            tracer,
            s_24_2,
            s_24_6,
            s_24_5,
            s_24_7,
        );
        // N s_24_9: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var n:i64
        let s_25_0: i64 = fn_state.n;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: call __id(s_25_1)
        let s_25_2: i128 = u__id(state, tracer, s_25_1);
        // D s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: const #31s : i
        let s_25_4: i128 = 31;
        // D s_25_5: cast zx s_25_3 -> i
        let s_25_5: i128 = (i128::try_from(s_25_3).unwrap());
        // D s_25_6: cmp-le s_25_5 s_25_4
        let s_25_6: bool = ((s_25_5) <= (s_25_4));
        // N s_25_7: branch s_25_6 b28 b26
        if s_25_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#287639 <= s_26_0
        fn_state.gs_287639 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#287639:u8
        let s_27_0: bool = fn_state.gs_287639;
        // D s_27_1: write-var gs#287640 <= s_27_0
        fn_state.gs_287640 = s_27_0;
        // N s_27_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var esize:i64
        let s_28_0: i64 = fn_state.esize;
        // D s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: call __id(s_28_1)
        let s_28_2: i128 = u__id(state, tracer, s_28_1);
        // D s_28_3: cast reint s_28_2 -> i64
        let s_28_3: i64 = (s_28_2 as i64);
        // C s_28_4: const #8s : i
        let s_28_4: i128 = 8;
        // D s_28_5: cast zx s_28_3 -> i
        let s_28_5: i128 = (i128::try_from(s_28_3).unwrap());
        // D s_28_6: cmp-eq s_28_5 s_28_4
        let s_28_6: bool = ((s_28_5) == (s_28_4));
        // N s_28_7: branch s_28_6 b40 b29
        if s_28_6 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var esize:i64
        let s_29_0: i64 = fn_state.esize;
        // D s_29_1: cast zx s_29_0 -> i
        let s_29_1: i128 = (i128::try_from(s_29_0).unwrap());
        // D s_29_2: call __id(s_29_1)
        let s_29_2: i128 = u__id(state, tracer, s_29_1);
        // D s_29_3: cast reint s_29_2 -> i64
        let s_29_3: i64 = (s_29_2 as i64);
        // C s_29_4: const #16s : i
        let s_29_4: i128 = 16;
        // D s_29_5: cast zx s_29_3 -> i
        let s_29_5: i128 = (i128::try_from(s_29_3).unwrap());
        // D s_29_6: cmp-eq s_29_5 s_29_4
        let s_29_6: bool = ((s_29_5) == (s_29_4));
        // D s_29_7: write-var gs#287633 <= s_29_6
        fn_state.gs_287633 = s_29_6;
        // N s_29_8: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#287633:u8
        let s_30_0: bool = fn_state.gs_287633;
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
        // D s_31_1: write-var gs#287638 <= s_31_0
        fn_state.gs_287638 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#287638:u8
        let s_32_0: bool = fn_state.gs_287638;
        // D s_32_1: write-var gs#287639 <= s_32_0
        fn_state.gs_287639 = s_32_0;
        // N s_32_2: jump b27
        return block_27(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#287637 <= s_34_0
        fn_state.gs_287637 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#287637:u8
        let s_35_0: bool = fn_state.gs_287637;
        // D s_35_1: write-var gs#287638 <= s_35_0
        fn_state.gs_287638 = s_35_0;
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
        // D s_37_1: write-var gs#287636 <= s_37_0
        fn_state.gs_287636 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#287636:u8
        let s_38_0: bool = fn_state.gs_287636;
        // D s_38_1: write-var gs#287637 <= s_38_0
        fn_state.gs_287637 = s_38_0;
        // N s_38_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#287636 <= s_39_0
        fn_state.gs_287636 = s_39_0;
        // N s_39_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#287633 <= s_40_0
        fn_state.gs_287633 = s_40_0;
        // N s_40_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var VL:i64
        let s_41_0: i64 = fn_state.VL;
        // C s_41_1: const #512s : i
        let s_41_1: i128 = 512;
        // D s_41_2: cast zx s_41_0 -> i
        let s_41_2: i128 = (i128::try_from(s_41_0).unwrap());
        // D s_41_3: cmp-eq s_41_2 s_41_1
        let s_41_3: bool = ((s_41_2) == (s_41_1));
        // D s_41_4: not s_41_3
        let s_41_4: bool = !s_41_3;
        // N s_41_5: branch s_41_4 b61 b42
        if s_41_4 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var n:i64
        let s_42_0: i64 = fn_state.n;
        // D s_42_1: cast zx s_42_0 -> i
        let s_42_1: i128 = (i128::try_from(s_42_0).unwrap());
        // D s_42_2: call __id(s_42_1)
        let s_42_2: i128 = u__id(state, tracer, s_42_1);
        // D s_42_3: cast reint s_42_2 -> i64
        let s_42_3: i64 = (s_42_2 as i64);
        // C s_42_4: const #0s : i
        let s_42_4: i128 = 0;
        // D s_42_5: cast zx s_42_3 -> i
        let s_42_5: i128 = (i128::try_from(s_42_3).unwrap());
        // D s_42_6: cmp-le s_42_4 s_42_5
        let s_42_6: bool = ((s_42_4) <= (s_42_5));
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
        // D s_43_1: write-var gs#287656 <= s_43_0
        fn_state.gs_287656 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#287656:u8
        let s_44_0: bool = fn_state.gs_287656;
        // N s_44_1: assert s_44_0
        let s_44_1: () = assert!(s_44_0);
        // C s_44_2: const #512s : i64
        let s_44_2: i64 = 512;
        // D s_44_3: read-var esize:i64
        let s_44_3: i64 = fn_state.esize;
        // D s_44_4: cast zx s_44_3 -> i
        let s_44_4: i128 = (i128::try_from(s_44_3).unwrap());
        // D s_44_5: cast reint s_44_4 -> i64
        let s_44_5: i64 = (s_44_4 as i64);
        // D s_44_6: read-var d:i64
        let s_44_6: i64 = fn_state.d;
        // D s_44_7: read-var n:i64
        let s_44_7: i64 = fn_state.n;
        // D s_44_8: call execute_SQCVTN_Z_MZ4__(s_44_2, s_44_6, s_44_5, s_44_7)
        let s_44_8: () = execute_SQCVTN_Z_MZ4__(
            state,
            tracer,
            s_44_2,
            s_44_6,
            s_44_5,
            s_44_7,
        );
        // N s_44_9: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var n:i64
        let s_45_0: i64 = fn_state.n;
        // D s_45_1: cast zx s_45_0 -> i
        let s_45_1: i128 = (i128::try_from(s_45_0).unwrap());
        // D s_45_2: call __id(s_45_1)
        let s_45_2: i128 = u__id(state, tracer, s_45_1);
        // D s_45_3: cast reint s_45_2 -> i64
        let s_45_3: i64 = (s_45_2 as i64);
        // C s_45_4: const #31s : i
        let s_45_4: i128 = 31;
        // D s_45_5: cast zx s_45_3 -> i
        let s_45_5: i128 = (i128::try_from(s_45_3).unwrap());
        // D s_45_6: cmp-le s_45_5 s_45_4
        let s_45_6: bool = ((s_45_5) <= (s_45_4));
        // N s_45_7: branch s_45_6 b48 b46
        if s_45_6 {
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
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#287655 <= s_46_0
        fn_state.gs_287655 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#287655:u8
        let s_47_0: bool = fn_state.gs_287655;
        // D s_47_1: write-var gs#287656 <= s_47_0
        fn_state.gs_287656 = s_47_0;
        // N s_47_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var esize:i64
        let s_48_0: i64 = fn_state.esize;
        // D s_48_1: cast zx s_48_0 -> i
        let s_48_1: i128 = (i128::try_from(s_48_0).unwrap());
        // D s_48_2: call __id(s_48_1)
        let s_48_2: i128 = u__id(state, tracer, s_48_1);
        // D s_48_3: cast reint s_48_2 -> i64
        let s_48_3: i64 = (s_48_2 as i64);
        // C s_48_4: const #8s : i
        let s_48_4: i128 = 8;
        // D s_48_5: cast zx s_48_3 -> i
        let s_48_5: i128 = (i128::try_from(s_48_3).unwrap());
        // D s_48_6: cmp-eq s_48_5 s_48_4
        let s_48_6: bool = ((s_48_5) == (s_48_4));
        // N s_48_7: branch s_48_6 b60 b49
        if s_48_6 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var esize:i64
        let s_49_0: i64 = fn_state.esize;
        // D s_49_1: cast zx s_49_0 -> i
        let s_49_1: i128 = (i128::try_from(s_49_0).unwrap());
        // D s_49_2: call __id(s_49_1)
        let s_49_2: i128 = u__id(state, tracer, s_49_1);
        // D s_49_3: cast reint s_49_2 -> i64
        let s_49_3: i64 = (s_49_2 as i64);
        // C s_49_4: const #16s : i
        let s_49_4: i128 = 16;
        // D s_49_5: cast zx s_49_3 -> i
        let s_49_5: i128 = (i128::try_from(s_49_3).unwrap());
        // D s_49_6: cmp-eq s_49_5 s_49_4
        let s_49_6: bool = ((s_49_5) == (s_49_4));
        // D s_49_7: write-var gs#287649 <= s_49_6
        fn_state.gs_287649 = s_49_6;
        // N s_49_8: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#287649:u8
        let s_50_0: bool = fn_state.gs_287649;
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
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#287654 <= s_51_0
        fn_state.gs_287654 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#287654:u8
        let s_52_0: bool = fn_state.gs_287654;
        // D s_52_1: write-var gs#287655 <= s_52_0
        fn_state.gs_287655 = s_52_0;
        // N s_52_2: jump b47
        return block_47(state, tracer, fn_state);
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
        // C s_53_4: const #0s : i
        let s_53_4: i128 = 0;
        // D s_53_5: cast zx s_53_3 -> i
        let s_53_5: i128 = (i128::try_from(s_53_3).unwrap());
        // D s_53_6: cmp-le s_53_4 s_53_5
        let s_53_6: bool = ((s_53_4) <= (s_53_5));
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
        // D s_54_1: write-var gs#287653 <= s_54_0
        fn_state.gs_287653 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#287653:u8
        let s_55_0: bool = fn_state.gs_287653;
        // D s_55_1: write-var gs#287654 <= s_55_0
        fn_state.gs_287654 = s_55_0;
        // N s_55_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var d:i64
        let s_56_0: i64 = fn_state.d;
        // D s_56_1: cast zx s_56_0 -> i
        let s_56_1: i128 = (i128::try_from(s_56_0).unwrap());
        // D s_56_2: call __id(s_56_1)
        let s_56_2: i128 = u__id(state, tracer, s_56_1);
        // D s_56_3: cast reint s_56_2 -> i64
        let s_56_3: i64 = (s_56_2 as i64);
        // C s_56_4: const #31s : i
        let s_56_4: i128 = 31;
        // D s_56_5: cast zx s_56_3 -> i
        let s_56_5: i128 = (i128::try_from(s_56_3).unwrap());
        // D s_56_6: cmp-le s_56_5 s_56_4
        let s_56_6: bool = ((s_56_5) <= (s_56_4));
        // N s_56_7: branch s_56_6 b59 b57
        if s_56_6 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#287652 <= s_57_0
        fn_state.gs_287652 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#287652:u8
        let s_58_0: bool = fn_state.gs_287652;
        // D s_58_1: write-var gs#287653 <= s_58_0
        fn_state.gs_287653 = s_58_0;
        // N s_58_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #1u : u8
        let s_59_0: bool = true;
        // D s_59_1: write-var gs#287652 <= s_59_0
        fn_state.gs_287652 = s_59_0;
        // N s_59_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#287649 <= s_60_0
        fn_state.gs_287649 = s_60_0;
        // N s_60_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var VL:i64
        let s_61_0: i64 = fn_state.VL;
        // C s_61_1: const #1024s : i
        let s_61_1: i128 = 1024;
        // D s_61_2: cast zx s_61_0 -> i
        let s_61_2: i128 = (i128::try_from(s_61_0).unwrap());
        // D s_61_3: cmp-eq s_61_2 s_61_1
        let s_61_3: bool = ((s_61_2) == (s_61_1));
        // D s_61_4: not s_61_3
        let s_61_4: bool = !s_61_3;
        // N s_61_5: branch s_61_4 b81 b62
        if s_61_4 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var n:i64
        let s_62_0: i64 = fn_state.n;
        // D s_62_1: cast zx s_62_0 -> i
        let s_62_1: i128 = (i128::try_from(s_62_0).unwrap());
        // D s_62_2: call __id(s_62_1)
        let s_62_2: i128 = u__id(state, tracer, s_62_1);
        // D s_62_3: cast reint s_62_2 -> i64
        let s_62_3: i64 = (s_62_2 as i64);
        // C s_62_4: const #0s : i
        let s_62_4: i128 = 0;
        // D s_62_5: cast zx s_62_3 -> i
        let s_62_5: i128 = (i128::try_from(s_62_3).unwrap());
        // D s_62_6: cmp-le s_62_4 s_62_5
        let s_62_6: bool = ((s_62_4) <= (s_62_5));
        // N s_62_7: branch s_62_6 b65 b63
        if s_62_6 {
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
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#287672 <= s_63_0
        fn_state.gs_287672 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#287672:u8
        let s_64_0: bool = fn_state.gs_287672;
        // N s_64_1: assert s_64_0
        let s_64_1: () = assert!(s_64_0);
        // C s_64_2: const #1024s : i64
        let s_64_2: i64 = 1024;
        // D s_64_3: read-var esize:i64
        let s_64_3: i64 = fn_state.esize;
        // D s_64_4: cast zx s_64_3 -> i
        let s_64_4: i128 = (i128::try_from(s_64_3).unwrap());
        // D s_64_5: cast reint s_64_4 -> i64
        let s_64_5: i64 = (s_64_4 as i64);
        // D s_64_6: read-var d:i64
        let s_64_6: i64 = fn_state.d;
        // D s_64_7: read-var n:i64
        let s_64_7: i64 = fn_state.n;
        // D s_64_8: call execute_SQCVTN_Z_MZ4__(s_64_2, s_64_6, s_64_5, s_64_7)
        let s_64_8: () = execute_SQCVTN_Z_MZ4__(
            state,
            tracer,
            s_64_2,
            s_64_6,
            s_64_5,
            s_64_7,
        );
        // N s_64_9: return
        return;
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var n:i64
        let s_65_0: i64 = fn_state.n;
        // D s_65_1: cast zx s_65_0 -> i
        let s_65_1: i128 = (i128::try_from(s_65_0).unwrap());
        // D s_65_2: call __id(s_65_1)
        let s_65_2: i128 = u__id(state, tracer, s_65_1);
        // D s_65_3: cast reint s_65_2 -> i64
        let s_65_3: i64 = (s_65_2 as i64);
        // C s_65_4: const #31s : i
        let s_65_4: i128 = 31;
        // D s_65_5: cast zx s_65_3 -> i
        let s_65_5: i128 = (i128::try_from(s_65_3).unwrap());
        // D s_65_6: cmp-le s_65_5 s_65_4
        let s_65_6: bool = ((s_65_5) <= (s_65_4));
        // N s_65_7: branch s_65_6 b68 b66
        if s_65_6 {
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
        // C s_66_0: const #0u : u8
        let s_66_0: bool = false;
        // D s_66_1: write-var gs#287671 <= s_66_0
        fn_state.gs_287671 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#287671:u8
        let s_67_0: bool = fn_state.gs_287671;
        // D s_67_1: write-var gs#287672 <= s_67_0
        fn_state.gs_287672 = s_67_0;
        // N s_67_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var esize:i64
        let s_68_0: i64 = fn_state.esize;
        // D s_68_1: cast zx s_68_0 -> i
        let s_68_1: i128 = (i128::try_from(s_68_0).unwrap());
        // D s_68_2: call __id(s_68_1)
        let s_68_2: i128 = u__id(state, tracer, s_68_1);
        // D s_68_3: cast reint s_68_2 -> i64
        let s_68_3: i64 = (s_68_2 as i64);
        // C s_68_4: const #8s : i
        let s_68_4: i128 = 8;
        // D s_68_5: cast zx s_68_3 -> i
        let s_68_5: i128 = (i128::try_from(s_68_3).unwrap());
        // D s_68_6: cmp-eq s_68_5 s_68_4
        let s_68_6: bool = ((s_68_5) == (s_68_4));
        // N s_68_7: branch s_68_6 b80 b69
        if s_68_6 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var esize:i64
        let s_69_0: i64 = fn_state.esize;
        // D s_69_1: cast zx s_69_0 -> i
        let s_69_1: i128 = (i128::try_from(s_69_0).unwrap());
        // D s_69_2: call __id(s_69_1)
        let s_69_2: i128 = u__id(state, tracer, s_69_1);
        // D s_69_3: cast reint s_69_2 -> i64
        let s_69_3: i64 = (s_69_2 as i64);
        // C s_69_4: const #16s : i
        let s_69_4: i128 = 16;
        // D s_69_5: cast zx s_69_3 -> i
        let s_69_5: i128 = (i128::try_from(s_69_3).unwrap());
        // D s_69_6: cmp-eq s_69_5 s_69_4
        let s_69_6: bool = ((s_69_5) == (s_69_4));
        // D s_69_7: write-var gs#287665 <= s_69_6
        fn_state.gs_287665 = s_69_6;
        // N s_69_8: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#287665:u8
        let s_70_0: bool = fn_state.gs_287665;
        // N s_70_1: branch s_70_0 b73 b71
        if s_70_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#287670 <= s_71_0
        fn_state.gs_287670 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#287670:u8
        let s_72_0: bool = fn_state.gs_287670;
        // D s_72_1: write-var gs#287671 <= s_72_0
        fn_state.gs_287671 = s_72_0;
        // N s_72_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var d:i64
        let s_73_0: i64 = fn_state.d;
        // D s_73_1: cast zx s_73_0 -> i
        let s_73_1: i128 = (i128::try_from(s_73_0).unwrap());
        // D s_73_2: call __id(s_73_1)
        let s_73_2: i128 = u__id(state, tracer, s_73_1);
        // D s_73_3: cast reint s_73_2 -> i64
        let s_73_3: i64 = (s_73_2 as i64);
        // C s_73_4: const #0s : i
        let s_73_4: i128 = 0;
        // D s_73_5: cast zx s_73_3 -> i
        let s_73_5: i128 = (i128::try_from(s_73_3).unwrap());
        // D s_73_6: cmp-le s_73_4 s_73_5
        let s_73_6: bool = ((s_73_4) <= (s_73_5));
        // N s_73_7: branch s_73_6 b76 b74
        if s_73_6 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var gs#287669 <= s_74_0
        fn_state.gs_287669 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#287669:u8
        let s_75_0: bool = fn_state.gs_287669;
        // D s_75_1: write-var gs#287670 <= s_75_0
        fn_state.gs_287670 = s_75_0;
        // N s_75_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var d:i64
        let s_76_0: i64 = fn_state.d;
        // D s_76_1: cast zx s_76_0 -> i
        let s_76_1: i128 = (i128::try_from(s_76_0).unwrap());
        // D s_76_2: call __id(s_76_1)
        let s_76_2: i128 = u__id(state, tracer, s_76_1);
        // D s_76_3: cast reint s_76_2 -> i64
        let s_76_3: i64 = (s_76_2 as i64);
        // C s_76_4: const #31s : i
        let s_76_4: i128 = 31;
        // D s_76_5: cast zx s_76_3 -> i
        let s_76_5: i128 = (i128::try_from(s_76_3).unwrap());
        // D s_76_6: cmp-le s_76_5 s_76_4
        let s_76_6: bool = ((s_76_5) <= (s_76_4));
        // N s_76_7: branch s_76_6 b79 b77
        if s_76_6 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#287668 <= s_77_0
        fn_state.gs_287668 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#287668:u8
        let s_78_0: bool = fn_state.gs_287668;
        // D s_78_1: write-var gs#287669 <= s_78_0
        fn_state.gs_287669 = s_78_0;
        // N s_78_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #1u : u8
        let s_79_0: bool = true;
        // D s_79_1: write-var gs#287668 <= s_79_0
        fn_state.gs_287668 = s_79_0;
        // N s_79_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #1u : u8
        let s_80_0: bool = true;
        // D s_80_1: write-var gs#287665 <= s_80_0
        fn_state.gs_287665 = s_80_0;
        // N s_80_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var VL:i64
        let s_81_0: i64 = fn_state.VL;
        // C s_81_1: const #2048s : i
        let s_81_1: i128 = 2048;
        // D s_81_2: cast zx s_81_0 -> i
        let s_81_2: i128 = (i128::try_from(s_81_0).unwrap());
        // D s_81_3: cmp-eq s_81_2 s_81_1
        let s_81_3: bool = ((s_81_2) == (s_81_1));
        // D s_81_4: not s_81_3
        let s_81_4: bool = !s_81_3;
        // N s_81_5: branch s_81_4 b101 b82
        if s_81_4 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var n:i64
        let s_82_0: i64 = fn_state.n;
        // D s_82_1: cast zx s_82_0 -> i
        let s_82_1: i128 = (i128::try_from(s_82_0).unwrap());
        // D s_82_2: call __id(s_82_1)
        let s_82_2: i128 = u__id(state, tracer, s_82_1);
        // D s_82_3: cast reint s_82_2 -> i64
        let s_82_3: i64 = (s_82_2 as i64);
        // C s_82_4: const #0s : i
        let s_82_4: i128 = 0;
        // D s_82_5: cast zx s_82_3 -> i
        let s_82_5: i128 = (i128::try_from(s_82_3).unwrap());
        // D s_82_6: cmp-le s_82_4 s_82_5
        let s_82_6: bool = ((s_82_4) <= (s_82_5));
        // N s_82_7: branch s_82_6 b85 b83
        if s_82_6 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#287688 <= s_83_0
        fn_state.gs_287688 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#287688:u8
        let s_84_0: bool = fn_state.gs_287688;
        // N s_84_1: assert s_84_0
        let s_84_1: () = assert!(s_84_0);
        // C s_84_2: const #2048s : i64
        let s_84_2: i64 = 2048;
        // D s_84_3: read-var esize:i64
        let s_84_3: i64 = fn_state.esize;
        // D s_84_4: cast zx s_84_3 -> i
        let s_84_4: i128 = (i128::try_from(s_84_3).unwrap());
        // D s_84_5: cast reint s_84_4 -> i64
        let s_84_5: i64 = (s_84_4 as i64);
        // D s_84_6: read-var d:i64
        let s_84_6: i64 = fn_state.d;
        // D s_84_7: read-var n:i64
        let s_84_7: i64 = fn_state.n;
        // D s_84_8: call execute_SQCVTN_Z_MZ4__(s_84_2, s_84_6, s_84_5, s_84_7)
        let s_84_8: () = execute_SQCVTN_Z_MZ4__(
            state,
            tracer,
            s_84_2,
            s_84_6,
            s_84_5,
            s_84_7,
        );
        // N s_84_9: return
        return;
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var n:i64
        let s_85_0: i64 = fn_state.n;
        // D s_85_1: cast zx s_85_0 -> i
        let s_85_1: i128 = (i128::try_from(s_85_0).unwrap());
        // D s_85_2: call __id(s_85_1)
        let s_85_2: i128 = u__id(state, tracer, s_85_1);
        // D s_85_3: cast reint s_85_2 -> i64
        let s_85_3: i64 = (s_85_2 as i64);
        // C s_85_4: const #31s : i
        let s_85_4: i128 = 31;
        // D s_85_5: cast zx s_85_3 -> i
        let s_85_5: i128 = (i128::try_from(s_85_3).unwrap());
        // D s_85_6: cmp-le s_85_5 s_85_4
        let s_85_6: bool = ((s_85_5) <= (s_85_4));
        // N s_85_7: branch s_85_6 b88 b86
        if s_85_6 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#287687 <= s_86_0
        fn_state.gs_287687 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#287687:u8
        let s_87_0: bool = fn_state.gs_287687;
        // D s_87_1: write-var gs#287688 <= s_87_0
        fn_state.gs_287688 = s_87_0;
        // N s_87_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var esize:i64
        let s_88_0: i64 = fn_state.esize;
        // D s_88_1: cast zx s_88_0 -> i
        let s_88_1: i128 = (i128::try_from(s_88_0).unwrap());
        // D s_88_2: call __id(s_88_1)
        let s_88_2: i128 = u__id(state, tracer, s_88_1);
        // D s_88_3: cast reint s_88_2 -> i64
        let s_88_3: i64 = (s_88_2 as i64);
        // C s_88_4: const #8s : i
        let s_88_4: i128 = 8;
        // D s_88_5: cast zx s_88_3 -> i
        let s_88_5: i128 = (i128::try_from(s_88_3).unwrap());
        // D s_88_6: cmp-eq s_88_5 s_88_4
        let s_88_6: bool = ((s_88_5) == (s_88_4));
        // N s_88_7: branch s_88_6 b100 b89
        if s_88_6 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var esize:i64
        let s_89_0: i64 = fn_state.esize;
        // D s_89_1: cast zx s_89_0 -> i
        let s_89_1: i128 = (i128::try_from(s_89_0).unwrap());
        // D s_89_2: call __id(s_89_1)
        let s_89_2: i128 = u__id(state, tracer, s_89_1);
        // D s_89_3: cast reint s_89_2 -> i64
        let s_89_3: i64 = (s_89_2 as i64);
        // C s_89_4: const #16s : i
        let s_89_4: i128 = 16;
        // D s_89_5: cast zx s_89_3 -> i
        let s_89_5: i128 = (i128::try_from(s_89_3).unwrap());
        // D s_89_6: cmp-eq s_89_5 s_89_4
        let s_89_6: bool = ((s_89_5) == (s_89_4));
        // D s_89_7: write-var gs#287681 <= s_89_6
        fn_state.gs_287681 = s_89_6;
        // N s_89_8: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#287681:u8
        let s_90_0: bool = fn_state.gs_287681;
        // N s_90_1: branch s_90_0 b93 b91
        if s_90_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #0u : u8
        let s_91_0: bool = false;
        // D s_91_1: write-var gs#287686 <= s_91_0
        fn_state.gs_287686 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#287686:u8
        let s_92_0: bool = fn_state.gs_287686;
        // D s_92_1: write-var gs#287687 <= s_92_0
        fn_state.gs_287687 = s_92_0;
        // N s_92_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var d:i64
        let s_93_0: i64 = fn_state.d;
        // D s_93_1: cast zx s_93_0 -> i
        let s_93_1: i128 = (i128::try_from(s_93_0).unwrap());
        // D s_93_2: call __id(s_93_1)
        let s_93_2: i128 = u__id(state, tracer, s_93_1);
        // D s_93_3: cast reint s_93_2 -> i64
        let s_93_3: i64 = (s_93_2 as i64);
        // C s_93_4: const #0s : i
        let s_93_4: i128 = 0;
        // D s_93_5: cast zx s_93_3 -> i
        let s_93_5: i128 = (i128::try_from(s_93_3).unwrap());
        // D s_93_6: cmp-le s_93_4 s_93_5
        let s_93_6: bool = ((s_93_4) <= (s_93_5));
        // N s_93_7: branch s_93_6 b96 b94
        if s_93_6 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #0u : u8
        let s_94_0: bool = false;
        // D s_94_1: write-var gs#287685 <= s_94_0
        fn_state.gs_287685 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#287685:u8
        let s_95_0: bool = fn_state.gs_287685;
        // D s_95_1: write-var gs#287686 <= s_95_0
        fn_state.gs_287686 = s_95_0;
        // N s_95_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var d:i64
        let s_96_0: i64 = fn_state.d;
        // D s_96_1: cast zx s_96_0 -> i
        let s_96_1: i128 = (i128::try_from(s_96_0).unwrap());
        // D s_96_2: call __id(s_96_1)
        let s_96_2: i128 = u__id(state, tracer, s_96_1);
        // D s_96_3: cast reint s_96_2 -> i64
        let s_96_3: i64 = (s_96_2 as i64);
        // C s_96_4: const #31s : i
        let s_96_4: i128 = 31;
        // D s_96_5: cast zx s_96_3 -> i
        let s_96_5: i128 = (i128::try_from(s_96_3).unwrap());
        // D s_96_6: cmp-le s_96_5 s_96_4
        let s_96_6: bool = ((s_96_5) <= (s_96_4));
        // N s_96_7: branch s_96_6 b99 b97
        if s_96_6 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0u : u8
        let s_97_0: bool = false;
        // D s_97_1: write-var gs#287684 <= s_97_0
        fn_state.gs_287684 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#287684:u8
        let s_98_0: bool = fn_state.gs_287684;
        // D s_98_1: write-var gs#287685 <= s_98_0
        fn_state.gs_287685 = s_98_0;
        // N s_98_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #1u : u8
        let s_99_0: bool = true;
        // D s_99_1: write-var gs#287684 <= s_99_0
        fn_state.gs_287684 = s_99_0;
        // N s_99_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #1u : u8
        let s_100_0: bool = true;
        // D s_100_1: write-var gs#287681 <= s_100_0
        fn_state.gs_287681 = s_100_0;
        // N s_100_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_101_0: return
        return;
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_102_0: panic
        panic!("{:?}", ());
        // N s_102_1: return
        return;
    }
}
