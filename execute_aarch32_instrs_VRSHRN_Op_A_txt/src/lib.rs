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
use CheckAdvSIMDEnabled::*;
use integer_subrange::*;
use D_set::*;
use Elem_read::*;
use RShr::*;
use D_read::*;
use Qin_read::*;
use common::*;
pub fn execute_aarch32_instrs_VRSHRN_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i64,
    esize: i64,
    m: i64,
    shift_amount: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        gs_318166: i64,
        esizeshadow_7767: i64,
        d: i128,
        d__arg: i64,
        elements: i64,
        esize: i64,
        m: i64,
        shift_amount: i128,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
        shift_amount,
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
        // D s_0_3: write-var esizeshadow#7767 <= s_0_2
        fn_state.esizeshadow_7767 = s_0_2;
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
        // D s_1_2: read-var elements:i64
        let s_1_2: i64 = fn_state.elements;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#318166 <= s_1_5
        fn_state.gs_318166 = s_1_5;
        // D s_1_7: write-var e <= s_1_0
        fn_state.e = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#318166:i64
        let s_2_1: i64 = fn_state.gs_318166;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1s : i64
        let s_3_0: i64 = 1;
        // D s_3_1: read-var m:i64
        let s_3_1: i64 = fn_state.m;
        // D s_3_2: lsr s_3_1 s_3_0
        let s_3_2: i64 = s_3_1 >> s_3_0;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: call Qin_read(s_3_3)
        let s_3_4: u128 = Qin_read(state, tracer, s_3_3);
        // C s_3_5: const #2s : i
        let s_3_5: i128 = 2;
        // D s_3_6: read-var esizeshadow#7767:i64
        let s_3_6: i64 = fn_state.esizeshadow_7767;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: mul s_3_5 s_3_7
        let s_3_8: i128 = ((s_3_5) * (s_3_7));
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: cast zx s_3_4 -> bv
        let s_3_12: Bits = Bits::new(s_3_4 as u128, 128u16);
        // D s_3_13: read-var e:i64
        let s_3_13: i64 = fn_state.e;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: cast zx s_3_11 -> i
        let s_3_15: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_16: call Elem_read(s_3_12, s_3_14, s_3_15)
        let s_3_16: Bits = Elem_read(state, tracer, s_3_12, s_3_14, s_3_15);
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (s_3_16.value() as i128);
        // D s_3_18: read-var shift_amount:i
        let s_3_18: i128 = fn_state.shift_amount;
        // C s_3_19: const #1u : u8
        let s_3_19: bool = true;
        // D s_3_20: call RShr(s_3_17, s_3_18, s_3_19)
        let s_3_20: i128 = RShr(state, tracer, s_3_17, s_3_18, s_3_19);
        // D s_3_21: read-var d:i
        let s_3_21: i128 = fn_state.d;
        // D s_3_22: call D_read(s_3_21)
        let s_3_22: u64 = D_read(state, tracer, s_3_21);
        // D s_3_23: read-var esizeshadow#7767:i64
        let s_3_23: i64 = fn_state.esizeshadow_7767;
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: cast reint s_3_24 -> i64
        let s_3_25: i64 = (s_3_24 as i64);
        // C s_3_26: const #1s : i
        let s_3_26: i128 = 1;
        // D s_3_27: read-var esizeshadow#7767:i64
        let s_3_27: i64 = fn_state.esizeshadow_7767;
        // D s_3_28: cast zx s_3_27 -> i
        let s_3_28: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_29: sub s_3_28 s_3_26
        let s_3_29: i128 = ((s_3_28) - (s_3_26));
        // D s_3_30: cast reint s_3_29 -> i64
        let s_3_30: i64 = (s_3_29 as i64);
        // C s_3_31: const #0s : i
        let s_3_31: i128 = 0;
        // D s_3_32: cast zx s_3_30 -> i
        let s_3_32: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_33: call integer_subrange(s_3_20, s_3_32, s_3_31)
        let s_3_33: Bits = integer_subrange(state, tracer, s_3_20, s_3_32, s_3_31);
        // D s_3_34: cast zx s_3_22 -> bv
        let s_3_34: Bits = Bits::new(s_3_22 as u128, 64u16);
        // D s_3_35: read-var e:i64
        let s_3_35: i64 = fn_state.e;
        // D s_3_36: cast zx s_3_35 -> i
        let s_3_36: i128 = (i128::try_from(s_3_35).unwrap());
        // D s_3_37: cast zx s_3_25 -> i
        let s_3_37: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_38: call Elem_set(s_3_34, s_3_36, s_3_37, s_3_33)
        let s_3_38: Bits = Elem_set(state, tracer, s_3_34, s_3_36, s_3_37, s_3_33);
        // D s_3_39: cast reint s_3_38 -> u64
        let s_3_39: u64 = (s_3_38.value() as u64);
        // D s_3_40: read-var d:i
        let s_3_40: i128 = fn_state.d;
        // D s_3_41: call D_set(s_3_40, s_3_39)
        let s_3_41: () = D_set(state, tracer, s_3_40, s_3_39);
        // D s_3_42: read-var e:i64
        let s_3_42: i64 = fn_state.e;
        // C s_3_43: const #1s : i64
        let s_3_43: i64 = 1;
        // D s_3_44: add s_3_42 s_3_43
        let s_3_44: i64 = (s_3_42 + s_3_43);
        // D s_3_45: write-var e <= s_3_44
        fn_state.e = s_3_44;
        // N s_3_46: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: return
        return;
    }
}
