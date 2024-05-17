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
use FPToFixed::*;
use Elem_read::*;
use CheckAdvSIMDEnabled::*;
use Elem_set::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VCVTA_asimd_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i64,
    esize: i64,
    m: i64,
    regs: i64,
    rounding: u32,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_365643: u64,
        r: i64,
        esizeshadow_7913: i64,
        e: i64,
        ga_365644: i64,
        ga_365646: i128,
        gs_325084: i64,
        d: i128,
        gs_325078: i64,
        ga_365645: Bits,
        d__arg: i64,
        elements: i64,
        esize: i64,
        m: i64,
        regs: i64,
        rounding: u32,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
        regs,
        rounding,
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
        // D s_0_3: write-var esizeshadow#7913 <= s_0_2
        fn_state.esizeshadow_7913 = s_0_2;
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
        // D s_1_6: write-var gs#325078 <= s_1_5
        fn_state.gs_325078 = s_1_5;
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
        // D s_2_1: read-var gs#325078:i64
        let s_2_1: i64 = fn_state.gs_325078;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b8 b3
        if s_2_2 {
            return block_8(state, tracer, fn_state);
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
        // D s_3_2: read-var elements:i64
        let s_3_2: i64 = fn_state.elements;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#325084 <= s_3_5
        fn_state.gs_325084 = s_3_5;
        // D s_3_7: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#325084:i64
        let s_4_1: i64 = fn_state.gs_325084;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b7 b5
        if s_4_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var r:i64
        let s_5_0: i64 = fn_state.r;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var d:i
        let s_5_2: i128 = fn_state.d;
        // D s_5_3: add s_5_2 s_5_1
        let s_5_3: i128 = (s_5_2 + s_5_1);
        // D s_5_4: write-var ga#365646 <= s_5_3
        fn_state.ga_365646 = s_5_3;
        // D s_5_5: read-var r:i64
        let s_5_5: i64 = fn_state.r;
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: read-var d:i
        let s_5_7: i128 = fn_state.d;
        // D s_5_8: add s_5_7 s_5_6
        let s_5_8: i128 = (s_5_7 + s_5_6);
        // D s_5_9: call D_read(s_5_8)
        let s_5_9: u64 = D_read(state, tracer, s_5_8);
        // D s_5_10: write-var ga#365643 <= s_5_9
        fn_state.ga_365643 = s_5_9;
        // D s_5_11: read-var esizeshadow#7913:i64
        let s_5_11: i64 = fn_state.esizeshadow_7913;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: write-var ga#365644 <= s_5_13
        fn_state.ga_365644 = s_5_13;
        // D s_5_15: read-var m:i64
        let s_5_15: i64 = fn_state.m;
        // D s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_17: read-var r:i64
        let s_5_17: i64 = fn_state.r;
        // D s_5_18: cast zx s_5_17 -> i
        let s_5_18: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_19: add s_5_16 s_5_18
        let s_5_19: i128 = (s_5_16 + s_5_18);
        // D s_5_20: cast reint s_5_19 -> i64
        let s_5_20: i64 = (s_5_19 as i64);
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: call D_read(s_5_21)
        let s_5_22: u64 = D_read(state, tracer, s_5_21);
        // D s_5_23: read-var esizeshadow#7913:i64
        let s_5_23: i64 = fn_state.esizeshadow_7913;
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_25: cast reint s_5_24 -> i64
        let s_5_25: i64 = (s_5_24 as i64);
        // D s_5_26: cast zx s_5_22 -> bv
        let s_5_26: Bits = Bits::new(s_5_22 as u128, 64u16);
        // D s_5_27: read-var e:i64
        let s_5_27: i64 = fn_state.e;
        // D s_5_28: cast zx s_5_27 -> i
        let s_5_28: i128 = (i128::try_from(s_5_27).unwrap());
        // D s_5_29: cast zx s_5_25 -> i
        let s_5_29: i128 = (i128::try_from(s_5_25).unwrap());
        // D s_5_30: call Elem_read(s_5_26, s_5_28, s_5_29)
        let s_5_30: Bits = Elem_read(state, tracer, s_5_26, s_5_28, s_5_29);
        // C s_5_31: const #() : ()
        let s_5_31: () = ();
        // S s_5_32: call StandardFPSCRValue(s_5_31)
        let s_5_32: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_5_31,
        );
        // D s_5_33: read-var esizeshadow#7913:i64
        let s_5_33: i64 = fn_state.esizeshadow_7913;
        // D s_5_34: cast zx s_5_33 -> i
        let s_5_34: i128 = (i128::try_from(s_5_33).unwrap());
        // D s_5_35: cast reint s_5_34 -> i64
        let s_5_35: i64 = (s_5_34 as i64);
        // C s_5_36: const #0s : i
        let s_5_36: i128 = 0;
        // D s_5_37: read-var is_unsigned:u8
        let s_5_37: bool = fn_state.is_unsigned;
        // D s_5_38: read-var rounding:u32
        let s_5_38: u32 = fn_state.rounding;
        // D s_5_39: call FPToFixed(s_5_30, s_5_36, s_5_37, s_5_32, s_5_38, s_5_35)
        let s_5_39: Bits = FPToFixed(
            state,
            tracer,
            s_5_30,
            s_5_36,
            s_5_37,
            s_5_32,
            s_5_38,
            s_5_35,
        );
        // D s_5_40: write-var ga#365645 <= s_5_39
        fn_state.ga_365645 = s_5_39;
        // N s_5_41: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#365643:u64
        let s_6_0: u64 = fn_state.ga_365643;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 64u16);
        // D s_6_2: read-var e:i64
        let s_6_2: i64 = fn_state.e;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: read-var ga#365644:i64
        let s_6_4: i64 = fn_state.ga_365644;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: read-var ga#365645:bv
        let s_6_6: Bits = fn_state.ga_365645;
        // D s_6_7: call Elem_set(s_6_1, s_6_3, s_6_5, s_6_6)
        let s_6_7: Bits = Elem_set(state, tracer, s_6_1, s_6_3, s_6_5, s_6_6);
        // D s_6_8: cast reint s_6_7 -> u64
        let s_6_8: u64 = (s_6_7.value() as u64);
        // D s_6_9: read-var ga#365646:i
        let s_6_9: i128 = fn_state.ga_365646;
        // D s_6_10: call D_set(s_6_9, s_6_8)
        let s_6_10: () = D_set(state, tracer, s_6_9, s_6_8);
        // D s_6_11: read-var e:i64
        let s_6_11: i64 = fn_state.e;
        // C s_6_12: const #1s : i64
        let s_6_12: i64 = 1;
        // D s_6_13: add s_6_11 s_6_12
        let s_6_13: i64 = (s_6_11 + s_6_12);
        // D s_6_14: write-var e <= s_6_13
        fn_state.e = s_6_13;
        // N s_6_15: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var r:i64
        let s_7_0: i64 = fn_state.r;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var r <= s_7_2
        fn_state.r = s_7_2;
        // N s_7_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: return
        return;
    }
}
