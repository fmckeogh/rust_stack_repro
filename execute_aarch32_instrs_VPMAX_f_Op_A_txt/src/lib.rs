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
use Elem_set::*;
use CheckAdvSIMDEnabled::*;
use D_set::*;
use FPMin::*;
use Elem_read::*;
use D_read::*;
use FPMax::*;
use common::*;
pub fn execute_aarch32_instrs_VPMAX_f_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    elements: i64,
    esize: i64,
    m: i64,
    maximum: bool,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        ga_357988: Bits,
        gs_315089: i64,
        ga_357999: i64,
        esizeshadow_7640: i64,
        op2: Bits,
        ga_358000: Bits,
        dest: u64,
        op1: Bits,
        ga_357998: i64,
        ga_357987: i64,
        h: i64,
        d: i64,
        elements: i64,
        esize: i64,
        m: i64,
        maximum: bool,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        elements,
        esize,
        m,
        maximum,
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
        // D s_0_3: write-var esizeshadow#7640 <= s_0_2
        fn_state.esizeshadow_7640 = s_0_2;
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
        // D s_1_12: write-var gs#315089 <= s_1_11
        fn_state.gs_315089 = s_1_11;
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
        // D s_2_1: read-var gs#315089:i64
        let s_2_1: i64 = fn_state.gs_315089;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b10 b3
        if s_2_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call D_read(s_3_1)
        let s_3_2: u64 = D_read(state, tracer, s_3_1);
        // C s_3_3: const #2s : i
        let s_3_3: i128 = 2;
        // D s_3_4: read-var e:i64
        let s_3_4: i64 = fn_state.e;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: mul s_3_3 s_3_5
        let s_3_6: i128 = ((s_3_3) * (s_3_5));
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: read-var esizeshadow#7640:i64
        let s_3_8: i64 = fn_state.esizeshadow_7640;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: cast zx s_3_2 -> bv
        let s_3_11: Bits = Bits::new(s_3_2 as u128, 64u16);
        // D s_3_12: cast zx s_3_7 -> i
        let s_3_12: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_13: cast zx s_3_10 -> i
        let s_3_13: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_14: call Elem_read(s_3_11, s_3_12, s_3_13)
        let s_3_14: Bits = Elem_read(state, tracer, s_3_11, s_3_12, s_3_13);
        // D s_3_15: write-var op1 <= s_3_14
        fn_state.op1 = s_3_14;
        // D s_3_16: read-var n:i64
        let s_3_16: i64 = fn_state.n;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: call D_read(s_3_17)
        let s_3_18: u64 = D_read(state, tracer, s_3_17);
        // C s_3_19: const #2s : i
        let s_3_19: i128 = 2;
        // D s_3_20: read-var e:i64
        let s_3_20: i64 = fn_state.e;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: mul s_3_19 s_3_21
        let s_3_22: i128 = ((s_3_19) * (s_3_21));
        // D s_3_23: cast reint s_3_22 -> i64
        let s_3_23: i64 = (s_3_22 as i64);
        // C s_3_24: const #1s : i
        let s_3_24: i128 = 1;
        // D s_3_25: cast zx s_3_23 -> i
        let s_3_25: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_26: add s_3_25 s_3_24
        let s_3_26: i128 = (s_3_25 + s_3_24);
        // D s_3_27: cast reint s_3_26 -> i64
        let s_3_27: i64 = (s_3_26 as i64);
        // D s_3_28: read-var esizeshadow#7640:i64
        let s_3_28: i64 = fn_state.esizeshadow_7640;
        // D s_3_29: cast zx s_3_28 -> i
        let s_3_29: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_30: cast reint s_3_29 -> i64
        let s_3_30: i64 = (s_3_29 as i64);
        // D s_3_31: cast zx s_3_18 -> bv
        let s_3_31: Bits = Bits::new(s_3_18 as u128, 64u16);
        // D s_3_32: cast zx s_3_27 -> i
        let s_3_32: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_33: cast zx s_3_30 -> i
        let s_3_33: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_34: call Elem_read(s_3_31, s_3_32, s_3_33)
        let s_3_34: Bits = Elem_read(state, tracer, s_3_31, s_3_32, s_3_33);
        // D s_3_35: write-var op2 <= s_3_34
        fn_state.op2 = s_3_34;
        // D s_3_36: read-var esizeshadow#7640:i64
        let s_3_36: i64 = fn_state.esizeshadow_7640;
        // D s_3_37: cast zx s_3_36 -> i
        let s_3_37: i128 = (i128::try_from(s_3_36).unwrap());
        // D s_3_38: cast reint s_3_37 -> i64
        let s_3_38: i64 = (s_3_37 as i64);
        // D s_3_39: write-var ga#357987 <= s_3_38
        fn_state.ga_357987 = s_3_38;
        // D s_3_40: read-var maximum:u8
        let s_3_40: bool = fn_state.maximum;
        // N s_3_41: branch s_3_40 b9 b4
        if s_3_40 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call StandardFPSCRValue(s_4_0)
        let s_4_1: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_4_0,
        );
        // D s_4_2: read-var op1:bv
        let s_4_2: Bits = fn_state.op1;
        // D s_4_3: read-var op2:bv
        let s_4_3: Bits = fn_state.op2;
        // D s_4_4: call FPMin(s_4_2, s_4_3, s_4_1)
        let s_4_4: Bits = FPMin(state, tracer, s_4_2, s_4_3, s_4_1);
        // D s_4_5: write-var ga#357988 <= s_4_4
        fn_state.ga_357988 = s_4_4;
        // N s_4_6: jump b5
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
        // D s_5_2: read-var e:i64
        let s_5_2: i64 = fn_state.e;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var ga#357987:i64
        let s_5_4: i64 = fn_state.ga_357987;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: read-var ga#357988:bv
        let s_5_6: Bits = fn_state.ga_357988;
        // D s_5_7: call Elem_set(s_5_1, s_5_3, s_5_5, s_5_6)
        let s_5_7: Bits = Elem_set(state, tracer, s_5_1, s_5_3, s_5_5, s_5_6);
        // D s_5_8: cast reint s_5_7 -> u64
        let s_5_8: u64 = (s_5_7.value() as u64);
        // D s_5_9: write-var dest <= s_5_8
        fn_state.dest = s_5_8;
        // D s_5_10: read-var m:i64
        let s_5_10: i64 = fn_state.m;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: call D_read(s_5_11)
        let s_5_12: u64 = D_read(state, tracer, s_5_11);
        // C s_5_13: const #2s : i
        let s_5_13: i128 = 2;
        // D s_5_14: read-var e:i64
        let s_5_14: i64 = fn_state.e;
        // D s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_16: mul s_5_13 s_5_15
        let s_5_16: i128 = ((s_5_13) * (s_5_15));
        // D s_5_17: cast reint s_5_16 -> i64
        let s_5_17: i64 = (s_5_16 as i64);
        // D s_5_18: read-var esizeshadow#7640:i64
        let s_5_18: i64 = fn_state.esizeshadow_7640;
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_20: cast reint s_5_19 -> i64
        let s_5_20: i64 = (s_5_19 as i64);
        // D s_5_21: cast zx s_5_12 -> bv
        let s_5_21: Bits = Bits::new(s_5_12 as u128, 64u16);
        // D s_5_22: cast zx s_5_17 -> i
        let s_5_22: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_23: cast zx s_5_20 -> i
        let s_5_23: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_24: call Elem_read(s_5_21, s_5_22, s_5_23)
        let s_5_24: Bits = Elem_read(state, tracer, s_5_21, s_5_22, s_5_23);
        // D s_5_25: write-var op1 <= s_5_24
        fn_state.op1 = s_5_24;
        // D s_5_26: read-var m:i64
        let s_5_26: i64 = fn_state.m;
        // D s_5_27: cast zx s_5_26 -> i
        let s_5_27: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_28: call D_read(s_5_27)
        let s_5_28: u64 = D_read(state, tracer, s_5_27);
        // C s_5_29: const #2s : i
        let s_5_29: i128 = 2;
        // D s_5_30: read-var e:i64
        let s_5_30: i64 = fn_state.e;
        // D s_5_31: cast zx s_5_30 -> i
        let s_5_31: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_32: mul s_5_29 s_5_31
        let s_5_32: i128 = ((s_5_29) * (s_5_31));
        // D s_5_33: cast reint s_5_32 -> i64
        let s_5_33: i64 = (s_5_32 as i64);
        // C s_5_34: const #1s : i
        let s_5_34: i128 = 1;
        // D s_5_35: cast zx s_5_33 -> i
        let s_5_35: i128 = (i128::try_from(s_5_33).unwrap());
        // D s_5_36: add s_5_35 s_5_34
        let s_5_36: i128 = (s_5_35 + s_5_34);
        // D s_5_37: cast reint s_5_36 -> i64
        let s_5_37: i64 = (s_5_36 as i64);
        // D s_5_38: read-var esizeshadow#7640:i64
        let s_5_38: i64 = fn_state.esizeshadow_7640;
        // D s_5_39: cast zx s_5_38 -> i
        let s_5_39: i128 = (i128::try_from(s_5_38).unwrap());
        // D s_5_40: cast reint s_5_39 -> i64
        let s_5_40: i64 = (s_5_39 as i64);
        // D s_5_41: cast zx s_5_28 -> bv
        let s_5_41: Bits = Bits::new(s_5_28 as u128, 64u16);
        // D s_5_42: cast zx s_5_37 -> i
        let s_5_42: i128 = (i128::try_from(s_5_37).unwrap());
        // D s_5_43: cast zx s_5_40 -> i
        let s_5_43: i128 = (i128::try_from(s_5_40).unwrap());
        // D s_5_44: call Elem_read(s_5_41, s_5_42, s_5_43)
        let s_5_44: Bits = Elem_read(state, tracer, s_5_41, s_5_42, s_5_43);
        // D s_5_45: write-var op2 <= s_5_44
        fn_state.op2 = s_5_44;
        // D s_5_46: read-var e:i64
        let s_5_46: i64 = fn_state.e;
        // D s_5_47: cast zx s_5_46 -> i
        let s_5_47: i128 = (i128::try_from(s_5_46).unwrap());
        // D s_5_48: read-var h:i64
        let s_5_48: i64 = fn_state.h;
        // D s_5_49: cast zx s_5_48 -> i
        let s_5_49: i128 = (i128::try_from(s_5_48).unwrap());
        // D s_5_50: add s_5_47 s_5_49
        let s_5_50: i128 = (s_5_47 + s_5_49);
        // D s_5_51: cast reint s_5_50 -> i64
        let s_5_51: i64 = (s_5_50 as i64);
        // D s_5_52: write-var ga#357998 <= s_5_51
        fn_state.ga_357998 = s_5_51;
        // D s_5_53: read-var esizeshadow#7640:i64
        let s_5_53: i64 = fn_state.esizeshadow_7640;
        // D s_5_54: cast zx s_5_53 -> i
        let s_5_54: i128 = (i128::try_from(s_5_53).unwrap());
        // D s_5_55: cast reint s_5_54 -> i64
        let s_5_55: i64 = (s_5_54 as i64);
        // D s_5_56: write-var ga#357999 <= s_5_55
        fn_state.ga_357999 = s_5_55;
        // D s_5_57: read-var maximum:u8
        let s_5_57: bool = fn_state.maximum;
        // N s_5_58: branch s_5_57 b8 b6
        if s_5_57 {
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
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call StandardFPSCRValue(s_6_0)
        let s_6_1: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_6_0,
        );
        // D s_6_2: read-var op1:bv
        let s_6_2: Bits = fn_state.op1;
        // D s_6_3: read-var op2:bv
        let s_6_3: Bits = fn_state.op2;
        // D s_6_4: call FPMin(s_6_2, s_6_3, s_6_1)
        let s_6_4: Bits = FPMin(state, tracer, s_6_2, s_6_3, s_6_1);
        // D s_6_5: write-var ga#358000 <= s_6_4
        fn_state.ga_358000 = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var dest:u64
        let s_7_0: u64 = fn_state.dest;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 64u16);
        // D s_7_2: read-var ga#357998:i64
        let s_7_2: i64 = fn_state.ga_357998;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var ga#357999:i64
        let s_7_4: i64 = fn_state.ga_357999;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var ga#358000:bv
        let s_7_6: Bits = fn_state.ga_358000;
        // D s_7_7: call Elem_set(s_7_1, s_7_3, s_7_5, s_7_6)
        let s_7_7: Bits = Elem_set(state, tracer, s_7_1, s_7_3, s_7_5, s_7_6);
        // D s_7_8: cast reint s_7_7 -> u64
        let s_7_8: u64 = (s_7_7.value() as u64);
        // D s_7_9: write-var dest <= s_7_8
        fn_state.dest = s_7_8;
        // D s_7_10: read-var e:i64
        let s_7_10: i64 = fn_state.e;
        // C s_7_11: const #1s : i64
        let s_7_11: i64 = 1;
        // D s_7_12: add s_7_10 s_7_11
        let s_7_12: i64 = (s_7_10 + s_7_11);
        // D s_7_13: write-var e <= s_7_12
        fn_state.e = s_7_12;
        // N s_7_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call StandardFPSCRValue(s_8_0)
        let s_8_1: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_8_0,
        );
        // D s_8_2: read-var op1:bv
        let s_8_2: Bits = fn_state.op1;
        // D s_8_3: read-var op2:bv
        let s_8_3: Bits = fn_state.op2;
        // D s_8_4: call FPMax(s_8_2, s_8_3, s_8_1)
        let s_8_4: Bits = FPMax(state, tracer, s_8_2, s_8_3, s_8_1);
        // D s_8_5: write-var ga#358000 <= s_8_4
        fn_state.ga_358000 = s_8_4;
        // N s_8_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call StandardFPSCRValue(s_9_0)
        let s_9_1: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_9_0,
        );
        // D s_9_2: read-var op1:bv
        let s_9_2: Bits = fn_state.op1;
        // D s_9_3: read-var op2:bv
        let s_9_3: Bits = fn_state.op2;
        // D s_9_4: call FPMax(s_9_2, s_9_3, s_9_1)
        let s_9_4: Bits = FPMax(state, tracer, s_9_2, s_9_3, s_9_1);
        // D s_9_5: write-var ga#357988 <= s_9_4
        fn_state.ga_357988 = s_9_4;
        // N s_9_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var d:i64
        let s_10_0: i64 = fn_state.d;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var dest:u64
        let s_10_2: u64 = fn_state.dest;
        // D s_10_3: call D_set(s_10_1, s_10_2)
        let s_10_3: () = D_set(state, tracer, s_10_1, s_10_2);
        // N s_10_4: return
        return;
    }
}
