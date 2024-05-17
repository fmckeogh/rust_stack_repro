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
use ZAslice_set::*;
use Elem_set::*;
use SP_read::*;
use CheckStreamingSVEAndZAEnabled::*;
use Mem_read::*;
use ActivePredicateElement::*;
use P_read::*;
use CreateAccDescSME::*;
use AnyActiveElement::*;
use X_read::*;
use Zeros::*;
use ConstrainUnpredictableBool::*;
use CheckSPAlignment::*;
use common::*;
pub fn execute_LD1H_ZA_P_RRR__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    m: i64,
    n: i64,
    offset: i64,
    s: i64,
    t: i64,
    vertical: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_257304: bool,
        e: i64,
        base: u64,
        addr: u64,
        mbytes: i64,
        ga_320652: i64,
        gs_257294: i64,
        VLshadow_5518: i64,
        mask: Bits,
        moffs: u64,
        dim: i64,
        accdesc: ProductType9878976b5bcce9c9,
        ga_320653: Bits,
        VLshadow_5517: i64,
        result: Bits,
        slice_name: i64,
        VL: i64,
        esize: i64,
        g: i64,
        m: i64,
        n: i64,
        offset: i64,
        s: i64,
        t: i64,
        vertical: bool,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        g,
        m,
        n,
        offset,
        s,
        t,
        vertical,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#5517 <= s_0_2
        fn_state.VLshadow_5517 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckStreamingSVEAndZAEnabled(s_0_4)
        let s_0_5: () = CheckStreamingSVEAndZAEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#5517:i64
        let s_1_0: i64 = fn_state.VLshadow_5517;
        // D s_1_1: write-var VLshadow#5518 <= s_1_0
        fn_state.VLshadow_5518 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#5518:i64
        let s_1_3: i64 = fn_state.VLshadow_5518;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#5518:i64
        let s_1_7: i64 = fn_state.VLshadow_5518;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esize:i64
        let s_1_9: i64 = fn_state.esize;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var dim <= s_1_12
        fn_state.dim = s_1_12;
        // D s_1_14: cast zx s_1_6 -> i
        let s_1_14: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var g:i64
        let s_1_16: i64 = fn_state.g;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call P_read(s_1_17, s_1_18)
        let s_1_19: Bits = P_read(state, tracer, s_1_17, s_1_18);
        // D s_1_20: write-var mask <= s_1_19
        fn_state.mask = s_1_19;
        // C s_1_21: const #64s : i64
        let s_1_21: i64 = 64;
        // D s_1_22: read-var m:i64
        let s_1_22: i64 = fn_state.m;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: call X_read(s_1_23, s_1_21)
        let s_1_24: Bits = X_read(state, tracer, s_1_23, s_1_21);
        // D s_1_25: cast reint s_1_24 -> u64
        let s_1_25: u64 = (s_1_24.value() as u64);
        // D s_1_26: write-var moffs <= s_1_25
        fn_state.moffs = s_1_25;
        // C s_1_27: const #32s : i64
        let s_1_27: i64 = 32;
        // D s_1_28: read-var s:i64
        let s_1_28: i64 = fn_state.s;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: call X_read(s_1_29, s_1_27)
        let s_1_30: Bits = X_read(state, tracer, s_1_29, s_1_27);
        // D s_1_31: cast reint s_1_30 -> u32
        let s_1_31: u32 = (s_1_30.value() as u32);
        // D s_1_32: cast zx s_1_31 -> bv
        let s_1_32: Bits = Bits::new(s_1_31 as u128, 32u16);
        // D s_1_33: cast zx s_1_32 -> i
        let s_1_33: i128 = (s_1_32.value() as i128);
        // D s_1_34: cast reint s_1_33 -> i64
        let s_1_34: i64 = (s_1_33 as i64);
        // D s_1_35: cast zx s_1_34 -> i
        let s_1_35: i128 = (i128::try_from(s_1_34).unwrap());
        // D s_1_36: read-var offset:i64
        let s_1_36: i64 = fn_state.offset;
        // D s_1_37: cast zx s_1_36 -> i
        let s_1_37: i128 = (i128::try_from(s_1_36).unwrap());
        // D s_1_38: add s_1_35 s_1_37
        let s_1_38: i128 = (s_1_35 + s_1_37);
        // D s_1_39: cast reint s_1_38 -> i64
        let s_1_39: i64 = (s_1_38 as i64);
        // D s_1_40: cast zx s_1_39 -> i
        let s_1_40: i128 = (i128::try_from(s_1_39).unwrap());
        // D s_1_41: read-var dim:i64
        let s_1_41: i64 = fn_state.dim;
        // D s_1_42: cast zx s_1_41 -> i
        let s_1_42: i128 = (i128::try_from(s_1_41).unwrap());
        // D s_1_43: mod s_1_40 s_1_42
        let s_1_43: i128 = ((s_1_40) % (s_1_42));
        // D s_1_44: cast reint s_1_43 -> i64
        let s_1_44: i64 = (s_1_43 as i64);
        // D s_1_45: write-var slice_name <= s_1_44
        fn_state.slice_name = s_1_44;
        // C s_1_46: const #8s : i
        let s_1_46: i128 = 8;
        // D s_1_47: read-var esize:i64
        let s_1_47: i64 = fn_state.esize;
        // D s_1_48: cast zx s_1_47 -> i
        let s_1_48: i128 = (i128::try_from(s_1_47).unwrap());
        // D s_1_49: div s_1_48 s_1_46
        let s_1_49: i128 = ((s_1_48) / (s_1_46));
        // D s_1_50: cast reint s_1_49 -> i64
        let s_1_50: i64 = (s_1_49 as i64);
        // D s_1_51: write-var mbytes <= s_1_50
        fn_state.mbytes = s_1_50;
        // C s_1_52: const #0u : u32
        let s_1_52: u32 = 0;
        // C s_1_53: const #0u : u8
        let s_1_53: bool = false;
        // C s_1_54: const #1u : u8
        let s_1_54: bool = true;
        // C s_1_55: const #1u : u8
        let s_1_55: bool = true;
        // S s_1_56: call CreateAccDescSME(s_1_52, s_1_53, s_1_54, s_1_55)
        let s_1_56: ProductType9878976b5bcce9c9 = CreateAccDescSME(
            state,
            tracer,
            s_1_52,
            s_1_53,
            s_1_54,
            s_1_55,
        );
        // D s_1_57: write-var accdesc <= s_1_56
        fn_state.accdesc = s_1_56;
        // C s_1_58: const #31s : i
        let s_1_58: i128 = 31;
        // D s_1_59: read-var n:i64
        let s_1_59: i64 = fn_state.n;
        // D s_1_60: cast zx s_1_59 -> i
        let s_1_60: i128 = (i128::try_from(s_1_59).unwrap());
        // D s_1_61: cmp-eq s_1_60 s_1_58
        let s_1_61: bool = ((s_1_60) == (s_1_58));
        // N s_1_62: branch s_1_61 b11 b2
        if s_1_61 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call X_read(s_2_2, s_2_0)
        let s_2_3: Bits = X_read(state, tracer, s_2_2, s_2_0);
        // D s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // D s_2_5: write-var base <= s_2_4
        fn_state.base = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var dim:i64
        let s_3_2: i64 = fn_state.dim;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#257294 <= s_3_5
        fn_state.gs_257294 = s_3_5;
        // D s_3_7: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#257294:i64
        let s_4_1: i64 = fn_state.gs_257294;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b10 b5
        if s_4_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var moffs:u64
        let s_5_0: u64 = fn_state.moffs;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 64u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: read-var mbytes:i64
        let s_5_3: i64 = fn_state.mbytes;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: mul s_5_2 s_5_4
        let s_5_5: i128 = ((s_5_2) * (s_5_4));
        // D s_5_6: read-var base:u64
        let s_5_6: u64 = fn_state.base;
        // D s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 64u16);
        // D s_5_8: cast cvt s_5_5 -> bv
        let s_5_8: Bits = Bits::new(s_5_5 as u128, 128);
        // D s_5_9: add s_5_7 s_5_8
        let s_5_9: Bits = (s_5_7 + s_5_8);
        // D s_5_10: cast reint s_5_9 -> u64
        let s_5_10: u64 = (s_5_9.value() as u64);
        // D s_5_11: write-var addr <= s_5_10
        fn_state.addr = s_5_10;
        // D s_5_12: read-var e:i64
        let s_5_12: i64 = fn_state.e;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: read-var esize:i64
        let s_5_14: i64 = fn_state.esize;
        // D s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_16: read-var mask:bv
        let s_5_16: Bits = fn_state.mask;
        // D s_5_17: call ActivePredicateElement(s_5_16, s_5_13, s_5_15)
        let s_5_17: bool = ActivePredicateElement(state, tracer, s_5_16, s_5_13, s_5_15);
        // N s_5_18: branch s_5_17 b8 b6
        if s_5_17 {
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
        // D s_6_0: read-var esize:i64
        let s_6_0: i64 = fn_state.esize;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var esize:i64
        let s_6_3: i64 = fn_state.esize;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: call Zeros(s_6_4)
        let s_6_5: Bits = Zeros(state, tracer, s_6_4);
        // D s_6_6: cast reint s_6_5 -> u16
        let s_6_6: u16 = (s_6_5.value() as u16);
        // D s_6_7: read-var e:i64
        let s_6_7: i64 = fn_state.e;
        // D s_6_8: cast zx s_6_7 -> i
        let s_6_8: i128 = (i128::try_from(s_6_7).unwrap());
        // D s_6_9: cast zx s_6_2 -> i
        let s_6_9: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_10: cast zx s_6_6 -> bv
        let s_6_10: Bits = Bits::new(s_6_6 as u128, 16u16);
        // D s_6_11: read-var result:bv
        let s_6_11: Bits = fn_state.result;
        // D s_6_12: call Elem_set(s_6_11, s_6_8, s_6_9, s_6_10)
        let s_6_12: Bits = Elem_set(state, tracer, s_6_11, s_6_8, s_6_9, s_6_10);
        // D s_6_13: write-var result <= s_6_12
        fn_state.result = s_6_12;
        // N s_6_14: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1s : i
        let s_7_0: i128 = 1;
        // D s_7_1: read-var moffs:u64
        let s_7_1: u64 = fn_state.moffs;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 64u16);
        // C s_7_3: cast cvt s_7_0 -> bv
        let s_7_3: Bits = Bits::new(s_7_0 as u128, 128);
        // D s_7_4: add s_7_2 s_7_3
        let s_7_4: Bits = (s_7_2 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> u64
        let s_7_5: u64 = (s_7_4.value() as u64);
        // D s_7_6: write-var moffs <= s_7_5
        fn_state.moffs = s_7_5;
        // D s_7_7: read-var e:i64
        let s_7_7: i64 = fn_state.e;
        // C s_7_8: const #1s : i64
        let s_7_8: i64 = 1;
        // D s_7_9: add s_7_7 s_7_8
        let s_7_9: i64 = (s_7_7 + s_7_8);
        // D s_7_10: write-var e <= s_7_9
        fn_state.e = s_7_9;
        // N s_7_11: jump b4
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
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: write-var ga#320652 <= s_8_2
        fn_state.ga_320652 = s_8_2;
        // D s_8_4: read-var mbytes:i64
        let s_8_4: i64 = fn_state.mbytes;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: read-var addr:u64
        let s_8_6: u64 = fn_state.addr;
        // D s_8_7: read-var accdesc:struct
        let s_8_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_8_8: call Mem_read(s_8_6, s_8_5, s_8_7)
        let s_8_8: Bits = Mem_read(state, tracer, s_8_6, s_8_5, s_8_7);
        // D s_8_9: write-var ga#320653 <= s_8_8
        fn_state.ga_320653 = s_8_8;
        // N s_8_10: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var e:i64
        let s_9_0: i64 = fn_state.e;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var ga#320652:i64
        let s_9_2: i64 = fn_state.ga_320652;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var result:bv
        let s_9_4: Bits = fn_state.result;
        // D s_9_5: read-var ga#320653:bv
        let s_9_5: Bits = fn_state.ga_320653;
        // D s_9_6: call Elem_set(s_9_4, s_9_1, s_9_3, s_9_5)
        let s_9_6: Bits = Elem_set(state, tracer, s_9_4, s_9_1, s_9_3, s_9_5);
        // D s_9_7: write-var result <= s_9_6
        fn_state.result = s_9_6;
        // N s_9_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esize:i64
        let s_10_0: i64 = fn_state.esize;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var VLshadow#5518:i64
        let s_10_3: i64 = fn_state.VLshadow_5518;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast reint s_10_4 -> i64
        let s_10_5: i64 = (s_10_4 as i64);
        // D s_10_6: read-var t:i64
        let s_10_6: i64 = fn_state.t;
        // D s_10_7: cast zx s_10_6 -> i
        let s_10_7: i128 = (i128::try_from(s_10_6).unwrap());
        // D s_10_8: cast zx s_10_2 -> i
        let s_10_8: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_9: read-var slice_name:i64
        let s_10_9: i64 = fn_state.slice_name;
        // D s_10_10: cast zx s_10_9 -> i
        let s_10_10: i128 = (i128::try_from(s_10_9).unwrap());
        // D s_10_11: cast zx s_10_5 -> i
        let s_10_11: i128 = (i128::try_from(s_10_5).unwrap());
        // D s_10_12: read-var vertical:u8
        let s_10_12: bool = fn_state.vertical;
        // D s_10_13: read-var result:bv
        let s_10_13: Bits = fn_state.result;
        // D s_10_14: call ZAslice_set(s_10_7, s_10_8, s_10_12, s_10_10, s_10_11, s_10_13)
        let s_10_14: () = ZAslice_set(
            state,
            tracer,
            s_10_7,
            s_10_8,
            s_10_12,
            s_10_10,
            s_10_11,
            s_10_13,
        );
        // N s_10_15: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esize:i64
        let s_11_0: i64 = fn_state.esize;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: read-var mask:bv
        let s_11_2: Bits = fn_state.mask;
        // D s_11_3: call AnyActiveElement(s_11_2, s_11_1)
        let s_11_3: bool = AnyActiveElement(state, tracer, s_11_2, s_11_1);
        // N s_11_4: branch s_11_3 b17 b12
        if s_11_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #53u : u32
        let s_12_0: u32 = 53;
        // S s_12_1: call ConstrainUnpredictableBool(s_12_0)
        let s_12_1: bool = ConstrainUnpredictableBool(state, tracer, s_12_0);
        // D s_12_2: write-var gs#257304 <= s_12_1
        fn_state.gs_257304 = s_12_1;
        // N s_12_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#257304:u8
        let s_13_0: bool = fn_state.gs_257304;
        // N s_13_1: branch s_13_0 b16 b14
        if s_13_0 {
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
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call SP_read(s_15_0)
        let s_15_1: u64 = SP_read(state, tracer, s_15_0);
        // D s_15_2: write-var base <= s_15_1
        fn_state.base = s_15_1;
        // N s_15_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call CheckSPAlignment(s_16_0)
        let s_16_1: () = CheckSPAlignment(state, tracer, s_16_0);
        // N s_16_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#257304 <= s_17_0
        fn_state.gs_257304 = s_17_0;
        // N s_17_2: jump b13
        return block_13(state, tracer, fn_state);
    }
}
