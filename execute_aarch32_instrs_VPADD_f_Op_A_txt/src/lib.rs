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
use StandardFPSCRValue::*;
use D_set::*;
use Elem_set::*;
use Elem_read::*;
use CheckAdvSIMDEnabled::*;
use FPAdd::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VPADD_f_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    elements: i64,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_357838: Bits,
        ga_357837: i64,
        e: i64,
        ga_357824: i64,
        ga_357825: Bits,
        dest: u64,
        esizeshadow_7633: i64,
        gs_314895: i64,
        ga_357836: i64,
        h: i64,
        d: i64,
        elements: i64,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        elements,
        esize,
        m,
        n,
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
        // D s_0_3: write-var esizeshadow#7633 <= s_0_2
        fn_state.esizeshadow_7633 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckAdvSIMDEnabled(s_0_4)
        let s_0_5: () = CheckAdvSIMDEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #2s : i
        let s_1_0: i128 = 2;
        // D s_1_1: read-var elements:i64
        let s_1_1: i64 = fn_state.elements;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: div s_1_2 s_1_0
        let s_1_3: i128 = ((s_1_2) / (s_1_0));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: write-var h <= s_1_4
        fn_state.h = s_1_4;
        // C s_1_6: const #0s : i64
        let s_1_6: i64 = 0;
        // C s_1_7: const #1s : i
        let s_1_7: i128 = 1;
        // D s_1_8: read-var h:i64
        let s_1_8: i64 = fn_state.h;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: sub s_1_9 s_1_7
        let s_1_10: i128 = ((s_1_9) - (s_1_7));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var gs#314895 <= s_1_11
        fn_state.gs_314895 = s_1_11;
        // D s_1_13: write-var e <= s_1_6
        fn_state.e = s_1_6;
        // N s_1_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#314895:i64
        let s_2_1: i64 = fn_state.gs_314895;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b6 b3
        if s_2_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#7633:i64
        let s_3_0: i64 = fn_state.esizeshadow_7633;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: write-var ga#357824 <= s_3_2
        fn_state.ga_357824 = s_3_2;
        // D s_3_4: read-var n:i64
        let s_3_4: i64 = fn_state.n;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: call D_read(s_3_5)
        let s_3_6: u64 = D_read(state, tracer, s_3_5);
        // C s_3_7: const #2s : i
        let s_3_7: i128 = 2;
        // D s_3_8: read-var e:i64
        let s_3_8: i64 = fn_state.e;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: mul s_3_7 s_3_9
        let s_3_10: i128 = ((s_3_7) * (s_3_9));
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: read-var esizeshadow#7633:i64
        let s_3_12: i64 = fn_state.esizeshadow_7633;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: cast zx s_3_6 -> bv
        let s_3_15: Bits = Bits::new(s_3_6 as u128, 64u16);
        // D s_3_16: cast zx s_3_11 -> i
        let s_3_16: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_17: cast zx s_3_14 -> i
        let s_3_17: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_18: call Elem_read(s_3_15, s_3_16, s_3_17)
        let s_3_18: Bits = Elem_read(state, tracer, s_3_15, s_3_16, s_3_17);
        // D s_3_19: read-var n:i64
        let s_3_19: i64 = fn_state.n;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: call D_read(s_3_20)
        let s_3_21: u64 = D_read(state, tracer, s_3_20);
        // C s_3_22: const #2s : i
        let s_3_22: i128 = 2;
        // D s_3_23: read-var e:i64
        let s_3_23: i64 = fn_state.e;
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: mul s_3_22 s_3_24
        let s_3_25: i128 = ((s_3_22) * (s_3_24));
        // D s_3_26: cast reint s_3_25 -> i64
        let s_3_26: i64 = (s_3_25 as i64);
        // C s_3_27: const #1s : i
        let s_3_27: i128 = 1;
        // D s_3_28: cast zx s_3_26 -> i
        let s_3_28: i128 = (i128::try_from(s_3_26).unwrap());
        // D s_3_29: add s_3_28 s_3_27
        let s_3_29: i128 = (s_3_28 + s_3_27);
        // D s_3_30: cast reint s_3_29 -> i64
        let s_3_30: i64 = (s_3_29 as i64);
        // D s_3_31: read-var esizeshadow#7633:i64
        let s_3_31: i64 = fn_state.esizeshadow_7633;
        // D s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_33: cast reint s_3_32 -> i64
        let s_3_33: i64 = (s_3_32 as i64);
        // D s_3_34: cast zx s_3_21 -> bv
        let s_3_34: Bits = Bits::new(s_3_21 as u128, 64u16);
        // D s_3_35: cast zx s_3_30 -> i
        let s_3_35: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_36: cast zx s_3_33 -> i
        let s_3_36: i128 = (i128::try_from(s_3_33).unwrap());
        // D s_3_37: call Elem_read(s_3_34, s_3_35, s_3_36)
        let s_3_37: Bits = Elem_read(state, tracer, s_3_34, s_3_35, s_3_36);
        // C s_3_38: const #() : ()
        let s_3_38: () = ();
        // S s_3_39: call StandardFPSCRValue(s_3_38)
        let s_3_39: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_3_38,
        );
        // D s_3_40: call FPAdd(s_3_18, s_3_37, s_3_39)
        let s_3_40: Bits = FPAdd(state, tracer, s_3_18, s_3_37, s_3_39);
        // D s_3_41: write-var ga#357825 <= s_3_40
        fn_state.ga_357825 = s_3_40;
        // N s_3_42: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var dest:u64
        let s_4_0: u64 = fn_state.dest;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 64u16);
        // D s_4_2: read-var e:i64
        let s_4_2: i64 = fn_state.e;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var ga#357824:i64
        let s_4_4: i64 = fn_state.ga_357824;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: read-var ga#357825:bv
        let s_4_6: Bits = fn_state.ga_357825;
        // D s_4_7: call Elem_set(s_4_1, s_4_3, s_4_5, s_4_6)
        let s_4_7: Bits = Elem_set(state, tracer, s_4_1, s_4_3, s_4_5, s_4_6);
        // D s_4_8: cast reint s_4_7 -> u64
        let s_4_8: u64 = (s_4_7.value() as u64);
        // D s_4_9: write-var dest <= s_4_8
        fn_state.dest = s_4_8;
        // D s_4_10: read-var e:i64
        let s_4_10: i64 = fn_state.e;
        // D s_4_11: cast zx s_4_10 -> i
        let s_4_11: i128 = (i128::try_from(s_4_10).unwrap());
        // D s_4_12: read-var h:i64
        let s_4_12: i64 = fn_state.h;
        // D s_4_13: cast zx s_4_12 -> i
        let s_4_13: i128 = (i128::try_from(s_4_12).unwrap());
        // D s_4_14: add s_4_11 s_4_13
        let s_4_14: i128 = (s_4_11 + s_4_13);
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: write-var ga#357836 <= s_4_15
        fn_state.ga_357836 = s_4_15;
        // D s_4_17: read-var esizeshadow#7633:i64
        let s_4_17: i64 = fn_state.esizeshadow_7633;
        // D s_4_18: cast zx s_4_17 -> i
        let s_4_18: i128 = (i128::try_from(s_4_17).unwrap());
        // D s_4_19: cast reint s_4_18 -> i64
        let s_4_19: i64 = (s_4_18 as i64);
        // D s_4_20: write-var ga#357837 <= s_4_19
        fn_state.ga_357837 = s_4_19;
        // D s_4_21: read-var m:i64
        let s_4_21: i64 = fn_state.m;
        // D s_4_22: cast zx s_4_21 -> i
        let s_4_22: i128 = (i128::try_from(s_4_21).unwrap());
        // D s_4_23: call D_read(s_4_22)
        let s_4_23: u64 = D_read(state, tracer, s_4_22);
        // C s_4_24: const #2s : i
        let s_4_24: i128 = 2;
        // D s_4_25: read-var e:i64
        let s_4_25: i64 = fn_state.e;
        // D s_4_26: cast zx s_4_25 -> i
        let s_4_26: i128 = (i128::try_from(s_4_25).unwrap());
        // D s_4_27: mul s_4_24 s_4_26
        let s_4_27: i128 = ((s_4_24) * (s_4_26));
        // D s_4_28: cast reint s_4_27 -> i64
        let s_4_28: i64 = (s_4_27 as i64);
        // D s_4_29: read-var esizeshadow#7633:i64
        let s_4_29: i64 = fn_state.esizeshadow_7633;
        // D s_4_30: cast zx s_4_29 -> i
        let s_4_30: i128 = (i128::try_from(s_4_29).unwrap());
        // D s_4_31: cast reint s_4_30 -> i64
        let s_4_31: i64 = (s_4_30 as i64);
        // D s_4_32: cast zx s_4_23 -> bv
        let s_4_32: Bits = Bits::new(s_4_23 as u128, 64u16);
        // D s_4_33: cast zx s_4_28 -> i
        let s_4_33: i128 = (i128::try_from(s_4_28).unwrap());
        // D s_4_34: cast zx s_4_31 -> i
        let s_4_34: i128 = (i128::try_from(s_4_31).unwrap());
        // D s_4_35: call Elem_read(s_4_32, s_4_33, s_4_34)
        let s_4_35: Bits = Elem_read(state, tracer, s_4_32, s_4_33, s_4_34);
        // D s_4_36: read-var m:i64
        let s_4_36: i64 = fn_state.m;
        // D s_4_37: cast zx s_4_36 -> i
        let s_4_37: i128 = (i128::try_from(s_4_36).unwrap());
        // D s_4_38: call D_read(s_4_37)
        let s_4_38: u64 = D_read(state, tracer, s_4_37);
        // C s_4_39: const #2s : i
        let s_4_39: i128 = 2;
        // D s_4_40: read-var e:i64
        let s_4_40: i64 = fn_state.e;
        // D s_4_41: cast zx s_4_40 -> i
        let s_4_41: i128 = (i128::try_from(s_4_40).unwrap());
        // D s_4_42: mul s_4_39 s_4_41
        let s_4_42: i128 = ((s_4_39) * (s_4_41));
        // D s_4_43: cast reint s_4_42 -> i64
        let s_4_43: i64 = (s_4_42 as i64);
        // C s_4_44: const #1s : i
        let s_4_44: i128 = 1;
        // D s_4_45: cast zx s_4_43 -> i
        let s_4_45: i128 = (i128::try_from(s_4_43).unwrap());
        // D s_4_46: add s_4_45 s_4_44
        let s_4_46: i128 = (s_4_45 + s_4_44);
        // D s_4_47: cast reint s_4_46 -> i64
        let s_4_47: i64 = (s_4_46 as i64);
        // D s_4_48: read-var esizeshadow#7633:i64
        let s_4_48: i64 = fn_state.esizeshadow_7633;
        // D s_4_49: cast zx s_4_48 -> i
        let s_4_49: i128 = (i128::try_from(s_4_48).unwrap());
        // D s_4_50: cast reint s_4_49 -> i64
        let s_4_50: i64 = (s_4_49 as i64);
        // D s_4_51: cast zx s_4_38 -> bv
        let s_4_51: Bits = Bits::new(s_4_38 as u128, 64u16);
        // D s_4_52: cast zx s_4_47 -> i
        let s_4_52: i128 = (i128::try_from(s_4_47).unwrap());
        // D s_4_53: cast zx s_4_50 -> i
        let s_4_53: i128 = (i128::try_from(s_4_50).unwrap());
        // D s_4_54: call Elem_read(s_4_51, s_4_52, s_4_53)
        let s_4_54: Bits = Elem_read(state, tracer, s_4_51, s_4_52, s_4_53);
        // C s_4_55: const #() : ()
        let s_4_55: () = ();
        // S s_4_56: call StandardFPSCRValue(s_4_55)
        let s_4_56: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_4_55,
        );
        // D s_4_57: call FPAdd(s_4_35, s_4_54, s_4_56)
        let s_4_57: Bits = FPAdd(state, tracer, s_4_35, s_4_54, s_4_56);
        // D s_4_58: write-var ga#357838 <= s_4_57
        fn_state.ga_357838 = s_4_57;
        // N s_4_59: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var dest:u64
        let s_5_0: u64 = fn_state.dest;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 64u16);
        // D s_5_2: read-var ga#357836:i64
        let s_5_2: i64 = fn_state.ga_357836;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var ga#357837:i64
        let s_5_4: i64 = fn_state.ga_357837;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: read-var ga#357838:bv
        let s_5_6: Bits = fn_state.ga_357838;
        // D s_5_7: call Elem_set(s_5_1, s_5_3, s_5_5, s_5_6)
        let s_5_7: Bits = Elem_set(state, tracer, s_5_1, s_5_3, s_5_5, s_5_6);
        // D s_5_8: cast reint s_5_7 -> u64
        let s_5_8: u64 = (s_5_7.value() as u64);
        // D s_5_9: write-var dest <= s_5_8
        fn_state.dest = s_5_8;
        // D s_5_10: read-var e:i64
        let s_5_10: i64 = fn_state.e;
        // C s_5_11: const #1s : i64
        let s_5_11: i64 = 1;
        // D s_5_12: add s_5_10 s_5_11
        let s_5_12: i64 = (s_5_10 + s_5_11);
        // D s_5_13: write-var e <= s_5_12
        fn_state.e = s_5_12;
        // N s_5_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var d:i64
        let s_6_0: i64 = fn_state.d;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var dest:u64
        let s_6_2: u64 = fn_state.dest;
        // D s_6_3: call D_set(s_6_1, s_6_2)
        let s_6_3: () = D_set(state, tracer, s_6_1, s_6_2);
        // N s_6_4: return
        return;
    }
}
