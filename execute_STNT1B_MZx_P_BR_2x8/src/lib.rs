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
use CheckStreamingSVEEnabled::*;
use SP_read::*;
use AnyActiveElement::*;
use P_read::*;
use ActivePredicateElement::*;
use ConstrainUnpredictableBool::*;
use X_read::*;
use CounterToPredicate::*;
use Elem_read::*;
use Z_read::*;
use CheckSPAlignment::*;
use Mem_set::*;
use common::*;
pub fn execute_STNT1B_MZx_P_BR_2x8<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    m: i64,
    n: i64,
    nreg: i64,
    t__arg: i64,
    tstride: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        t: i128,
        e: i64,
        base: u64,
        gs_261164: i64,
        gs_261170: i64,
        mbytes: i64,
        VLshadow_5642: i64,
        VLshadow_5643: i64,
        mask: Bits,
        accdesc: ProductType9878976b5bcce9c9,
        gs_261159: bool,
        elements: i64,
        offset: u64,
        src: Bits,
        VL: i64,
        esize: i64,
        g: i64,
        m: i64,
        n: i64,
        nreg: i64,
        t__arg: i64,
        tstride: i64,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        g,
        m,
        n,
        nreg,
        t__arg,
        tstride,
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
        // D s_0_3: write-var VLshadow#5642 <= s_0_2
        fn_state.VLshadow_5642 = s_0_2;
        // D s_0_4: read-var t__arg:i64
        let s_0_4: i64 = fn_state.t__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var t <= s_0_5
        fn_state.t = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call CheckStreamingSVEEnabled(s_0_7)
        let s_0_8: () = CheckStreamingSVEEnabled(state, tracer, s_0_7);
        // N s_0_9: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#5642:i64
        let s_1_0: i64 = fn_state.VLshadow_5642;
        // D s_1_1: write-var VLshadow#5643 <= s_1_0
        fn_state.VLshadow_5643 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#5643:i64
        let s_1_3: i64 = fn_state.VLshadow_5643;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#5643:i64
        let s_1_7: i64 = fn_state.VLshadow_5643;
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
        // C s_1_14: const #8s : i
        let s_1_14: i128 = 8;
        // D s_1_15: read-var esize:i64
        let s_1_15: i64 = fn_state.esize;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: div s_1_16 s_1_14
        let s_1_17: i128 = ((s_1_16) / (s_1_14));
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: write-var mbytes <= s_1_18
        fn_state.mbytes = s_1_18;
        // D s_1_20: cast zx s_1_6 -> i
        let s_1_20: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_21: cast reint s_1_20 -> i64
        let s_1_21: i64 = (s_1_20 as i64);
        // D s_1_22: read-var g:i64
        let s_1_22: i64 = fn_state.g;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: cast zx s_1_21 -> i
        let s_1_24: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_25: call P_read(s_1_23, s_1_24)
        let s_1_25: Bits = P_read(state, tracer, s_1_23, s_1_24);
        // C s_1_26: const #0s : i
        let s_1_26: i128 = 0;
        // C s_1_27: const #1s : i64
        let s_1_27: i64 = 1;
        // C s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // C s_1_29: const #15s : i
        let s_1_29: i128 = 15;
        // C s_1_30: add s_1_29 s_1_28
        let s_1_30: i128 = (s_1_29 + s_1_28);
        // D s_1_31: bit-extract s_1_25 s_1_26 s_1_30
        let s_1_31: Bits = (Bits::new(
            ((s_1_25) >> (s_1_26)).value(),
            u16::try_from(s_1_30).unwrap(),
        ));
        // D s_1_32: cast reint s_1_31 -> u16
        let s_1_32: u16 = (s_1_31.value() as u16);
        // D s_1_33: cast zx s_1_6 -> i
        let s_1_33: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_34: read-var nreg:i64
        let s_1_34: i64 = fn_state.nreg;
        // D s_1_35: cast zx s_1_34 -> i
        let s_1_35: i128 = (i128::try_from(s_1_34).unwrap());
        // D s_1_36: mul s_1_33 s_1_35
        let s_1_36: i128 = ((s_1_33) * (s_1_35));
        // D s_1_37: cast reint s_1_36 -> i64
        let s_1_37: i64 = (s_1_36 as i64);
        // D s_1_38: cast zx s_1_37 -> i
        let s_1_38: i128 = (i128::try_from(s_1_37).unwrap());
        // D s_1_39: cast reint s_1_38 -> i64
        let s_1_39: i64 = (s_1_38 as i64);
        // D s_1_40: cast zx s_1_39 -> i
        let s_1_40: i128 = (i128::try_from(s_1_39).unwrap());
        // D s_1_41: call CounterToPredicate(s_1_32, s_1_40)
        let s_1_41: Bits = CounterToPredicate(state, tracer, s_1_32, s_1_40);
        // D s_1_42: write-var mask <= s_1_41
        fn_state.mask = s_1_41;
        // C s_1_43: const #1u : u32
        let s_1_43: u32 = 1;
        // C s_1_44: const #1u : u8
        let s_1_44: bool = true;
        // C s_1_45: const #1u : u8
        let s_1_45: bool = true;
        // C s_1_46: const #1u : u8
        let s_1_46: bool = true;
        // S s_1_47: call CreateAccDescSVE(s_1_43, s_1_44, s_1_45, s_1_46)
        let s_1_47: ProductType9878976b5bcce9c9 = CreateAccDescSVE(
            state,
            tracer,
            s_1_43,
            s_1_44,
            s_1_45,
            s_1_46,
        );
        // D s_1_48: write-var accdesc <= s_1_47
        fn_state.accdesc = s_1_47;
        // D s_1_49: read-var esize:i64
        let s_1_49: i64 = fn_state.esize;
        // D s_1_50: cast zx s_1_49 -> i
        let s_1_50: i128 = (i128::try_from(s_1_49).unwrap());
        // D s_1_51: read-var mask:bv
        let s_1_51: Bits = fn_state.mask;
        // D s_1_52: call AnyActiveElement(s_1_51, s_1_50)
        let s_1_52: bool = AnyActiveElement(state, tracer, s_1_51, s_1_50);
        // D s_1_53: not s_1_52
        let s_1_53: bool = !s_1_52;
        // N s_1_54: branch s_1_53 b19 b2
        if s_1_53 {
            return block_19(state, tracer, fn_state);
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
        // N s_2_4: branch s_2_3 b18 b3
        if s_2_3 {
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
        // N s_4_4: branch s_4_3 b17 b5
        if s_4_3 {
            return block_17(state, tracer, fn_state);
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
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: read-var m:i64
        let s_6_1: i64 = fn_state.m;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: call X_read(s_6_2, s_6_0)
        let s_6_3: Bits = X_read(state, tracer, s_6_2, s_6_0);
        // D s_6_4: cast reint s_6_3 -> u64
        let s_6_4: u64 = (s_6_3.value() as u64);
        // D s_6_5: write-var offset <= s_6_4
        fn_state.offset = s_6_4;
        // N s_6_6: jump b7
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
        // D s_7_2: read-var nreg:i64
        let s_7_2: i64 = fn_state.nreg;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: sub s_7_3 s_7_1
        let s_7_4: i128 = ((s_7_3) - (s_7_1));
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: write-var gs#261164 <= s_7_5
        fn_state.gs_261164 = s_7_5;
        // D s_7_7: write-var r <= s_7_0
        fn_state.r = s_7_0;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var r:i64
        let s_8_0: i64 = fn_state.r;
        // D s_8_1: read-var gs#261164:i64
        let s_8_1: i64 = fn_state.gs_261164;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b16 b9
        if s_8_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VLshadow#5643:i64
        let s_9_0: i64 = fn_state.VLshadow_5643;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var t:i
        let s_9_4: i128 = fn_state.t;
        // D s_9_5: call Z_read(s_9_4, s_9_3)
        let s_9_5: Bits = Z_read(state, tracer, s_9_4, s_9_3);
        // D s_9_6: write-var src <= s_9_5
        fn_state.src = s_9_5;
        // C s_9_7: const #0s : i64
        let s_9_7: i64 = 0;
        // C s_9_8: const #1s : i
        let s_9_8: i128 = 1;
        // D s_9_9: read-var elements:i64
        let s_9_9: i64 = fn_state.elements;
        // D s_9_10: cast zx s_9_9 -> i
        let s_9_10: i128 = (i128::try_from(s_9_9).unwrap());
        // D s_9_11: sub s_9_10 s_9_8
        let s_9_11: i128 = ((s_9_10) - (s_9_8));
        // D s_9_12: cast reint s_9_11 -> i64
        let s_9_12: i64 = (s_9_11 as i64);
        // D s_9_13: write-var gs#261170 <= s_9_12
        fn_state.gs_261170 = s_9_12;
        // D s_9_14: write-var e <= s_9_7
        fn_state.e = s_9_7;
        // N s_9_15: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var e:i64
        let s_10_0: i64 = fn_state.e;
        // D s_10_1: read-var gs#261170:i64
        let s_10_1: i64 = fn_state.gs_261170;
        // D s_10_2: cmp-gt s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) > (s_10_1));
        // N s_10_3: branch s_10_2 b15 b11
        if s_10_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var r:i64
        let s_11_0: i64 = fn_state.r;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: read-var elements:i64
        let s_11_2: i64 = fn_state.elements;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: mul s_11_1 s_11_3
        let s_11_4: i128 = ((s_11_1) * (s_11_3));
        // D s_11_5: cast reint s_11_4 -> i64
        let s_11_5: i64 = (s_11_4 as i64);
        // D s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // D s_11_7: read-var e:i64
        let s_11_7: i64 = fn_state.e;
        // D s_11_8: cast zx s_11_7 -> i
        let s_11_8: i128 = (i128::try_from(s_11_7).unwrap());
        // D s_11_9: add s_11_6 s_11_8
        let s_11_9: i128 = (s_11_6 + s_11_8);
        // D s_11_10: cast reint s_11_9 -> i64
        let s_11_10: i64 = (s_11_9 as i64);
        // D s_11_11: cast zx s_11_10 -> i
        let s_11_11: i128 = (i128::try_from(s_11_10).unwrap());
        // D s_11_12: read-var esize:i64
        let s_11_12: i64 = fn_state.esize;
        // D s_11_13: cast zx s_11_12 -> i
        let s_11_13: i128 = (i128::try_from(s_11_12).unwrap());
        // D s_11_14: read-var mask:bv
        let s_11_14: Bits = fn_state.mask;
        // D s_11_15: call ActivePredicateElement(s_11_14, s_11_11, s_11_13)
        let s_11_15: bool = ActivePredicateElement(
            state,
            tracer,
            s_11_14,
            s_11_11,
            s_11_13,
        );
        // N s_11_16: branch s_11_15 b14 b12
        if s_11_15 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var e:i64
        let s_13_0: i64 = fn_state.e;
        // C s_13_1: const #1s : i64
        let s_13_1: i64 = 1;
        // D s_13_2: add s_13_0 s_13_1
        let s_13_2: i64 = (s_13_0 + s_13_1);
        // D s_13_3: write-var e <= s_13_2
        fn_state.e = s_13_2;
        // N s_13_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var offset:u64
        let s_14_0: u64 = fn_state.offset;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 64u16);
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (s_14_1.value() as i128);
        // D s_14_3: read-var r:i64
        let s_14_3: i64 = fn_state.r;
        // D s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_5: read-var elements:i64
        let s_14_5: i64 = fn_state.elements;
        // D s_14_6: cast zx s_14_5 -> i
        let s_14_6: i128 = (i128::try_from(s_14_5).unwrap());
        // D s_14_7: mul s_14_4 s_14_6
        let s_14_7: i128 = ((s_14_4) * (s_14_6));
        // D s_14_8: cast reint s_14_7 -> i64
        let s_14_8: i64 = (s_14_7 as i64);
        // D s_14_9: cast zx s_14_8 -> i
        let s_14_9: i128 = (i128::try_from(s_14_8).unwrap());
        // D s_14_10: add s_14_2 s_14_9
        let s_14_10: i128 = (s_14_2 + s_14_9);
        // D s_14_11: read-var e:i64
        let s_14_11: i64 = fn_state.e;
        // D s_14_12: cast zx s_14_11 -> i
        let s_14_12: i128 = (i128::try_from(s_14_11).unwrap());
        // D s_14_13: add s_14_10 s_14_12
        let s_14_13: i128 = (s_14_10 + s_14_12);
        // D s_14_14: read-var mbytes:i64
        let s_14_14: i64 = fn_state.mbytes;
        // D s_14_15: cast zx s_14_14 -> i
        let s_14_15: i128 = (i128::try_from(s_14_14).unwrap());
        // D s_14_16: mul s_14_13 s_14_15
        let s_14_16: i128 = ((s_14_13) * (s_14_15));
        // D s_14_17: read-var base:u64
        let s_14_17: u64 = fn_state.base;
        // D s_14_18: cast zx s_14_17 -> bv
        let s_14_18: Bits = Bits::new(s_14_17 as u128, 64u16);
        // D s_14_19: cast cvt s_14_16 -> bv
        let s_14_19: Bits = Bits::new(s_14_16 as u128, 128);
        // D s_14_20: add s_14_18 s_14_19
        let s_14_20: Bits = (s_14_18 + s_14_19);
        // D s_14_21: cast reint s_14_20 -> u64
        let s_14_21: u64 = (s_14_20.value() as u64);
        // D s_14_22: read-var esize:i64
        let s_14_22: i64 = fn_state.esize;
        // D s_14_23: cast zx s_14_22 -> i
        let s_14_23: i128 = (i128::try_from(s_14_22).unwrap());
        // D s_14_24: cast reint s_14_23 -> i64
        let s_14_24: i64 = (s_14_23 as i64);
        // D s_14_25: read-var e:i64
        let s_14_25: i64 = fn_state.e;
        // D s_14_26: cast zx s_14_25 -> i
        let s_14_26: i128 = (i128::try_from(s_14_25).unwrap());
        // D s_14_27: cast zx s_14_24 -> i
        let s_14_27: i128 = (i128::try_from(s_14_24).unwrap());
        // D s_14_28: read-var src:bv
        let s_14_28: Bits = fn_state.src;
        // D s_14_29: call Elem_read(s_14_28, s_14_26, s_14_27)
        let s_14_29: Bits = Elem_read(state, tracer, s_14_28, s_14_26, s_14_27);
        // D s_14_30: read-var mbytes:i64
        let s_14_30: i64 = fn_state.mbytes;
        // D s_14_31: cast zx s_14_30 -> i
        let s_14_31: i128 = (i128::try_from(s_14_30).unwrap());
        // D s_14_32: read-var accdesc:struct
        let s_14_32: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_14_33: call Mem_set(s_14_21, s_14_31, s_14_32, s_14_29)
        let s_14_33: () = Mem_set(state, tracer, s_14_21, s_14_31, s_14_32, s_14_29);
        // N s_14_34: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var tstride:i64
        let s_15_0: i64 = fn_state.tstride;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: read-var t:i
        let s_15_2: i128 = fn_state.t;
        // D s_15_3: add s_15_2 s_15_1
        let s_15_3: i128 = (s_15_2 + s_15_1);
        // D s_15_4: write-var t <= s_15_3
        fn_state.t = s_15_3;
        // D s_15_5: read-var r:i64
        let s_15_5: i64 = fn_state.r;
        // C s_15_6: const #1s : i64
        let s_15_6: i64 = 1;
        // D s_15_7: add s_15_5 s_15_6
        let s_15_7: i64 = (s_15_5 + s_15_6);
        // D s_15_8: write-var r <= s_15_7
        fn_state.r = s_15_7;
        // N s_15_9: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call SP_read(s_17_0)
        let s_17_1: u64 = SP_read(state, tracer, s_17_0);
        // D s_17_2: write-var base <= s_17_1
        fn_state.base = s_17_1;
        // N s_17_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call CheckSPAlignment(s_18_0)
        let s_18_1: () = CheckSPAlignment(state, tracer, s_18_0);
        // N s_18_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #31s : i
        let s_19_0: i128 = 31;
        // D s_19_1: read-var n:i64
        let s_19_1: i64 = fn_state.n;
        // D s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (i128::try_from(s_19_1).unwrap());
        // D s_19_3: cmp-eq s_19_2 s_19_0
        let s_19_3: bool = ((s_19_2) == (s_19_0));
        // N s_19_4: branch s_19_3 b25 b20
        if s_19_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#261159 <= s_20_0
        fn_state.gs_261159 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#261159:u8
        let s_21_0: bool = fn_state.gs_261159;
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
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call CheckSPAlignment(s_24_0)
        let s_24_1: () = CheckSPAlignment(state, tracer, s_24_0);
        // N s_24_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #53u : u32
        let s_25_0: u32 = 53;
        // S s_25_1: call ConstrainUnpredictableBool(s_25_0)
        let s_25_1: bool = ConstrainUnpredictableBool(state, tracer, s_25_0);
        // D s_25_2: write-var gs#261159 <= s_25_1
        fn_state.gs_261159 = s_25_1;
        // N s_25_3: jump b21
        return block_21(state, tracer, fn_state);
    }
}
