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
use SatQ::*;
use Elem_set::*;
use u_update_FPSCR_Type_QC::*;
use CheckAdvSIMDEnabled::*;
use u_shl_int_general::*;
use u_shr_int_general::*;
use asl_Int::*;
use D_set::*;
use Elem_read::*;
use FPSCR_read__1::*;
use FPSCR_write::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VQRSHL_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    regs: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_7715: i64,
        r: i64,
        e: i64,
        ga_359434: ProductTypef506aa96a892fe52,
        shift: i128,
        gs_316806: i64,
        d: i128,
        element: i128,
        gs_316812: i64,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        regs: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
        n,
        regs,
        is_unsigned,
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
        // D s_0_3: write-var esizeshadow#7715 <= s_0_2
        fn_state.esizeshadow_7715 = s_0_2;
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
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: read-var regs:i64
        let s_1_2: i64 = fn_state.regs;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#316806 <= s_1_5
        fn_state.gs_316806 = s_1_5;
        // D s_1_7: write-var r <= s_1_0
        fn_state.r = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#316806:i64
        let s_2_1: i64 = fn_state.gs_316806;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b13 b3
        if s_2_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
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
        // D s_3_5: write-var gs#316812 <= s_3_4
        fn_state.gs_316812 = s_3_4;
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
        // D s_4_1: read-var gs#316812:i64
        let s_4_1: i64 = fn_state.gs_316812;
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
        // D s_5_0: read-var m:i64
        let s_5_0: i64 = fn_state.m;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var r:i64
        let s_5_2: i64 = fn_state.r;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: add s_5_1 s_5_3
        let s_5_4: i128 = (s_5_1 + s_5_3);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: call D_read(s_5_6)
        let s_5_7: u64 = D_read(state, tracer, s_5_6);
        // D s_5_8: read-var esizeshadow#7715:i64
        let s_5_8: i64 = fn_state.esizeshadow_7715;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: cast zx s_5_7 -> bv
        let s_5_11: Bits = Bits::new(s_5_7 as u128, 64u16);
        // D s_5_12: read-var e:i64
        let s_5_12: i64 = fn_state.e;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: cast zx s_5_10 -> i
        let s_5_14: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_15: call Elem_read(s_5_11, s_5_13, s_5_14)
        let s_5_15: Bits = Elem_read(state, tracer, s_5_11, s_5_13, s_5_14);
        // D s_5_16: read-var is_unsigned:u8
        let s_5_16: bool = fn_state.is_unsigned;
        // D s_5_17: call asl_Int(s_5_15, s_5_16)
        let s_5_17: i128 = asl_Int(state, tracer, s_5_15, s_5_16);
        // D s_5_18: write-var element <= s_5_17
        fn_state.element = s_5_17;
        // D s_5_19: read-var n:i64
        let s_5_19: i64 = fn_state.n;
        // D s_5_20: cast zx s_5_19 -> i
        let s_5_20: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_21: read-var r:i64
        let s_5_21: i64 = fn_state.r;
        // D s_5_22: cast zx s_5_21 -> i
        let s_5_22: i128 = (i128::try_from(s_5_21).unwrap());
        // D s_5_23: add s_5_20 s_5_22
        let s_5_23: i128 = (s_5_20 + s_5_22);
        // D s_5_24: cast reint s_5_23 -> i64
        let s_5_24: i64 = (s_5_23 as i64);
        // D s_5_25: cast zx s_5_24 -> i
        let s_5_25: i128 = (i128::try_from(s_5_24).unwrap());
        // D s_5_26: call D_read(s_5_25)
        let s_5_26: u64 = D_read(state, tracer, s_5_25);
        // D s_5_27: read-var esizeshadow#7715:i64
        let s_5_27: i64 = fn_state.esizeshadow_7715;
        // D s_5_28: cast zx s_5_27 -> i
        let s_5_28: i128 = (i128::try_from(s_5_27).unwrap());
        // D s_5_29: cast reint s_5_28 -> i64
        let s_5_29: i64 = (s_5_28 as i64);
        // D s_5_30: cast zx s_5_26 -> bv
        let s_5_30: Bits = Bits::new(s_5_26 as u128, 64u16);
        // D s_5_31: read-var e:i64
        let s_5_31: i64 = fn_state.e;
        // D s_5_32: cast zx s_5_31 -> i
        let s_5_32: i128 = (i128::try_from(s_5_31).unwrap());
        // D s_5_33: cast zx s_5_29 -> i
        let s_5_33: i128 = (i128::try_from(s_5_29).unwrap());
        // D s_5_34: call Elem_read(s_5_30, s_5_32, s_5_33)
        let s_5_34: Bits = Elem_read(state, tracer, s_5_30, s_5_32, s_5_33);
        // C s_5_35: const #0s : i
        let s_5_35: i128 = 0;
        // C s_5_36: const #1s : i64
        let s_5_36: i64 = 1;
        // C s_5_37: cast zx s_5_36 -> i
        let s_5_37: i128 = (i128::try_from(s_5_36).unwrap());
        // C s_5_38: const #7s : i
        let s_5_38: i128 = 7;
        // C s_5_39: add s_5_38 s_5_37
        let s_5_39: i128 = (s_5_38 + s_5_37);
        // D s_5_40: bit-extract s_5_34 s_5_35 s_5_39
        let s_5_40: Bits = (Bits::new(
            ((s_5_34) >> (s_5_35)).value(),
            u16::try_from(s_5_39).unwrap(),
        ));
        // D s_5_41: cast reint s_5_40 -> u8
        let s_5_41: u8 = (s_5_40.value() as u8);
        // D s_5_42: cast zx s_5_41 -> bv
        let s_5_42: Bits = Bits::new(s_5_41 as u128, 8u16);
        // D s_5_43: cast sx s_5_42 -> i
        let s_5_43: i128 = {
            let sign_bit = s_5_42.length() - 1;
            let mut result = s_5_42.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_44: write-var shift <= s_5_43
        fn_state.shift = s_5_43;
        // C s_5_45: const #0s : i
        let s_5_45: i128 = 0;
        // D s_5_46: read-var shift:i
        let s_5_46: i128 = fn_state.shift;
        // D s_5_47: cmp-ge s_5_46 s_5_45
        let s_5_47: bool = ((s_5_46) >= (s_5_45));
        // N s_5_48: branch s_5_47 b11 b6
        if s_5_47 {
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
        // D s_6_0: read-var shift:i
        let s_6_0: i128 = fn_state.shift;
        // D s_6_1: neg s_6_0
        let s_6_1: i128 = -s_6_0;
        // C s_6_2: const #1s : i
        let s_6_2: i128 = 1;
        // D s_6_3: sub s_6_1 s_6_2
        let s_6_3: i128 = ((s_6_1) - (s_6_2));
        // C s_6_4: const #1s : i
        let s_6_4: i128 = 1;
        // D s_6_5: call _shl_int_general(s_6_4, s_6_3)
        let s_6_5: i128 = u_shl_int_general(state, tracer, s_6_4, s_6_3);
        // D s_6_6: read-var element:i
        let s_6_6: i128 = fn_state.element;
        // D s_6_7: add s_6_6 s_6_5
        let s_6_7: i128 = (s_6_6 + s_6_5);
        // D s_6_8: call _shr_int_general(s_6_7, s_6_1)
        let s_6_8: i128 = u_shr_int_general(state, tracer, s_6_7, s_6_1);
        // D s_6_9: write-var element <= s_6_8
        fn_state.element = s_6_8;
        // N s_6_10: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var element:i
        let s_7_0: i128 = fn_state.element;
        // D s_7_1: read-var esizeshadow#7715:i64
        let s_7_1: i64 = fn_state.esizeshadow_7715;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: read-var is_unsigned:u8
        let s_7_5: bool = fn_state.is_unsigned;
        // D s_7_6: call SatQ(s_7_0, s_7_4, s_7_5)
        let s_7_6: ProductTypef506aa96a892fe52 = SatQ(
            state,
            tracer,
            s_7_0,
            s_7_4,
            s_7_5,
        );
        // D s_7_7: write-var ga#359434 <= s_7_6
        fn_state.ga_359434 = s_7_6;
        // D s_7_8: read-var ga#359434.0:struct
        let s_7_8: Bits = fn_state.ga_359434._0;
        // D s_7_9: read-var ga#359434.1:struct
        let s_7_9: bool = fn_state.ga_359434._1;
        // D s_7_10: read-var r:i64
        let s_7_10: i64 = fn_state.r;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: read-var d:i
        let s_7_12: i128 = fn_state.d;
        // D s_7_13: add s_7_12 s_7_11
        let s_7_13: i128 = (s_7_12 + s_7_11);
        // D s_7_14: read-var r:i64
        let s_7_14: i64 = fn_state.r;
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: read-var d:i
        let s_7_16: i128 = fn_state.d;
        // D s_7_17: add s_7_16 s_7_15
        let s_7_17: i128 = (s_7_16 + s_7_15);
        // D s_7_18: call D_read(s_7_17)
        let s_7_18: u64 = D_read(state, tracer, s_7_17);
        // D s_7_19: read-var esizeshadow#7715:i64
        let s_7_19: i64 = fn_state.esizeshadow_7715;
        // D s_7_20: cast zx s_7_19 -> i
        let s_7_20: i128 = (i128::try_from(s_7_19).unwrap());
        // D s_7_21: cast reint s_7_20 -> i64
        let s_7_21: i64 = (s_7_20 as i64);
        // D s_7_22: cast zx s_7_18 -> bv
        let s_7_22: Bits = Bits::new(s_7_18 as u128, 64u16);
        // D s_7_23: read-var e:i64
        let s_7_23: i64 = fn_state.e;
        // D s_7_24: cast zx s_7_23 -> i
        let s_7_24: i128 = (i128::try_from(s_7_23).unwrap());
        // D s_7_25: cast zx s_7_21 -> i
        let s_7_25: i128 = (i128::try_from(s_7_21).unwrap());
        // D s_7_26: call Elem_set(s_7_22, s_7_24, s_7_25, s_7_8)
        let s_7_26: Bits = Elem_set(state, tracer, s_7_22, s_7_24, s_7_25, s_7_8);
        // D s_7_27: cast reint s_7_26 -> u64
        let s_7_27: u64 = (s_7_26.value() as u64);
        // D s_7_28: call D_set(s_7_13, s_7_27)
        let s_7_28: () = D_set(state, tracer, s_7_13, s_7_27);
        // N s_7_29: branch s_7_9 b10 b8
        if s_7_9 {
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
        // D s_11_0: read-var element:i
        let s_11_0: i128 = fn_state.element;
        // D s_11_1: read-var shift:i
        let s_11_1: i128 = fn_state.shift;
        // D s_11_2: call _shl_int_general(s_11_0, s_11_1)
        let s_11_2: i128 = u_shl_int_general(state, tracer, s_11_0, s_11_1);
        // D s_11_3: write-var element <= s_11_2
        fn_state.element = s_11_2;
        // N s_11_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var r:i64
        let s_12_0: i64 = fn_state.r;
        // C s_12_1: const #1s : i64
        let s_12_1: i64 = 1;
        // D s_12_2: add s_12_0 s_12_1
        let s_12_2: i64 = (s_12_0 + s_12_1);
        // D s_12_3: write-var r <= s_12_2
        fn_state.r = s_12_2;
        // N s_12_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: return
        return;
    }
}
