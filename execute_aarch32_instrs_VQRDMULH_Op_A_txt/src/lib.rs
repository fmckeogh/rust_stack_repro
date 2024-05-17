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
use Elem_set::*;
use u_update_FPSCR_Type_QC::*;
use CheckAdvSIMDEnabled::*;
use D_set::*;
use FPSCR_write::*;
use FPSCR_read__1::*;
use Elem_read::*;
use RShr::*;
use D_read::*;
use SignedSatQ::*;
use common::*;
pub fn execute_aarch32_instrs_VQRDMULH_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    index: i128,
    m: i64,
    n: i64,
    regs: i64,
    scalar_form: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        ga_358948: ProductTypef506aa96a892fe52,
        e: i64,
        op2: i128,
        d: i128,
        op1: i128,
        gs_316224: i64,
        esizeshadow_7688: i64,
        gs_316230: i64,
        d__arg: i64,
        elements: i128,
        esize: i64,
        index: i128,
        m: i64,
        n: i64,
        regs: i64,
        scalar_form: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        index,
        m,
        n,
        regs,
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
        // D s_0_3: write-var esizeshadow#7688 <= s_0_2
        fn_state.esizeshadow_7688 = s_0_2;
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
        // N s_1_1: branch s_1_0 b16 b2
        if s_1_0 {
            return block_16(state, tracer, fn_state);
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
        // D s_3_2: read-var regs:i64
        let s_3_2: i64 = fn_state.regs;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#316224 <= s_3_5
        fn_state.gs_316224 = s_3_5;
        // D s_3_7: write-var r <= s_3_0
        fn_state.r = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var r:i64
        let s_4_0: i64 = fn_state.r;
        // D s_4_1: read-var gs#316224:i64
        let s_4_1: i64 = fn_state.gs_316224;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b15 b5
        if s_4_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i64
        let s_5_0: i64 = 0;
        // C s_5_1: const #1s : i
        let s_5_1: i128 = 1;
        // D s_5_2: read-var elements:i
        let s_5_2: i128 = fn_state.elements;
        // D s_5_3: sub s_5_2 s_5_1
        let s_5_3: i128 = ((s_5_2) - (s_5_1));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: write-var gs#316230 <= s_5_4
        fn_state.gs_316230 = s_5_4;
        // D s_5_6: write-var e <= s_5_0
        fn_state.e = s_5_0;
        // N s_5_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#316230:i64
        let s_6_1: i64 = fn_state.gs_316230;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b14 b7
        if s_6_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
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
        // D s_7_2: read-var r:i64
        let s_7_2: i64 = fn_state.r;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: add s_7_1 s_7_3
        let s_7_4: i128 = (s_7_1 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: call D_read(s_7_6)
        let s_7_7: u64 = D_read(state, tracer, s_7_6);
        // D s_7_8: read-var esizeshadow#7688:i64
        let s_7_8: i64 = fn_state.esizeshadow_7688;
        // D s_7_9: cast zx s_7_8 -> i
        let s_7_9: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // D s_7_11: cast zx s_7_7 -> bv
        let s_7_11: Bits = Bits::new(s_7_7 as u128, 64u16);
        // D s_7_12: read-var e:i64
        let s_7_12: i64 = fn_state.e;
        // D s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_14: cast zx s_7_10 -> i
        let s_7_14: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_15: call Elem_read(s_7_11, s_7_13, s_7_14)
        let s_7_15: Bits = Elem_read(state, tracer, s_7_11, s_7_13, s_7_14);
        // D s_7_16: cast sx s_7_15 -> i
        let s_7_16: i128 = {
            let sign_bit = s_7_15.length() - 1;
            let mut result = s_7_15.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_7_17: write-var op1 <= s_7_16
        fn_state.op1 = s_7_16;
        // D s_7_18: read-var scalar_form:u8
        let s_7_18: bool = fn_state.scalar_form;
        // D s_7_19: not s_7_18
        let s_7_19: bool = !s_7_18;
        // N s_7_20: branch s_7_19 b13 b8
        if s_7_19 {
            return block_13(state, tracer, fn_state);
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
        // C s_9_0: const #2s : i
        let s_9_0: i128 = 2;
        // D s_9_1: read-var op1:i
        let s_9_1: i128 = fn_state.op1;
        // D s_9_2: mul s_9_0 s_9_1
        let s_9_2: i128 = ((s_9_0) * (s_9_1));
        // D s_9_3: read-var op2:i
        let s_9_3: i128 = fn_state.op2;
        // D s_9_4: mul s_9_2 s_9_3
        let s_9_4: i128 = ((s_9_2) * (s_9_3));
        // D s_9_5: read-var esizeshadow#7688:i64
        let s_9_5: i64 = fn_state.esizeshadow_7688;
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // C s_9_7: const #1u : u8
        let s_9_7: bool = true;
        // D s_9_8: call RShr(s_9_4, s_9_6, s_9_7)
        let s_9_8: i128 = RShr(state, tracer, s_9_4, s_9_6, s_9_7);
        // D s_9_9: read-var esizeshadow#7688:i64
        let s_9_9: i64 = fn_state.esizeshadow_7688;
        // D s_9_10: cast zx s_9_9 -> i
        let s_9_10: i128 = (i128::try_from(s_9_9).unwrap());
        // D s_9_11: cast reint s_9_10 -> i64
        let s_9_11: i64 = (s_9_10 as i64);
        // D s_9_12: cast zx s_9_11 -> i
        let s_9_12: i128 = (i128::try_from(s_9_11).unwrap());
        // D s_9_13: call SignedSatQ(s_9_8, s_9_12)
        let s_9_13: ProductTypef506aa96a892fe52 = SignedSatQ(
            state,
            tracer,
            s_9_8,
            s_9_12,
        );
        // D s_9_14: write-var ga#358948 <= s_9_13
        fn_state.ga_358948 = s_9_13;
        // D s_9_15: read-var ga#358948.0:struct
        let s_9_15: Bits = fn_state.ga_358948._0;
        // D s_9_16: read-var ga#358948.1:struct
        let s_9_16: bool = fn_state.ga_358948._1;
        // D s_9_17: read-var r:i64
        let s_9_17: i64 = fn_state.r;
        // D s_9_18: cast zx s_9_17 -> i
        let s_9_18: i128 = (i128::try_from(s_9_17).unwrap());
        // D s_9_19: read-var d:i
        let s_9_19: i128 = fn_state.d;
        // D s_9_20: add s_9_19 s_9_18
        let s_9_20: i128 = (s_9_19 + s_9_18);
        // D s_9_21: read-var r:i64
        let s_9_21: i64 = fn_state.r;
        // D s_9_22: cast zx s_9_21 -> i
        let s_9_22: i128 = (i128::try_from(s_9_21).unwrap());
        // D s_9_23: read-var d:i
        let s_9_23: i128 = fn_state.d;
        // D s_9_24: add s_9_23 s_9_22
        let s_9_24: i128 = (s_9_23 + s_9_22);
        // D s_9_25: call D_read(s_9_24)
        let s_9_25: u64 = D_read(state, tracer, s_9_24);
        // D s_9_26: read-var esizeshadow#7688:i64
        let s_9_26: i64 = fn_state.esizeshadow_7688;
        // D s_9_27: cast zx s_9_26 -> i
        let s_9_27: i128 = (i128::try_from(s_9_26).unwrap());
        // D s_9_28: cast reint s_9_27 -> i64
        let s_9_28: i64 = (s_9_27 as i64);
        // D s_9_29: cast zx s_9_25 -> bv
        let s_9_29: Bits = Bits::new(s_9_25 as u128, 64u16);
        // D s_9_30: read-var e:i64
        let s_9_30: i64 = fn_state.e;
        // D s_9_31: cast zx s_9_30 -> i
        let s_9_31: i128 = (i128::try_from(s_9_30).unwrap());
        // D s_9_32: cast zx s_9_28 -> i
        let s_9_32: i128 = (i128::try_from(s_9_28).unwrap());
        // D s_9_33: call Elem_set(s_9_29, s_9_31, s_9_32, s_9_15)
        let s_9_33: Bits = Elem_set(state, tracer, s_9_29, s_9_31, s_9_32, s_9_15);
        // D s_9_34: cast reint s_9_33 -> u64
        let s_9_34: u64 = (s_9_33.value() as u64);
        // D s_9_35: call D_set(s_9_20, s_9_34)
        let s_9_35: () = D_set(state, tracer, s_9_20, s_9_34);
        // N s_9_36: branch s_9_16 b12 b10
        if s_9_16 {
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
        // N s_10_0: jump b11
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
        // N s_11_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call FPSCR_read__1(s_12_0)
        let s_12_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_12_0);
        // C s_12_2: const #1u : u8
        let s_12_2: bool = true;
        // S s_12_3: call _update_FPSCR_Type_QC(s_12_1, s_12_2)
        let s_12_3: ProductType700c18a878c5601b = u_update_FPSCR_Type_QC(
            state,
            tracer,
            s_12_1,
            s_12_2,
        );
        // S s_12_4: call FPSCR_write(s_12_3)
        let s_12_4: () = FPSCR_write(state, tracer, s_12_3);
        // N s_12_5: jump b11
        return block_11(state, tracer, fn_state);
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
        // D s_13_2: read-var r:i64
        let s_13_2: i64 = fn_state.r;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: add s_13_1 s_13_3
        let s_13_4: i128 = (s_13_1 + s_13_3);
        // D s_13_5: cast reint s_13_4 -> i64
        let s_13_5: i64 = (s_13_4 as i64);
        // D s_13_6: cast zx s_13_5 -> i
        let s_13_6: i128 = (i128::try_from(s_13_5).unwrap());
        // D s_13_7: call D_read(s_13_6)
        let s_13_7: u64 = D_read(state, tracer, s_13_6);
        // D s_13_8: read-var esizeshadow#7688:i64
        let s_13_8: i64 = fn_state.esizeshadow_7688;
        // D s_13_9: cast zx s_13_8 -> i
        let s_13_9: i128 = (i128::try_from(s_13_8).unwrap());
        // D s_13_10: cast reint s_13_9 -> i64
        let s_13_10: i64 = (s_13_9 as i64);
        // D s_13_11: cast zx s_13_7 -> bv
        let s_13_11: Bits = Bits::new(s_13_7 as u128, 64u16);
        // D s_13_12: read-var e:i64
        let s_13_12: i64 = fn_state.e;
        // D s_13_13: cast zx s_13_12 -> i
        let s_13_13: i128 = (i128::try_from(s_13_12).unwrap());
        // D s_13_14: cast zx s_13_10 -> i
        let s_13_14: i128 = (i128::try_from(s_13_10).unwrap());
        // D s_13_15: call Elem_read(s_13_11, s_13_13, s_13_14)
        let s_13_15: Bits = Elem_read(state, tracer, s_13_11, s_13_13, s_13_14);
        // D s_13_16: cast sx s_13_15 -> i
        let s_13_16: i128 = {
            let sign_bit = s_13_15.length() - 1;
            let mut result = s_13_15.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_13_17: write-var op2 <= s_13_16
        fn_state.op2 = s_13_16;
        // N s_13_18: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var r:i64
        let s_14_0: i64 = fn_state.r;
        // C s_14_1: const #1s : i64
        let s_14_1: i64 = 1;
        // D s_14_2: add s_14_0 s_14_1
        let s_14_2: i64 = (s_14_0 + s_14_1);
        // D s_14_3: write-var r <= s_14_2
        fn_state.r = s_14_2;
        // N s_14_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var m:i64
        let s_16_0: i64 = fn_state.m;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: call D_read(s_16_1)
        let s_16_2: u64 = D_read(state, tracer, s_16_1);
        // D s_16_3: read-var esizeshadow#7688:i64
        let s_16_3: i64 = fn_state.esizeshadow_7688;
        // D s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_5: cast reint s_16_4 -> i64
        let s_16_5: i64 = (s_16_4 as i64);
        // D s_16_6: cast zx s_16_2 -> bv
        let s_16_6: Bits = Bits::new(s_16_2 as u128, 64u16);
        // D s_16_7: cast zx s_16_5 -> i
        let s_16_7: i128 = (i128::try_from(s_16_5).unwrap());
        // D s_16_8: read-var index:i
        let s_16_8: i128 = fn_state.index;
        // D s_16_9: call Elem_read(s_16_6, s_16_8, s_16_7)
        let s_16_9: Bits = Elem_read(state, tracer, s_16_6, s_16_8, s_16_7);
        // D s_16_10: cast sx s_16_9 -> i
        let s_16_10: i128 = {
            let sign_bit = s_16_9.length() - 1;
            let mut result = s_16_9.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_16_11: write-var op2 <= s_16_10
        fn_state.op2 = s_16_10;
        // N s_16_12: jump b3
        return block_3(state, tracer, fn_state);
    }
}
