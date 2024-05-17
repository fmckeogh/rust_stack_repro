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
use Vpart_read::*;
use Elem_set::*;
use V_set::*;
use u_shl_int_general::*;
use integer_subrange::*;
use asl_Int::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_shift_left_long<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    n: i64,
    part: i64,
    shift: i128,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        esizeshadow_1981: i64,
        e: i64,
        result: Bits,
        gs_172943: i64,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        n: i64,
        part: i64,
        shift: i128,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        n,
        part,
        shift,
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
        // D s_0_3: write-var esizeshadow#1981 <= s_0_2
        fn_state.esizeshadow_1981 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckFPAdvSIMDEnabled64(s_0_4)
        let s_0_5: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasize:i64
        let s_1_0: i64 = fn_state.datasize;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: read-var part:i64
        let s_1_5: i64 = fn_state.part;
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_7: cast zx s_1_2 -> i
        let s_1_7: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_8: call Vpart_read(s_1_4, s_1_6, s_1_7)
        let s_1_8: Bits = Vpart_read(state, tracer, s_1_4, s_1_6, s_1_7);
        // D s_1_9: write-var operand <= s_1_8
        fn_state.operand = s_1_8;
        // C s_1_10: const #0s : i64
        let s_1_10: i64 = 0;
        // C s_1_11: const #1s : i
        let s_1_11: i128 = 1;
        // D s_1_12: read-var elements:i
        let s_1_12: i128 = fn_state.elements;
        // D s_1_13: sub s_1_12 s_1_11
        let s_1_13: i128 = ((s_1_12) - (s_1_11));
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: write-var gs#172943 <= s_1_14
        fn_state.gs_172943 = s_1_14;
        // D s_1_16: write-var e <= s_1_10
        fn_state.e = s_1_10;
        // N s_1_17: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#172943:i64
        let s_2_1: i64 = fn_state.gs_172943;
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
        // D s_3_0: read-var esizeshadow#1981:i64
        let s_3_0: i64 = fn_state.esizeshadow_1981;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var operand:bv
        let s_3_6: Bits = fn_state.operand;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: read-var is_unsigned:u8
        let s_3_8: bool = fn_state.is_unsigned;
        // D s_3_9: call asl_Int(s_3_7, s_3_8)
        let s_3_9: i128 = asl_Int(state, tracer, s_3_7, s_3_8);
        // D s_3_10: read-var shift:i
        let s_3_10: i128 = fn_state.shift;
        // D s_3_11: call _shl_int_general(s_3_9, s_3_10)
        let s_3_11: i128 = u_shl_int_general(state, tracer, s_3_9, s_3_10);
        // C s_3_12: const #2s : i
        let s_3_12: i128 = 2;
        // D s_3_13: read-var esizeshadow#1981:i64
        let s_3_13: i64 = fn_state.esizeshadow_1981;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: mul s_3_12 s_3_14
        let s_3_15: i128 = ((s_3_12) * (s_3_14));
        // D s_3_16: cast reint s_3_15 -> i64
        let s_3_16: i64 = (s_3_15 as i64);
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // C s_3_19: const #2s : i
        let s_3_19: i128 = 2;
        // D s_3_20: read-var esizeshadow#1981:i64
        let s_3_20: i64 = fn_state.esizeshadow_1981;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: mul s_3_19 s_3_21
        let s_3_22: i128 = ((s_3_19) * (s_3_21));
        // D s_3_23: cast reint s_3_22 -> i64
        let s_3_23: i64 = (s_3_22 as i64);
        // C s_3_24: const #1s : i
        let s_3_24: i128 = 1;
        // D s_3_25: cast zx s_3_23 -> i
        let s_3_25: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_26: sub s_3_25 s_3_24
        let s_3_26: i128 = ((s_3_25) - (s_3_24));
        // D s_3_27: cast reint s_3_26 -> i64
        let s_3_27: i64 = (s_3_26 as i64);
        // C s_3_28: const #0s : i
        let s_3_28: i128 = 0;
        // D s_3_29: cast zx s_3_27 -> i
        let s_3_29: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_30: call integer_subrange(s_3_11, s_3_29, s_3_28)
        let s_3_30: Bits = integer_subrange(state, tracer, s_3_11, s_3_29, s_3_28);
        // D s_3_31: read-var e:i64
        let s_3_31: i64 = fn_state.e;
        // D s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_33: cast zx s_3_18 -> i
        let s_3_33: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_34: read-var result:bv
        let s_3_34: Bits = fn_state.result;
        // D s_3_35: call Elem_set(s_3_34, s_3_32, s_3_33, s_3_30)
        let s_3_35: Bits = Elem_set(state, tracer, s_3_34, s_3_32, s_3_33, s_3_30);
        // D s_3_36: write-var result <= s_3_35
        fn_state.result = s_3_35;
        // D s_3_37: read-var e:i64
        let s_3_37: i64 = fn_state.e;
        // C s_3_38: const #1s : i64
        let s_3_38: i64 = 1;
        // D s_3_39: add s_3_37 s_3_38
        let s_3_39: i64 = (s_3_37 + s_3_38);
        // D s_3_40: write-var e <= s_3_39
        fn_state.e = s_3_39;
        // N s_3_41: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2s : i
        let s_4_0: i128 = 2;
        // D s_4_1: read-var datasize:i64
        let s_4_1: i64 = fn_state.datasize;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: mul s_4_2 s_4_0
        let s_4_3: i128 = ((s_4_2) * (s_4_0));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: cast reint s_4_5 -> i64
        let s_4_6: i64 = (s_4_5 as i64);
        // D s_4_7: read-var d:i64
        let s_4_7: i64 = fn_state.d;
        // D s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (i128::try_from(s_4_7).unwrap());
        // D s_4_9: read-var result:bv
        let s_4_9: Bits = fn_state.result;
        // D s_4_10: call V_set(s_4_8, s_4_6, s_4_9)
        let s_4_10: () = V_set(state, tracer, s_4_8, s_4_6, s_4_9);
        // N s_4_11: return
        return;
    }
}
