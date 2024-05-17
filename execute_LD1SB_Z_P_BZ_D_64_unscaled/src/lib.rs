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
use Elem_read::*;
use Z_read::*;
use CheckNonStreamingSVEEnabled::*;
use Elem_set::*;
use SP_read::*;
use Mem_read::*;
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use asl_Int::*;
use CheckSPAlignment::*;
use Z_set::*;
use common::*;
pub fn execute_LD1SB_Z_P_BZ_D_64_unscaled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    m: i64,
    msize: i64,
    n: i64,
    offs_size: i64,
    offs_unsigned: bool,
    scale: i64,
    t: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        base: u64,
        gs_240477: bool,
        mbytes: i64,
        VLshadow_4960: i64,
        gs_240460: i64,
        mask: Bits,
        VLshadow_4959: i64,
        ga_310746: i64,
        data: Bits,
        accdesc: ProductType9878976b5bcce9c9,
        elements: i64,
        offset: Bits,
        result: Bits,
        ga_310747: u64,
        VL: i64,
        esize: i64,
        g: i64,
        m: i64,
        msize: i64,
        n: i64,
        offs_size: i64,
        offs_unsigned: bool,
        scale: i64,
        t: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        g,
        m,
        msize,
        n,
        offs_size,
        offs_unsigned,
        scale,
        t,
        is_unsigned,
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
        // D s_0_3: write-var VLshadow#4959 <= s_0_2
        fn_state.VLshadow_4959 = s_0_2;
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
        // D s_1_0: read-var VLshadow#4959:i64
        let s_1_0: i64 = fn_state.VLshadow_4959;
        // D s_1_1: write-var VLshadow#4960 <= s_1_0
        fn_state.VLshadow_4960 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#4960:i64
        let s_1_3: i64 = fn_state.VLshadow_4960;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#4960:i64
        let s_1_7: i64 = fn_state.VLshadow_4960;
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
        // D s_1_13: write-var elements <= s_1_12
        fn_state.elements = s_1_12;
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
        // C s_1_21: const #8s : i
        let s_1_21: i128 = 8;
        // D s_1_22: read-var msize:i64
        let s_1_22: i64 = fn_state.msize;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: div s_1_23 s_1_21
        let s_1_24: i128 = ((s_1_23) / (s_1_21));
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: write-var mbytes <= s_1_25
        fn_state.mbytes = s_1_25;
        // C s_1_27: const #0u : u32
        let s_1_27: u32 = 0;
        // C s_1_28: const #0u : u8
        let s_1_28: bool = false;
        // C s_1_29: const #0u : u8
        let s_1_29: bool = false;
        // C s_1_30: const #1u : u8
        let s_1_30: bool = true;
        // S s_1_31: call CreateAccDescSVE(s_1_27, s_1_28, s_1_29, s_1_30)
        let s_1_31: ProductType9878976b5bcce9c9 = CreateAccDescSVE(
            state,
            tracer,
            s_1_27,
            s_1_28,
            s_1_29,
            s_1_30,
        );
        // D s_1_32: write-var accdesc <= s_1_31
        fn_state.accdesc = s_1_31;
        // D s_1_33: read-var esize:i64
        let s_1_33: i64 = fn_state.esize;
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: read-var mask:bv
        let s_1_35: Bits = fn_state.mask;
        // D s_1_36: call AnyActiveElement(s_1_35, s_1_34)
        let s_1_36: bool = AnyActiveElement(state, tracer, s_1_35, s_1_34);
        // D s_1_37: not s_1_36
        let s_1_37: bool = !s_1_36;
        // N s_1_38: branch s_1_37 b20 b2
        if s_1_37 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #31s : i
        let s_2_0: i128 = 31;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_0
        let s_2_3: bool = ((s_2_2) == (s_2_0));
        // N s_2_4: branch s_2_3 b19 b3
        if s_2_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #31s : i
        let s_4_0: i128 = 31;
        // D s_4_1: read-var n:i64
        let s_4_1: i64 = fn_state.n;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_0
        let s_4_3: bool = ((s_4_2) == (s_4_0));
        // N s_4_4: branch s_4_3 b18 b5
        if s_4_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: call X_read(s_5_2, s_5_0)
        let s_5_3: Bits = X_read(state, tracer, s_5_2, s_5_0);
        // D s_5_4: cast reint s_5_3 -> u64
        let s_5_4: u64 = (s_5_3.value() as u64);
        // D s_5_5: write-var base <= s_5_4
        fn_state.base = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var VLshadow#4960:i64
        let s_6_0: i64 = fn_state.VLshadow_4960;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var m:i64
        let s_6_3: i64 = fn_state.m;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast zx s_6_2 -> i
        let s_6_5: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_6: call Z_read(s_6_4, s_6_5)
        let s_6_6: Bits = Z_read(state, tracer, s_6_4, s_6_5);
        // D s_6_7: write-var offset <= s_6_6
        fn_state.offset = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0s : i64
        let s_7_0: i64 = 0;
        // C s_7_1: const #1s : i
        let s_7_1: i128 = 1;
        // D s_7_2: read-var elements:i64
        let s_7_2: i64 = fn_state.elements;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: sub s_7_3 s_7_1
        let s_7_4: i128 = ((s_7_3) - (s_7_1));
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: write-var gs#240460 <= s_7_5
        fn_state.gs_240460 = s_7_5;
        // D s_7_7: write-var e <= s_7_0
        fn_state.e = s_7_0;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var e:i64
        let s_8_0: i64 = fn_state.e;
        // D s_8_1: read-var gs#240460:i64
        let s_8_1: i64 = fn_state.gs_240460;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b17 b9
        if s_8_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
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
        // D s_9_2: read-var esize:i64
        let s_9_2: i64 = fn_state.esize;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var mask:bv
        let s_9_4: Bits = fn_state.mask;
        // D s_9_5: call ActivePredicateElement(s_9_4, s_9_1, s_9_3)
        let s_9_5: bool = ActivePredicateElement(state, tracer, s_9_4, s_9_1, s_9_3);
        // N s_9_6: branch s_9_5 b12 b10
        if s_9_5 {
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
        // D s_10_0: read-var esize:i64
        let s_10_0: i64 = fn_state.esize;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var esize:i64
        let s_10_3: i64 = fn_state.esize;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: call Zeros(s_10_4)
        let s_10_5: Bits = Zeros(state, tracer, s_10_4);
        // D s_10_6: cast reint s_10_5 -> u64
        let s_10_6: u64 = (s_10_5.value() as u64);
        // D s_10_7: read-var e:i64
        let s_10_7: i64 = fn_state.e;
        // D s_10_8: cast zx s_10_7 -> i
        let s_10_8: i128 = (i128::try_from(s_10_7).unwrap());
        // D s_10_9: cast zx s_10_2 -> i
        let s_10_9: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_10: cast zx s_10_6 -> bv
        let s_10_10: Bits = Bits::new(s_10_6 as u128, 64u16);
        // D s_10_11: read-var result:bv
        let s_10_11: Bits = fn_state.result;
        // D s_10_12: call Elem_set(s_10_11, s_10_8, s_10_9, s_10_10)
        let s_10_12: Bits = Elem_set(state, tracer, s_10_11, s_10_8, s_10_9, s_10_10);
        // D s_10_13: write-var result <= s_10_12
        fn_state.result = s_10_12;
        // N s_10_14: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var e:i64
        let s_11_0: i64 = fn_state.e;
        // C s_11_1: const #1s : i64
        let s_11_1: i64 = 1;
        // D s_11_2: add s_11_0 s_11_1
        let s_11_2: i64 = (s_11_0 + s_11_1);
        // D s_11_3: write-var e <= s_11_2
        fn_state.e = s_11_2;
        // N s_11_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var esize:i64
        let s_12_0: i64 = fn_state.esize;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // D s_12_3: read-var e:i64
        let s_12_3: i64 = fn_state.e;
        // D s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_5: cast zx s_12_2 -> i
        let s_12_5: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_6: read-var offset:bv
        let s_12_6: Bits = fn_state.offset;
        // D s_12_7: call Elem_read(s_12_6, s_12_4, s_12_5)
        let s_12_7: Bits = Elem_read(state, tracer, s_12_6, s_12_4, s_12_5);
        // D s_12_8: cast reint s_12_7 -> u64
        let s_12_8: u64 = (s_12_7.value() as u64);
        // C s_12_9: const #0s : i
        let s_12_9: i128 = 0;
        // C s_12_10: const #64s : i
        let s_12_10: i128 = 64;
        // D s_12_11: cast zx s_12_8 -> bv
        let s_12_11: Bits = Bits::new(s_12_8 as u128, 64u16);
        // D s_12_12: bit-extract s_12_11 s_12_9 s_12_10
        let s_12_12: Bits = (Bits::new(
            ((s_12_11) >> (s_12_9)).value(),
            u16::try_from(s_12_10).unwrap(),
        ));
        // D s_12_13: cast reint s_12_12 -> u64
        let s_12_13: u64 = (s_12_12.value() as u64);
        // D s_12_14: cast zx s_12_13 -> bv
        let s_12_14: Bits = Bits::new(s_12_13 as u128, 64u16);
        // D s_12_15: read-var offs_unsigned:u8
        let s_12_15: bool = fn_state.offs_unsigned;
        // D s_12_16: call asl_Int(s_12_14, s_12_15)
        let s_12_16: i128 = asl_Int(state, tracer, s_12_14, s_12_15);
        // D s_12_17: read-var scale:i64
        let s_12_17: i64 = fn_state.scale;
        // D s_12_18: cast zx s_12_17 -> i
        let s_12_18: i128 = (i128::try_from(s_12_17).unwrap());
        // D s_12_19: lsl s_12_16 s_12_18
        let s_12_19: i128 = s_12_16 << s_12_18;
        // D s_12_20: read-var base:u64
        let s_12_20: u64 = fn_state.base;
        // D s_12_21: cast zx s_12_20 -> bv
        let s_12_21: Bits = Bits::new(s_12_20 as u128, 64u16);
        // D s_12_22: cast cvt s_12_19 -> bv
        let s_12_22: Bits = Bits::new(s_12_19 as u128, 128);
        // D s_12_23: add s_12_21 s_12_22
        let s_12_23: Bits = (s_12_21 + s_12_22);
        // D s_12_24: cast reint s_12_23 -> u64
        let s_12_24: u64 = (s_12_23.value() as u64);
        // D s_12_25: read-var mbytes:i64
        let s_12_25: i64 = fn_state.mbytes;
        // D s_12_26: cast zx s_12_25 -> i
        let s_12_26: i128 = (i128::try_from(s_12_25).unwrap());
        // D s_12_27: read-var accdesc:struct
        let s_12_27: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_12_28: call Mem_read(s_12_24, s_12_26, s_12_27)
        let s_12_28: Bits = Mem_read(state, tracer, s_12_24, s_12_26, s_12_27);
        // D s_12_29: write-var data <= s_12_28
        fn_state.data = s_12_28;
        // N s_12_30: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var esize:i64
        let s_13_0: i64 = fn_state.esize;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // D s_13_3: write-var ga#310746 <= s_13_2
        fn_state.ga_310746 = s_13_2;
        // D s_13_4: read-var is_unsigned:u8
        let s_13_4: bool = fn_state.is_unsigned;
        // N s_13_5: branch s_13_4 b16 b14
        if s_13_4 {
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
        // D s_14_0: read-var esize:i64
        let s_14_0: i64 = fn_state.esize;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: read-var data:bv
        let s_14_2: Bits = fn_state.data;
        // D s_14_3: bits-cast sx s_14_2 -> bv length s_14_1
        let s_14_3: Bits = s_14_2.sign_extend(s_14_1);
        // D s_14_4: cast reint s_14_3 -> u64
        let s_14_4: u64 = (s_14_3.value() as u64);
        // D s_14_5: write-var ga#310747 <= s_14_4
        fn_state.ga_310747 = s_14_4;
        // N s_14_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var e:i64
        let s_15_0: i64 = fn_state.e;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: read-var ga#310746:i64
        let s_15_2: i64 = fn_state.ga_310746;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: read-var ga#310747:u64
        let s_15_4: u64 = fn_state.ga_310747;
        // D s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 64u16);
        // D s_15_6: read-var result:bv
        let s_15_6: Bits = fn_state.result;
        // D s_15_7: call Elem_set(s_15_6, s_15_1, s_15_3, s_15_5)
        let s_15_7: Bits = Elem_set(state, tracer, s_15_6, s_15_1, s_15_3, s_15_5);
        // D s_15_8: write-var result <= s_15_7
        fn_state.result = s_15_7;
        // N s_15_9: jump b11
        return block_11(state, tracer, fn_state);
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
        // D s_16_2: read-var data:bv
        let s_16_2: Bits = fn_state.data;
        // D s_16_3: bits-cast zx s_16_2 -> bv length s_16_1
        let s_16_3: Bits = s_16_2.zero_extend(s_16_1);
        // D s_16_4: cast reint s_16_3 -> u64
        let s_16_4: u64 = (s_16_3.value() as u64);
        // D s_16_5: write-var ga#310747 <= s_16_4
        fn_state.ga_310747 = s_16_4;
        // N s_16_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var VLshadow#4960:i64
        let s_17_0: i64 = fn_state.VLshadow_4960;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: cast reint s_17_1 -> i64
        let s_17_2: i64 = (s_17_1 as i64);
        // D s_17_3: read-var t:i64
        let s_17_3: i64 = fn_state.t;
        // D s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_5: cast zx s_17_2 -> i
        let s_17_5: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_6: read-var result:bv
        let s_17_6: Bits = fn_state.result;
        // D s_17_7: call Z_set(s_17_4, s_17_5, s_17_6)
        let s_17_7: () = Z_set(state, tracer, s_17_4, s_17_5, s_17_6);
        // N s_17_8: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call SP_read(s_18_0)
        let s_18_1: u64 = SP_read(state, tracer, s_18_0);
        // D s_18_2: write-var base <= s_18_1
        fn_state.base = s_18_1;
        // N s_18_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call CheckSPAlignment(s_19_0)
        let s_19_1: () = CheckSPAlignment(state, tracer, s_19_0);
        // N s_19_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #31s : i
        let s_20_0: i128 = 31;
        // D s_20_1: read-var n:i64
        let s_20_1: i64 = fn_state.n;
        // D s_20_2: cast zx s_20_1 -> i
        let s_20_2: i128 = (i128::try_from(s_20_1).unwrap());
        // D s_20_3: cmp-eq s_20_2 s_20_0
        let s_20_3: bool = ((s_20_2) == (s_20_0));
        // N s_20_4: branch s_20_3 b26 b21
        if s_20_3 {
            return block_26(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#240477 <= s_21_0
        fn_state.gs_240477 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#240477:u8
        let s_22_0: bool = fn_state.gs_240477;
        // N s_22_1: branch s_22_0 b25 b23
        if s_22_0 {
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
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call CheckSPAlignment(s_25_0)
        let s_25_1: () = CheckSPAlignment(state, tracer, s_25_0);
        // N s_25_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #53u : u32
        let s_26_0: u32 = 53;
        // S s_26_1: call ConstrainUnpredictableBool(s_26_0)
        let s_26_1: bool = ConstrainUnpredictableBool(state, tracer, s_26_0);
        // D s_26_2: write-var gs#240477 <= s_26_1
        fn_state.gs_240477 = s_26_1;
        // N s_26_3: jump b22
        return block_22(state, tracer, fn_state);
    }
}
