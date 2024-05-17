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
use CheckSVEEnabled::*;
use ConstrainUnpredictableBool::*;
use HaveSVE2p1::*;
use X_read::*;
use Zeros::*;
use Elem_set::*;
use SP_read::*;
use Mem_read::*;
use AnyActiveElement::*;
use P_read::*;
use ActivePredicateElement::*;
use CounterToPredicate::*;
use CheckSPAlignment::*;
use Z_set::*;
use common::*;
pub fn execute_LD1D_MZ_P_BR_4<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    m: i64,
    n: i64,
    nreg: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        gs_250141: i64,
        e: i64,
        values_name: [Bits; 4usize],
        VLshadow_5303: i64,
        base: u64,
        mbytes: i64,
        mask: Bits,
        u_6892: i64,
        ga_315737: Bits,
        gs_250122: i64,
        gs_250116: i64,
        VLshadow_5304: i64,
        accdesc: ProductType9878976b5bcce9c9,
        elements: i64,
        offset: u64,
        ga_315739: Bits,
        gs_250136: bool,
        ga_315738: i64,
        VL: i64,
        esize: i64,
        g: i64,
        m: i64,
        n: i64,
        nreg: i64,
        t: i64,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        g,
        m,
        n,
        nreg,
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
        // D s_0_3: write-var VLshadow#5303 <= s_0_2
        fn_state.VLshadow_5303 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call HaveSVE2p1(s_0_4)
        let s_0_5: bool = HaveSVE2p1(state, tracer, s_0_4);
        // N s_0_6: branch s_0_5 b31 b1
        if s_0_5 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call CheckStreamingSVEEnabled(s_1_0)
        let s_1_1: () = CheckStreamingSVEEnabled(state, tracer, s_1_0);
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#5303:i64
        let s_2_0: i64 = fn_state.VLshadow_5303;
        // D s_2_1: write-var VLshadow#5304 <= s_2_0
        fn_state.VLshadow_5304 = s_2_0;
        // C s_2_2: const #8s : i
        let s_2_2: i128 = 8;
        // D s_2_3: read-var VLshadow#5304:i64
        let s_2_3: i64 = fn_state.VLshadow_5304;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: div s_2_4 s_2_2
        let s_2_5: i128 = ((s_2_4) / (s_2_2));
        // D s_2_6: cast reint s_2_5 -> i64
        let s_2_6: i64 = (s_2_5 as i64);
        // D s_2_7: read-var VLshadow#5304:i64
        let s_2_7: i64 = fn_state.VLshadow_5304;
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: read-var esize:i64
        let s_2_9: i64 = fn_state.esize;
        // D s_2_10: cast zx s_2_9 -> i
        let s_2_10: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_11: div s_2_8 s_2_10
        let s_2_11: i128 = ((s_2_8) / (s_2_10));
        // D s_2_12: cast reint s_2_11 -> i64
        let s_2_12: i64 = (s_2_11 as i64);
        // D s_2_13: write-var elements <= s_2_12
        fn_state.elements = s_2_12;
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
        // D s_2_20: cast zx s_2_6 -> i
        let s_2_20: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_21: cast reint s_2_20 -> i64
        let s_2_21: i64 = (s_2_20 as i64);
        // D s_2_22: read-var g:i64
        let s_2_22: i64 = fn_state.g;
        // D s_2_23: cast zx s_2_22 -> i
        let s_2_23: i128 = (i128::try_from(s_2_22).unwrap());
        // D s_2_24: cast zx s_2_21 -> i
        let s_2_24: i128 = (i128::try_from(s_2_21).unwrap());
        // D s_2_25: call P_read(s_2_23, s_2_24)
        let s_2_25: Bits = P_read(state, tracer, s_2_23, s_2_24);
        // C s_2_26: const #0s : i
        let s_2_26: i128 = 0;
        // C s_2_27: const #1s : i64
        let s_2_27: i64 = 1;
        // C s_2_28: cast zx s_2_27 -> i
        let s_2_28: i128 = (i128::try_from(s_2_27).unwrap());
        // C s_2_29: const #15s : i
        let s_2_29: i128 = 15;
        // C s_2_30: add s_2_29 s_2_28
        let s_2_30: i128 = (s_2_29 + s_2_28);
        // D s_2_31: bit-extract s_2_25 s_2_26 s_2_30
        let s_2_31: Bits = (Bits::new(
            ((s_2_25) >> (s_2_26)).value(),
            u16::try_from(s_2_30).unwrap(),
        ));
        // D s_2_32: cast reint s_2_31 -> u16
        let s_2_32: u16 = (s_2_31.value() as u16);
        // D s_2_33: cast zx s_2_6 -> i
        let s_2_33: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_34: read-var nreg:i64
        let s_2_34: i64 = fn_state.nreg;
        // D s_2_35: cast zx s_2_34 -> i
        let s_2_35: i128 = (i128::try_from(s_2_34).unwrap());
        // D s_2_36: mul s_2_33 s_2_35
        let s_2_36: i128 = ((s_2_33) * (s_2_35));
        // D s_2_37: cast reint s_2_36 -> i64
        let s_2_37: i64 = (s_2_36 as i64);
        // D s_2_38: cast zx s_2_37 -> i
        let s_2_38: i128 = (i128::try_from(s_2_37).unwrap());
        // D s_2_39: cast reint s_2_38 -> i64
        let s_2_39: i64 = (s_2_38 as i64);
        // D s_2_40: cast zx s_2_39 -> i
        let s_2_40: i128 = (i128::try_from(s_2_39).unwrap());
        // D s_2_41: call CounterToPredicate(s_2_32, s_2_40)
        let s_2_41: Bits = CounterToPredicate(state, tracer, s_2_32, s_2_40);
        // D s_2_42: write-var mask <= s_2_41
        fn_state.mask = s_2_41;
        // C s_2_43: const #0u : u32
        let s_2_43: u32 = 0;
        // C s_2_44: const #0u : u8
        let s_2_44: bool = false;
        // C s_2_45: const #1u : u8
        let s_2_45: bool = true;
        // C s_2_46: const #1u : u8
        let s_2_46: bool = true;
        // S s_2_47: call CreateAccDescSVE(s_2_43, s_2_44, s_2_45, s_2_46)
        let s_2_47: ProductType9878976b5bcce9c9 = CreateAccDescSVE(
            state,
            tracer,
            s_2_43,
            s_2_44,
            s_2_45,
            s_2_46,
        );
        // D s_2_48: write-var accdesc <= s_2_47
        fn_state.accdesc = s_2_47;
        // D s_2_49: read-var esize:i64
        let s_2_49: i64 = fn_state.esize;
        // D s_2_50: cast zx s_2_49 -> i
        let s_2_50: i128 = (i128::try_from(s_2_49).unwrap());
        // D s_2_51: read-var mask:bv
        let s_2_51: Bits = fn_state.mask;
        // D s_2_52: call AnyActiveElement(s_2_51, s_2_50)
        let s_2_52: bool = AnyActiveElement(state, tracer, s_2_51, s_2_50);
        // D s_2_53: not s_2_52
        let s_2_53: bool = !s_2_52;
        // N s_2_54: branch s_2_53 b24 b3
        if s_2_53 {
            return block_24(state, tracer, fn_state);
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
        // N s_3_4: branch s_3_3 b23 b4
        if s_3_3 {
            return block_23(state, tracer, fn_state);
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
        // N s_5_4: branch s_5_3 b22 b6
        if s_5_3 {
            return block_22(state, tracer, fn_state);
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
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // D s_7_1: read-var m:i64
        let s_7_1: i64 = fn_state.m;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: call X_read(s_7_2, s_7_0)
        let s_7_3: Bits = X_read(state, tracer, s_7_2, s_7_0);
        // D s_7_4: cast reint s_7_3 -> u64
        let s_7_4: u64 = (s_7_3.value() as u64);
        // D s_7_5: write-var offset <= s_7_4
        fn_state.offset = s_7_4;
        // N s_7_6: jump b8
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
        // D s_8_2: read-var nreg:i64
        let s_8_2: i64 = fn_state.nreg;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: sub s_8_3 s_8_1
        let s_8_4: i128 = ((s_8_3) - (s_8_1));
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // D s_8_6: write-var gs#250116 <= s_8_5
        fn_state.gs_250116 = s_8_5;
        // D s_8_7: write-var r <= s_8_0
        fn_state.r = s_8_0;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var r:i64
        let s_9_0: i64 = fn_state.r;
        // D s_9_1: read-var gs#250116:i64
        let s_9_1: i64 = fn_state.gs_250116;
        // D s_9_2: cmp-gt s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) > (s_9_1));
        // N s_9_3: branch s_9_2 b18 b10
        if s_9_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i64
        let s_10_0: i64 = 0;
        // C s_10_1: const #1s : i
        let s_10_1: i128 = 1;
        // D s_10_2: read-var elements:i64
        let s_10_2: i64 = fn_state.elements;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: sub s_10_3 s_10_1
        let s_10_4: i128 = ((s_10_3) - (s_10_1));
        // D s_10_5: cast reint s_10_4 -> i64
        let s_10_5: i64 = (s_10_4 as i64);
        // D s_10_6: write-var gs#250122 <= s_10_5
        fn_state.gs_250122 = s_10_5;
        // D s_10_7: write-var e <= s_10_0
        fn_state.e = s_10_0;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var e:i64
        let s_11_0: i64 = fn_state.e;
        // D s_11_1: read-var gs#250122:i64
        let s_11_1: i64 = fn_state.gs_250122;
        // D s_11_2: cmp-gt s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) > (s_11_1));
        // N s_11_3: branch s_11_2 b17 b12
        if s_11_2 {
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
        // D s_12_0: read-var r:i64
        let s_12_0: i64 = fn_state.r;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var elements:i64
        let s_12_2: i64 = fn_state.elements;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: mul s_12_1 s_12_3
        let s_12_4: i128 = ((s_12_1) * (s_12_3));
        // D s_12_5: cast reint s_12_4 -> i64
        let s_12_5: i64 = (s_12_4 as i64);
        // D s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // D s_12_7: read-var e:i64
        let s_12_7: i64 = fn_state.e;
        // D s_12_8: cast zx s_12_7 -> i
        let s_12_8: i128 = (i128::try_from(s_12_7).unwrap());
        // D s_12_9: add s_12_6 s_12_8
        let s_12_9: i128 = (s_12_6 + s_12_8);
        // D s_12_10: cast reint s_12_9 -> i64
        let s_12_10: i64 = (s_12_9 as i64);
        // D s_12_11: cast zx s_12_10 -> i
        let s_12_11: i128 = (i128::try_from(s_12_10).unwrap());
        // D s_12_12: read-var esize:i64
        let s_12_12: i64 = fn_state.esize;
        // D s_12_13: cast zx s_12_12 -> i
        let s_12_13: i128 = (i128::try_from(s_12_12).unwrap());
        // D s_12_14: read-var mask:bv
        let s_12_14: Bits = fn_state.mask;
        // D s_12_15: call ActivePredicateElement(s_12_14, s_12_11, s_12_13)
        let s_12_15: bool = ActivePredicateElement(
            state,
            tracer,
            s_12_14,
            s_12_11,
            s_12_13,
        );
        // N s_12_16: branch s_12_15 b15 b13
        if s_12_15 {
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
        // D s_13_0: read-var values_name:[bv; 4]
        let s_13_0: [Bits; 4usize] = fn_state.values_name;
        // D s_13_1: cast cvt s_13_0 -> [bv; 0]
        let s_13_1: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_13_0);
        // D s_13_2: read-var r:i64
        let s_13_2: i64 = fn_state.r;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: read-element s_13_1[s_13_3]
        let s_13_4: Bits = s_13_1[(s_13_3) as usize];
        // D s_13_5: read-var esize:i64
        let s_13_5: i64 = fn_state.esize;
        // D s_13_6: cast zx s_13_5 -> i
        let s_13_6: i128 = (i128::try_from(s_13_5).unwrap());
        // D s_13_7: cast reint s_13_6 -> i64
        let s_13_7: i64 = (s_13_6 as i64);
        // D s_13_8: read-var esize:i64
        let s_13_8: i64 = fn_state.esize;
        // D s_13_9: cast zx s_13_8 -> i
        let s_13_9: i128 = (i128::try_from(s_13_8).unwrap());
        // D s_13_10: call Zeros(s_13_9)
        let s_13_10: Bits = Zeros(state, tracer, s_13_9);
        // D s_13_11: cast reint s_13_10 -> u64
        let s_13_11: u64 = (s_13_10.value() as u64);
        // D s_13_12: read-var e:i64
        let s_13_12: i64 = fn_state.e;
        // D s_13_13: cast zx s_13_12 -> i
        let s_13_13: i128 = (i128::try_from(s_13_12).unwrap());
        // D s_13_14: cast zx s_13_7 -> i
        let s_13_14: i128 = (i128::try_from(s_13_7).unwrap());
        // D s_13_15: cast zx s_13_11 -> bv
        let s_13_15: Bits = Bits::new(s_13_11 as u128, 64u16);
        // D s_13_16: call Elem_set(s_13_4, s_13_13, s_13_14, s_13_15)
        let s_13_16: Bits = Elem_set(state, tracer, s_13_4, s_13_13, s_13_14, s_13_15);
        // D s_13_17: read-var values_name:[bv; 4]
        let s_13_17: [Bits; 4usize] = fn_state.values_name;
        // D s_13_18: cast cvt s_13_17 -> [bv; 0]
        let s_13_18: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_13_17);
        // D s_13_19: read-var r:i64
        let s_13_19: i64 = fn_state.r;
        // D s_13_20: cast zx s_13_19 -> i
        let s_13_20: i128 = (i128::try_from(s_13_19).unwrap());
        // D s_13_21: mutate-element s_13_18[s_13_20] <= s_13_16
        let s_13_21: alloc::vec::Vec<Bits> = {
            let mut local = s_13_18.clone();
            local[(s_13_20) as usize] = s_13_16;
            local
        };
        // D s_13_22: cast cvt s_13_21 -> [bv; 4]
        let s_13_22: [Bits; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_13_21);
            buf
        };
        // D s_13_23: write-var values_name <= s_13_22
        fn_state.values_name = s_13_22;
        // N s_13_24: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var e:i64
        let s_14_0: i64 = fn_state.e;
        // C s_14_1: const #1s : i64
        let s_14_1: i64 = 1;
        // D s_14_2: add s_14_0 s_14_1
        let s_14_2: i64 = (s_14_0 + s_14_1);
        // D s_14_3: write-var e <= s_14_2
        fn_state.e = s_14_2;
        // N s_14_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var offset:u64
        let s_15_0: u64 = fn_state.offset;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 64u16);
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (s_15_1.value() as i128);
        // D s_15_3: read-var r:i64
        let s_15_3: i64 = fn_state.r;
        // D s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_5: read-var elements:i64
        let s_15_5: i64 = fn_state.elements;
        // D s_15_6: cast zx s_15_5 -> i
        let s_15_6: i128 = (i128::try_from(s_15_5).unwrap());
        // D s_15_7: mul s_15_4 s_15_6
        let s_15_7: i128 = ((s_15_4) * (s_15_6));
        // D s_15_8: cast reint s_15_7 -> i64
        let s_15_8: i64 = (s_15_7 as i64);
        // D s_15_9: cast zx s_15_8 -> i
        let s_15_9: i128 = (i128::try_from(s_15_8).unwrap());
        // D s_15_10: add s_15_2 s_15_9
        let s_15_10: i128 = (s_15_2 + s_15_9);
        // D s_15_11: read-var e:i64
        let s_15_11: i64 = fn_state.e;
        // D s_15_12: cast zx s_15_11 -> i
        let s_15_12: i128 = (i128::try_from(s_15_11).unwrap());
        // D s_15_13: add s_15_10 s_15_12
        let s_15_13: i128 = (s_15_10 + s_15_12);
        // D s_15_14: read-var mbytes:i64
        let s_15_14: i64 = fn_state.mbytes;
        // D s_15_15: cast zx s_15_14 -> i
        let s_15_15: i128 = (i128::try_from(s_15_14).unwrap());
        // D s_15_16: mul s_15_13 s_15_15
        let s_15_16: i128 = ((s_15_13) * (s_15_15));
        // D s_15_17: read-var base:u64
        let s_15_17: u64 = fn_state.base;
        // D s_15_18: cast zx s_15_17 -> bv
        let s_15_18: Bits = Bits::new(s_15_17 as u128, 64u16);
        // D s_15_19: cast cvt s_15_16 -> bv
        let s_15_19: Bits = Bits::new(s_15_16 as u128, 128);
        // D s_15_20: add s_15_18 s_15_19
        let s_15_20: Bits = (s_15_18 + s_15_19);
        // D s_15_21: cast reint s_15_20 -> u64
        let s_15_21: u64 = (s_15_20.value() as u64);
        // D s_15_22: read-var values_name:[bv; 4]
        let s_15_22: [Bits; 4usize] = fn_state.values_name;
        // D s_15_23: cast cvt s_15_22 -> [bv; 0]
        let s_15_23: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_15_22);
        // D s_15_24: read-var r:i64
        let s_15_24: i64 = fn_state.r;
        // D s_15_25: cast zx s_15_24 -> i
        let s_15_25: i128 = (i128::try_from(s_15_24).unwrap());
        // D s_15_26: read-element s_15_23[s_15_25]
        let s_15_26: Bits = s_15_23[(s_15_25) as usize];
        // D s_15_27: write-var ga#315737 <= s_15_26
        fn_state.ga_315737 = s_15_26;
        // D s_15_28: read-var esize:i64
        let s_15_28: i64 = fn_state.esize;
        // D s_15_29: cast zx s_15_28 -> i
        let s_15_29: i128 = (i128::try_from(s_15_28).unwrap());
        // D s_15_30: cast reint s_15_29 -> i64
        let s_15_30: i64 = (s_15_29 as i64);
        // D s_15_31: write-var ga#315738 <= s_15_30
        fn_state.ga_315738 = s_15_30;
        // D s_15_32: read-var mbytes:i64
        let s_15_32: i64 = fn_state.mbytes;
        // D s_15_33: cast zx s_15_32 -> i
        let s_15_33: i128 = (i128::try_from(s_15_32).unwrap());
        // D s_15_34: read-var accdesc:struct
        let s_15_34: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_15_35: call Mem_read(s_15_21, s_15_33, s_15_34)
        let s_15_35: Bits = Mem_read(state, tracer, s_15_21, s_15_33, s_15_34);
        // D s_15_36: write-var ga#315739 <= s_15_35
        fn_state.ga_315739 = s_15_35;
        // N s_15_37: jump b16
        return block_16(state, tracer, fn_state);
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
        // D s_16_2: read-var ga#315738:i64
        let s_16_2: i64 = fn_state.ga_315738;
        // D s_16_3: cast zx s_16_2 -> i
        let s_16_3: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_4: read-var ga#315737:bv
        let s_16_4: Bits = fn_state.ga_315737;
        // D s_16_5: read-var ga#315739:bv
        let s_16_5: Bits = fn_state.ga_315739;
        // D s_16_6: call Elem_set(s_16_4, s_16_1, s_16_3, s_16_5)
        let s_16_6: Bits = Elem_set(state, tracer, s_16_4, s_16_1, s_16_3, s_16_5);
        // D s_16_7: read-var values_name:[bv; 4]
        let s_16_7: [Bits; 4usize] = fn_state.values_name;
        // D s_16_8: cast cvt s_16_7 -> [bv; 0]
        let s_16_8: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_16_7);
        // D s_16_9: read-var r:i64
        let s_16_9: i64 = fn_state.r;
        // D s_16_10: cast zx s_16_9 -> i
        let s_16_10: i128 = (i128::try_from(s_16_9).unwrap());
        // D s_16_11: mutate-element s_16_8[s_16_10] <= s_16_6
        let s_16_11: alloc::vec::Vec<Bits> = {
            let mut local = s_16_8.clone();
            local[(s_16_10) as usize] = s_16_6;
            local
        };
        // D s_16_12: cast cvt s_16_11 -> [bv; 4]
        let s_16_12: [Bits; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_16_11);
            buf
        };
        // D s_16_13: write-var values_name <= s_16_12
        fn_state.values_name = s_16_12;
        // N s_16_14: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var r:i64
        let s_17_0: i64 = fn_state.r;
        // C s_17_1: const #1s : i64
        let s_17_1: i64 = 1;
        // D s_17_2: add s_17_0 s_17_1
        let s_17_2: i64 = (s_17_0 + s_17_1);
        // D s_17_3: write-var r <= s_17_2
        fn_state.r = s_17_2;
        // N s_17_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0s : i64
        let s_18_0: i64 = 0;
        // C s_18_1: const #1s : i
        let s_18_1: i128 = 1;
        // D s_18_2: read-var nreg:i64
        let s_18_2: i64 = fn_state.nreg;
        // D s_18_3: cast zx s_18_2 -> i
        let s_18_3: i128 = (i128::try_from(s_18_2).unwrap());
        // D s_18_4: sub s_18_3 s_18_1
        let s_18_4: i128 = ((s_18_3) - (s_18_1));
        // D s_18_5: cast reint s_18_4 -> i64
        let s_18_5: i64 = (s_18_4 as i64);
        // D s_18_6: write-var gs#250141 <= s_18_5
        fn_state.gs_250141 = s_18_5;
        // D s_18_7: write-var u#6892 <= s_18_0
        fn_state.u_6892 = s_18_0;
        // N s_18_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var u#6892:i64
        let s_19_0: i64 = fn_state.u_6892;
        // D s_19_1: read-var gs#250141:i64
        let s_19_1: i64 = fn_state.gs_250141;
        // D s_19_2: cmp-gt s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) > (s_19_1));
        // N s_19_3: branch s_19_2 b21 b20
        if s_19_2 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var t:i64
        let s_20_0: i64 = fn_state.t;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: read-var u#6892:i64
        let s_20_2: i64 = fn_state.u_6892;
        // D s_20_3: cast zx s_20_2 -> i
        let s_20_3: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_4: add s_20_1 s_20_3
        let s_20_4: i128 = (s_20_1 + s_20_3);
        // D s_20_5: cast reint s_20_4 -> i64
        let s_20_5: i64 = (s_20_4 as i64);
        // D s_20_6: read-var VLshadow#5304:i64
        let s_20_6: i64 = fn_state.VLshadow_5304;
        // D s_20_7: cast zx s_20_6 -> i
        let s_20_7: i128 = (i128::try_from(s_20_6).unwrap());
        // D s_20_8: cast reint s_20_7 -> i64
        let s_20_8: i64 = (s_20_7 as i64);
        // D s_20_9: read-var values_name:[bv; 4]
        let s_20_9: [Bits; 4usize] = fn_state.values_name;
        // D s_20_10: cast cvt s_20_9 -> [bv; 0]
        let s_20_10: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_20_9);
        // D s_20_11: read-var u#6892:i64
        let s_20_11: i64 = fn_state.u_6892;
        // D s_20_12: cast zx s_20_11 -> i
        let s_20_12: i128 = (i128::try_from(s_20_11).unwrap());
        // D s_20_13: read-element s_20_10[s_20_12]
        let s_20_13: Bits = s_20_10[(s_20_12) as usize];
        // D s_20_14: cast zx s_20_5 -> i
        let s_20_14: i128 = (i128::try_from(s_20_5).unwrap());
        // D s_20_15: cast zx s_20_8 -> i
        let s_20_15: i128 = (i128::try_from(s_20_8).unwrap());
        // D s_20_16: call Z_set(s_20_14, s_20_15, s_20_13)
        let s_20_16: () = Z_set(state, tracer, s_20_14, s_20_15, s_20_13);
        // D s_20_17: read-var u#6892:i64
        let s_20_17: i64 = fn_state.u_6892;
        // C s_20_18: const #1s : i64
        let s_20_18: i64 = 1;
        // D s_20_19: add s_20_17 s_20_18
        let s_20_19: i64 = (s_20_17 + s_20_18);
        // D s_20_20: write-var u#6892 <= s_20_19
        fn_state.u_6892 = s_20_19;
        // N s_20_21: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call SP_read(s_22_0)
        let s_22_1: u64 = SP_read(state, tracer, s_22_0);
        // D s_22_2: write-var base <= s_22_1
        fn_state.base = s_22_1;
        // N s_22_3: jump b7
        return block_7(state, tracer, fn_state);
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
        // N s_23_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #31s : i
        let s_24_0: i128 = 31;
        // D s_24_1: read-var n:i64
        let s_24_1: i64 = fn_state.n;
        // D s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (i128::try_from(s_24_1).unwrap());
        // D s_24_3: cmp-eq s_24_2 s_24_0
        let s_24_3: bool = ((s_24_2) == (s_24_0));
        // N s_24_4: branch s_24_3 b30 b25
        if s_24_3 {
            return block_30(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#250136 <= s_25_0
        fn_state.gs_250136 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#250136:u8
        let s_26_0: bool = fn_state.gs_250136;
        // N s_26_1: branch s_26_0 b29 b27
        if s_26_0 {
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
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call CheckSPAlignment(s_29_0)
        let s_29_1: () = CheckSPAlignment(state, tracer, s_29_0);
        // N s_29_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #53u : u32
        let s_30_0: u32 = 53;
        // S s_30_1: call ConstrainUnpredictableBool(s_30_0)
        let s_30_1: bool = ConstrainUnpredictableBool(state, tracer, s_30_0);
        // D s_30_2: write-var gs#250136 <= s_30_1
        fn_state.gs_250136 = s_30_1;
        // N s_30_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call CheckSVEEnabled(s_31_0)
        let s_31_1: () = CheckSVEEnabled(state, tracer, s_31_0);
        // N s_31_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
