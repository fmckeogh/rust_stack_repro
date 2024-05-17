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
use CreateAccDescSVENF::*;
use ConstrainUnpredictableBool::*;
use X_read::*;
use Zeros::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use neq_int::*;
use Elem_set::*;
use ElemFFR_set::*;
use ElemFFR_read::*;
use AnyActiveElement::*;
use P_read::*;
use ActivePredicateElement::*;
use MemNF_read::*;
use SP_read::*;
use CheckSPAlignment::*;
use Z_set::*;
use common::*;
pub fn execute_LDNF1B_Z_P_BI_U8<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    msize: i64,
    n: i64,
    offset: i128,
    t: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_233752: bool,
        e: i64,
        VLshadow_4749: i64,
        fault: bool,
        base: u64,
        gs_233725: i64,
        ga_306990: i64,
        ga_306983: ProductTypef506aa96a892fe52,
        unknown: bool,
        mbytes: i64,
        gs_233728: bool,
        mask: Bits,
        faulted: bool,
        orig: Bits,
        gs_233729: bool,
        data: Bits,
        accdesc: ProductType9878976b5bcce9c9,
        ga_306998: i64,
        elements: i64,
        gs_233744: bool,
        ga_306986: ProductTypef506aa96a892fe52,
        result: Bits,
        VLshadow_4750: i64,
        ga_306999: u8,
        ga_306991: u8,
        VL: i64,
        esize: i64,
        g: i64,
        msize: i64,
        n: i64,
        offset: i128,
        t: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        g,
        msize,
        n,
        offset,
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
        // D s_0_3: write-var VLshadow#4749 <= s_0_2
        fn_state.VLshadow_4749 = s_0_2;
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
        // D s_1_0: read-var VLshadow#4749:i64
        let s_1_0: i64 = fn_state.VLshadow_4749;
        // D s_1_1: write-var VLshadow#4750 <= s_1_0
        fn_state.VLshadow_4750 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#4750:i64
        let s_1_3: i64 = fn_state.VLshadow_4750;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#4750:i64
        let s_1_7: i64 = fn_state.VLshadow_4750;
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
        // D s_1_21: read-var VLshadow#4750:i64
        let s_1_21: i64 = fn_state.VLshadow_4750;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: read-var t:i64
        let s_1_24: i64 = fn_state.t;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast zx s_1_23 -> i
        let s_1_26: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_27: call Z_read(s_1_25, s_1_26)
        let s_1_27: Bits = Z_read(state, tracer, s_1_25, s_1_26);
        // D s_1_28: write-var orig <= s_1_27
        fn_state.orig = s_1_27;
        // C s_1_29: const #8s : i
        let s_1_29: i128 = 8;
        // D s_1_30: read-var msize:i64
        let s_1_30: i64 = fn_state.msize;
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_32: div s_1_31 s_1_29
        let s_1_32: i128 = ((s_1_31) / (s_1_29));
        // D s_1_33: cast reint s_1_32 -> i64
        let s_1_33: i64 = (s_1_32 as i64);
        // D s_1_34: write-var mbytes <= s_1_33
        fn_state.mbytes = s_1_33;
        // C s_1_35: const #0u : u8
        let s_1_35: bool = false;
        // D s_1_36: write-var fault <= s_1_35
        fn_state.fault = s_1_35;
        // C s_1_37: const #0u : u8
        let s_1_37: bool = false;
        // D s_1_38: write-var faulted <= s_1_37
        fn_state.faulted = s_1_37;
        // C s_1_39: const #0u : u8
        let s_1_39: bool = false;
        // D s_1_40: write-var unknown <= s_1_39
        fn_state.unknown = s_1_39;
        // C s_1_41: const #31s : i
        let s_1_41: i128 = 31;
        // D s_1_42: read-var n:i64
        let s_1_42: i64 = fn_state.n;
        // D s_1_43: cast zx s_1_42 -> i
        let s_1_43: i128 = (i128::try_from(s_1_42).unwrap());
        // D s_1_44: call neq_int(s_1_43, s_1_41)
        let s_1_44: bool = neq_int(state, tracer, s_1_43, s_1_41);
        // C s_1_45: const #1u : u8
        let s_1_45: bool = true;
        // D s_1_46: call CreateAccDescSVENF(s_1_45, s_1_44)
        let s_1_46: ProductType9878976b5bcce9c9 = CreateAccDescSVENF(
            state,
            tracer,
            s_1_45,
            s_1_44,
        );
        // D s_1_47: write-var accdesc <= s_1_46
        fn_state.accdesc = s_1_46;
        // D s_1_48: read-var esize:i64
        let s_1_48: i64 = fn_state.esize;
        // D s_1_49: cast zx s_1_48 -> i
        let s_1_49: i128 = (i128::try_from(s_1_48).unwrap());
        // D s_1_50: read-var mask:bv
        let s_1_50: Bits = fn_state.mask;
        // D s_1_51: call AnyActiveElement(s_1_50, s_1_49)
        let s_1_51: bool = AnyActiveElement(state, tracer, s_1_50, s_1_49);
        // D s_1_52: not s_1_51
        let s_1_52: bool = !s_1_51;
        // N s_1_53: branch s_1_52 b42 b2
        if s_1_52 {
            return block_42(state, tracer, fn_state);
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
        // N s_2_4: branch s_2_3 b41 b3
        if s_2_3 {
            return block_41(state, tracer, fn_state);
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
        // N s_4_4: branch s_4_3 b40 b5
        if s_4_3 {
            return block_40(state, tracer, fn_state);
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
        // N s_6_0: jump b7
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
        // D s_7_6: write-var gs#233725 <= s_7_5
        fn_state.gs_233725 = s_7_5;
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
        // D s_8_1: read-var gs#233725:i64
        let s_8_1: i64 = fn_state.gs_233725;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b39 b9
        if s_8_2 {
            return block_39(state, tracer, fn_state);
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
        // N s_9_6: branch s_9_5 b37 b10
        if s_9_5 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var msize:i64
        let s_10_0: i64 = fn_state.msize;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: call Zeros(s_10_1)
        let s_10_2: Bits = Zeros(state, tracer, s_10_1);
        // C s_10_3: const #0u : u8
        let s_10_3: bool = false;
        // D s_10_4: create-product struct = ["s_10_2", "s_10_3"]
        let s_10_4: ProductTypef506aa96a892fe52 = ProductTypef506aa96a892fe52 {
            _0: s_10_2,
            _1: s_10_3,
        };
        // D s_10_5: write-var ga#306986 <= s_10_4
        fn_state.ga_306986 = s_10_4;
        // D s_10_6: read-var ga#306986.0:struct
        let s_10_6: Bits = fn_state.ga_306986._0;
        // D s_10_7: read-var ga#306986.1:struct
        let s_10_7: bool = fn_state.ga_306986._1;
        // D s_10_8: write-var data <= s_10_6
        fn_state.data = s_10_6;
        // D s_10_9: write-var fault <= s_10_7
        fn_state.fault = s_10_7;
        // N s_10_10: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var faulted:u8
        let s_11_0: bool = fn_state.faulted;
        // N s_11_1: branch s_11_0 b36 b12
        if s_11_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var fault:u8
        let s_12_0: bool = fn_state.fault;
        // D s_12_1: write-var gs#233728 <= s_12_0
        fn_state.gs_233728 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#233728:u8
        let s_13_0: bool = fn_state.gs_233728;
        // D s_13_1: write-var faulted <= s_13_0
        fn_state.faulted = s_13_0;
        // D s_13_2: read-var faulted:u8
        let s_13_2: bool = fn_state.faulted;
        // N s_13_3: branch s_13_2 b35 b14
        if s_13_2 {
            return block_35(state, tracer, fn_state);
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
        // D s_15_0: read-var unknown:u8
        let s_15_0: bool = fn_state.unknown;
        // N s_15_1: branch s_15_0 b34 b16
        if s_15_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var e:i64
        let s_16_0: i64 = fn_state.e;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: read-var esize:i64
        let s_16_2: i64 = fn_state.esize;
        // D s_16_3: cast zx s_16_2 -> i
        let s_16_3: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_4: call ElemFFR_read(s_16_1, s_16_3)
        let s_16_4: bool = ElemFFR_read(state, tracer, s_16_1, s_16_3);
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // C s_16_6: const #0u : u8
        let s_16_6: bool = false;
        // C s_16_7: cast zx s_16_6 -> bv
        let s_16_7: Bits = Bits::new(s_16_6 as u128, 1u16);
        // D s_16_8: cmp-eq s_16_5 s_16_7
        let s_16_8: bool = ((s_16_5) == (s_16_7));
        // D s_16_9: write-var gs#233729 <= s_16_8
        fn_state.gs_233729 = s_16_8;
        // N s_16_10: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#233729:u8
        let s_17_0: bool = fn_state.gs_233729;
        // D s_17_1: write-var unknown <= s_17_0
        fn_state.unknown = s_17_0;
        // D s_17_2: read-var unknown:u8
        let s_17_2: bool = fn_state.unknown;
        // N s_17_3: branch s_17_2 b23 b18
        if s_17_2 {
            return block_23(state, tracer, fn_state);
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
        // D s_18_2: cast reint s_18_1 -> i64
        let s_18_2: i64 = (s_18_1 as i64);
        // D s_18_3: write-var ga#306998 <= s_18_2
        fn_state.ga_306998 = s_18_2;
        // D s_18_4: read-var is_unsigned:u8
        let s_18_4: bool = fn_state.is_unsigned;
        // N s_18_5: branch s_18_4 b22 b19
        if s_18_4 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var esize:i64
        let s_19_0: i64 = fn_state.esize;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: read-var data:bv
        let s_19_2: Bits = fn_state.data;
        // D s_19_3: bits-cast sx s_19_2 -> bv length s_19_1
        let s_19_3: Bits = s_19_2.sign_extend(s_19_1);
        // D s_19_4: cast reint s_19_3 -> u8
        let s_19_4: u8 = (s_19_3.value() as u8);
        // D s_19_5: write-var ga#306999 <= s_19_4
        fn_state.ga_306999 = s_19_4;
        // N s_19_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var e:i64
        let s_20_0: i64 = fn_state.e;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: read-var ga#306998:i64
        let s_20_2: i64 = fn_state.ga_306998;
        // D s_20_3: cast zx s_20_2 -> i
        let s_20_3: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_4: read-var ga#306999:u8
        let s_20_4: u8 = fn_state.ga_306999;
        // D s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 8u16);
        // D s_20_6: read-var result:bv
        let s_20_6: Bits = fn_state.result;
        // D s_20_7: call Elem_set(s_20_6, s_20_1, s_20_3, s_20_5)
        let s_20_7: Bits = Elem_set(state, tracer, s_20_6, s_20_1, s_20_3, s_20_5);
        // D s_20_8: write-var result <= s_20_7
        fn_state.result = s_20_7;
        // N s_20_9: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var e:i64
        let s_21_0: i64 = fn_state.e;
        // C s_21_1: const #1s : i64
        let s_21_1: i64 = 1;
        // D s_21_2: add s_21_0 s_21_1
        let s_21_2: i64 = (s_21_0 + s_21_1);
        // D s_21_3: write-var e <= s_21_2
        fn_state.e = s_21_2;
        // N s_21_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var esize:i64
        let s_22_0: i64 = fn_state.esize;
        // D s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_2: read-var data:bv
        let s_22_2: Bits = fn_state.data;
        // D s_22_3: bits-cast zx s_22_2 -> bv length s_22_1
        let s_22_3: Bits = s_22_2.zero_extend(s_22_1);
        // D s_22_4: cast reint s_22_3 -> u8
        let s_22_4: u8 = (s_22_3.value() as u8);
        // D s_22_5: write-var ga#306999 <= s_22_4
        fn_state.ga_306999 = s_22_4;
        // N s_22_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var fault:u8
        let s_23_0: bool = fn_state.fault;
        // D s_23_1: not s_23_0
        let s_23_1: bool = !s_23_0;
        // N s_23_2: branch s_23_1 b33 b24
        if s_23_1 {
            return block_33(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#233744 <= s_24_0
        fn_state.gs_233744 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#233744:u8
        let s_25_0: bool = fn_state.gs_233744;
        // N s_25_1: branch s_25_0 b29 b26
        if s_25_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #52u : u32
        let s_26_0: u32 = 52;
        // S s_26_1: call ConstrainUnpredictableBool(s_26_0)
        let s_26_1: bool = ConstrainUnpredictableBool(state, tracer, s_26_0);
        // N s_26_2: branch s_26_1 b28 b27
        if s_26_1 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var esize:i64
        let s_27_0: i64 = fn_state.esize;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: cast reint s_27_1 -> i64
        let s_27_2: i64 = (s_27_1 as i64);
        // D s_27_3: read-var esize:i64
        let s_27_3: i64 = fn_state.esize;
        // D s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_5: cast reint s_27_4 -> i64
        let s_27_5: i64 = (s_27_4 as i64);
        // D s_27_6: read-var e:i64
        let s_27_6: i64 = fn_state.e;
        // D s_27_7: cast zx s_27_6 -> i
        let s_27_7: i128 = (i128::try_from(s_27_6).unwrap());
        // D s_27_8: cast zx s_27_5 -> i
        let s_27_8: i128 = (i128::try_from(s_27_5).unwrap());
        // D s_27_9: read-var orig:bv
        let s_27_9: Bits = fn_state.orig;
        // D s_27_10: call Elem_read(s_27_9, s_27_7, s_27_8)
        let s_27_10: Bits = Elem_read(state, tracer, s_27_9, s_27_7, s_27_8);
        // D s_27_11: cast reint s_27_10 -> u8
        let s_27_11: u8 = (s_27_10.value() as u8);
        // D s_27_12: read-var e:i64
        let s_27_12: i64 = fn_state.e;
        // D s_27_13: cast zx s_27_12 -> i
        let s_27_13: i128 = (i128::try_from(s_27_12).unwrap());
        // D s_27_14: cast zx s_27_2 -> i
        let s_27_14: i128 = (i128::try_from(s_27_2).unwrap());
        // D s_27_15: cast zx s_27_11 -> bv
        let s_27_15: Bits = Bits::new(s_27_11 as u128, 8u16);
        // D s_27_16: read-var result:bv
        let s_27_16: Bits = fn_state.result;
        // D s_27_17: call Elem_set(s_27_16, s_27_13, s_27_14, s_27_15)
        let s_27_17: Bits = Elem_set(state, tracer, s_27_16, s_27_13, s_27_14, s_27_15);
        // D s_27_18: write-var result <= s_27_17
        fn_state.result = s_27_17;
        // N s_27_19: jump b21
        return block_21(state, tracer, fn_state);
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
        // D s_28_2: cast reint s_28_1 -> i64
        let s_28_2: i64 = (s_28_1 as i64);
        // D s_28_3: read-var esize:i64
        let s_28_3: i64 = fn_state.esize;
        // D s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // D s_28_5: call Zeros(s_28_4)
        let s_28_5: Bits = Zeros(state, tracer, s_28_4);
        // D s_28_6: cast reint s_28_5 -> u8
        let s_28_6: u8 = (s_28_5.value() as u8);
        // D s_28_7: read-var e:i64
        let s_28_7: i64 = fn_state.e;
        // D s_28_8: cast zx s_28_7 -> i
        let s_28_8: i128 = (i128::try_from(s_28_7).unwrap());
        // D s_28_9: cast zx s_28_2 -> i
        let s_28_9: i128 = (i128::try_from(s_28_2).unwrap());
        // D s_28_10: cast zx s_28_6 -> bv
        let s_28_10: Bits = Bits::new(s_28_6 as u128, 8u16);
        // D s_28_11: read-var result:bv
        let s_28_11: Bits = fn_state.result;
        // D s_28_12: call Elem_set(s_28_11, s_28_8, s_28_9, s_28_10)
        let s_28_12: Bits = Elem_set(state, tracer, s_28_11, s_28_8, s_28_9, s_28_10);
        // D s_28_13: write-var result <= s_28_12
        fn_state.result = s_28_12;
        // N s_28_14: jump b21
        return block_21(state, tracer, fn_state);
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
        // D s_29_2: cast reint s_29_1 -> i64
        let s_29_2: i64 = (s_29_1 as i64);
        // D s_29_3: write-var ga#306990 <= s_29_2
        fn_state.ga_306990 = s_29_2;
        // D s_29_4: read-var is_unsigned:u8
        let s_29_4: bool = fn_state.is_unsigned;
        // N s_29_5: branch s_29_4 b32 b30
        if s_29_4 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var esize:i64
        let s_30_0: i64 = fn_state.esize;
        // D s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // D s_30_2: read-var data:bv
        let s_30_2: Bits = fn_state.data;
        // D s_30_3: bits-cast sx s_30_2 -> bv length s_30_1
        let s_30_3: Bits = s_30_2.sign_extend(s_30_1);
        // D s_30_4: cast reint s_30_3 -> u8
        let s_30_4: u8 = (s_30_3.value() as u8);
        // D s_30_5: write-var ga#306991 <= s_30_4
        fn_state.ga_306991 = s_30_4;
        // N s_30_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var e:i64
        let s_31_0: i64 = fn_state.e;
        // D s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: read-var ga#306990:i64
        let s_31_2: i64 = fn_state.ga_306990;
        // D s_31_3: cast zx s_31_2 -> i
        let s_31_3: i128 = (i128::try_from(s_31_2).unwrap());
        // D s_31_4: read-var ga#306991:u8
        let s_31_4: u8 = fn_state.ga_306991;
        // D s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 8u16);
        // D s_31_6: read-var result:bv
        let s_31_6: Bits = fn_state.result;
        // D s_31_7: call Elem_set(s_31_6, s_31_1, s_31_3, s_31_5)
        let s_31_7: Bits = Elem_set(state, tracer, s_31_6, s_31_1, s_31_3, s_31_5);
        // D s_31_8: write-var result <= s_31_7
        fn_state.result = s_31_7;
        // N s_31_9: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var esize:i64
        let s_32_0: i64 = fn_state.esize;
        // D s_32_1: cast zx s_32_0 -> i
        let s_32_1: i128 = (i128::try_from(s_32_0).unwrap());
        // D s_32_2: read-var data:bv
        let s_32_2: Bits = fn_state.data;
        // D s_32_3: bits-cast zx s_32_2 -> bv length s_32_1
        let s_32_3: Bits = s_32_2.zero_extend(s_32_1);
        // D s_32_4: cast reint s_32_3 -> u8
        let s_32_4: u8 = (s_32_3.value() as u8);
        // D s_32_5: write-var ga#306991 <= s_32_4
        fn_state.ga_306991 = s_32_4;
        // N s_32_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #51u : u32
        let s_33_0: u32 = 51;
        // S s_33_1: call ConstrainUnpredictableBool(s_33_0)
        let s_33_1: bool = ConstrainUnpredictableBool(state, tracer, s_33_0);
        // D s_33_2: write-var gs#233744 <= s_33_1
        fn_state.gs_233744 = s_33_1;
        // N s_33_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#233729 <= s_34_0
        fn_state.gs_233729 = s_34_0;
        // N s_34_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var e:i64
        let s_35_0: i64 = fn_state.e;
        // D s_35_1: cast zx s_35_0 -> i
        let s_35_1: i128 = (i128::try_from(s_35_0).unwrap());
        // D s_35_2: read-var esize:i64
        let s_35_2: i64 = fn_state.esize;
        // D s_35_3: cast zx s_35_2 -> i
        let s_35_3: i128 = (i128::try_from(s_35_2).unwrap());
        // C s_35_4: const #0u : u8
        let s_35_4: bool = false;
        // D s_35_5: call ElemFFR_set(s_35_1, s_35_3, s_35_4)
        let s_35_5: () = ElemFFR_set(state, tracer, s_35_1, s_35_3, s_35_4);
        // N s_35_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#233728 <= s_36_0
        fn_state.gs_233728 = s_36_0;
        // N s_36_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var elements:i64
        let s_37_0: i64 = fn_state.elements;
        // D s_37_1: cast zx s_37_0 -> i
        let s_37_1: i128 = (i128::try_from(s_37_0).unwrap());
        // D s_37_2: read-var offset:i
        let s_37_2: i128 = fn_state.offset;
        // D s_37_3: mul s_37_2 s_37_1
        let s_37_3: i128 = ((s_37_2) * (s_37_1));
        // D s_37_4: read-var e:i64
        let s_37_4: i64 = fn_state.e;
        // D s_37_5: cast zx s_37_4 -> i
        let s_37_5: i128 = (i128::try_from(s_37_4).unwrap());
        // D s_37_6: add s_37_3 s_37_5
        let s_37_6: i128 = (s_37_3 + s_37_5);
        // D s_37_7: read-var mbytes:i64
        let s_37_7: i64 = fn_state.mbytes;
        // D s_37_8: cast zx s_37_7 -> i
        let s_37_8: i128 = (i128::try_from(s_37_7).unwrap());
        // D s_37_9: mul s_37_6 s_37_8
        let s_37_9: i128 = ((s_37_6) * (s_37_8));
        // D s_37_10: read-var base:u64
        let s_37_10: u64 = fn_state.base;
        // D s_37_11: cast zx s_37_10 -> bv
        let s_37_11: Bits = Bits::new(s_37_10 as u128, 64u16);
        // D s_37_12: cast cvt s_37_9 -> bv
        let s_37_12: Bits = Bits::new(s_37_9 as u128, 128);
        // D s_37_13: add s_37_11 s_37_12
        let s_37_13: Bits = (s_37_11 + s_37_12);
        // D s_37_14: cast reint s_37_13 -> u64
        let s_37_14: u64 = (s_37_13.value() as u64);
        // D s_37_15: read-var mbytes:i64
        let s_37_15: i64 = fn_state.mbytes;
        // D s_37_16: read-var accdesc:struct
        let s_37_16: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_37_17: call MemNF_read(s_37_14, s_37_15, s_37_16)
        let s_37_17: ProductTypef506aa96a892fe52 = MemNF_read(
            state,
            tracer,
            s_37_14,
            s_37_15,
            s_37_16,
        );
        // D s_37_18: write-var ga#306983 <= s_37_17
        fn_state.ga_306983 = s_37_17;
        // N s_37_19: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var ga#306983.0:struct
        let s_38_0: Bits = fn_state.ga_306983._0;
        // D s_38_1: read-var ga#306983.1:struct
        let s_38_1: bool = fn_state.ga_306983._1;
        // D s_38_2: write-var data <= s_38_0
        fn_state.data = s_38_0;
        // D s_38_3: write-var fault <= s_38_1
        fn_state.fault = s_38_1;
        // N s_38_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var VLshadow#4750:i64
        let s_39_0: i64 = fn_state.VLshadow_4750;
        // D s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_2: cast reint s_39_1 -> i64
        let s_39_2: i64 = (s_39_1 as i64);
        // D s_39_3: read-var t:i64
        let s_39_3: i64 = fn_state.t;
        // D s_39_4: cast zx s_39_3 -> i
        let s_39_4: i128 = (i128::try_from(s_39_3).unwrap());
        // D s_39_5: cast zx s_39_2 -> i
        let s_39_5: i128 = (i128::try_from(s_39_2).unwrap());
        // D s_39_6: read-var result:bv
        let s_39_6: Bits = fn_state.result;
        // D s_39_7: call Z_set(s_39_4, s_39_5, s_39_6)
        let s_39_7: () = Z_set(state, tracer, s_39_4, s_39_5, s_39_6);
        // N s_39_8: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call SP_read(s_40_0)
        let s_40_1: u64 = SP_read(state, tracer, s_40_0);
        // D s_40_2: write-var base <= s_40_1
        fn_state.base = s_40_1;
        // N s_40_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call CheckSPAlignment(s_41_0)
        let s_41_1: () = CheckSPAlignment(state, tracer, s_41_0);
        // N s_41_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #31s : i
        let s_42_0: i128 = 31;
        // D s_42_1: read-var n:i64
        let s_42_1: i64 = fn_state.n;
        // D s_42_2: cast zx s_42_1 -> i
        let s_42_2: i128 = (i128::try_from(s_42_1).unwrap());
        // D s_42_3: cmp-eq s_42_2 s_42_0
        let s_42_3: bool = ((s_42_2) == (s_42_0));
        // N s_42_4: branch s_42_3 b48 b43
        if s_42_3 {
            return block_48(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#233752 <= s_43_0
        fn_state.gs_233752 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#233752:u8
        let s_44_0: bool = fn_state.gs_233752;
        // N s_44_1: branch s_44_0 b47 b45
        if s_44_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call CheckSPAlignment(s_47_0)
        let s_47_1: () = CheckSPAlignment(state, tracer, s_47_0);
        // N s_47_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #53u : u32
        let s_48_0: u32 = 53;
        // S s_48_1: call ConstrainUnpredictableBool(s_48_0)
        let s_48_1: bool = ConstrainUnpredictableBool(state, tracer, s_48_0);
        // D s_48_2: write-var gs#233752 <= s_48_1
        fn_state.gs_233752 = s_48_1;
        // N s_48_3: jump b44
        return block_44(state, tracer, fn_state);
    }
}