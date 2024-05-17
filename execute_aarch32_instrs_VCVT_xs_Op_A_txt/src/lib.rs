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
use FPToFixed::*;
use Elem_read::*;
use D_read::*;
use FixedToFP::*;
use common::*;
pub fn execute_aarch32_instrs_VCVT_xs_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i64,
    esize: i64,
    frac_bits: i64,
    m: i64,
    regs: i64,
    to_fixed: bool,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        gs_308243: i64,
        d: i128,
        op1: Bits,
        result: Bits,
        gs_308237: i64,
        esizeshadow_7418: i64,
        d__arg: i64,
        elements: i64,
        esize: i64,
        frac_bits: i64,
        m: i64,
        regs: i64,
        to_fixed: bool,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        frac_bits,
        m,
        regs,
        to_fixed,
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
        // D s_0_3: write-var esizeshadow#7418 <= s_0_2
        fn_state.esizeshadow_7418 = s_0_2;
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
        // D s_1_6: write-var gs#308237 <= s_1_5
        fn_state.gs_308237 = s_1_5;
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
        // D s_2_1: read-var gs#308237:i64
        let s_2_1: i64 = fn_state.gs_308237;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b12 b3
        if s_2_2 {
            return block_12(state, tracer, fn_state);
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
        // D s_3_6: write-var gs#308243 <= s_3_5
        fn_state.gs_308243 = s_3_5;
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
        // D s_4_1: read-var gs#308243:i64
        let s_4_1: i64 = fn_state.gs_308243;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b11 b5
        if s_4_2 {
            return block_11(state, tracer, fn_state);
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
        // D s_5_8: read-var esizeshadow#7418:i64
        let s_5_8: i64 = fn_state.esizeshadow_7418;
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
        // D s_5_16: write-var op1 <= s_5_15
        fn_state.op1 = s_5_15;
        // D s_5_17: read-var to_fixed:u8
        let s_5_17: bool = fn_state.to_fixed;
        // N s_5_18: branch s_5_17 b9 b6
        if s_5_17 {
            return block_9(state, tracer, fn_state);
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
        // D s_6_2: read-var esizeshadow#7418:i64
        let s_6_2: i64 = fn_state.esizeshadow_7418;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // D s_6_5: read-var frac_bits:i64
        let s_6_5: i64 = fn_state.frac_bits;
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: read-var op1:bv
        let s_6_7: Bits = fn_state.op1;
        // D s_6_8: read-var is_unsigned:u8
        let s_6_8: bool = fn_state.is_unsigned;
        // C s_6_9: const #0u : u32
        let s_6_9: u32 = 0;
        // D s_6_10: call FixedToFP(s_6_7, s_6_6, s_6_8, s_6_1, s_6_9, s_6_4)
        let s_6_10: Bits = FixedToFP(
            state,
            tracer,
            s_6_7,
            s_6_6,
            s_6_8,
            s_6_1,
            s_6_9,
            s_6_4,
        );
        // D s_6_11: write-var result <= s_6_10
        fn_state.result = s_6_10;
        // N s_6_12: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var r:i64
        let s_8_0: i64 = fn_state.r;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var d:i
        let s_8_2: i128 = fn_state.d;
        // D s_8_3: add s_8_2 s_8_1
        let s_8_3: i128 = (s_8_2 + s_8_1);
        // D s_8_4: read-var r:i64
        let s_8_4: i64 = fn_state.r;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: read-var d:i
        let s_8_6: i128 = fn_state.d;
        // D s_8_7: add s_8_6 s_8_5
        let s_8_7: i128 = (s_8_6 + s_8_5);
        // D s_8_8: call D_read(s_8_7)
        let s_8_8: u64 = D_read(state, tracer, s_8_7);
        // D s_8_9: read-var esizeshadow#7418:i64
        let s_8_9: i64 = fn_state.esizeshadow_7418;
        // D s_8_10: cast zx s_8_9 -> i
        let s_8_10: i128 = (i128::try_from(s_8_9).unwrap());
        // D s_8_11: cast reint s_8_10 -> i64
        let s_8_11: i64 = (s_8_10 as i64);
        // D s_8_12: cast zx s_8_8 -> bv
        let s_8_12: Bits = Bits::new(s_8_8 as u128, 64u16);
        // D s_8_13: read-var e:i64
        let s_8_13: i64 = fn_state.e;
        // D s_8_14: cast zx s_8_13 -> i
        let s_8_14: i128 = (i128::try_from(s_8_13).unwrap());
        // D s_8_15: cast zx s_8_11 -> i
        let s_8_15: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_16: read-var result:bv
        let s_8_16: Bits = fn_state.result;
        // D s_8_17: call Elem_set(s_8_12, s_8_14, s_8_15, s_8_16)
        let s_8_17: Bits = Elem_set(state, tracer, s_8_12, s_8_14, s_8_15, s_8_16);
        // D s_8_18: cast reint s_8_17 -> u64
        let s_8_18: u64 = (s_8_17.value() as u64);
        // D s_8_19: call D_set(s_8_3, s_8_18)
        let s_8_19: () = D_set(state, tracer, s_8_3, s_8_18);
        // D s_8_20: read-var e:i64
        let s_8_20: i64 = fn_state.e;
        // C s_8_21: const #1s : i64
        let s_8_21: i64 = 1;
        // D s_8_22: add s_8_20 s_8_21
        let s_8_22: i64 = (s_8_20 + s_8_21);
        // D s_8_23: write-var e <= s_8_22
        fn_state.e = s_8_22;
        // N s_8_24: jump b4
        return block_4(state, tracer, fn_state);
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
        // D s_9_2: read-var esizeshadow#7418:i64
        let s_9_2: i64 = fn_state.esizeshadow_7418;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: cast reint s_9_3 -> i64
        let s_9_4: i64 = (s_9_3 as i64);
        // D s_9_5: read-var frac_bits:i64
        let s_9_5: i64 = fn_state.frac_bits;
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: read-var op1:bv
        let s_9_7: Bits = fn_state.op1;
        // D s_9_8: read-var is_unsigned:u8
        let s_9_8: bool = fn_state.is_unsigned;
        // C s_9_9: const #3u : u32
        let s_9_9: u32 = 3;
        // D s_9_10: call FPToFixed(s_9_7, s_9_6, s_9_8, s_9_1, s_9_9, s_9_4)
        let s_9_10: Bits = FPToFixed(
            state,
            tracer,
            s_9_7,
            s_9_6,
            s_9_8,
            s_9_1,
            s_9_9,
            s_9_4,
        );
        // D s_9_11: write-var result <= s_9_10
        fn_state.result = s_9_10;
        // N s_9_12: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var r:i64
        let s_11_0: i64 = fn_state.r;
        // C s_11_1: const #1s : i64
        let s_11_1: i64 = 1;
        // D s_11_2: add s_11_0 s_11_1
        let s_11_2: i64 = (s_11_0 + s_11_1);
        // D s_11_3: write-var r <= s_11_2
        fn_state.r = s_11_2;
        // N s_11_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: return
        return;
    }
}
