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
use u__id::*;
use CheckAdvSIMDEnabled::*;
use integer_subrange::*;
use Q_set::*;
use Din_read::*;
use asl_Int::*;
use Elem_read::*;
use common::*;
pub fn execute_aarch32_instrs_VMOVL_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        gs_313055: i64,
        d: i128,
        gs_313062: bool,
        esizeshadow_7549: i64,
        gs_313063: bool,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
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
        // D s_0_3: write-var esizeshadow#7549 <= s_0_2
        fn_state.esizeshadow_7549 = s_0_2;
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
        // D s_1_5: write-var gs#313055 <= s_1_4
        fn_state.gs_313055 = s_1_4;
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
        // D s_2_1: read-var gs#313055:i64
        let s_2_1: i64 = fn_state.gs_313055;
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
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call __id(s_3_1)
        let s_3_2: i128 = u__id(state, tracer, s_3_1);
        // D s_3_3: read-var esizeshadow#7549:i64
        let s_3_3: i64 = fn_state.esizeshadow_7549;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: call __id(s_3_4)
        let s_3_5: i128 = u__id(state, tracer, s_3_4);
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: mul s_3_2 s_3_7
        let s_3_8: i128 = ((s_3_2) * (s_3_7));
        // D s_3_9: read-var e:i64
        let s_3_9: i64 = fn_state.e;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: call __id(s_3_10)
        let s_3_11: i128 = u__id(state, tracer, s_3_10);
        // D s_3_12: read-var esizeshadow#7549:i64
        let s_3_12: i64 = fn_state.esizeshadow_7549;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: call __id(s_3_13)
        let s_3_14: i128 = u__id(state, tracer, s_3_13);
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: mul s_3_11 s_3_16
        let s_3_17: i128 = ((s_3_11) * (s_3_16));
        // D s_3_18: read-var esizeshadow#7549:i64
        let s_3_18: i64 = fn_state.esizeshadow_7549;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: call __id(s_3_19)
        let s_3_20: i128 = u__id(state, tracer, s_3_19);
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // D s_3_22: cast zx s_3_21 -> i
        let s_3_22: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_23: add s_3_17 s_3_22
        let s_3_23: i128 = (s_3_17 + s_3_22);
        // C s_3_24: const #1s : i
        let s_3_24: i128 = 1;
        // D s_3_25: sub s_3_23 s_3_24
        let s_3_25: i128 = ((s_3_23) - (s_3_24));
        // D s_3_26: cmp-le s_3_8 s_3_25
        let s_3_26: bool = ((s_3_8) <= (s_3_25));
        // N s_3_27: branch s_3_26 b9 b4
        if s_3_26 {
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
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call __id(s_4_1)
        let s_4_2: i128 = u__id(state, tracer, s_4_1);
        // C s_4_3: const #0s : i
        let s_4_3: i128 = 0;
        // D s_4_4: cmp-ge s_4_2 s_4_3
        let s_4_4: bool = ((s_4_2) >= (s_4_3));
        // N s_4_5: branch s_4_4 b8 b5
        if s_4_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#313062 <= s_5_0
        fn_state.gs_313062 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#313062:u8
        let s_6_0: bool = fn_state.gs_313062;
        // D s_6_1: not s_6_0
        let s_6_1: bool = !s_6_0;
        // D s_6_2: write-var gs#313063 <= s_6_1
        fn_state.gs_313063 = s_6_1;
        // N s_6_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#313063:u8
        let s_7_0: bool = fn_state.gs_313063;
        // N s_7_1: assert s_7_0
        let s_7_1: () = assert!(s_7_0);
        // D s_7_2: read-var m:i64
        let s_7_2: i64 = fn_state.m;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: call Din_read(s_7_3)
        let s_7_4: u64 = Din_read(state, tracer, s_7_3);
        // D s_7_5: read-var esizeshadow#7549:i64
        let s_7_5: i64 = fn_state.esizeshadow_7549;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: cast reint s_7_6 -> i64
        let s_7_7: i64 = (s_7_6 as i64);
        // D s_7_8: cast zx s_7_4 -> bv
        let s_7_8: Bits = Bits::new(s_7_4 as u128, 64u16);
        // D s_7_9: read-var e:i64
        let s_7_9: i64 = fn_state.e;
        // D s_7_10: cast zx s_7_9 -> i
        let s_7_10: i128 = (i128::try_from(s_7_9).unwrap());
        // D s_7_11: cast zx s_7_7 -> i
        let s_7_11: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_12: call Elem_read(s_7_8, s_7_10, s_7_11)
        let s_7_12: Bits = Elem_read(state, tracer, s_7_8, s_7_10, s_7_11);
        // D s_7_13: read-var is_unsigned:u8
        let s_7_13: bool = fn_state.is_unsigned;
        // D s_7_14: call asl_Int(s_7_12, s_7_13)
        let s_7_14: i128 = asl_Int(state, tracer, s_7_12, s_7_13);
        // C s_7_15: const #1s : i
        let s_7_15: i128 = 1;
        // D s_7_16: read-var d:i
        let s_7_16: i128 = fn_state.d;
        // D s_7_17: lsr s_7_16 s_7_15
        let s_7_17: i128 = s_7_16 >> s_7_15;
        // C s_7_18: const #1s : i
        let s_7_18: i128 = 1;
        // D s_7_19: read-var d:i
        let s_7_19: i128 = fn_state.d;
        // D s_7_20: lsr s_7_19 s_7_18
        let s_7_20: i128 = s_7_19 >> s_7_18;
        // D s_7_21: call Q_read(s_7_20)
        let s_7_21: u128 = Q_read(state, tracer, s_7_20);
        // C s_7_22: const #2s : i
        let s_7_22: i128 = 2;
        // D s_7_23: read-var esizeshadow#7549:i64
        let s_7_23: i64 = fn_state.esizeshadow_7549;
        // D s_7_24: cast zx s_7_23 -> i
        let s_7_24: i128 = (i128::try_from(s_7_23).unwrap());
        // D s_7_25: mul s_7_22 s_7_24
        let s_7_25: i128 = ((s_7_22) * (s_7_24));
        // D s_7_26: cast reint s_7_25 -> i64
        let s_7_26: i64 = (s_7_25 as i64);
        // D s_7_27: cast zx s_7_26 -> i
        let s_7_27: i128 = (i128::try_from(s_7_26).unwrap());
        // D s_7_28: cast reint s_7_27 -> i64
        let s_7_28: i64 = (s_7_27 as i64);
        // C s_7_29: const #2s : i
        let s_7_29: i128 = 2;
        // D s_7_30: read-var esizeshadow#7549:i64
        let s_7_30: i64 = fn_state.esizeshadow_7549;
        // D s_7_31: cast zx s_7_30 -> i
        let s_7_31: i128 = (i128::try_from(s_7_30).unwrap());
        // D s_7_32: mul s_7_29 s_7_31
        let s_7_32: i128 = ((s_7_29) * (s_7_31));
        // D s_7_33: cast reint s_7_32 -> i64
        let s_7_33: i64 = (s_7_32 as i64);
        // C s_7_34: const #1s : i
        let s_7_34: i128 = 1;
        // D s_7_35: cast zx s_7_33 -> i
        let s_7_35: i128 = (i128::try_from(s_7_33).unwrap());
        // D s_7_36: sub s_7_35 s_7_34
        let s_7_36: i128 = ((s_7_35) - (s_7_34));
        // D s_7_37: cast reint s_7_36 -> i64
        let s_7_37: i64 = (s_7_36 as i64);
        // C s_7_38: const #0s : i
        let s_7_38: i128 = 0;
        // D s_7_39: cast zx s_7_37 -> i
        let s_7_39: i128 = (i128::try_from(s_7_37).unwrap());
        // D s_7_40: call integer_subrange(s_7_14, s_7_39, s_7_38)
        let s_7_40: Bits = integer_subrange(state, tracer, s_7_14, s_7_39, s_7_38);
        // D s_7_41: cast zx s_7_21 -> bv
        let s_7_41: Bits = Bits::new(s_7_21 as u128, 128u16);
        // D s_7_42: read-var e:i64
        let s_7_42: i64 = fn_state.e;
        // D s_7_43: cast zx s_7_42 -> i
        let s_7_43: i128 = (i128::try_from(s_7_42).unwrap());
        // D s_7_44: cast zx s_7_28 -> i
        let s_7_44: i128 = (i128::try_from(s_7_28).unwrap());
        // D s_7_45: call Elem_set(s_7_41, s_7_43, s_7_44, s_7_40)
        let s_7_45: Bits = Elem_set(state, tracer, s_7_41, s_7_43, s_7_44, s_7_40);
        // D s_7_46: cast reint s_7_45 -> u128
        let s_7_46: u128 = (s_7_45.value() as u128);
        // D s_7_47: call Q_set(s_7_17, s_7_46)
        let s_7_47: () = Q_set(state, tracer, s_7_17, s_7_46);
        // D s_7_48: read-var e:i64
        let s_7_48: i64 = fn_state.e;
        // C s_7_49: const #1s : i64
        let s_7_49: i64 = 1;
        // D s_7_50: add s_7_48 s_7_49
        let s_7_50: i64 = (s_7_48 + s_7_49);
        // D s_7_51: write-var e <= s_7_50
        fn_state.e = s_7_50;
        // N s_7_52: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var e:i64
        let s_8_0: i64 = fn_state.e;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call __id(s_8_1)
        let s_8_2: i128 = u__id(state, tracer, s_8_1);
        // C s_8_3: const #1s : i
        let s_8_3: i128 = 1;
        // D s_8_4: add s_8_2 s_8_3
        let s_8_4: i128 = (s_8_2 + s_8_3);
        // D s_8_5: read-var esizeshadow#7549:i64
        let s_8_5: i64 = fn_state.esizeshadow_7549;
        // D s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_7: call __id(s_8_6)
        let s_8_7: i128 = u__id(state, tracer, s_8_6);
        // D s_8_8: cast reint s_8_7 -> i64
        let s_8_8: i64 = (s_8_7 as i64);
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: mul s_8_4 s_8_9
        let s_8_10: i128 = ((s_8_4) * (s_8_9));
        // C s_8_11: const #64s : i
        let s_8_11: i128 = 64;
        // D s_8_12: cmp-le s_8_10 s_8_11
        let s_8_12: bool = ((s_8_10) <= (s_8_11));
        // D s_8_13: write-var gs#313062 <= s_8_12
        fn_state.gs_313062 = s_8_12;
        // N s_8_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#313063 <= s_9_0
        fn_state.gs_313063 = s_9_0;
        // N s_9_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: return
        return;
    }
}
