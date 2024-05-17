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
use common::*;
pub fn execute_aarch32_instrs_VQDMULL_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
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
        esizeshadow_7677: i64,
        gs_315937: i64,
        e: i64,
        op2: i128,
        d: i128,
        ga_358703: ProductTypef506aa96a892fe52,
        d__arg: i64,
        elements: i128,
        esize: i64,
        index: i128,
        m: i64,
        n: i64,
        scalar_form: bool,
    }
    let fn_state = FunctionState {
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
        // D s_0_3: write-var esizeshadow#7677 <= s_0_2
        fn_state.esizeshadow_7677 = s_0_2;
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
        // N s_1_1: branch s_1_0 b13 b2
        if s_1_0 {
            return block_13(state, tracer, fn_state);
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
        // D s_3_5: write-var gs#315937 <= s_3_4
        fn_state.gs_315937 = s_3_4;
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
        // D s_4_1: read-var gs#315937:i64
        let s_4_1: i64 = fn_state.gs_315937;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b12 b5
        if s_4_2 {
            return block_12(state, tracer, fn_state);
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
        // N s_5_2: branch s_5_1 b11 b6
        if s_5_1 {
            return block_11(state, tracer, fn_state);
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
        // D s_7_3: read-var esizeshadow#7677:i64
        let s_7_3: i64 = fn_state.esizeshadow_7677;
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
        // D s_7_17: read-var esizeshadow#7677:i64
        let s_7_17: i64 = fn_state.esizeshadow_7677;
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
        // D s_7_25: write-var ga#358703 <= s_7_24
        fn_state.ga_358703 = s_7_24;
        // D s_7_26: read-var ga#358703.0:struct
        let s_7_26: Bits = fn_state.ga_358703._0;
        // D s_7_27: read-var ga#358703.1:struct
        let s_7_27: bool = fn_state.ga_358703._1;
        // C s_7_28: const #1s : i
        let s_7_28: i128 = 1;
        // D s_7_29: read-var d:i
        let s_7_29: i128 = fn_state.d;
        // D s_7_30: lsr s_7_29 s_7_28
        let s_7_30: i128 = s_7_29 >> s_7_28;
        // C s_7_31: const #1s : i
        let s_7_31: i128 = 1;
        // D s_7_32: read-var d:i
        let s_7_32: i128 = fn_state.d;
        // D s_7_33: lsr s_7_32 s_7_31
        let s_7_33: i128 = s_7_32 >> s_7_31;
        // D s_7_34: call Q_read(s_7_33)
        let s_7_34: u128 = Q_read(state, tracer, s_7_33);
        // C s_7_35: const #2s : i
        let s_7_35: i128 = 2;
        // D s_7_36: read-var esizeshadow#7677:i64
        let s_7_36: i64 = fn_state.esizeshadow_7677;
        // D s_7_37: cast zx s_7_36 -> i
        let s_7_37: i128 = (i128::try_from(s_7_36).unwrap());
        // D s_7_38: mul s_7_35 s_7_37
        let s_7_38: i128 = ((s_7_35) * (s_7_37));
        // D s_7_39: cast reint s_7_38 -> i64
        let s_7_39: i64 = (s_7_38 as i64);
        // D s_7_40: cast zx s_7_39 -> i
        let s_7_40: i128 = (i128::try_from(s_7_39).unwrap());
        // D s_7_41: cast reint s_7_40 -> i64
        let s_7_41: i64 = (s_7_40 as i64);
        // D s_7_42: cast zx s_7_34 -> bv
        let s_7_42: Bits = Bits::new(s_7_34 as u128, 128u16);
        // D s_7_43: read-var e:i64
        let s_7_43: i64 = fn_state.e;
        // D s_7_44: cast zx s_7_43 -> i
        let s_7_44: i128 = (i128::try_from(s_7_43).unwrap());
        // D s_7_45: cast zx s_7_41 -> i
        let s_7_45: i128 = (i128::try_from(s_7_41).unwrap());
        // D s_7_46: call Elem_set(s_7_42, s_7_44, s_7_45, s_7_26)
        let s_7_46: Bits = Elem_set(state, tracer, s_7_42, s_7_44, s_7_45, s_7_26);
        // D s_7_47: cast reint s_7_46 -> u128
        let s_7_47: u128 = (s_7_46.value() as u128);
        // D s_7_48: call Q_set(s_7_30, s_7_47)
        let s_7_48: () = Q_set(state, tracer, s_7_30, s_7_47);
        // N s_7_49: branch s_7_27 b10 b8
        if s_7_27 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var e:i64
        let s_9_0: i64 = fn_state.e;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var e <= s_9_2
        fn_state.e = s_9_2;
        // N s_9_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call FPSCR_read__1(s_10_0)
        let s_10_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_10_0);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // S s_10_3: call _update_FPSCR_Type_QC(s_10_1, s_10_2)
        let s_10_3: ProductType700c18a878c5601b = u_update_FPSCR_Type_QC(
            state,
            tracer,
            s_10_1,
            s_10_2,
        );
        // S s_10_4: call FPSCR_write(s_10_3)
        let s_10_4: () = FPSCR_write(state, tracer, s_10_3);
        // N s_10_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var m:i64
        let s_11_0: i64 = fn_state.m;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call Din_read(s_11_1)
        let s_11_2: u64 = Din_read(state, tracer, s_11_1);
        // D s_11_3: read-var esizeshadow#7677:i64
        let s_11_3: i64 = fn_state.esizeshadow_7677;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: cast reint s_11_4 -> i64
        let s_11_5: i64 = (s_11_4 as i64);
        // D s_11_6: cast zx s_11_2 -> bv
        let s_11_6: Bits = Bits::new(s_11_2 as u128, 64u16);
        // D s_11_7: read-var e:i64
        let s_11_7: i64 = fn_state.e;
        // D s_11_8: cast zx s_11_7 -> i
        let s_11_8: i128 = (i128::try_from(s_11_7).unwrap());
        // D s_11_9: cast zx s_11_5 -> i
        let s_11_9: i128 = (i128::try_from(s_11_5).unwrap());
        // D s_11_10: call Elem_read(s_11_6, s_11_8, s_11_9)
        let s_11_10: Bits = Elem_read(state, tracer, s_11_6, s_11_8, s_11_9);
        // D s_11_11: cast sx s_11_10 -> i
        let s_11_11: i128 = {
            let sign_bit = s_11_10.length() - 1;
            let mut result = s_11_10.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_11_12: write-var op2 <= s_11_11
        fn_state.op2 = s_11_11;
        // N s_11_13: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var m:i64
        let s_13_0: i64 = fn_state.m;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call Din_read(s_13_1)
        let s_13_2: u64 = Din_read(state, tracer, s_13_1);
        // D s_13_3: read-var esizeshadow#7677:i64
        let s_13_3: i64 = fn_state.esizeshadow_7677;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: cast reint s_13_4 -> i64
        let s_13_5: i64 = (s_13_4 as i64);
        // D s_13_6: cast zx s_13_2 -> bv
        let s_13_6: Bits = Bits::new(s_13_2 as u128, 64u16);
        // D s_13_7: cast zx s_13_5 -> i
        let s_13_7: i128 = (i128::try_from(s_13_5).unwrap());
        // D s_13_8: read-var index:i
        let s_13_8: i128 = fn_state.index;
        // D s_13_9: call Elem_read(s_13_6, s_13_8, s_13_7)
        let s_13_9: Bits = Elem_read(state, tracer, s_13_6, s_13_8, s_13_7);
        // D s_13_10: cast sx s_13_9 -> i
        let s_13_10: i128 = {
            let sign_bit = s_13_9.length() - 1;
            let mut result = s_13_9.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_13_11: write-var op2 <= s_13_10
        fn_state.op2 = s_13_10;
        // N s_13_12: jump b3
        return block_3(state, tracer, fn_state);
    }
}
