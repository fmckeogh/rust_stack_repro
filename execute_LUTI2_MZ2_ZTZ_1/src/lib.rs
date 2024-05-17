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
use Elem_set::*;
use CheckStreamingSVEEnabled::*;
use u__id::*;
use ZT0_read::*;
use CheckSMEZT0Enabled::*;
use Elem_read::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_LUTI2_MZ2_ZTZ_1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d__arg: i64,
    dstride: i64,
    esize: i64,
    imm: i64,
    isize: i64,
    n: i64,
    nreg: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        segment: i64,
        base: i64,
        gs_289582: i64,
        indexes: Bits,
        VLshadow_6843: i64,
        esizeshadow_6841: i64,
        gs_289575: i64,
        table: u64,
        d: i128,
        elements: i64,
        VLshadow_6842: i64,
        result: Bits,
        VL: i64,
        d__arg: i64,
        dstride: i64,
        esize: i64,
        imm: i64,
        isize: i64,
        n: i64,
        nreg: i64,
    }
    let fn_state = FunctionState {
        VL,
        d__arg,
        dstride,
        esize,
        imm,
        isize,
        n,
        nreg,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#6841 <= s_0_2
        fn_state.esizeshadow_6841 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#6842 <= s_0_6
        fn_state.VLshadow_6842 = s_0_6;
        // D s_0_8: read-var d__arg:i64
        let s_0_8: i64 = fn_state.d__arg;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: write-var d <= s_0_9
        fn_state.d = s_0_9;
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call CheckStreamingSVEEnabled(s_0_11)
        let s_0_12: () = CheckStreamingSVEEnabled(state, tracer, s_0_11);
        // N s_0_13: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call CheckSMEZT0Enabled(s_1_0)
        let s_1_1: () = CheckSMEZT0Enabled(state, tracer, s_1_0);
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#6842:i64
        let s_2_0: i64 = fn_state.VLshadow_6842;
        // D s_2_1: write-var VLshadow#6843 <= s_2_0
        fn_state.VLshadow_6843 = s_2_0;
        // D s_2_2: read-var VLshadow#6843:i64
        let s_2_2: i64 = fn_state.VLshadow_6843;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: read-var esizeshadow#6841:i64
        let s_2_4: i64 = fn_state.esizeshadow_6841;
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_6: div s_2_3 s_2_5
        let s_2_6: i128 = ((s_2_3) / (s_2_5));
        // D s_2_7: cast reint s_2_6 -> i64
        let s_2_7: i64 = (s_2_6 as i64);
        // D s_2_8: write-var elements <= s_2_7
        fn_state.elements = s_2_7;
        // D s_2_9: read-var isize:i64
        let s_2_9: i64 = fn_state.isize;
        // D s_2_10: cast zx s_2_9 -> i
        let s_2_10: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_11: read-var nreg:i64
        let s_2_11: i64 = fn_state.nreg;
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_13: mul s_2_10 s_2_12
        let s_2_13: i128 = ((s_2_10) * (s_2_12));
        // D s_2_14: cast reint s_2_13 -> i64
        let s_2_14: i64 = (s_2_13 as i64);
        // D s_2_15: read-var esizeshadow#6841:i64
        let s_2_15: i64 = fn_state.esizeshadow_6841;
        // D s_2_16: cast zx s_2_15 -> i
        let s_2_16: i128 = (i128::try_from(s_2_15).unwrap());
        // D s_2_17: cast zx s_2_14 -> i
        let s_2_17: i128 = (i128::try_from(s_2_14).unwrap());
        // D s_2_18: div s_2_16 s_2_17
        let s_2_18: i128 = ((s_2_16) / (s_2_17));
        // D s_2_19: cast reint s_2_18 -> i64
        let s_2_19: i64 = (s_2_18 as i64);
        // D s_2_20: read-var imm:i64
        let s_2_20: i64 = fn_state.imm;
        // D s_2_21: cast zx s_2_20 -> i
        let s_2_21: i128 = (i128::try_from(s_2_20).unwrap());
        // D s_2_22: cast zx s_2_19 -> i
        let s_2_22: i128 = (i128::try_from(s_2_19).unwrap());
        // D s_2_23: mod s_2_21 s_2_22
        let s_2_23: i128 = ((s_2_21) % (s_2_22));
        // D s_2_24: cast reint s_2_23 -> i64
        let s_2_24: i64 = (s_2_23 as i64);
        // D s_2_25: write-var segment <= s_2_24
        fn_state.segment = s_2_24;
        // D s_2_26: read-var VLshadow#6843:i64
        let s_2_26: i64 = fn_state.VLshadow_6843;
        // D s_2_27: cast zx s_2_26 -> i
        let s_2_27: i128 = (i128::try_from(s_2_26).unwrap());
        // D s_2_28: cast reint s_2_27 -> i64
        let s_2_28: i64 = (s_2_27 as i64);
        // D s_2_29: read-var n:i64
        let s_2_29: i64 = fn_state.n;
        // D s_2_30: cast zx s_2_29 -> i
        let s_2_30: i128 = (i128::try_from(s_2_29).unwrap());
        // D s_2_31: cast zx s_2_28 -> i
        let s_2_31: i128 = (i128::try_from(s_2_28).unwrap());
        // D s_2_32: call Z_read(s_2_30, s_2_31)
        let s_2_32: Bits = Z_read(state, tracer, s_2_30, s_2_31);
        // D s_2_33: write-var indexes <= s_2_32
        fn_state.indexes = s_2_32;
        // C s_2_34: const #512s : i64
        let s_2_34: i64 = 512;
        // S s_2_35: call ZT0_read(s_2_34)
        let s_2_35: Bits = ZT0_read(state, tracer, s_2_34);
        // S s_2_36: cast reint s_2_35 -> u512
        let s_2_36: u64 = (s_2_35.value() as u64);
        // D s_2_37: write-var table <= s_2_36
        fn_state.table = s_2_36;
        // C s_2_38: const #0s : i64
        let s_2_38: i64 = 0;
        // C s_2_39: const #1s : i
        let s_2_39: i128 = 1;
        // D s_2_40: read-var nreg:i64
        let s_2_40: i64 = fn_state.nreg;
        // D s_2_41: cast zx s_2_40 -> i
        let s_2_41: i128 = (i128::try_from(s_2_40).unwrap());
        // D s_2_42: sub s_2_41 s_2_39
        let s_2_42: i128 = ((s_2_41) - (s_2_39));
        // D s_2_43: cast reint s_2_42 -> i64
        let s_2_43: i64 = (s_2_42 as i64);
        // D s_2_44: write-var gs#289575 <= s_2_43
        fn_state.gs_289575 = s_2_43;
        // D s_2_45: write-var r <= s_2_38
        fn_state.r = s_2_38;
        // N s_2_46: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var r:i64
        let s_3_0: i64 = fn_state.r;
        // D s_3_1: read-var gs#289575:i64
        let s_3_1: i64 = fn_state.gs_289575;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b8 b4
        if s_3_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var segment:i64
        let s_4_0: i64 = fn_state.segment;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var nreg:i64
        let s_4_2: i64 = fn_state.nreg;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: mul s_4_1 s_4_3
        let s_4_4: i128 = ((s_4_1) * (s_4_3));
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: read-var r:i64
        let s_4_7: i64 = fn_state.r;
        // D s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (i128::try_from(s_4_7).unwrap());
        // D s_4_9: add s_4_6 s_4_8
        let s_4_9: i128 = (s_4_6 + s_4_8);
        // D s_4_10: cast reint s_4_9 -> i64
        let s_4_10: i64 = (s_4_9 as i64);
        // D s_4_11: cast zx s_4_10 -> i
        let s_4_11: i128 = (i128::try_from(s_4_10).unwrap());
        // D s_4_12: read-var elements:i64
        let s_4_12: i64 = fn_state.elements;
        // D s_4_13: cast zx s_4_12 -> i
        let s_4_13: i128 = (i128::try_from(s_4_12).unwrap());
        // D s_4_14: mul s_4_11 s_4_13
        let s_4_14: i128 = ((s_4_11) * (s_4_13));
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: write-var base <= s_4_15
        fn_state.base = s_4_15;
        // C s_4_17: const #0s : i64
        let s_4_17: i64 = 0;
        // C s_4_18: const #1s : i
        let s_4_18: i128 = 1;
        // D s_4_19: read-var elements:i64
        let s_4_19: i64 = fn_state.elements;
        // D s_4_20: cast zx s_4_19 -> i
        let s_4_20: i128 = (i128::try_from(s_4_19).unwrap());
        // D s_4_21: sub s_4_20 s_4_18
        let s_4_21: i128 = ((s_4_20) - (s_4_18));
        // D s_4_22: cast reint s_4_21 -> i64
        let s_4_22: i64 = (s_4_21 as i64);
        // D s_4_23: write-var gs#289582 <= s_4_22
        fn_state.gs_289582 = s_4_22;
        // D s_4_24: write-var e <= s_4_17
        fn_state.e = s_4_17;
        // N s_4_25: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // D s_5_1: read-var gs#289582:i64
        let s_5_1: i64 = fn_state.gs_289582;
        // D s_5_2: cmp-gt s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) > (s_5_1));
        // N s_5_3: branch s_5_2 b7 b6
        if s_5_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var base:i64
        let s_6_0: i64 = fn_state.base;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var e:i64
        let s_6_2: i64 = fn_state.e;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: add s_6_1 s_6_3
        let s_6_4: i128 = (s_6_1 + s_6_3);
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: read-var isize:i64
        let s_6_6: i64 = fn_state.isize;
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_8: cast reint s_6_7 -> i64
        let s_6_8: i64 = (s_6_7 as i64);
        // D s_6_9: cast zx s_6_5 -> i
        let s_6_9: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_10: cast zx s_6_8 -> i
        let s_6_10: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_11: read-var indexes:bv
        let s_6_11: Bits = fn_state.indexes;
        // D s_6_12: call Elem_read(s_6_11, s_6_9, s_6_10)
        let s_6_12: Bits = Elem_read(state, tracer, s_6_11, s_6_9, s_6_10);
        // D s_6_13: cast reint s_6_12 -> u8
        let s_6_13: u8 = (s_6_12.value() as u8);
        // D s_6_14: cast zx s_6_13 -> bv
        let s_6_14: Bits = Bits::new(s_6_13 as u128, 2u16);
        // D s_6_15: cast zx s_6_14 -> i
        let s_6_15: i128 = (s_6_14.value() as i128);
        // D s_6_16: cast reint s_6_15 -> i64
        let s_6_16: i64 = (s_6_15 as i64);
        // D s_6_17: read-var esizeshadow#6841:i64
        let s_6_17: i64 = fn_state.esizeshadow_6841;
        // D s_6_18: cast zx s_6_17 -> i
        let s_6_18: i128 = (i128::try_from(s_6_17).unwrap());
        // D s_6_19: call __id(s_6_18)
        let s_6_19: i128 = u__id(state, tracer, s_6_18);
        // D s_6_20: cast reint s_6_19 -> i64
        let s_6_20: i64 = (s_6_19 as i64);
        // C s_6_21: const #1s : i
        let s_6_21: i128 = 1;
        // D s_6_22: cast zx s_6_20 -> i
        let s_6_22: i128 = (i128::try_from(s_6_20).unwrap());
        // D s_6_23: sub s_6_22 s_6_21
        let s_6_23: i128 = ((s_6_22) - (s_6_21));
        // D s_6_24: cast reint s_6_23 -> i64
        let s_6_24: i64 = (s_6_23 as i64);
        // C s_6_25: const #32s : i
        let s_6_25: i128 = 32;
        // D s_6_26: cast zx s_6_24 -> i
        let s_6_26: i128 = (i128::try_from(s_6_24).unwrap());
        // D s_6_27: cmp-lt s_6_26 s_6_25
        let s_6_27: bool = ((s_6_26) < (s_6_25));
        // N s_6_28: assert s_6_27
        let s_6_28: () = assert!(s_6_27);
        // D s_6_29: read-var esizeshadow#6841:i64
        let s_6_29: i64 = fn_state.esizeshadow_6841;
        // D s_6_30: cast zx s_6_29 -> i
        let s_6_30: i128 = (i128::try_from(s_6_29).unwrap());
        // D s_6_31: cast reint s_6_30 -> i64
        let s_6_31: i64 = (s_6_30 as i64);
        // C s_6_32: const #32s : i64
        let s_6_32: i64 = 32;
        // D s_6_33: read-var table:u512
        let s_6_33: u64 = fn_state.table;
        // D s_6_34: cast zx s_6_33 -> bv
        let s_6_34: Bits = Bits::new(s_6_33 as u128, 512u16);
        // D s_6_35: cast zx s_6_16 -> i
        let s_6_35: i128 = (i128::try_from(s_6_16).unwrap());
        // C s_6_36: cast zx s_6_32 -> i
        let s_6_36: i128 = (i128::try_from(s_6_32).unwrap());
        // D s_6_37: call Elem_read(s_6_34, s_6_35, s_6_36)
        let s_6_37: Bits = Elem_read(state, tracer, s_6_34, s_6_35, s_6_36);
        // D s_6_38: cast reint s_6_37 -> u32
        let s_6_38: u32 = (s_6_37.value() as u32);
        // C s_6_39: const #1s : i
        let s_6_39: i128 = 1;
        // D s_6_40: read-var esizeshadow#6841:i64
        let s_6_40: i64 = fn_state.esizeshadow_6841;
        // D s_6_41: cast zx s_6_40 -> i
        let s_6_41: i128 = (i128::try_from(s_6_40).unwrap());
        // D s_6_42: sub s_6_41 s_6_39
        let s_6_42: i128 = ((s_6_41) - (s_6_39));
        // D s_6_43: cast reint s_6_42 -> i64
        let s_6_43: i64 = (s_6_42 as i64);
        // C s_6_44: const #0s : i
        let s_6_44: i128 = 0;
        // D s_6_45: cast zx s_6_38 -> bv
        let s_6_45: Bits = Bits::new(s_6_38 as u128, 32u16);
        // D s_6_46: cast zx s_6_43 -> i
        let s_6_46: i128 = (i128::try_from(s_6_43).unwrap());
        // C s_6_47: const #1s : i64
        let s_6_47: i64 = 1;
        // C s_6_48: cast zx s_6_47 -> i
        let s_6_48: i128 = (i128::try_from(s_6_47).unwrap());
        // D s_6_49: sub s_6_46 s_6_44
        let s_6_49: i128 = ((s_6_46) - (s_6_44));
        // D s_6_50: add s_6_49 s_6_48
        let s_6_50: i128 = (s_6_49 + s_6_48);
        // D s_6_51: bit-extract s_6_45 s_6_44 s_6_50
        let s_6_51: Bits = (Bits::new(
            ((s_6_45) >> (s_6_44)).value(),
            u16::try_from(s_6_50).unwrap(),
        ));
        // D s_6_52: read-var e:i64
        let s_6_52: i64 = fn_state.e;
        // D s_6_53: cast zx s_6_52 -> i
        let s_6_53: i128 = (i128::try_from(s_6_52).unwrap());
        // D s_6_54: cast zx s_6_31 -> i
        let s_6_54: i128 = (i128::try_from(s_6_31).unwrap());
        // D s_6_55: read-var result:bv
        let s_6_55: Bits = fn_state.result;
        // D s_6_56: call Elem_set(s_6_55, s_6_53, s_6_54, s_6_51)
        let s_6_56: Bits = Elem_set(state, tracer, s_6_55, s_6_53, s_6_54, s_6_51);
        // D s_6_57: write-var result <= s_6_56
        fn_state.result = s_6_56;
        // D s_6_58: read-var e:i64
        let s_6_58: i64 = fn_state.e;
        // C s_6_59: const #1s : i64
        let s_6_59: i64 = 1;
        // D s_6_60: add s_6_58 s_6_59
        let s_6_60: i64 = (s_6_58 + s_6_59);
        // D s_6_61: write-var e <= s_6_60
        fn_state.e = s_6_60;
        // N s_6_62: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VLshadow#6843:i64
        let s_7_0: i64 = fn_state.VLshadow_6843;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var d:i
        let s_7_4: i128 = fn_state.d;
        // D s_7_5: read-var result:bv
        let s_7_5: Bits = fn_state.result;
        // D s_7_6: call Z_set(s_7_4, s_7_3, s_7_5)
        let s_7_6: () = Z_set(state, tracer, s_7_4, s_7_3, s_7_5);
        // D s_7_7: read-var dstride:i64
        let s_7_7: i64 = fn_state.dstride;
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: read-var d:i
        let s_7_9: i128 = fn_state.d;
        // D s_7_10: add s_7_9 s_7_8
        let s_7_10: i128 = (s_7_9 + s_7_8);
        // D s_7_11: write-var d <= s_7_10
        fn_state.d = s_7_10;
        // D s_7_12: read-var r:i64
        let s_7_12: i64 = fn_state.r;
        // C s_7_13: const #1s : i64
        let s_7_13: i64 = 1;
        // D s_7_14: add s_7_12 s_7_13
        let s_7_14: i64 = (s_7_12 + s_7_13);
        // D s_7_15: write-var r <= s_7_14
        fn_state.r = s_7_14;
        // N s_7_16: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: return
        return;
    }
}
