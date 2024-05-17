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
use CreateAccDescSVE::*;
use ConstrainUnpredictableBool::*;
use X_read::*;
use Zeros::*;
use CheckNonStreamingSVEEnabled::*;
use neq_int::*;
use Elem_set::*;
use SP_read::*;
use Mem_read::*;
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use replicate_bits_borealis_internal::*;
use CheckSPAlignment::*;
use Z_set::*;
use common::*;
pub fn execute_LD1ROB_Z_P_BI_U8<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    n: i64,
    offset: i128,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_308751: Bits,
        e: i64,
        VLshadow_4832: i64,
        base: u64,
        mbytes: i64,
        PL: i64,
        mask: Bits,
        VLshadow_4831: i64,
        gs_236806: i64,
        ga_308750: i64,
        accdesc: ProductType9878976b5bcce9c9,
        elements: i64,
        result: u64,
        gs_236819: bool,
        VL: i64,
        esize: i64,
        g: i64,
        n: i64,
        offset: i128,
        t: i64,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        g,
        n,
        offset,
        t,
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
        // D s_0_3: write-var VLshadow#4831 <= s_0_2
        fn_state.VLshadow_4831 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckNonStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#4831:i64
        let s_1_0: i64 = fn_state.VLshadow_4831;
        // D s_1_1: write-var VLshadow#4832 <= s_1_0
        fn_state.VLshadow_4832 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#4832:i64
        let s_1_3: i64 = fn_state.VLshadow_4832;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: write-var PL <= s_1_6
        fn_state.PL = s_1_6;
        // C s_1_8: const #256s : i
        let s_1_8: i128 = 256;
        // D s_1_9: read-var VLshadow#4832:i64
        let s_1_9: i64 = fn_state.VLshadow_4832;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: cmp-lt s_1_10 s_1_8
        let s_1_11: bool = ((s_1_10) < (s_1_8));
        // N s_1_12: branch s_1_11 b25 b2
        if s_1_11 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #256s : i
        let s_2_0: i128 = 256;
        // D s_2_1: read-var esize:i64
        let s_2_1: i64 = fn_state.esize;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: div s_2_0 s_2_2
        let s_2_3: i128 = ((s_2_0) / (s_2_2));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: write-var elements <= s_2_4
        fn_state.elements = s_2_4;
        // D s_2_6: read-var PL:i64
        let s_2_6: i64 = fn_state.PL;
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // D s_2_9: read-var g:i64
        let s_2_9: i64 = fn_state.g;
        // D s_2_10: cast zx s_2_9 -> i
        let s_2_10: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_11: cast zx s_2_8 -> i
        let s_2_11: i128 = (i128::try_from(s_2_8).unwrap());
        // D s_2_12: call P_read(s_2_10, s_2_11)
        let s_2_12: Bits = P_read(state, tracer, s_2_10, s_2_11);
        // D s_2_13: write-var mask <= s_2_12
        fn_state.mask = s_2_12;
        // C s_2_14: const #8s : i
        let s_2_14: i128 = 8;
        // D s_2_15: read-var esize:i64
        let s_2_15: i64 = fn_state.esize;
        // D s_2_16: cast zx s_2_15 -> i
        let s_2_16: i128 = (i128::try_from(s_2_15).unwrap());
        // D s_2_17: div s_2_16 s_2_14
        let s_2_17: i128 = ((s_2_16) / (s_2_14));
        // D s_2_18: cast reint s_2_17 -> i64
        let s_2_18: i64 = (s_2_17 as i64);
        // D s_2_19: write-var mbytes <= s_2_18
        fn_state.mbytes = s_2_18;
        // C s_2_20: const #31s : i
        let s_2_20: i128 = 31;
        // D s_2_21: read-var n:i64
        let s_2_21: i64 = fn_state.n;
        // D s_2_22: cast zx s_2_21 -> i
        let s_2_22: i128 = (i128::try_from(s_2_21).unwrap());
        // D s_2_23: call neq_int(s_2_22, s_2_20)
        let s_2_23: bool = neq_int(state, tracer, s_2_22, s_2_20);
        // C s_2_24: const #0u : u32
        let s_2_24: u32 = 0;
        // C s_2_25: const #0u : u8
        let s_2_25: bool = false;
        // C s_2_26: const #1u : u8
        let s_2_26: bool = true;
        // D s_2_27: call CreateAccDescSVE(s_2_24, s_2_25, s_2_26, s_2_23)
        let s_2_27: ProductType9878976b5bcce9c9 = CreateAccDescSVE(
            state,
            tracer,
            s_2_24,
            s_2_25,
            s_2_26,
            s_2_23,
        );
        // D s_2_28: write-var accdesc <= s_2_27
        fn_state.accdesc = s_2_27;
        // D s_2_29: read-var esize:i64
        let s_2_29: i64 = fn_state.esize;
        // D s_2_30: cast zx s_2_29 -> i
        let s_2_30: i128 = (i128::try_from(s_2_29).unwrap());
        // D s_2_31: read-var mask:bv
        let s_2_31: Bits = fn_state.mask;
        // D s_2_32: call AnyActiveElement(s_2_31, s_2_30)
        let s_2_32: bool = AnyActiveElement(state, tracer, s_2_31, s_2_30);
        // D s_2_33: not s_2_32
        let s_2_33: bool = !s_2_32;
        // N s_2_34: branch s_2_33 b18 b3
        if s_2_33 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #31s : i
        let s_3_0: i128 = 31;
        // D s_3_1: read-var n:i64
        let s_3_1: i64 = fn_state.n;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // N s_3_4: branch s_3_3 b17 b4
        if s_3_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #31s : i
        let s_5_0: i128 = 31;
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // N s_5_4: branch s_5_3 b16 b6
        if s_5_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: read-var n:i64
        let s_6_1: i64 = fn_state.n;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: call X_read(s_6_2, s_6_0)
        let s_6_3: Bits = X_read(state, tracer, s_6_2, s_6_0);
        // D s_6_4: cast reint s_6_3 -> u64
        let s_6_4: u64 = (s_6_3.value() as u64);
        // D s_6_5: write-var base <= s_6_4
        fn_state.base = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0s : i64
        let s_8_0: i64 = 0;
        // C s_8_1: const #1s : i
        let s_8_1: i128 = 1;
        // D s_8_2: read-var elements:i64
        let s_8_2: i64 = fn_state.elements;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: sub s_8_3 s_8_1
        let s_8_4: i128 = ((s_8_3) - (s_8_1));
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // D s_8_6: write-var gs#236806 <= s_8_5
        fn_state.gs_236806 = s_8_5;
        // D s_8_7: write-var e <= s_8_0
        fn_state.e = s_8_0;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var e:i64
        let s_9_0: i64 = fn_state.e;
        // D s_9_1: read-var gs#236806:i64
        let s_9_1: i64 = fn_state.gs_236806;
        // D s_9_2: cmp-gt s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) > (s_9_1));
        // N s_9_3: branch s_9_2 b15 b10
        if s_9_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var e:i64
        let s_10_0: i64 = fn_state.e;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var esize:i64
        let s_10_2: i64 = fn_state.esize;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: read-var mask:bv
        let s_10_4: Bits = fn_state.mask;
        // D s_10_5: call ActivePredicateElement(s_10_4, s_10_1, s_10_3)
        let s_10_5: bool = ActivePredicateElement(state, tracer, s_10_4, s_10_1, s_10_3);
        // N s_10_6: branch s_10_5 b13 b11
        if s_10_5 {
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
        // D s_11_0: read-var esize:i64
        let s_11_0: i64 = fn_state.esize;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // D s_11_3: read-var esize:i64
        let s_11_3: i64 = fn_state.esize;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: call Zeros(s_11_4)
        let s_11_5: Bits = Zeros(state, tracer, s_11_4);
        // D s_11_6: cast reint s_11_5 -> u8
        let s_11_6: u8 = (s_11_5.value() as u8);
        // D s_11_7: read-var result:u256
        let s_11_7: u64 = fn_state.result;
        // D s_11_8: cast zx s_11_7 -> bv
        let s_11_8: Bits = Bits::new(s_11_7 as u128, 256u16);
        // D s_11_9: read-var e:i64
        let s_11_9: i64 = fn_state.e;
        // D s_11_10: cast zx s_11_9 -> i
        let s_11_10: i128 = (i128::try_from(s_11_9).unwrap());
        // D s_11_11: cast zx s_11_2 -> i
        let s_11_11: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_12: cast zx s_11_6 -> bv
        let s_11_12: Bits = Bits::new(s_11_6 as u128, 8u16);
        // D s_11_13: call Elem_set(s_11_8, s_11_10, s_11_11, s_11_12)
        let s_11_13: Bits = Elem_set(state, tracer, s_11_8, s_11_10, s_11_11, s_11_12);
        // D s_11_14: cast reint s_11_13 -> u256
        let s_11_14: u64 = (s_11_13.value() as u64);
        // D s_11_15: write-var result <= s_11_14
        fn_state.result = s_11_14;
        // N s_11_16: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var e:i64
        let s_12_0: i64 = fn_state.e;
        // C s_12_1: const #1s : i64
        let s_12_1: i64 = 1;
        // D s_12_2: add s_12_0 s_12_1
        let s_12_2: i64 = (s_12_0 + s_12_1);
        // D s_12_3: write-var e <= s_12_2
        fn_state.e = s_12_2;
        // N s_12_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var elements:i64
        let s_13_0: i64 = fn_state.elements;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var offset:i
        let s_13_2: i128 = fn_state.offset;
        // D s_13_3: mul s_13_2 s_13_1
        let s_13_3: i128 = ((s_13_2) * (s_13_1));
        // D s_13_4: read-var e:i64
        let s_13_4: i64 = fn_state.e;
        // D s_13_5: cast zx s_13_4 -> i
        let s_13_5: i128 = (i128::try_from(s_13_4).unwrap());
        // D s_13_6: add s_13_3 s_13_5
        let s_13_6: i128 = (s_13_3 + s_13_5);
        // D s_13_7: read-var mbytes:i64
        let s_13_7: i64 = fn_state.mbytes;
        // D s_13_8: cast zx s_13_7 -> i
        let s_13_8: i128 = (i128::try_from(s_13_7).unwrap());
        // D s_13_9: mul s_13_6 s_13_8
        let s_13_9: i128 = ((s_13_6) * (s_13_8));
        // D s_13_10: read-var base:u64
        let s_13_10: u64 = fn_state.base;
        // D s_13_11: cast zx s_13_10 -> bv
        let s_13_11: Bits = Bits::new(s_13_10 as u128, 64u16);
        // D s_13_12: cast cvt s_13_9 -> bv
        let s_13_12: Bits = Bits::new(s_13_9 as u128, 128);
        // D s_13_13: add s_13_11 s_13_12
        let s_13_13: Bits = (s_13_11 + s_13_12);
        // D s_13_14: cast reint s_13_13 -> u64
        let s_13_14: u64 = (s_13_13.value() as u64);
        // D s_13_15: read-var esize:i64
        let s_13_15: i64 = fn_state.esize;
        // D s_13_16: cast zx s_13_15 -> i
        let s_13_16: i128 = (i128::try_from(s_13_15).unwrap());
        // D s_13_17: cast reint s_13_16 -> i64
        let s_13_17: i64 = (s_13_16 as i64);
        // D s_13_18: write-var ga#308750 <= s_13_17
        fn_state.ga_308750 = s_13_17;
        // D s_13_19: read-var mbytes:i64
        let s_13_19: i64 = fn_state.mbytes;
        // D s_13_20: cast zx s_13_19 -> i
        let s_13_20: i128 = (i128::try_from(s_13_19).unwrap());
        // D s_13_21: read-var accdesc:struct
        let s_13_21: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_13_22: call Mem_read(s_13_14, s_13_20, s_13_21)
        let s_13_22: Bits = Mem_read(state, tracer, s_13_14, s_13_20, s_13_21);
        // D s_13_23: write-var ga#308751 <= s_13_22
        fn_state.ga_308751 = s_13_22;
        // N s_13_24: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var result:u256
        let s_14_0: u64 = fn_state.result;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 256u16);
        // D s_14_2: read-var e:i64
        let s_14_2: i64 = fn_state.e;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: read-var ga#308750:i64
        let s_14_4: i64 = fn_state.ga_308750;
        // D s_14_5: cast zx s_14_4 -> i
        let s_14_5: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_6: read-var ga#308751:bv
        let s_14_6: Bits = fn_state.ga_308751;
        // D s_14_7: call Elem_set(s_14_1, s_14_3, s_14_5, s_14_6)
        let s_14_7: Bits = Elem_set(state, tracer, s_14_1, s_14_3, s_14_5, s_14_6);
        // D s_14_8: cast reint s_14_7 -> u256
        let s_14_8: u64 = (s_14_7.value() as u64);
        // D s_14_9: write-var result <= s_14_8
        fn_state.result = s_14_8;
        // N s_14_10: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var VLshadow#4832:i64
        let s_15_0: i64 = fn_state.VLshadow_4832;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: cast reint s_15_1 -> i64
        let s_15_2: i64 = (s_15_1 as i64);
        // C s_15_3: const #256s : i
        let s_15_3: i128 = 256;
        // D s_15_4: read-var VLshadow#4832:i64
        let s_15_4: i64 = fn_state.VLshadow_4832;
        // D s_15_5: cast zx s_15_4 -> i
        let s_15_5: i128 = (i128::try_from(s_15_4).unwrap());
        // D s_15_6: div s_15_5 s_15_3
        let s_15_6: i128 = ((s_15_5) / (s_15_3));
        // D s_15_7: cast reint s_15_6 -> i64
        let s_15_7: i64 = (s_15_6 as i64);
        // D s_15_8: read-var result:u256
        let s_15_8: u64 = fn_state.result;
        // D s_15_9: cast zx s_15_8 -> bv
        let s_15_9: Bits = Bits::new(s_15_8 as u128, 256u16);
        // D s_15_10: cast zx s_15_7 -> i
        let s_15_10: i128 = (i128::try_from(s_15_7).unwrap());
        // D s_15_11: cast reint s_15_10 -> u64
        let s_15_11: u64 = (s_15_10 as u64);
        // D s_15_12: call replicate_bits_borealis_internal(s_15_9, s_15_11)
        let s_15_12: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_15_9,
            s_15_11,
        );
        // D s_15_13: read-var VLshadow#4832:i64
        let s_15_13: i64 = fn_state.VLshadow_4832;
        // D s_15_14: cast zx s_15_13 -> i
        let s_15_14: i128 = (i128::try_from(s_15_13).unwrap());
        // D s_15_15: bits-cast zx s_15_12 -> bv length s_15_14
        let s_15_15: Bits = s_15_12.zero_extend(s_15_14);
        // D s_15_16: read-var t:i64
        let s_15_16: i64 = fn_state.t;
        // D s_15_17: cast zx s_15_16 -> i
        let s_15_17: i128 = (i128::try_from(s_15_16).unwrap());
        // D s_15_18: cast zx s_15_2 -> i
        let s_15_18: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_19: call Z_set(s_15_17, s_15_18, s_15_15)
        let s_15_19: () = Z_set(state, tracer, s_15_17, s_15_18, s_15_15);
        // N s_15_20: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call SP_read(s_16_0)
        let s_16_1: u64 = SP_read(state, tracer, s_16_0);
        // D s_16_2: write-var base <= s_16_1
        fn_state.base = s_16_1;
        // N s_16_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call CheckSPAlignment(s_17_0)
        let s_17_1: () = CheckSPAlignment(state, tracer, s_17_0);
        // N s_17_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #31s : i
        let s_18_0: i128 = 31;
        // D s_18_1: read-var n:i64
        let s_18_1: i64 = fn_state.n;
        // D s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (i128::try_from(s_18_1).unwrap());
        // D s_18_3: cmp-eq s_18_2 s_18_0
        let s_18_3: bool = ((s_18_2) == (s_18_0));
        // N s_18_4: branch s_18_3 b24 b19
        if s_18_3 {
            return block_24(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#236819 <= s_19_0
        fn_state.gs_236819 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#236819:u8
        let s_20_0: bool = fn_state.gs_236819;
        // N s_20_1: branch s_20_0 b23 b21
        if s_20_0 {
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
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call CheckSPAlignment(s_23_0)
        let s_23_1: () = CheckSPAlignment(state, tracer, s_23_0);
        // N s_23_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #53u : u32
        let s_24_0: u32 = 53;
        // S s_24_1: call ConstrainUnpredictableBool(s_24_0)
        let s_24_1: bool = ConstrainUnpredictableBool(state, tracer, s_24_0);
        // D s_24_2: write-var gs#236819 <= s_24_1
        fn_state.gs_236819 = s_24_1;
        // N s_24_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
}
