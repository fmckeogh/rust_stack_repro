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
use Elem_set::*;
use Mem_read::*;
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use X_read::*;
use Zeros::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_LDNT1D_Z_P_AR_D_64_unscaled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    m: i64,
    msize: i64,
    n: i64,
    t: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        ga_315000: i64,
        VLshadow_5230: i64,
        base: Bits,
        gs_248794: i64,
        data: Bits,
        mbytes: i64,
        accdesc: ProductType9878976b5bcce9c9,
        baseshadow_5232: Bits,
        elements: i64,
        offset: u64,
        result: Bits,
        offsetshadow_5231: u64,
        ga_315001: u64,
        mask: Bits,
        VLshadow_5229: i64,
        VL: i64,
        esize: i64,
        g: i64,
        m: i64,
        msize: i64,
        n: i64,
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
        // D s_0_3: write-var VLshadow#5229 <= s_0_2
        fn_state.VLshadow_5229 = s_0_2;
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
        // D s_1_0: read-var VLshadow#5229:i64
        let s_1_0: i64 = fn_state.VLshadow_5229;
        // D s_1_1: write-var VLshadow#5230 <= s_1_0
        fn_state.VLshadow_5230 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#5230:i64
        let s_1_3: i64 = fn_state.VLshadow_5230;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#5230:i64
        let s_1_7: i64 = fn_state.VLshadow_5230;
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
        // C s_1_28: const #1u : u8
        let s_1_28: bool = true;
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
        // N s_1_37: branch s_1_36 b14 b2
        if s_1_36 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var offset:u64
        let s_3_0: u64 = fn_state.offset;
        // D s_3_1: write-var offsetshadow#5231 <= s_3_0
        fn_state.offsetshadow_5231 = s_3_0;
        // D s_3_2: read-var base:bv
        let s_3_2: Bits = fn_state.base;
        // D s_3_3: write-var baseshadow#5232 <= s_3_2
        fn_state.baseshadow_5232 = s_3_2;
        // C s_3_4: const #0s : i64
        let s_3_4: i64 = 0;
        // C s_3_5: const #1s : i
        let s_3_5: i128 = 1;
        // D s_3_6: read-var elements:i64
        let s_3_6: i64 = fn_state.elements;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: sub s_3_7 s_3_5
        let s_3_8: i128 = ((s_3_7) - (s_3_5));
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: write-var gs#248794 <= s_3_9
        fn_state.gs_248794 = s_3_9;
        // D s_3_11: write-var e <= s_3_4
        fn_state.e = s_3_4;
        // N s_3_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#248794:i64
        let s_4_1: i64 = fn_state.gs_248794;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b13 b5
        if s_4_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var esize:i64
        let s_5_2: i64 = fn_state.esize;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var mask:bv
        let s_5_4: Bits = fn_state.mask;
        // D s_5_5: call ActivePredicateElement(s_5_4, s_5_1, s_5_3)
        let s_5_5: bool = ActivePredicateElement(state, tracer, s_5_4, s_5_1, s_5_3);
        // N s_5_6: branch s_5_5 b8 b6
        if s_5_5 {
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
        // D s_6_6: cast reint s_6_5 -> u64
        let s_6_6: u64 = (s_6_5.value() as u64);
        // D s_6_7: read-var e:i64
        let s_6_7: i64 = fn_state.e;
        // D s_6_8: cast zx s_6_7 -> i
        let s_6_8: i128 = (i128::try_from(s_6_7).unwrap());
        // D s_6_9: cast zx s_6_2 -> i
        let s_6_9: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_10: cast zx s_6_6 -> bv
        let s_6_10: Bits = Bits::new(s_6_6 as u128, 64u16);
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
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var e <= s_7_2
        fn_state.e = s_7_2;
        // N s_7_4: jump b4
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
        // D s_8_3: read-var e:i64
        let s_8_3: i64 = fn_state.e;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cast zx s_8_2 -> i
        let s_8_5: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_6: read-var baseshadow#5232:bv
        let s_8_6: Bits = fn_state.baseshadow_5232;
        // D s_8_7: call Elem_read(s_8_6, s_8_4, s_8_5)
        let s_8_7: Bits = Elem_read(state, tracer, s_8_6, s_8_4, s_8_5);
        // D s_8_8: cast reint s_8_7 -> u64
        let s_8_8: u64 = (s_8_7.value() as u64);
        // C s_8_9: const #64s : i
        let s_8_9: i128 = 64;
        // D s_8_10: cast zx s_8_8 -> bv
        let s_8_10: Bits = Bits::new(s_8_8 as u128, 64u16);
        // D s_8_11: bits-cast zx s_8_10 -> bv length s_8_9
        let s_8_11: Bits = s_8_10.zero_extend(s_8_9);
        // D s_8_12: cast reint s_8_11 -> u64
        let s_8_12: u64 = (s_8_11.value() as u64);
        // D s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 64u16);
        // D s_8_14: read-var offsetshadow#5231:u64
        let s_8_14: u64 = fn_state.offsetshadow_5231;
        // D s_8_15: cast zx s_8_14 -> bv
        let s_8_15: Bits = Bits::new(s_8_14 as u128, 64u16);
        // D s_8_16: add s_8_13 s_8_15
        let s_8_16: Bits = (s_8_13 + s_8_15);
        // D s_8_17: cast reint s_8_16 -> u64
        let s_8_17: u64 = (s_8_16.value() as u64);
        // D s_8_18: read-var mbytes:i64
        let s_8_18: i64 = fn_state.mbytes;
        // D s_8_19: cast zx s_8_18 -> i
        let s_8_19: i128 = (i128::try_from(s_8_18).unwrap());
        // D s_8_20: read-var accdesc:struct
        let s_8_20: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_8_21: call Mem_read(s_8_17, s_8_19, s_8_20)
        let s_8_21: Bits = Mem_read(state, tracer, s_8_17, s_8_19, s_8_20);
        // D s_8_22: write-var data <= s_8_21
        fn_state.data = s_8_21;
        // N s_8_23: jump b9
        return block_9(state, tracer, fn_state);
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
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: write-var ga#315000 <= s_9_2
        fn_state.ga_315000 = s_9_2;
        // D s_9_4: read-var is_unsigned:u8
        let s_9_4: bool = fn_state.is_unsigned;
        // N s_9_5: branch s_9_4 b12 b10
        if s_9_4 {
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
        // D s_10_2: read-var data:bv
        let s_10_2: Bits = fn_state.data;
        // D s_10_3: bits-cast sx s_10_2 -> bv length s_10_1
        let s_10_3: Bits = s_10_2.sign_extend(s_10_1);
        // D s_10_4: cast reint s_10_3 -> u64
        let s_10_4: u64 = (s_10_3.value() as u64);
        // D s_10_5: write-var ga#315001 <= s_10_4
        fn_state.ga_315001 = s_10_4;
        // N s_10_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var e:i64
        let s_11_0: i64 = fn_state.e;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: read-var ga#315000:i64
        let s_11_2: i64 = fn_state.ga_315000;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: read-var ga#315001:u64
        let s_11_4: u64 = fn_state.ga_315001;
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 64u16);
        // D s_11_6: read-var result:bv
        let s_11_6: Bits = fn_state.result;
        // D s_11_7: call Elem_set(s_11_6, s_11_1, s_11_3, s_11_5)
        let s_11_7: Bits = Elem_set(state, tracer, s_11_6, s_11_1, s_11_3, s_11_5);
        // D s_11_8: write-var result <= s_11_7
        fn_state.result = s_11_7;
        // N s_11_9: jump b7
        return block_7(state, tracer, fn_state);
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
        // D s_12_2: read-var data:bv
        let s_12_2: Bits = fn_state.data;
        // D s_12_3: bits-cast zx s_12_2 -> bv length s_12_1
        let s_12_3: Bits = s_12_2.zero_extend(s_12_1);
        // D s_12_4: cast reint s_12_3 -> u64
        let s_12_4: u64 = (s_12_3.value() as u64);
        // D s_12_5: write-var ga#315001 <= s_12_4
        fn_state.ga_315001 = s_12_4;
        // N s_12_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var VLshadow#5230:i64
        let s_13_0: i64 = fn_state.VLshadow_5230;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // D s_13_3: read-var t:i64
        let s_13_3: i64 = fn_state.t;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: cast zx s_13_2 -> i
        let s_13_5: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_6: read-var result:bv
        let s_13_6: Bits = fn_state.result;
        // D s_13_7: call Z_set(s_13_4, s_13_5, s_13_6)
        let s_13_7: () = Z_set(state, tracer, s_13_4, s_13_5, s_13_6);
        // N s_13_8: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var VLshadow#5230:i64
        let s_14_0: i64 = fn_state.VLshadow_5230;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: cast reint s_14_1 -> i64
        let s_14_2: i64 = (s_14_1 as i64);
        // D s_14_3: read-var n:i64
        let s_14_3: i64 = fn_state.n;
        // D s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_5: cast zx s_14_2 -> i
        let s_14_5: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_6: call Z_read(s_14_4, s_14_5)
        let s_14_6: Bits = Z_read(state, tracer, s_14_4, s_14_5);
        // D s_14_7: write-var base <= s_14_6
        fn_state.base = s_14_6;
        // C s_14_8: const #64s : i64
        let s_14_8: i64 = 64;
        // D s_14_9: read-var m:i64
        let s_14_9: i64 = fn_state.m;
        // D s_14_10: cast zx s_14_9 -> i
        let s_14_10: i128 = (i128::try_from(s_14_9).unwrap());
        // D s_14_11: call X_read(s_14_10, s_14_8)
        let s_14_11: Bits = X_read(state, tracer, s_14_10, s_14_8);
        // D s_14_12: cast reint s_14_11 -> u64
        let s_14_12: u64 = (s_14_11.value() as u64);
        // D s_14_13: write-var offset <= s_14_12
        fn_state.offset = s_14_12;
        // N s_14_14: jump b3
        return block_3(state, tracer, fn_state);
    }
}
