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
use CheckAdvSIMDEnabled::*;
use u_shl_int_general::*;
use integer_subrange::*;
use Q_set::*;
use Din_read::*;
use asl_Int::*;
use Elem_read::*;
use common::*;
pub fn execute_aarch32_instrs_VSHLL_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
    shift_amount: i128,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_7799: i64,
        e: i64,
        d: i128,
        gs_318865: i64,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        shift_amount: i128,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
        shift_amount,
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
        // D s_0_3: write-var esizeshadow#7799 <= s_0_2
        fn_state.esizeshadow_7799 = s_0_2;
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
        // D s_1_2: read-var elements:i
        let s_1_2: i128 = fn_state.elements;
        // D s_1_3: sub s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) - (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: write-var gs#318865 <= s_1_4
        fn_state.gs_318865 = s_1_4;
        // D s_1_6: write-var e <= s_1_0
        fn_state.e = s_1_0;
        // N s_1_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#318865:i64
        let s_2_1: i64 = fn_state.gs_318865;
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
        // D s_3_0: read-var m:i64
        let s_3_0: i64 = fn_state.m;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call Din_read(s_3_1)
        let s_3_2: u64 = Din_read(state, tracer, s_3_1);
        // D s_3_3: read-var esizeshadow#7799:i64
        let s_3_3: i64 = fn_state.esizeshadow_7799;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: cast zx s_3_2 -> bv
        let s_3_6: Bits = Bits::new(s_3_2 as u128, 64u16);
        // D s_3_7: read-var e:i64
        let s_3_7: i64 = fn_state.e;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: cast zx s_3_5 -> i
        let s_3_9: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_10: call Elem_read(s_3_6, s_3_8, s_3_9)
        let s_3_10: Bits = Elem_read(state, tracer, s_3_6, s_3_8, s_3_9);
        // D s_3_11: read-var is_unsigned:u8
        let s_3_11: bool = fn_state.is_unsigned;
        // D s_3_12: call asl_Int(s_3_10, s_3_11)
        let s_3_12: i128 = asl_Int(state, tracer, s_3_10, s_3_11);
        // D s_3_13: read-var shift_amount:i
        let s_3_13: i128 = fn_state.shift_amount;
        // D s_3_14: call _shl_int_general(s_3_12, s_3_13)
        let s_3_14: i128 = u_shl_int_general(state, tracer, s_3_12, s_3_13);
        // C s_3_15: const #1s : i
        let s_3_15: i128 = 1;
        // D s_3_16: read-var d:i
        let s_3_16: i128 = fn_state.d;
        // D s_3_17: lsr s_3_16 s_3_15
        let s_3_17: i128 = s_3_16 >> s_3_15;
        // C s_3_18: const #1s : i
        let s_3_18: i128 = 1;
        // D s_3_19: read-var d:i
        let s_3_19: i128 = fn_state.d;
        // D s_3_20: lsr s_3_19 s_3_18
        let s_3_20: i128 = s_3_19 >> s_3_18;
        // D s_3_21: call Q_read(s_3_20)
        let s_3_21: u128 = Q_read(state, tracer, s_3_20);
        // C s_3_22: const #2s : i
        let s_3_22: i128 = 2;
        // D s_3_23: read-var esizeshadow#7799:i64
        let s_3_23: i64 = fn_state.esizeshadow_7799;
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: mul s_3_22 s_3_24
        let s_3_25: i128 = ((s_3_22) * (s_3_24));
        // D s_3_26: cast reint s_3_25 -> i64
        let s_3_26: i64 = (s_3_25 as i64);
        // D s_3_27: cast zx s_3_26 -> i
        let s_3_27: i128 = (i128::try_from(s_3_26).unwrap());
        // D s_3_28: cast reint s_3_27 -> i64
        let s_3_28: i64 = (s_3_27 as i64);
        // C s_3_29: const #2s : i
        let s_3_29: i128 = 2;
        // D s_3_30: read-var esizeshadow#7799:i64
        let s_3_30: i64 = fn_state.esizeshadow_7799;
        // D s_3_31: cast zx s_3_30 -> i
        let s_3_31: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_32: mul s_3_29 s_3_31
        let s_3_32: i128 = ((s_3_29) * (s_3_31));
        // D s_3_33: cast reint s_3_32 -> i64
        let s_3_33: i64 = (s_3_32 as i64);
        // C s_3_34: const #1s : i
        let s_3_34: i128 = 1;
        // D s_3_35: cast zx s_3_33 -> i
        let s_3_35: i128 = (i128::try_from(s_3_33).unwrap());
        // D s_3_36: sub s_3_35 s_3_34
        let s_3_36: i128 = ((s_3_35) - (s_3_34));
        // D s_3_37: cast reint s_3_36 -> i64
        let s_3_37: i64 = (s_3_36 as i64);
        // C s_3_38: const #0s : i
        let s_3_38: i128 = 0;
        // D s_3_39: cast zx s_3_37 -> i
        let s_3_39: i128 = (i128::try_from(s_3_37).unwrap());
        // D s_3_40: call integer_subrange(s_3_14, s_3_39, s_3_38)
        let s_3_40: Bits = integer_subrange(state, tracer, s_3_14, s_3_39, s_3_38);
        // D s_3_41: cast zx s_3_21 -> bv
        let s_3_41: Bits = Bits::new(s_3_21 as u128, 128u16);
        // D s_3_42: read-var e:i64
        let s_3_42: i64 = fn_state.e;
        // D s_3_43: cast zx s_3_42 -> i
        let s_3_43: i128 = (i128::try_from(s_3_42).unwrap());
        // D s_3_44: cast zx s_3_28 -> i
        let s_3_44: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_45: call Elem_set(s_3_41, s_3_43, s_3_44, s_3_40)
        let s_3_45: Bits = Elem_set(state, tracer, s_3_41, s_3_43, s_3_44, s_3_40);
        // D s_3_46: cast reint s_3_45 -> u128
        let s_3_46: u128 = (s_3_45.value() as u128);
        // D s_3_47: call Q_set(s_3_17, s_3_46)
        let s_3_47: () = Q_set(state, tracer, s_3_17, s_3_46);
        // D s_3_48: read-var e:i64
        let s_3_48: i64 = fn_state.e;
        // C s_3_49: const #1s : i64
        let s_3_49: i64 = 1;
        // D s_3_50: add s_3_48 s_3_49
        let s_3_50: i64 = (s_3_48 + s_3_49);
        // D s_3_51: write-var e <= s_3_50
        fn_state.e = s_3_50;
        // N s_3_52: jump b2
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
