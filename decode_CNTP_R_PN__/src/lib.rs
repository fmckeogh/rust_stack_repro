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
use HaveSVE2p1::*;
use CurrentVL_read::*;
use u__id::*;
use HaveSME2::*;
use execute_CNTP_R_PN__::*;
use common::*;
pub fn decode_CNTP_R_PN__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    size: u8,
    vl: bool,
    PNn: u8,
    Rd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_221800: bool,
        gs_221931: bool,
        gs_221758: bool,
        gs_221914: bool,
        gs_221948: bool,
        gs_221873: bool,
        gs_221884: bool,
        gs_221910: bool,
        gs_221950: bool,
        width: i128,
        gs_221876: bool,
        gs_221799: bool,
        gs_221797: bool,
        gs_221779: bool,
        gs_221893: bool,
        gs_221834: bool,
        gs_221877: bool,
        gs_221835: bool,
        gs_221891: bool,
        gs_221922: bool,
        gs_221798: bool,
        gs_221855: bool,
        gs_221839: bool,
        gs_221953: bool,
        gs_221813: bool,
        d: i64,
        gs_221874: bool,
        gs_221801: bool,
        gs_221927: bool,
        gs_221912: bool,
        gs_221837: bool,
        gs_221875: bool,
        gs_221777: bool,
        gs_221913: bool,
        gs_221846: bool,
        gs_221853: bool,
        gs_221949: bool,
        gs_221915: bool,
        gs_221770: bool,
        gs_221838: bool,
        gs_221952: bool,
        gs_221951: bool,
        gs_221872: bool,
        esize: i64,
        gs_221889: bool,
        gs_221911: bool,
        n: i64,
        gs_221929: bool,
        gs_221817: bool,
        gs_221815: bool,
        gs_221796: bool,
        VL: i64,
        gs_221775: bool,
        gs_221836: bool,
        gs_221851: bool,
        gs_221808: bool,
        size: u8,
        vl: bool,
        PNn: u8,
        Rd: u8,
    }
    let fn_state = FunctionState {
        size,
        vl,
        PNn,
        Rd,
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
        // N s_0_6: branch s_0_5 b193 b1
        if s_0_5 {
            return block_193(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#221758 <= s_1_0
        fn_state.gs_221758 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#221758:u8
        let s_2_0: bool = fn_state.gs_221758;
        // N s_2_1: branch s_2_0 b192 b3
        if s_2_0 {
            return block_192(state, tracer, fn_state);
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
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: const #8s : i64
        let s_3_4: i64 = 8;
        // D s_3_5: lsl s_3_4 s_3_3
        let s_3_5: i64 = s_3_4 << s_3_3;
        // D s_3_6: write-var esize <= s_3_5
        fn_state.esize = s_3_5;
        // D s_3_7: read-var PNn:u8
        let s_3_7: u8 = fn_state.PNn;
        // D s_3_8: cast zx s_3_7 -> bv
        let s_3_8: Bits = Bits::new(s_3_7 as u128, 4u16);
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (s_3_8.value() as i128);
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: write-var n <= s_3_10
        fn_state.n = s_3_10;
        // D s_3_12: read-var Rd:u8
        let s_3_12: u8 = fn_state.Rd;
        // D s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 5u16);
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (s_3_13.value() as i128);
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: write-var d <= s_3_15
        fn_state.d = s_3_15;
        // D s_3_17: read-var vl:u8
        let s_3_17: bool = fn_state.vl;
        // D s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 1u16);
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (s_3_18.value() as i128);
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // C s_3_21: const #2s : i
        let s_3_21: i128 = 2;
        // D s_3_22: cast zx s_3_20 -> i
        let s_3_22: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_23: lsl s_3_21 s_3_22
        let s_3_23: i128 = s_3_21 << s_3_22;
        // D s_3_24: write-var width <= s_3_23
        fn_state.width = s_3_23;
        // D s_3_25: read-var VL:i64
        let s_3_25: i64 = fn_state.VL;
        // C s_3_26: const #128s : i
        let s_3_26: i128 = 128;
        // D s_3_27: cast zx s_3_25 -> i
        let s_3_27: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_28: cmp-eq s_3_27 s_3_26
        let s_3_28: bool = ((s_3_27) == (s_3_26));
        // D s_3_29: not s_3_28
        let s_3_29: bool = !s_3_28;
        // N s_3_30: branch s_3_29 b37 b4
        if s_3_29 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var width:i
        let s_4_0: i128 = fn_state.width;
        // D s_4_1: call __id(s_4_0)
        let s_4_1: i128 = u__id(state, tracer, s_4_0);
        // C s_4_2: const #2s : i
        let s_4_2: i128 = 2;
        // D s_4_3: cmp-eq s_4_1 s_4_2
        let s_4_3: bool = ((s_4_1) == (s_4_2));
        // N s_4_4: branch s_4_3 b36 b5
        if s_4_3 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var width:i
        let s_5_0: i128 = fn_state.width;
        // D s_5_1: call __id(s_5_0)
        let s_5_1: i128 = u__id(state, tracer, s_5_0);
        // C s_5_2: const #4s : i
        let s_5_2: i128 = 4;
        // D s_5_3: cmp-eq s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) == (s_5_2));
        // D s_5_4: write-var gs#221770 <= s_5_3
        fn_state.gs_221770 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#221770:u8
        let s_6_0: bool = fn_state.gs_221770;
        // N s_6_1: branch s_6_0 b9 b7
        if s_6_0 {
            return block_9(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#221801 <= s_7_0
        fn_state.gs_221801 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#221801:u8
        let s_8_0: bool = fn_state.gs_221801;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // C s_8_2: const #128s : i64
        let s_8_2: i64 = 128;
        // D s_8_3: read-var width:i
        let s_8_3: i128 = fn_state.width;
        // D s_8_4: cast reint s_8_3 -> i64
        let s_8_4: i64 = (s_8_3 as i64);
        // D s_8_5: read-var d:i64
        let s_8_5: i64 = fn_state.d;
        // D s_8_6: read-var esize:i64
        let s_8_6: i64 = fn_state.esize;
        // D s_8_7: read-var n:i64
        let s_8_7: i64 = fn_state.n;
        // D s_8_8: call execute_CNTP_R_PN__(s_8_2, s_8_5, s_8_6, s_8_7, s_8_4)
        let s_8_8: () = execute_CNTP_R_PN__(
            state,
            tracer,
            s_8_2,
            s_8_5,
            s_8_6,
            s_8_7,
            s_8_4,
        );
        // N s_8_9: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var n:i64
        let s_9_0: i64 = fn_state.n;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call __id(s_9_1)
        let s_9_2: i128 = u__id(state, tracer, s_9_1);
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #0s : i
        let s_9_4: i128 = 0;
        // D s_9_5: cast zx s_9_3 -> i
        let s_9_5: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_6: cmp-le s_9_4 s_9_5
        let s_9_6: bool = ((s_9_4) <= (s_9_5));
        // N s_9_7: branch s_9_6 b12 b10
        if s_9_6 {
            return block_12(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#221800 <= s_10_0
        fn_state.gs_221800 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#221800:u8
        let s_11_0: bool = fn_state.gs_221800;
        // D s_11_1: write-var gs#221801 <= s_11_0
        fn_state.gs_221801 = s_11_0;
        // N s_11_2: jump b8
        return block_8(state, tracer, fn_state);
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
        // C s_12_4: const #15s : i
        let s_12_4: i128 = 15;
        // D s_12_5: cast zx s_12_3 -> i
        let s_12_5: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_6: cmp-le s_12_5 s_12_4
        let s_12_6: bool = ((s_12_5) <= (s_12_4));
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
        // D s_13_1: write-var gs#221799 <= s_13_0
        fn_state.gs_221799 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#221799:u8
        let s_14_0: bool = fn_state.gs_221799;
        // D s_14_1: write-var gs#221800 <= s_14_0
        fn_state.gs_221800 = s_14_0;
        // N s_14_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var esize:i64
        let s_15_0: i64 = fn_state.esize;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call __id(s_15_1)
        let s_15_2: i128 = u__id(state, tracer, s_15_1);
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: const #8s : i
        let s_15_4: i128 = 8;
        // D s_15_5: cast zx s_15_3 -> i
        let s_15_5: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_6: cmp-eq s_15_5 s_15_4
        let s_15_6: bool = ((s_15_5) == (s_15_4));
        // N s_15_7: branch s_15_6 b35 b16
        if s_15_6 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var esize:i64
        let s_16_0: i64 = fn_state.esize;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: call __id(s_16_1)
        let s_16_2: i128 = u__id(state, tracer, s_16_1);
        // D s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // C s_16_4: const #16s : i
        let s_16_4: i128 = 16;
        // D s_16_5: cast zx s_16_3 -> i
        let s_16_5: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_6: cmp-eq s_16_5 s_16_4
        let s_16_6: bool = ((s_16_5) == (s_16_4));
        // D s_16_7: write-var gs#221775 <= s_16_6
        fn_state.gs_221775 = s_16_6;
        // N s_16_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#221775:u8
        let s_17_0: bool = fn_state.gs_221775;
        // N s_17_1: branch s_17_0 b34 b18
        if s_17_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var esize:i64
        let s_18_0: i64 = fn_state.esize;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: call __id(s_18_1)
        let s_18_2: i128 = u__id(state, tracer, s_18_1);
        // D s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: const #32s : i
        let s_18_4: i128 = 32;
        // D s_18_5: cast zx s_18_3 -> i
        let s_18_5: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_6: cmp-eq s_18_5 s_18_4
        let s_18_6: bool = ((s_18_5) == (s_18_4));
        // D s_18_7: write-var gs#221777 <= s_18_6
        fn_state.gs_221777 = s_18_6;
        // N s_18_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#221777:u8
        let s_19_0: bool = fn_state.gs_221777;
        // N s_19_1: branch s_19_0 b33 b20
        if s_19_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var esize:i64
        let s_20_0: i64 = fn_state.esize;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: call __id(s_20_1)
        let s_20_2: i128 = u__id(state, tracer, s_20_1);
        // D s_20_3: cast reint s_20_2 -> i64
        let s_20_3: i64 = (s_20_2 as i64);
        // C s_20_4: const #64s : i
        let s_20_4: i128 = 64;
        // D s_20_5: cast zx s_20_3 -> i
        let s_20_5: i128 = (i128::try_from(s_20_3).unwrap());
        // D s_20_6: cmp-eq s_20_5 s_20_4
        let s_20_6: bool = ((s_20_5) == (s_20_4));
        // D s_20_7: write-var gs#221779 <= s_20_6
        fn_state.gs_221779 = s_20_6;
        // N s_20_8: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#221779:u8
        let s_21_0: bool = fn_state.gs_221779;
        // N s_21_1: branch s_21_0 b24 b22
        if s_21_0 {
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
        // D s_22_1: write-var gs#221798 <= s_22_0
        fn_state.gs_221798 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#221798:u8
        let s_23_0: bool = fn_state.gs_221798;
        // D s_23_1: write-var gs#221799 <= s_23_0
        fn_state.gs_221799 = s_23_0;
        // N s_23_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var d:i64
        let s_24_0: i64 = fn_state.d;
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
        // D s_25_1: write-var gs#221797 <= s_25_0
        fn_state.gs_221797 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#221797:u8
        let s_26_0: bool = fn_state.gs_221797;
        // D s_26_1: write-var gs#221798 <= s_26_0
        fn_state.gs_221798 = s_26_0;
        // N s_26_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var d:i64
        let s_27_0: i64 = fn_state.d;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: call __id(s_27_1)
        let s_27_2: i128 = u__id(state, tracer, s_27_1);
        // D s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: const #31s : i
        let s_27_4: i128 = 31;
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
        // D s_28_1: write-var gs#221796 <= s_28_0
        fn_state.gs_221796 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#221796:u8
        let s_29_0: bool = fn_state.gs_221796;
        // D s_29_1: write-var gs#221797 <= s_29_0
        fn_state.gs_221797 = s_29_0;
        // N s_29_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#221796 <= s_32_0
        fn_state.gs_221796 = s_32_0;
        // N s_32_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#221779 <= s_33_0
        fn_state.gs_221779 = s_33_0;
        // N s_33_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#221777 <= s_34_0
        fn_state.gs_221777 = s_34_0;
        // N s_34_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#221775 <= s_35_0
        fn_state.gs_221775 = s_35_0;
        // N s_35_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#221770 <= s_36_0
        fn_state.gs_221770 = s_36_0;
        // N s_36_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var VL:i64
        let s_37_0: i64 = fn_state.VL;
        // C s_37_1: const #256s : i
        let s_37_1: i128 = 256;
        // D s_37_2: cast zx s_37_0 -> i
        let s_37_2: i128 = (i128::try_from(s_37_0).unwrap());
        // D s_37_3: cmp-eq s_37_2 s_37_1
        let s_37_3: bool = ((s_37_2) == (s_37_1));
        // D s_37_4: not s_37_3
        let s_37_4: bool = !s_37_3;
        // N s_37_5: branch s_37_4 b73 b38
        if s_37_4 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var width:i
        let s_38_0: i128 = fn_state.width;
        // D s_38_1: call __id(s_38_0)
        let s_38_1: i128 = u__id(state, tracer, s_38_0);
        // C s_38_2: const #2s : i
        let s_38_2: i128 = 2;
        // D s_38_3: cmp-eq s_38_1 s_38_2
        let s_38_3: bool = ((s_38_1) == (s_38_2));
        // N s_38_4: branch s_38_3 b72 b39
        if s_38_3 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var width:i
        let s_39_0: i128 = fn_state.width;
        // D s_39_1: call __id(s_39_0)
        let s_39_1: i128 = u__id(state, tracer, s_39_0);
        // C s_39_2: const #4s : i
        let s_39_2: i128 = 4;
        // D s_39_3: cmp-eq s_39_1 s_39_2
        let s_39_3: bool = ((s_39_1) == (s_39_2));
        // D s_39_4: write-var gs#221808 <= s_39_3
        fn_state.gs_221808 = s_39_3;
        // N s_39_5: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#221808:u8
        let s_40_0: bool = fn_state.gs_221808;
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
        // D s_41_1: write-var gs#221839 <= s_41_0
        fn_state.gs_221839 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#221839:u8
        let s_42_0: bool = fn_state.gs_221839;
        // N s_42_1: assert s_42_0
        let s_42_1: () = assert!(s_42_0);
        // C s_42_2: const #256s : i64
        let s_42_2: i64 = 256;
        // D s_42_3: read-var width:i
        let s_42_3: i128 = fn_state.width;
        // D s_42_4: cast reint s_42_3 -> i64
        let s_42_4: i64 = (s_42_3 as i64);
        // D s_42_5: read-var d:i64
        let s_42_5: i64 = fn_state.d;
        // D s_42_6: read-var esize:i64
        let s_42_6: i64 = fn_state.esize;
        // D s_42_7: read-var n:i64
        let s_42_7: i64 = fn_state.n;
        // D s_42_8: call execute_CNTP_R_PN__(s_42_2, s_42_5, s_42_6, s_42_7, s_42_4)
        let s_42_8: () = execute_CNTP_R_PN__(
            state,
            tracer,
            s_42_2,
            s_42_5,
            s_42_6,
            s_42_7,
            s_42_4,
        );
        // N s_42_9: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var n:i64
        let s_43_0: i64 = fn_state.n;
        // D s_43_1: cast zx s_43_0 -> i
        let s_43_1: i128 = (i128::try_from(s_43_0).unwrap());
        // D s_43_2: call __id(s_43_1)
        let s_43_2: i128 = u__id(state, tracer, s_43_1);
        // D s_43_3: cast reint s_43_2 -> i64
        let s_43_3: i64 = (s_43_2 as i64);
        // C s_43_4: const #0s : i
        let s_43_4: i128 = 0;
        // D s_43_5: cast zx s_43_3 -> i
        let s_43_5: i128 = (i128::try_from(s_43_3).unwrap());
        // D s_43_6: cmp-le s_43_4 s_43_5
        let s_43_6: bool = ((s_43_4) <= (s_43_5));
        // N s_43_7: branch s_43_6 b46 b44
        if s_43_6 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#221838 <= s_44_0
        fn_state.gs_221838 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#221838:u8
        let s_45_0: bool = fn_state.gs_221838;
        // D s_45_1: write-var gs#221839 <= s_45_0
        fn_state.gs_221839 = s_45_0;
        // N s_45_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var n:i64
        let s_46_0: i64 = fn_state.n;
        // D s_46_1: cast zx s_46_0 -> i
        let s_46_1: i128 = (i128::try_from(s_46_0).unwrap());
        // D s_46_2: call __id(s_46_1)
        let s_46_2: i128 = u__id(state, tracer, s_46_1);
        // D s_46_3: cast reint s_46_2 -> i64
        let s_46_3: i64 = (s_46_2 as i64);
        // C s_46_4: const #15s : i
        let s_46_4: i128 = 15;
        // D s_46_5: cast zx s_46_3 -> i
        let s_46_5: i128 = (i128::try_from(s_46_3).unwrap());
        // D s_46_6: cmp-le s_46_5 s_46_4
        let s_46_6: bool = ((s_46_5) <= (s_46_4));
        // N s_46_7: branch s_46_6 b49 b47
        if s_46_6 {
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
        // D s_47_1: write-var gs#221837 <= s_47_0
        fn_state.gs_221837 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#221837:u8
        let s_48_0: bool = fn_state.gs_221837;
        // D s_48_1: write-var gs#221838 <= s_48_0
        fn_state.gs_221838 = s_48_0;
        // N s_48_2: jump b45
        return block_45(state, tracer, fn_state);
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
        // C s_49_4: const #8s : i
        let s_49_4: i128 = 8;
        // D s_49_5: cast zx s_49_3 -> i
        let s_49_5: i128 = (i128::try_from(s_49_3).unwrap());
        // D s_49_6: cmp-eq s_49_5 s_49_4
        let s_49_6: bool = ((s_49_5) == (s_49_4));
        // N s_49_7: branch s_49_6 b71 b50
        if s_49_6 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var esize:i64
        let s_50_0: i64 = fn_state.esize;
        // D s_50_1: cast zx s_50_0 -> i
        let s_50_1: i128 = (i128::try_from(s_50_0).unwrap());
        // D s_50_2: call __id(s_50_1)
        let s_50_2: i128 = u__id(state, tracer, s_50_1);
        // D s_50_3: cast reint s_50_2 -> i64
        let s_50_3: i64 = (s_50_2 as i64);
        // C s_50_4: const #16s : i
        let s_50_4: i128 = 16;
        // D s_50_5: cast zx s_50_3 -> i
        let s_50_5: i128 = (i128::try_from(s_50_3).unwrap());
        // D s_50_6: cmp-eq s_50_5 s_50_4
        let s_50_6: bool = ((s_50_5) == (s_50_4));
        // D s_50_7: write-var gs#221813 <= s_50_6
        fn_state.gs_221813 = s_50_6;
        // N s_50_8: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#221813:u8
        let s_51_0: bool = fn_state.gs_221813;
        // N s_51_1: branch s_51_0 b70 b52
        if s_51_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var esize:i64
        let s_52_0: i64 = fn_state.esize;
        // D s_52_1: cast zx s_52_0 -> i
        let s_52_1: i128 = (i128::try_from(s_52_0).unwrap());
        // D s_52_2: call __id(s_52_1)
        let s_52_2: i128 = u__id(state, tracer, s_52_1);
        // D s_52_3: cast reint s_52_2 -> i64
        let s_52_3: i64 = (s_52_2 as i64);
        // C s_52_4: const #32s : i
        let s_52_4: i128 = 32;
        // D s_52_5: cast zx s_52_3 -> i
        let s_52_5: i128 = (i128::try_from(s_52_3).unwrap());
        // D s_52_6: cmp-eq s_52_5 s_52_4
        let s_52_6: bool = ((s_52_5) == (s_52_4));
        // D s_52_7: write-var gs#221815 <= s_52_6
        fn_state.gs_221815 = s_52_6;
        // N s_52_8: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#221815:u8
        let s_53_0: bool = fn_state.gs_221815;
        // N s_53_1: branch s_53_0 b69 b54
        if s_53_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var esize:i64
        let s_54_0: i64 = fn_state.esize;
        // D s_54_1: cast zx s_54_0 -> i
        let s_54_1: i128 = (i128::try_from(s_54_0).unwrap());
        // D s_54_2: call __id(s_54_1)
        let s_54_2: i128 = u__id(state, tracer, s_54_1);
        // D s_54_3: cast reint s_54_2 -> i64
        let s_54_3: i64 = (s_54_2 as i64);
        // C s_54_4: const #64s : i
        let s_54_4: i128 = 64;
        // D s_54_5: cast zx s_54_3 -> i
        let s_54_5: i128 = (i128::try_from(s_54_3).unwrap());
        // D s_54_6: cmp-eq s_54_5 s_54_4
        let s_54_6: bool = ((s_54_5) == (s_54_4));
        // D s_54_7: write-var gs#221817 <= s_54_6
        fn_state.gs_221817 = s_54_6;
        // N s_54_8: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#221817:u8
        let s_55_0: bool = fn_state.gs_221817;
        // N s_55_1: branch s_55_0 b58 b56
        if s_55_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#221836 <= s_56_0
        fn_state.gs_221836 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#221836:u8
        let s_57_0: bool = fn_state.gs_221836;
        // D s_57_1: write-var gs#221837 <= s_57_0
        fn_state.gs_221837 = s_57_0;
        // N s_57_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var d:i64
        let s_58_0: i64 = fn_state.d;
        // D s_58_1: cast zx s_58_0 -> i
        let s_58_1: i128 = (i128::try_from(s_58_0).unwrap());
        // D s_58_2: call __id(s_58_1)
        let s_58_2: i128 = u__id(state, tracer, s_58_1);
        // D s_58_3: cast reint s_58_2 -> i64
        let s_58_3: i64 = (s_58_2 as i64);
        // C s_58_4: const #0s : i
        let s_58_4: i128 = 0;
        // D s_58_5: cast zx s_58_3 -> i
        let s_58_5: i128 = (i128::try_from(s_58_3).unwrap());
        // D s_58_6: cmp-le s_58_4 s_58_5
        let s_58_6: bool = ((s_58_4) <= (s_58_5));
        // N s_58_7: branch s_58_6 b61 b59
        if s_58_6 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#221835 <= s_59_0
        fn_state.gs_221835 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#221835:u8
        let s_60_0: bool = fn_state.gs_221835;
        // D s_60_1: write-var gs#221836 <= s_60_0
        fn_state.gs_221836 = s_60_0;
        // N s_60_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var d:i64
        let s_61_0: i64 = fn_state.d;
        // D s_61_1: cast zx s_61_0 -> i
        let s_61_1: i128 = (i128::try_from(s_61_0).unwrap());
        // D s_61_2: call __id(s_61_1)
        let s_61_2: i128 = u__id(state, tracer, s_61_1);
        // D s_61_3: cast reint s_61_2 -> i64
        let s_61_3: i64 = (s_61_2 as i64);
        // C s_61_4: const #31s : i
        let s_61_4: i128 = 31;
        // D s_61_5: cast zx s_61_3 -> i
        let s_61_5: i128 = (i128::try_from(s_61_3).unwrap());
        // D s_61_6: cmp-le s_61_5 s_61_4
        let s_61_6: bool = ((s_61_5) <= (s_61_4));
        // N s_61_7: branch s_61_6 b64 b62
        if s_61_6 {
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
        // C s_62_0: const #0u : u8
        let s_62_0: bool = false;
        // D s_62_1: write-var gs#221834 <= s_62_0
        fn_state.gs_221834 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#221834:u8
        let s_63_0: bool = fn_state.gs_221834;
        // D s_63_1: write-var gs#221835 <= s_63_0
        fn_state.gs_221835 = s_63_0;
        // N s_63_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_64_0: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_65_0: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_66_0: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_67_0: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#221834 <= s_68_0
        fn_state.gs_221834 = s_68_0;
        // N s_68_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // D s_69_1: write-var gs#221817 <= s_69_0
        fn_state.gs_221817 = s_69_0;
        // N s_69_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var gs#221815 <= s_70_0
        fn_state.gs_221815 = s_70_0;
        // N s_70_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var gs#221813 <= s_71_0
        fn_state.gs_221813 = s_71_0;
        // N s_71_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#221808 <= s_72_0
        fn_state.gs_221808 = s_72_0;
        // N s_72_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var VL:i64
        let s_73_0: i64 = fn_state.VL;
        // C s_73_1: const #512s : i
        let s_73_1: i128 = 512;
        // D s_73_2: cast zx s_73_0 -> i
        let s_73_2: i128 = (i128::try_from(s_73_0).unwrap());
        // D s_73_3: cmp-eq s_73_2 s_73_1
        let s_73_3: bool = ((s_73_2) == (s_73_1));
        // D s_73_4: not s_73_3
        let s_73_4: bool = !s_73_3;
        // N s_73_5: branch s_73_4 b111 b74
        if s_73_4 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var width:i
        let s_74_0: i128 = fn_state.width;
        // D s_74_1: call __id(s_74_0)
        let s_74_1: i128 = u__id(state, tracer, s_74_0);
        // C s_74_2: const #2s : i
        let s_74_2: i128 = 2;
        // D s_74_3: cmp-eq s_74_1 s_74_2
        let s_74_3: bool = ((s_74_1) == (s_74_2));
        // N s_74_4: branch s_74_3 b110 b75
        if s_74_3 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var width:i
        let s_75_0: i128 = fn_state.width;
        // D s_75_1: call __id(s_75_0)
        let s_75_1: i128 = u__id(state, tracer, s_75_0);
        // C s_75_2: const #4s : i
        let s_75_2: i128 = 4;
        // D s_75_3: cmp-eq s_75_1 s_75_2
        let s_75_3: bool = ((s_75_1) == (s_75_2));
        // D s_75_4: write-var gs#221846 <= s_75_3
        fn_state.gs_221846 = s_75_3;
        // N s_75_5: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#221846:u8
        let s_76_0: bool = fn_state.gs_221846;
        // N s_76_1: branch s_76_0 b79 b77
        if s_76_0 {
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
        // D s_77_1: write-var gs#221877 <= s_77_0
        fn_state.gs_221877 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#221877:u8
        let s_78_0: bool = fn_state.gs_221877;
        // N s_78_1: assert s_78_0
        let s_78_1: () = assert!(s_78_0);
        // C s_78_2: const #512s : i64
        let s_78_2: i64 = 512;
        // D s_78_3: read-var width:i
        let s_78_3: i128 = fn_state.width;
        // D s_78_4: cast reint s_78_3 -> i64
        let s_78_4: i64 = (s_78_3 as i64);
        // D s_78_5: read-var d:i64
        let s_78_5: i64 = fn_state.d;
        // D s_78_6: read-var esize:i64
        let s_78_6: i64 = fn_state.esize;
        // D s_78_7: read-var n:i64
        let s_78_7: i64 = fn_state.n;
        // D s_78_8: call execute_CNTP_R_PN__(s_78_2, s_78_5, s_78_6, s_78_7, s_78_4)
        let s_78_8: () = execute_CNTP_R_PN__(
            state,
            tracer,
            s_78_2,
            s_78_5,
            s_78_6,
            s_78_7,
            s_78_4,
        );
        // N s_78_9: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var n:i64
        let s_79_0: i64 = fn_state.n;
        // D s_79_1: cast zx s_79_0 -> i
        let s_79_1: i128 = (i128::try_from(s_79_0).unwrap());
        // D s_79_2: call __id(s_79_1)
        let s_79_2: i128 = u__id(state, tracer, s_79_1);
        // D s_79_3: cast reint s_79_2 -> i64
        let s_79_3: i64 = (s_79_2 as i64);
        // C s_79_4: const #0s : i
        let s_79_4: i128 = 0;
        // D s_79_5: cast zx s_79_3 -> i
        let s_79_5: i128 = (i128::try_from(s_79_3).unwrap());
        // D s_79_6: cmp-le s_79_4 s_79_5
        let s_79_6: bool = ((s_79_4) <= (s_79_5));
        // N s_79_7: branch s_79_6 b82 b80
        if s_79_6 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var gs#221876 <= s_80_0
        fn_state.gs_221876 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#221876:u8
        let s_81_0: bool = fn_state.gs_221876;
        // D s_81_1: write-var gs#221877 <= s_81_0
        fn_state.gs_221877 = s_81_0;
        // N s_81_2: jump b78
        return block_78(state, tracer, fn_state);
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
        // C s_82_4: const #15s : i
        let s_82_4: i128 = 15;
        // D s_82_5: cast zx s_82_3 -> i
        let s_82_5: i128 = (i128::try_from(s_82_3).unwrap());
        // D s_82_6: cmp-le s_82_5 s_82_4
        let s_82_6: bool = ((s_82_5) <= (s_82_4));
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
        // D s_83_1: write-var gs#221875 <= s_83_0
        fn_state.gs_221875 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#221875:u8
        let s_84_0: bool = fn_state.gs_221875;
        // D s_84_1: write-var gs#221876 <= s_84_0
        fn_state.gs_221876 = s_84_0;
        // N s_84_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var esize:i64
        let s_85_0: i64 = fn_state.esize;
        // D s_85_1: cast zx s_85_0 -> i
        let s_85_1: i128 = (i128::try_from(s_85_0).unwrap());
        // D s_85_2: call __id(s_85_1)
        let s_85_2: i128 = u__id(state, tracer, s_85_1);
        // D s_85_3: cast reint s_85_2 -> i64
        let s_85_3: i64 = (s_85_2 as i64);
        // C s_85_4: const #8s : i
        let s_85_4: i128 = 8;
        // D s_85_5: cast zx s_85_3 -> i
        let s_85_5: i128 = (i128::try_from(s_85_3).unwrap());
        // D s_85_6: cmp-eq s_85_5 s_85_4
        let s_85_6: bool = ((s_85_5) == (s_85_4));
        // N s_85_7: branch s_85_6 b109 b86
        if s_85_6 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var esize:i64
        let s_86_0: i64 = fn_state.esize;
        // D s_86_1: cast zx s_86_0 -> i
        let s_86_1: i128 = (i128::try_from(s_86_0).unwrap());
        // D s_86_2: call __id(s_86_1)
        let s_86_2: i128 = u__id(state, tracer, s_86_1);
        // D s_86_3: cast reint s_86_2 -> i64
        let s_86_3: i64 = (s_86_2 as i64);
        // C s_86_4: const #16s : i
        let s_86_4: i128 = 16;
        // D s_86_5: cast zx s_86_3 -> i
        let s_86_5: i128 = (i128::try_from(s_86_3).unwrap());
        // D s_86_6: cmp-eq s_86_5 s_86_4
        let s_86_6: bool = ((s_86_5) == (s_86_4));
        // D s_86_7: write-var gs#221851 <= s_86_6
        fn_state.gs_221851 = s_86_6;
        // N s_86_8: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#221851:u8
        let s_87_0: bool = fn_state.gs_221851;
        // N s_87_1: branch s_87_0 b108 b88
        if s_87_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
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
        // C s_88_4: const #32s : i
        let s_88_4: i128 = 32;
        // D s_88_5: cast zx s_88_3 -> i
        let s_88_5: i128 = (i128::try_from(s_88_3).unwrap());
        // D s_88_6: cmp-eq s_88_5 s_88_4
        let s_88_6: bool = ((s_88_5) == (s_88_4));
        // D s_88_7: write-var gs#221853 <= s_88_6
        fn_state.gs_221853 = s_88_6;
        // N s_88_8: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#221853:u8
        let s_89_0: bool = fn_state.gs_221853;
        // N s_89_1: branch s_89_0 b107 b90
        if s_89_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var esize:i64
        let s_90_0: i64 = fn_state.esize;
        // D s_90_1: cast zx s_90_0 -> i
        let s_90_1: i128 = (i128::try_from(s_90_0).unwrap());
        // D s_90_2: call __id(s_90_1)
        let s_90_2: i128 = u__id(state, tracer, s_90_1);
        // D s_90_3: cast reint s_90_2 -> i64
        let s_90_3: i64 = (s_90_2 as i64);
        // C s_90_4: const #64s : i
        let s_90_4: i128 = 64;
        // D s_90_5: cast zx s_90_3 -> i
        let s_90_5: i128 = (i128::try_from(s_90_3).unwrap());
        // D s_90_6: cmp-eq s_90_5 s_90_4
        let s_90_6: bool = ((s_90_5) == (s_90_4));
        // D s_90_7: write-var gs#221855 <= s_90_6
        fn_state.gs_221855 = s_90_6;
        // N s_90_8: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#221855:u8
        let s_91_0: bool = fn_state.gs_221855;
        // N s_91_1: branch s_91_0 b94 b92
        if s_91_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #0u : u8
        let s_92_0: bool = false;
        // D s_92_1: write-var gs#221874 <= s_92_0
        fn_state.gs_221874 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#221874:u8
        let s_93_0: bool = fn_state.gs_221874;
        // D s_93_1: write-var gs#221875 <= s_93_0
        fn_state.gs_221875 = s_93_0;
        // N s_93_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var d:i64
        let s_94_0: i64 = fn_state.d;
        // D s_94_1: cast zx s_94_0 -> i
        let s_94_1: i128 = (i128::try_from(s_94_0).unwrap());
        // D s_94_2: call __id(s_94_1)
        let s_94_2: i128 = u__id(state, tracer, s_94_1);
        // D s_94_3: cast reint s_94_2 -> i64
        let s_94_3: i64 = (s_94_2 as i64);
        // C s_94_4: const #0s : i
        let s_94_4: i128 = 0;
        // D s_94_5: cast zx s_94_3 -> i
        let s_94_5: i128 = (i128::try_from(s_94_3).unwrap());
        // D s_94_6: cmp-le s_94_4 s_94_5
        let s_94_6: bool = ((s_94_4) <= (s_94_5));
        // N s_94_7: branch s_94_6 b97 b95
        if s_94_6 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #0u : u8
        let s_95_0: bool = false;
        // D s_95_1: write-var gs#221873 <= s_95_0
        fn_state.gs_221873 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#221873:u8
        let s_96_0: bool = fn_state.gs_221873;
        // D s_96_1: write-var gs#221874 <= s_96_0
        fn_state.gs_221874 = s_96_0;
        // N s_96_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var d:i64
        let s_97_0: i64 = fn_state.d;
        // D s_97_1: cast zx s_97_0 -> i
        let s_97_1: i128 = (i128::try_from(s_97_0).unwrap());
        // D s_97_2: call __id(s_97_1)
        let s_97_2: i128 = u__id(state, tracer, s_97_1);
        // D s_97_3: cast reint s_97_2 -> i64
        let s_97_3: i64 = (s_97_2 as i64);
        // C s_97_4: const #31s : i
        let s_97_4: i128 = 31;
        // D s_97_5: cast zx s_97_3 -> i
        let s_97_5: i128 = (i128::try_from(s_97_3).unwrap());
        // D s_97_6: cmp-le s_97_5 s_97_4
        let s_97_6: bool = ((s_97_5) <= (s_97_4));
        // N s_97_7: branch s_97_6 b100 b98
        if s_97_6 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // D s_98_1: write-var gs#221872 <= s_98_0
        fn_state.gs_221872 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#221872:u8
        let s_99_0: bool = fn_state.gs_221872;
        // D s_99_1: write-var gs#221873 <= s_99_0
        fn_state.gs_221873 = s_99_0;
        // N s_99_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_100_0: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_101_0: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_102_0: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_103_0: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_104_0: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_105_0: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #1u : u8
        let s_106_0: bool = true;
        // D s_106_1: write-var gs#221872 <= s_106_0
        fn_state.gs_221872 = s_106_0;
        // N s_106_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #1u : u8
        let s_107_0: bool = true;
        // D s_107_1: write-var gs#221855 <= s_107_0
        fn_state.gs_221855 = s_107_0;
        // N s_107_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #1u : u8
        let s_108_0: bool = true;
        // D s_108_1: write-var gs#221853 <= s_108_0
        fn_state.gs_221853 = s_108_0;
        // N s_108_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #1u : u8
        let s_109_0: bool = true;
        // D s_109_1: write-var gs#221851 <= s_109_0
        fn_state.gs_221851 = s_109_0;
        // N s_109_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #1u : u8
        let s_110_0: bool = true;
        // D s_110_1: write-var gs#221846 <= s_110_0
        fn_state.gs_221846 = s_110_0;
        // N s_110_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var VL:i64
        let s_111_0: i64 = fn_state.VL;
        // C s_111_1: const #1024s : i
        let s_111_1: i128 = 1024;
        // D s_111_2: cast zx s_111_0 -> i
        let s_111_2: i128 = (i128::try_from(s_111_0).unwrap());
        // D s_111_3: cmp-eq s_111_2 s_111_1
        let s_111_3: bool = ((s_111_2) == (s_111_1));
        // D s_111_4: not s_111_3
        let s_111_4: bool = !s_111_3;
        // N s_111_5: branch s_111_4 b151 b112
        if s_111_4 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var width:i
        let s_112_0: i128 = fn_state.width;
        // D s_112_1: call __id(s_112_0)
        let s_112_1: i128 = u__id(state, tracer, s_112_0);
        // C s_112_2: const #2s : i
        let s_112_2: i128 = 2;
        // D s_112_3: cmp-eq s_112_1 s_112_2
        let s_112_3: bool = ((s_112_1) == (s_112_2));
        // N s_112_4: branch s_112_3 b150 b113
        if s_112_3 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var width:i
        let s_113_0: i128 = fn_state.width;
        // D s_113_1: call __id(s_113_0)
        let s_113_1: i128 = u__id(state, tracer, s_113_0);
        // C s_113_2: const #4s : i
        let s_113_2: i128 = 4;
        // D s_113_3: cmp-eq s_113_1 s_113_2
        let s_113_3: bool = ((s_113_1) == (s_113_2));
        // D s_113_4: write-var gs#221884 <= s_113_3
        fn_state.gs_221884 = s_113_3;
        // N s_113_5: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#221884:u8
        let s_114_0: bool = fn_state.gs_221884;
        // N s_114_1: branch s_114_0 b117 b115
        if s_114_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #0u : u8
        let s_115_0: bool = false;
        // D s_115_1: write-var gs#221915 <= s_115_0
        fn_state.gs_221915 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#221915:u8
        let s_116_0: bool = fn_state.gs_221915;
        // N s_116_1: assert s_116_0
        let s_116_1: () = assert!(s_116_0);
        // C s_116_2: const #1024s : i64
        let s_116_2: i64 = 1024;
        // D s_116_3: read-var width:i
        let s_116_3: i128 = fn_state.width;
        // D s_116_4: cast reint s_116_3 -> i64
        let s_116_4: i64 = (s_116_3 as i64);
        // D s_116_5: read-var d:i64
        let s_116_5: i64 = fn_state.d;
        // D s_116_6: read-var esize:i64
        let s_116_6: i64 = fn_state.esize;
        // D s_116_7: read-var n:i64
        let s_116_7: i64 = fn_state.n;
        // D s_116_8: call execute_CNTP_R_PN__(s_116_2, s_116_5, s_116_6, s_116_7, s_116_4)
        let s_116_8: () = execute_CNTP_R_PN__(
            state,
            tracer,
            s_116_2,
            s_116_5,
            s_116_6,
            s_116_7,
            s_116_4,
        );
        // N s_116_9: return
        return;
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var n:i64
        let s_117_0: i64 = fn_state.n;
        // D s_117_1: cast zx s_117_0 -> i
        let s_117_1: i128 = (i128::try_from(s_117_0).unwrap());
        // D s_117_2: call __id(s_117_1)
        let s_117_2: i128 = u__id(state, tracer, s_117_1);
        // D s_117_3: cast reint s_117_2 -> i64
        let s_117_3: i64 = (s_117_2 as i64);
        // C s_117_4: const #0s : i
        let s_117_4: i128 = 0;
        // D s_117_5: cast zx s_117_3 -> i
        let s_117_5: i128 = (i128::try_from(s_117_3).unwrap());
        // D s_117_6: cmp-le s_117_4 s_117_5
        let s_117_6: bool = ((s_117_4) <= (s_117_5));
        // N s_117_7: branch s_117_6 b120 b118
        if s_117_6 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #0u : u8
        let s_118_0: bool = false;
        // D s_118_1: write-var gs#221914 <= s_118_0
        fn_state.gs_221914 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#221914:u8
        let s_119_0: bool = fn_state.gs_221914;
        // D s_119_1: write-var gs#221915 <= s_119_0
        fn_state.gs_221915 = s_119_0;
        // N s_119_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var n:i64
        let s_120_0: i64 = fn_state.n;
        // D s_120_1: cast zx s_120_0 -> i
        let s_120_1: i128 = (i128::try_from(s_120_0).unwrap());
        // D s_120_2: call __id(s_120_1)
        let s_120_2: i128 = u__id(state, tracer, s_120_1);
        // D s_120_3: cast reint s_120_2 -> i64
        let s_120_3: i64 = (s_120_2 as i64);
        // C s_120_4: const #15s : i
        let s_120_4: i128 = 15;
        // D s_120_5: cast zx s_120_3 -> i
        let s_120_5: i128 = (i128::try_from(s_120_3).unwrap());
        // D s_120_6: cmp-le s_120_5 s_120_4
        let s_120_6: bool = ((s_120_5) <= (s_120_4));
        // N s_120_7: branch s_120_6 b123 b121
        if s_120_6 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #0u : u8
        let s_121_0: bool = false;
        // D s_121_1: write-var gs#221913 <= s_121_0
        fn_state.gs_221913 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#221913:u8
        let s_122_0: bool = fn_state.gs_221913;
        // D s_122_1: write-var gs#221914 <= s_122_0
        fn_state.gs_221914 = s_122_0;
        // N s_122_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var esize:i64
        let s_123_0: i64 = fn_state.esize;
        // D s_123_1: cast zx s_123_0 -> i
        let s_123_1: i128 = (i128::try_from(s_123_0).unwrap());
        // D s_123_2: call __id(s_123_1)
        let s_123_2: i128 = u__id(state, tracer, s_123_1);
        // D s_123_3: cast reint s_123_2 -> i64
        let s_123_3: i64 = (s_123_2 as i64);
        // C s_123_4: const #8s : i
        let s_123_4: i128 = 8;
        // D s_123_5: cast zx s_123_3 -> i
        let s_123_5: i128 = (i128::try_from(s_123_3).unwrap());
        // D s_123_6: cmp-eq s_123_5 s_123_4
        let s_123_6: bool = ((s_123_5) == (s_123_4));
        // N s_123_7: branch s_123_6 b149 b124
        if s_123_6 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var esize:i64
        let s_124_0: i64 = fn_state.esize;
        // D s_124_1: cast zx s_124_0 -> i
        let s_124_1: i128 = (i128::try_from(s_124_0).unwrap());
        // D s_124_2: call __id(s_124_1)
        let s_124_2: i128 = u__id(state, tracer, s_124_1);
        // D s_124_3: cast reint s_124_2 -> i64
        let s_124_3: i64 = (s_124_2 as i64);
        // C s_124_4: const #16s : i
        let s_124_4: i128 = 16;
        // D s_124_5: cast zx s_124_3 -> i
        let s_124_5: i128 = (i128::try_from(s_124_3).unwrap());
        // D s_124_6: cmp-eq s_124_5 s_124_4
        let s_124_6: bool = ((s_124_5) == (s_124_4));
        // D s_124_7: write-var gs#221889 <= s_124_6
        fn_state.gs_221889 = s_124_6;
        // N s_124_8: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#221889:u8
        let s_125_0: bool = fn_state.gs_221889;
        // N s_125_1: branch s_125_0 b148 b126
        if s_125_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var esize:i64
        let s_126_0: i64 = fn_state.esize;
        // D s_126_1: cast zx s_126_0 -> i
        let s_126_1: i128 = (i128::try_from(s_126_0).unwrap());
        // D s_126_2: call __id(s_126_1)
        let s_126_2: i128 = u__id(state, tracer, s_126_1);
        // D s_126_3: cast reint s_126_2 -> i64
        let s_126_3: i64 = (s_126_2 as i64);
        // C s_126_4: const #32s : i
        let s_126_4: i128 = 32;
        // D s_126_5: cast zx s_126_3 -> i
        let s_126_5: i128 = (i128::try_from(s_126_3).unwrap());
        // D s_126_6: cmp-eq s_126_5 s_126_4
        let s_126_6: bool = ((s_126_5) == (s_126_4));
        // D s_126_7: write-var gs#221891 <= s_126_6
        fn_state.gs_221891 = s_126_6;
        // N s_126_8: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#221891:u8
        let s_127_0: bool = fn_state.gs_221891;
        // N s_127_1: branch s_127_0 b147 b128
        if s_127_0 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var esize:i64
        let s_128_0: i64 = fn_state.esize;
        // D s_128_1: cast zx s_128_0 -> i
        let s_128_1: i128 = (i128::try_from(s_128_0).unwrap());
        // D s_128_2: call __id(s_128_1)
        let s_128_2: i128 = u__id(state, tracer, s_128_1);
        // D s_128_3: cast reint s_128_2 -> i64
        let s_128_3: i64 = (s_128_2 as i64);
        // C s_128_4: const #64s : i
        let s_128_4: i128 = 64;
        // D s_128_5: cast zx s_128_3 -> i
        let s_128_5: i128 = (i128::try_from(s_128_3).unwrap());
        // D s_128_6: cmp-eq s_128_5 s_128_4
        let s_128_6: bool = ((s_128_5) == (s_128_4));
        // D s_128_7: write-var gs#221893 <= s_128_6
        fn_state.gs_221893 = s_128_6;
        // N s_128_8: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#221893:u8
        let s_129_0: bool = fn_state.gs_221893;
        // N s_129_1: branch s_129_0 b132 b130
        if s_129_0 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #0u : u8
        let s_130_0: bool = false;
        // D s_130_1: write-var gs#221912 <= s_130_0
        fn_state.gs_221912 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#221912:u8
        let s_131_0: bool = fn_state.gs_221912;
        // D s_131_1: write-var gs#221913 <= s_131_0
        fn_state.gs_221913 = s_131_0;
        // N s_131_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var d:i64
        let s_132_0: i64 = fn_state.d;
        // D s_132_1: cast zx s_132_0 -> i
        let s_132_1: i128 = (i128::try_from(s_132_0).unwrap());
        // D s_132_2: call __id(s_132_1)
        let s_132_2: i128 = u__id(state, tracer, s_132_1);
        // D s_132_3: cast reint s_132_2 -> i64
        let s_132_3: i64 = (s_132_2 as i64);
        // C s_132_4: const #0s : i
        let s_132_4: i128 = 0;
        // D s_132_5: cast zx s_132_3 -> i
        let s_132_5: i128 = (i128::try_from(s_132_3).unwrap());
        // D s_132_6: cmp-le s_132_4 s_132_5
        let s_132_6: bool = ((s_132_4) <= (s_132_5));
        // N s_132_7: branch s_132_6 b135 b133
        if s_132_6 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #0u : u8
        let s_133_0: bool = false;
        // D s_133_1: write-var gs#221911 <= s_133_0
        fn_state.gs_221911 = s_133_0;
        // N s_133_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#221911:u8
        let s_134_0: bool = fn_state.gs_221911;
        // D s_134_1: write-var gs#221912 <= s_134_0
        fn_state.gs_221912 = s_134_0;
        // N s_134_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var d:i64
        let s_135_0: i64 = fn_state.d;
        // D s_135_1: cast zx s_135_0 -> i
        let s_135_1: i128 = (i128::try_from(s_135_0).unwrap());
        // D s_135_2: call __id(s_135_1)
        let s_135_2: i128 = u__id(state, tracer, s_135_1);
        // D s_135_3: cast reint s_135_2 -> i64
        let s_135_3: i64 = (s_135_2 as i64);
        // C s_135_4: const #31s : i
        let s_135_4: i128 = 31;
        // D s_135_5: cast zx s_135_3 -> i
        let s_135_5: i128 = (i128::try_from(s_135_3).unwrap());
        // D s_135_6: cmp-le s_135_5 s_135_4
        let s_135_6: bool = ((s_135_5) <= (s_135_4));
        // N s_135_7: branch s_135_6 b138 b136
        if s_135_6 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #0u : u8
        let s_136_0: bool = false;
        // D s_136_1: write-var gs#221910 <= s_136_0
        fn_state.gs_221910 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#221910:u8
        let s_137_0: bool = fn_state.gs_221910;
        // D s_137_1: write-var gs#221911 <= s_137_0
        fn_state.gs_221911 = s_137_0;
        // N s_137_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_138_0: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_139_0: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_140_0: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_141_0: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_142_0: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_143_0: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_144_0: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_145_0: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #1u : u8
        let s_146_0: bool = true;
        // D s_146_1: write-var gs#221910 <= s_146_0
        fn_state.gs_221910 = s_146_0;
        // N s_146_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #1u : u8
        let s_147_0: bool = true;
        // D s_147_1: write-var gs#221893 <= s_147_0
        fn_state.gs_221893 = s_147_0;
        // N s_147_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #1u : u8
        let s_148_0: bool = true;
        // D s_148_1: write-var gs#221891 <= s_148_0
        fn_state.gs_221891 = s_148_0;
        // N s_148_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #1u : u8
        let s_149_0: bool = true;
        // D s_149_1: write-var gs#221889 <= s_149_0
        fn_state.gs_221889 = s_149_0;
        // N s_149_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #1u : u8
        let s_150_0: bool = true;
        // D s_150_1: write-var gs#221884 <= s_150_0
        fn_state.gs_221884 = s_150_0;
        // N s_150_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var VL:i64
        let s_151_0: i64 = fn_state.VL;
        // C s_151_1: const #2048s : i
        let s_151_1: i128 = 2048;
        // D s_151_2: cast zx s_151_0 -> i
        let s_151_2: i128 = (i128::try_from(s_151_0).unwrap());
        // D s_151_3: cmp-eq s_151_2 s_151_1
        let s_151_3: bool = ((s_151_2) == (s_151_1));
        // D s_151_4: not s_151_3
        let s_151_4: bool = !s_151_3;
        // N s_151_5: branch s_151_4 b191 b152
        if s_151_4 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var width:i
        let s_152_0: i128 = fn_state.width;
        // D s_152_1: call __id(s_152_0)
        let s_152_1: i128 = u__id(state, tracer, s_152_0);
        // C s_152_2: const #2s : i
        let s_152_2: i128 = 2;
        // D s_152_3: cmp-eq s_152_1 s_152_2
        let s_152_3: bool = ((s_152_1) == (s_152_2));
        // N s_152_4: branch s_152_3 b190 b153
        if s_152_3 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var width:i
        let s_153_0: i128 = fn_state.width;
        // D s_153_1: call __id(s_153_0)
        let s_153_1: i128 = u__id(state, tracer, s_153_0);
        // C s_153_2: const #4s : i
        let s_153_2: i128 = 4;
        // D s_153_3: cmp-eq s_153_1 s_153_2
        let s_153_3: bool = ((s_153_1) == (s_153_2));
        // D s_153_4: write-var gs#221922 <= s_153_3
        fn_state.gs_221922 = s_153_3;
        // N s_153_5: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#221922:u8
        let s_154_0: bool = fn_state.gs_221922;
        // N s_154_1: branch s_154_0 b157 b155
        if s_154_0 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #0u : u8
        let s_155_0: bool = false;
        // D s_155_1: write-var gs#221953 <= s_155_0
        fn_state.gs_221953 = s_155_0;
        // N s_155_2: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var gs#221953:u8
        let s_156_0: bool = fn_state.gs_221953;
        // N s_156_1: assert s_156_0
        let s_156_1: () = assert!(s_156_0);
        // C s_156_2: const #2048s : i64
        let s_156_2: i64 = 2048;
        // D s_156_3: read-var width:i
        let s_156_3: i128 = fn_state.width;
        // D s_156_4: cast reint s_156_3 -> i64
        let s_156_4: i64 = (s_156_3 as i64);
        // D s_156_5: read-var d:i64
        let s_156_5: i64 = fn_state.d;
        // D s_156_6: read-var esize:i64
        let s_156_6: i64 = fn_state.esize;
        // D s_156_7: read-var n:i64
        let s_156_7: i64 = fn_state.n;
        // D s_156_8: call execute_CNTP_R_PN__(s_156_2, s_156_5, s_156_6, s_156_7, s_156_4)
        let s_156_8: () = execute_CNTP_R_PN__(
            state,
            tracer,
            s_156_2,
            s_156_5,
            s_156_6,
            s_156_7,
            s_156_4,
        );
        // N s_156_9: return
        return;
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var n:i64
        let s_157_0: i64 = fn_state.n;
        // D s_157_1: cast zx s_157_0 -> i
        let s_157_1: i128 = (i128::try_from(s_157_0).unwrap());
        // D s_157_2: call __id(s_157_1)
        let s_157_2: i128 = u__id(state, tracer, s_157_1);
        // D s_157_3: cast reint s_157_2 -> i64
        let s_157_3: i64 = (s_157_2 as i64);
        // C s_157_4: const #0s : i
        let s_157_4: i128 = 0;
        // D s_157_5: cast zx s_157_3 -> i
        let s_157_5: i128 = (i128::try_from(s_157_3).unwrap());
        // D s_157_6: cmp-le s_157_4 s_157_5
        let s_157_6: bool = ((s_157_4) <= (s_157_5));
        // N s_157_7: branch s_157_6 b160 b158
        if s_157_6 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #0u : u8
        let s_158_0: bool = false;
        // D s_158_1: write-var gs#221952 <= s_158_0
        fn_state.gs_221952 = s_158_0;
        // N s_158_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var gs#221952:u8
        let s_159_0: bool = fn_state.gs_221952;
        // D s_159_1: write-var gs#221953 <= s_159_0
        fn_state.gs_221953 = s_159_0;
        // N s_159_2: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var n:i64
        let s_160_0: i64 = fn_state.n;
        // D s_160_1: cast zx s_160_0 -> i
        let s_160_1: i128 = (i128::try_from(s_160_0).unwrap());
        // D s_160_2: call __id(s_160_1)
        let s_160_2: i128 = u__id(state, tracer, s_160_1);
        // D s_160_3: cast reint s_160_2 -> i64
        let s_160_3: i64 = (s_160_2 as i64);
        // C s_160_4: const #15s : i
        let s_160_4: i128 = 15;
        // D s_160_5: cast zx s_160_3 -> i
        let s_160_5: i128 = (i128::try_from(s_160_3).unwrap());
        // D s_160_6: cmp-le s_160_5 s_160_4
        let s_160_6: bool = ((s_160_5) <= (s_160_4));
        // N s_160_7: branch s_160_6 b163 b161
        if s_160_6 {
            return block_163(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #0u : u8
        let s_161_0: bool = false;
        // D s_161_1: write-var gs#221951 <= s_161_0
        fn_state.gs_221951 = s_161_0;
        // N s_161_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var gs#221951:u8
        let s_162_0: bool = fn_state.gs_221951;
        // D s_162_1: write-var gs#221952 <= s_162_0
        fn_state.gs_221952 = s_162_0;
        // N s_162_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_163_0: read-var esize:i64
        let s_163_0: i64 = fn_state.esize;
        // D s_163_1: cast zx s_163_0 -> i
        let s_163_1: i128 = (i128::try_from(s_163_0).unwrap());
        // D s_163_2: call __id(s_163_1)
        let s_163_2: i128 = u__id(state, tracer, s_163_1);
        // D s_163_3: cast reint s_163_2 -> i64
        let s_163_3: i64 = (s_163_2 as i64);
        // C s_163_4: const #8s : i
        let s_163_4: i128 = 8;
        // D s_163_5: cast zx s_163_3 -> i
        let s_163_5: i128 = (i128::try_from(s_163_3).unwrap());
        // D s_163_6: cmp-eq s_163_5 s_163_4
        let s_163_6: bool = ((s_163_5) == (s_163_4));
        // N s_163_7: branch s_163_6 b189 b164
        if s_163_6 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_164(state, tracer, fn_state);
        };
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var esize:i64
        let s_164_0: i64 = fn_state.esize;
        // D s_164_1: cast zx s_164_0 -> i
        let s_164_1: i128 = (i128::try_from(s_164_0).unwrap());
        // D s_164_2: call __id(s_164_1)
        let s_164_2: i128 = u__id(state, tracer, s_164_1);
        // D s_164_3: cast reint s_164_2 -> i64
        let s_164_3: i64 = (s_164_2 as i64);
        // C s_164_4: const #16s : i
        let s_164_4: i128 = 16;
        // D s_164_5: cast zx s_164_3 -> i
        let s_164_5: i128 = (i128::try_from(s_164_3).unwrap());
        // D s_164_6: cmp-eq s_164_5 s_164_4
        let s_164_6: bool = ((s_164_5) == (s_164_4));
        // D s_164_7: write-var gs#221927 <= s_164_6
        fn_state.gs_221927 = s_164_6;
        // N s_164_8: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var gs#221927:u8
        let s_165_0: bool = fn_state.gs_221927;
        // N s_165_1: branch s_165_0 b188 b166
        if s_165_0 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_166_0: read-var esize:i64
        let s_166_0: i64 = fn_state.esize;
        // D s_166_1: cast zx s_166_0 -> i
        let s_166_1: i128 = (i128::try_from(s_166_0).unwrap());
        // D s_166_2: call __id(s_166_1)
        let s_166_2: i128 = u__id(state, tracer, s_166_1);
        // D s_166_3: cast reint s_166_2 -> i64
        let s_166_3: i64 = (s_166_2 as i64);
        // C s_166_4: const #32s : i
        let s_166_4: i128 = 32;
        // D s_166_5: cast zx s_166_3 -> i
        let s_166_5: i128 = (i128::try_from(s_166_3).unwrap());
        // D s_166_6: cmp-eq s_166_5 s_166_4
        let s_166_6: bool = ((s_166_5) == (s_166_4));
        // D s_166_7: write-var gs#221929 <= s_166_6
        fn_state.gs_221929 = s_166_6;
        // N s_166_8: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var gs#221929:u8
        let s_167_0: bool = fn_state.gs_221929;
        // N s_167_1: branch s_167_0 b187 b168
        if s_167_0 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var esize:i64
        let s_168_0: i64 = fn_state.esize;
        // D s_168_1: cast zx s_168_0 -> i
        let s_168_1: i128 = (i128::try_from(s_168_0).unwrap());
        // D s_168_2: call __id(s_168_1)
        let s_168_2: i128 = u__id(state, tracer, s_168_1);
        // D s_168_3: cast reint s_168_2 -> i64
        let s_168_3: i64 = (s_168_2 as i64);
        // C s_168_4: const #64s : i
        let s_168_4: i128 = 64;
        // D s_168_5: cast zx s_168_3 -> i
        let s_168_5: i128 = (i128::try_from(s_168_3).unwrap());
        // D s_168_6: cmp-eq s_168_5 s_168_4
        let s_168_6: bool = ((s_168_5) == (s_168_4));
        // D s_168_7: write-var gs#221931 <= s_168_6
        fn_state.gs_221931 = s_168_6;
        // N s_168_8: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_169_0: read-var gs#221931:u8
        let s_169_0: bool = fn_state.gs_221931;
        // N s_169_1: branch s_169_0 b172 b170
        if s_169_0 {
            return block_172(state, tracer, fn_state);
        } else {
            return block_170(state, tracer, fn_state);
        };
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #0u : u8
        let s_170_0: bool = false;
        // D s_170_1: write-var gs#221950 <= s_170_0
        fn_state.gs_221950 = s_170_0;
        // N s_170_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var gs#221950:u8
        let s_171_0: bool = fn_state.gs_221950;
        // D s_171_1: write-var gs#221951 <= s_171_0
        fn_state.gs_221951 = s_171_0;
        // N s_171_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var d:i64
        let s_172_0: i64 = fn_state.d;
        // D s_172_1: cast zx s_172_0 -> i
        let s_172_1: i128 = (i128::try_from(s_172_0).unwrap());
        // D s_172_2: call __id(s_172_1)
        let s_172_2: i128 = u__id(state, tracer, s_172_1);
        // D s_172_3: cast reint s_172_2 -> i64
        let s_172_3: i64 = (s_172_2 as i64);
        // C s_172_4: const #0s : i
        let s_172_4: i128 = 0;
        // D s_172_5: cast zx s_172_3 -> i
        let s_172_5: i128 = (i128::try_from(s_172_3).unwrap());
        // D s_172_6: cmp-le s_172_4 s_172_5
        let s_172_6: bool = ((s_172_4) <= (s_172_5));
        // N s_172_7: branch s_172_6 b175 b173
        if s_172_6 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #0u : u8
        let s_173_0: bool = false;
        // D s_173_1: write-var gs#221949 <= s_173_0
        fn_state.gs_221949 = s_173_0;
        // N s_173_2: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var gs#221949:u8
        let s_174_0: bool = fn_state.gs_221949;
        // D s_174_1: write-var gs#221950 <= s_174_0
        fn_state.gs_221950 = s_174_0;
        // N s_174_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var d:i64
        let s_175_0: i64 = fn_state.d;
        // D s_175_1: cast zx s_175_0 -> i
        let s_175_1: i128 = (i128::try_from(s_175_0).unwrap());
        // D s_175_2: call __id(s_175_1)
        let s_175_2: i128 = u__id(state, tracer, s_175_1);
        // D s_175_3: cast reint s_175_2 -> i64
        let s_175_3: i64 = (s_175_2 as i64);
        // C s_175_4: const #31s : i
        let s_175_4: i128 = 31;
        // D s_175_5: cast zx s_175_3 -> i
        let s_175_5: i128 = (i128::try_from(s_175_3).unwrap());
        // D s_175_6: cmp-le s_175_5 s_175_4
        let s_175_6: bool = ((s_175_5) <= (s_175_4));
        // N s_175_7: branch s_175_6 b178 b176
        if s_175_6 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_176(state, tracer, fn_state);
        };
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #0u : u8
        let s_176_0: bool = false;
        // D s_176_1: write-var gs#221948 <= s_176_0
        fn_state.gs_221948 = s_176_0;
        // N s_176_2: jump b177
        return block_177(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_177_0: read-var gs#221948:u8
        let s_177_0: bool = fn_state.gs_221948;
        // D s_177_1: write-var gs#221949 <= s_177_0
        fn_state.gs_221949 = s_177_0;
        // N s_177_2: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_178_0: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_179_0: jump b180
        return block_180(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_180_0: jump b181
        return block_181(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_181_0: jump b182
        return block_182(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_182_0: jump b183
        return block_183(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_183_0: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_184_0: jump b185
        return block_185(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_185_0: jump b186
        return block_186(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #1u : u8
        let s_186_0: bool = true;
        // D s_186_1: write-var gs#221948 <= s_186_0
        fn_state.gs_221948 = s_186_0;
        // N s_186_2: jump b177
        return block_177(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #1u : u8
        let s_187_0: bool = true;
        // D s_187_1: write-var gs#221931 <= s_187_0
        fn_state.gs_221931 = s_187_0;
        // N s_187_2: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #1u : u8
        let s_188_0: bool = true;
        // D s_188_1: write-var gs#221929 <= s_188_0
        fn_state.gs_221929 = s_188_0;
        // N s_188_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #1u : u8
        let s_189_0: bool = true;
        // D s_189_1: write-var gs#221927 <= s_189_0
        fn_state.gs_221927 = s_189_0;
        // N s_189_2: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_190_0: const #1u : u8
        let s_190_0: bool = true;
        // D s_190_1: write-var gs#221922 <= s_190_0
        fn_state.gs_221922 = s_190_0;
        // N s_190_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_191_0: return
        return;
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_192_0: panic
        panic!("{:?}", ());
        // N s_192_1: return
        return;
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #() : ()
        let s_193_0: () = ();
        // S s_193_1: call HaveSVE2p1(s_193_0)
        let s_193_1: bool = HaveSVE2p1(state, tracer, s_193_0);
        // S s_193_2: not s_193_1
        let s_193_2: bool = !s_193_1;
        // D s_193_3: write-var gs#221758 <= s_193_2
        fn_state.gs_221758 = s_193_2;
        // N s_193_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
