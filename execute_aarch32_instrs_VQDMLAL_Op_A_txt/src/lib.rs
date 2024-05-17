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
use Q_read::*;
use Elem_set::*;
use u_update_FPSCR_Type_QC::*;
use CheckAdvSIMDEnabled::*;
use Q_set::*;
use FPSCR_write::*;
use Din_read::*;
use FPSCR_read__1::*;
use Elem_read::*;
use SignedSatQ::*;
use Qin_read::*;
use common::*;
pub fn execute_aarch32_instrs_VQDMLAL_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    d__arg: i64,
    elements: i128,
    esize: i64,
    index: i128,
    m: i64,
    n: i64,
    scalar_form: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        product: Bits,
        e: i64,
        ga_358209: ProductTypef506aa96a892fe52,
        gs_315384: bool,
        esizeshadow_7648: i64,
        sat1: bool,
        sat2: bool,
        op2: i128,
        d: i128,
        ga_358226: ProductTypef506aa96a892fe52,
        result: i128,
        gs_315360: i64,
        add: bool,
        d__arg: i64,
        elements: i128,
        esize: i64,
        index: i128,
        m: i64,
        n: i64,
        scalar_form: bool,
    }
    let fn_state = FunctionState {
        add,
        d__arg,
        elements,
        esize,
        index,
        m,
        n,
        scalar_form,
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
        // D s_0_3: write-var esizeshadow#7648 <= s_0_2
        fn_state.esizeshadow_7648 = s_0_2;
        // D s_0_4: read-var d__arg:i64
        let s_0_4: i64 = fn_state.d__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var d <= s_0_5
        fn_state.d = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call CheckAdvSIMDEnabled(s_0_7)
        let s_0_8: () = CheckAdvSIMDEnabled(state, tracer, s_0_7);
        // N s_0_9: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var scalar_form:u8
        let s_1_0: bool = fn_state.scalar_form;
        // N s_1_1: branch s_1_0 b19 b2
        if s_1_0 {
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
        // N s_2_0: jump b3
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
        // D s_3_2: read-var elements:i
        let s_3_2: i128 = fn_state.elements;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var gs#315360 <= s_3_4
        fn_state.gs_315360 = s_3_4;
        // D s_3_6: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#315360:i64
        let s_4_1: i64 = fn_state.gs_315360;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b18 b5
        if s_4_2 {
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
        // D s_5_0: read-var scalar_form:u8
        let s_5_0: bool = fn_state.scalar_form;
        // D s_5_1: not s_5_0
        let s_5_1: bool = !s_5_0;
        // N s_5_2: branch s_5_1 b17 b6
        if s_5_1 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
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
        // D s_7_0: read-var n:i64
        let s_7_0: i64 = fn_state.n;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call Din_read(s_7_1)
        let s_7_2: u64 = Din_read(state, tracer, s_7_1);
        // D s_7_3: read-var esizeshadow#7648:i64
        let s_7_3: i64 = fn_state.esizeshadow_7648;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: cast zx s_7_2 -> bv
        let s_7_6: Bits = Bits::new(s_7_2 as u128, 64u16);
        // D s_7_7: read-var e:i64
        let s_7_7: i64 = fn_state.e;
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: cast zx s_7_5 -> i
        let s_7_9: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_10: call Elem_read(s_7_6, s_7_8, s_7_9)
        let s_7_10: Bits = Elem_read(state, tracer, s_7_6, s_7_8, s_7_9);
        // D s_7_11: cast sx s_7_10 -> i
        let s_7_11: i128 = {
            let sign_bit = s_7_10.length() - 1;
            let mut result = s_7_10.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // C s_7_12: const #2s : i
        let s_7_12: i128 = 2;
        // D s_7_13: mul s_7_12 s_7_11
        let s_7_13: i128 = ((s_7_12) * (s_7_11));
        // D s_7_14: read-var op2:i
        let s_7_14: i128 = fn_state.op2;
        // D s_7_15: mul s_7_13 s_7_14
        let s_7_15: i128 = ((s_7_13) * (s_7_14));
        // C s_7_16: const #2s : i
        let s_7_16: i128 = 2;
        // D s_7_17: read-var esizeshadow#7648:i64
        let s_7_17: i64 = fn_state.esizeshadow_7648;
        // D s_7_18: cast zx s_7_17 -> i
        let s_7_18: i128 = (i128::try_from(s_7_17).unwrap());
        // D s_7_19: mul s_7_16 s_7_18
        let s_7_19: i128 = ((s_7_16) * (s_7_18));
        // D s_7_20: cast reint s_7_19 -> i64
        let s_7_20: i64 = (s_7_19 as i64);
        // D s_7_21: cast zx s_7_20 -> i
        let s_7_21: i128 = (i128::try_from(s_7_20).unwrap());
        // D s_7_22: cast reint s_7_21 -> i64
        let s_7_22: i64 = (s_7_21 as i64);
        // D s_7_23: cast zx s_7_22 -> i
        let s_7_23: i128 = (i128::try_from(s_7_22).unwrap());
        // D s_7_24: call SignedSatQ(s_7_15, s_7_23)
        let s_7_24: ProductTypef506aa96a892fe52 = SignedSatQ(
            state,
            tracer,
            s_7_15,
            s_7_23,
        );
        // D s_7_25: write-var ga#358209 <= s_7_24
        fn_state.ga_358209 = s_7_24;
        // D s_7_26: read-var ga#358209.0:struct
        let s_7_26: Bits = fn_state.ga_358209._0;
        // D s_7_27: read-var ga#358209.1:struct
        let s_7_27: bool = fn_state.ga_358209._1;
        // D s_7_28: write-var product <= s_7_26
        fn_state.product = s_7_26;
        // D s_7_29: write-var sat1 <= s_7_27
        fn_state.sat1 = s_7_27;
        // D s_7_30: read-var add:u8
        let s_7_30: bool = fn_state.add;
        // N s_7_31: branch s_7_30 b16 b8
        if s_7_30 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1s : i
        let s_8_0: i128 = 1;
        // D s_8_1: read-var d:i
        let s_8_1: i128 = fn_state.d;
        // D s_8_2: lsr s_8_1 s_8_0
        let s_8_2: i128 = s_8_1 >> s_8_0;
        // D s_8_3: call Qin_read(s_8_2)
        let s_8_3: u128 = Qin_read(state, tracer, s_8_2);
        // C s_8_4: const #2s : i
        let s_8_4: i128 = 2;
        // D s_8_5: read-var esizeshadow#7648:i64
        let s_8_5: i64 = fn_state.esizeshadow_7648;
        // D s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_7: mul s_8_4 s_8_6
        let s_8_7: i128 = ((s_8_4) * (s_8_6));
        // D s_8_8: cast reint s_8_7 -> i64
        let s_8_8: i64 = (s_8_7 as i64);
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // D s_8_11: cast zx s_8_3 -> bv
        let s_8_11: Bits = Bits::new(s_8_3 as u128, 128u16);
        // D s_8_12: read-var e:i64
        let s_8_12: i64 = fn_state.e;
        // D s_8_13: cast zx s_8_12 -> i
        let s_8_13: i128 = (i128::try_from(s_8_12).unwrap());
        // D s_8_14: cast zx s_8_10 -> i
        let s_8_14: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_15: call Elem_read(s_8_11, s_8_13, s_8_14)
        let s_8_15: Bits = Elem_read(state, tracer, s_8_11, s_8_13, s_8_14);
        // D s_8_16: cast sx s_8_15 -> i
        let s_8_16: i128 = {
            let sign_bit = s_8_15.length() - 1;
            let mut result = s_8_15.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_8_17: read-var product:bv
        let s_8_17: Bits = fn_state.product;
        // D s_8_18: cast sx s_8_17 -> i
        let s_8_18: i128 = {
            let sign_bit = s_8_17.length() - 1;
            let mut result = s_8_17.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_8_19: sub s_8_16 s_8_18
        let s_8_19: i128 = ((s_8_16) - (s_8_18));
        // D s_8_20: write-var result <= s_8_19
        fn_state.result = s_8_19;
        // N s_8_21: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var result:i
        let s_9_0: i128 = fn_state.result;
        // C s_9_1: const #2s : i
        let s_9_1: i128 = 2;
        // D s_9_2: read-var esizeshadow#7648:i64
        let s_9_2: i64 = fn_state.esizeshadow_7648;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: mul s_9_1 s_9_3
        let s_9_4: i128 = ((s_9_1) * (s_9_3));
        // D s_9_5: cast reint s_9_4 -> i64
        let s_9_5: i64 = (s_9_4 as i64);
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: cast reint s_9_6 -> i64
        let s_9_7: i64 = (s_9_6 as i64);
        // D s_9_8: cast zx s_9_7 -> i
        let s_9_8: i128 = (i128::try_from(s_9_7).unwrap());
        // D s_9_9: call SignedSatQ(s_9_0, s_9_8)
        let s_9_9: ProductTypef506aa96a892fe52 = SignedSatQ(state, tracer, s_9_0, s_9_8);
        // D s_9_10: write-var ga#358226 <= s_9_9
        fn_state.ga_358226 = s_9_9;
        // D s_9_11: read-var ga#358226.0:struct
        let s_9_11: Bits = fn_state.ga_358226._0;
        // D s_9_12: read-var ga#358226.1:struct
        let s_9_12: bool = fn_state.ga_358226._1;
        // C s_9_13: const #1s : i
        let s_9_13: i128 = 1;
        // D s_9_14: read-var d:i
        let s_9_14: i128 = fn_state.d;
        // D s_9_15: lsr s_9_14 s_9_13
        let s_9_15: i128 = s_9_14 >> s_9_13;
        // C s_9_16: const #1s : i
        let s_9_16: i128 = 1;
        // D s_9_17: read-var d:i
        let s_9_17: i128 = fn_state.d;
        // D s_9_18: lsr s_9_17 s_9_16
        let s_9_18: i128 = s_9_17 >> s_9_16;
        // D s_9_19: call Q_read(s_9_18)
        let s_9_19: u128 = Q_read(state, tracer, s_9_18);
        // C s_9_20: const #2s : i
        let s_9_20: i128 = 2;
        // D s_9_21: read-var esizeshadow#7648:i64
        let s_9_21: i64 = fn_state.esizeshadow_7648;
        // D s_9_22: cast zx s_9_21 -> i
        let s_9_22: i128 = (i128::try_from(s_9_21).unwrap());
        // D s_9_23: mul s_9_20 s_9_22
        let s_9_23: i128 = ((s_9_20) * (s_9_22));
        // D s_9_24: cast reint s_9_23 -> i64
        let s_9_24: i64 = (s_9_23 as i64);
        // D s_9_25: cast zx s_9_24 -> i
        let s_9_25: i128 = (i128::try_from(s_9_24).unwrap());
        // D s_9_26: cast reint s_9_25 -> i64
        let s_9_26: i64 = (s_9_25 as i64);
        // D s_9_27: cast zx s_9_19 -> bv
        let s_9_27: Bits = Bits::new(s_9_19 as u128, 128u16);
        // D s_9_28: read-var e:i64
        let s_9_28: i64 = fn_state.e;
        // D s_9_29: cast zx s_9_28 -> i
        let s_9_29: i128 = (i128::try_from(s_9_28).unwrap());
        // D s_9_30: cast zx s_9_26 -> i
        let s_9_30: i128 = (i128::try_from(s_9_26).unwrap());
        // D s_9_31: call Elem_set(s_9_27, s_9_29, s_9_30, s_9_11)
        let s_9_31: Bits = Elem_set(state, tracer, s_9_27, s_9_29, s_9_30, s_9_11);
        // D s_9_32: cast reint s_9_31 -> u128
        let s_9_32: u128 = (s_9_31.value() as u128);
        // D s_9_33: call Q_set(s_9_15, s_9_32)
        let s_9_33: () = Q_set(state, tracer, s_9_15, s_9_32);
        // D s_9_34: write-var sat2 <= s_9_12
        fn_state.sat2 = s_9_12;
        // D s_9_35: read-var sat1:u8
        let s_9_35: bool = fn_state.sat1;
        // N s_9_36: branch s_9_35 b15 b10
        if s_9_35 {
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
        // D s_10_0: read-var sat2:u8
        let s_10_0: bool = fn_state.sat2;
        // D s_10_1: write-var gs#315384 <= s_10_0
        fn_state.gs_315384 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#315384:u8
        let s_11_0: bool = fn_state.gs_315384;
        // N s_11_1: branch s_11_0 b14 b12
        if s_11_0 {
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
        // N s_13_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call FPSCR_read__1(s_14_0)
        let s_14_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_14_0);
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // S s_14_3: call _update_FPSCR_Type_QC(s_14_1, s_14_2)
        let s_14_3: ProductType700c18a878c5601b = u_update_FPSCR_Type_QC(
            state,
            tracer,
            s_14_1,
            s_14_2,
        );
        // S s_14_4: call FPSCR_write(s_14_3)
        let s_14_4: () = FPSCR_write(state, tracer, s_14_3);
        // N s_14_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#315384 <= s_15_0
        fn_state.gs_315384 = s_15_0;
        // N s_15_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1s : i
        let s_16_0: i128 = 1;
        // D s_16_1: read-var d:i
        let s_16_1: i128 = fn_state.d;
        // D s_16_2: lsr s_16_1 s_16_0
        let s_16_2: i128 = s_16_1 >> s_16_0;
        // D s_16_3: call Qin_read(s_16_2)
        let s_16_3: u128 = Qin_read(state, tracer, s_16_2);
        // C s_16_4: const #2s : i
        let s_16_4: i128 = 2;
        // D s_16_5: read-var esizeshadow#7648:i64
        let s_16_5: i64 = fn_state.esizeshadow_7648;
        // D s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (i128::try_from(s_16_5).unwrap());
        // D s_16_7: mul s_16_4 s_16_6
        let s_16_7: i128 = ((s_16_4) * (s_16_6));
        // D s_16_8: cast reint s_16_7 -> i64
        let s_16_8: i64 = (s_16_7 as i64);
        // D s_16_9: cast zx s_16_8 -> i
        let s_16_9: i128 = (i128::try_from(s_16_8).unwrap());
        // D s_16_10: cast reint s_16_9 -> i64
        let s_16_10: i64 = (s_16_9 as i64);
        // D s_16_11: cast zx s_16_3 -> bv
        let s_16_11: Bits = Bits::new(s_16_3 as u128, 128u16);
        // D s_16_12: read-var e:i64
        let s_16_12: i64 = fn_state.e;
        // D s_16_13: cast zx s_16_12 -> i
        let s_16_13: i128 = (i128::try_from(s_16_12).unwrap());
        // D s_16_14: cast zx s_16_10 -> i
        let s_16_14: i128 = (i128::try_from(s_16_10).unwrap());
        // D s_16_15: call Elem_read(s_16_11, s_16_13, s_16_14)
        let s_16_15: Bits = Elem_read(state, tracer, s_16_11, s_16_13, s_16_14);
        // D s_16_16: cast sx s_16_15 -> i
        let s_16_16: i128 = {
            let sign_bit = s_16_15.length() - 1;
            let mut result = s_16_15.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_16_17: read-var product:bv
        let s_16_17: Bits = fn_state.product;
        // D s_16_18: cast sx s_16_17 -> i
        let s_16_18: i128 = {
            let sign_bit = s_16_17.length() - 1;
            let mut result = s_16_17.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_16_19: add s_16_16 s_16_18
        let s_16_19: i128 = (s_16_16 + s_16_18);
        // D s_16_20: write-var result <= s_16_19
        fn_state.result = s_16_19;
        // N s_16_21: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var m:i64
        let s_17_0: i64 = fn_state.m;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: call Din_read(s_17_1)
        let s_17_2: u64 = Din_read(state, tracer, s_17_1);
        // D s_17_3: read-var esizeshadow#7648:i64
        let s_17_3: i64 = fn_state.esizeshadow_7648;
        // D s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_5: cast reint s_17_4 -> i64
        let s_17_5: i64 = (s_17_4 as i64);
        // D s_17_6: cast zx s_17_2 -> bv
        let s_17_6: Bits = Bits::new(s_17_2 as u128, 64u16);
        // D s_17_7: read-var e:i64
        let s_17_7: i64 = fn_state.e;
        // D s_17_8: cast zx s_17_7 -> i
        let s_17_8: i128 = (i128::try_from(s_17_7).unwrap());
        // D s_17_9: cast zx s_17_5 -> i
        let s_17_9: i128 = (i128::try_from(s_17_5).unwrap());
        // D s_17_10: call Elem_read(s_17_6, s_17_8, s_17_9)
        let s_17_10: Bits = Elem_read(state, tracer, s_17_6, s_17_8, s_17_9);
        // D s_17_11: cast sx s_17_10 -> i
        let s_17_11: i128 = {
            let sign_bit = s_17_10.length() - 1;
            let mut result = s_17_10.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_17_12: write-var op2 <= s_17_11
        fn_state.op2 = s_17_11;
        // N s_17_13: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var m:i64
        let s_19_0: i64 = fn_state.m;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: call Din_read(s_19_1)
        let s_19_2: u64 = Din_read(state, tracer, s_19_1);
        // D s_19_3: read-var esizeshadow#7648:i64
        let s_19_3: i64 = fn_state.esizeshadow_7648;
        // D s_19_4: cast zx s_19_3 -> i
        let s_19_4: i128 = (i128::try_from(s_19_3).unwrap());
        // D s_19_5: cast reint s_19_4 -> i64
        let s_19_5: i64 = (s_19_4 as i64);
        // D s_19_6: cast zx s_19_2 -> bv
        let s_19_6: Bits = Bits::new(s_19_2 as u128, 64u16);
        // D s_19_7: cast zx s_19_5 -> i
        let s_19_7: i128 = (i128::try_from(s_19_5).unwrap());
        // D s_19_8: read-var index:i
        let s_19_8: i128 = fn_state.index;
        // D s_19_9: call Elem_read(s_19_6, s_19_8, s_19_7)
        let s_19_9: Bits = Elem_read(state, tracer, s_19_6, s_19_8, s_19_7);
        // D s_19_10: cast sx s_19_9 -> i
        let s_19_10: i128 = {
            let sign_bit = s_19_9.length() - 1;
            let mut result = s_19_9.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_19_11: write-var op2 <= s_19_10
        fn_state.op2 = s_19_10;
        // N s_19_12: jump b3
        return block_3(state, tracer, fn_state);
    }
}
