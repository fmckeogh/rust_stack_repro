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
use D_set::*;
use Elem_set::*;
use Elem_read::*;
use CheckAdvSIMDEnabled::*;
use CountLeadingZeroBits::*;
use D_read::*;
use integer_subrange::*;
use common::*;
pub fn execute_aarch32_instrs_VCLZ_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        d: i128,
        gs_307626: i64,
        gs_307632: i64,
        esizeshadow_7389: i64,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        regs: i64,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
        regs,
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
        // D s_0_3: write-var esizeshadow#7389 <= s_0_2
        fn_state.esizeshadow_7389 = s_0_2;
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
        // D s_1_6: write-var gs#307626 <= s_1_5
        fn_state.gs_307626 = s_1_5;
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
        // D s_2_1: read-var gs#307626:i64
        let s_2_1: i64 = fn_state.gs_307626;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
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
        // D s_3_5: write-var gs#307632 <= s_3_4
        fn_state.gs_307632 = s_3_4;
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
        // D s_4_1: read-var gs#307632:i64
        let s_4_1: i64 = fn_state.gs_307632;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
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
        // D s_5_4: read-var r:i64
        let s_5_4: i64 = fn_state.r;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: read-var d:i
        let s_5_6: i128 = fn_state.d;
        // D s_5_7: add s_5_6 s_5_5
        let s_5_7: i128 = (s_5_6 + s_5_5);
        // D s_5_8: call D_read(s_5_7)
        let s_5_8: u64 = D_read(state, tracer, s_5_7);
        // D s_5_9: read-var esizeshadow#7389:i64
        let s_5_9: i64 = fn_state.esizeshadow_7389;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // D s_5_12: read-var m:i64
        let s_5_12: i64 = fn_state.m;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: read-var r:i64
        let s_5_14: i64 = fn_state.r;
        // D s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_16: add s_5_13 s_5_15
        let s_5_16: i128 = (s_5_13 + s_5_15);
        // D s_5_17: cast reint s_5_16 -> i64
        let s_5_17: i64 = (s_5_16 as i64);
        // D s_5_18: cast zx s_5_17 -> i
        let s_5_18: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_19: call D_read(s_5_18)
        let s_5_19: u64 = D_read(state, tracer, s_5_18);
        // D s_5_20: read-var esizeshadow#7389:i64
        let s_5_20: i64 = fn_state.esizeshadow_7389;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: cast reint s_5_21 -> i64
        let s_5_22: i64 = (s_5_21 as i64);
        // D s_5_23: cast zx s_5_19 -> bv
        let s_5_23: Bits = Bits::new(s_5_19 as u128, 64u16);
        // D s_5_24: read-var e:i64
        let s_5_24: i64 = fn_state.e;
        // D s_5_25: cast zx s_5_24 -> i
        let s_5_25: i128 = (i128::try_from(s_5_24).unwrap());
        // D s_5_26: cast zx s_5_22 -> i
        let s_5_26: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_27: call Elem_read(s_5_23, s_5_25, s_5_26)
        let s_5_27: Bits = Elem_read(state, tracer, s_5_23, s_5_25, s_5_26);
        // D s_5_28: call CountLeadingZeroBits(s_5_27)
        let s_5_28: i128 = CountLeadingZeroBits(state, tracer, s_5_27);
        // C s_5_29: const #1s : i
        let s_5_29: i128 = 1;
        // D s_5_30: read-var esizeshadow#7389:i64
        let s_5_30: i64 = fn_state.esizeshadow_7389;
        // D s_5_31: cast zx s_5_30 -> i
        let s_5_31: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_32: sub s_5_31 s_5_29
        let s_5_32: i128 = ((s_5_31) - (s_5_29));
        // D s_5_33: cast reint s_5_32 -> i64
        let s_5_33: i64 = (s_5_32 as i64);
        // C s_5_34: const #0s : i
        let s_5_34: i128 = 0;
        // D s_5_35: cast zx s_5_33 -> i
        let s_5_35: i128 = (i128::try_from(s_5_33).unwrap());
        // D s_5_36: call integer_subrange(s_5_28, s_5_35, s_5_34)
        let s_5_36: Bits = integer_subrange(state, tracer, s_5_28, s_5_35, s_5_34);
        // D s_5_37: cast zx s_5_8 -> bv
        let s_5_37: Bits = Bits::new(s_5_8 as u128, 64u16);
        // D s_5_38: read-var e:i64
        let s_5_38: i64 = fn_state.e;
        // D s_5_39: cast zx s_5_38 -> i
        let s_5_39: i128 = (i128::try_from(s_5_38).unwrap());
        // D s_5_40: cast zx s_5_11 -> i
        let s_5_40: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_41: call Elem_set(s_5_37, s_5_39, s_5_40, s_5_36)
        let s_5_41: Bits = Elem_set(state, tracer, s_5_37, s_5_39, s_5_40, s_5_36);
        // D s_5_42: cast reint s_5_41 -> u64
        let s_5_42: u64 = (s_5_41.value() as u64);
        // D s_5_43: call D_set(s_5_3, s_5_42)
        let s_5_43: () = D_set(state, tracer, s_5_3, s_5_42);
        // D s_5_44: read-var e:i64
        let s_5_44: i64 = fn_state.e;
        // C s_5_45: const #1s : i64
        let s_5_45: i64 = 1;
        // D s_5_46: add s_5_44 s_5_45
        let s_5_46: i64 = (s_5_44 + s_5_45);
        // D s_5_47: write-var e <= s_5_46
        fn_state.e = s_5_46;
        // N s_5_48: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var r:i64
        let s_6_0: i64 = fn_state.r;
        // C s_6_1: const #1s : i64
        let s_6_1: i64 = 1;
        // D s_6_2: add s_6_0 s_6_1
        let s_6_2: i64 = (s_6_0 + s_6_1);
        // D s_6_3: write-var r <= s_6_2
        fn_state.r = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
}
