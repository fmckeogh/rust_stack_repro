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
use Vpart_set::*;
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use RShr::*;
use integer_subrange::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_shift_right_narrow_logical<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    n: i64,
    part: i64,
    round: bool,
    shift: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        gs_166838: i64,
        result: Bits,
        esizeshadow_1800: i64,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        n: i64,
        part: i64,
        round: bool,
        shift: i128,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        n,
        part,
        round,
        shift,
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
        // D s_0_3: write-var esizeshadow#1800 <= s_0_2
        fn_state.esizeshadow_1800 = s_0_2;
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
        // C s_1_0: const #2s : i
        let s_1_0: i128 = 2;
        // D s_1_1: read-var datasize:i64
        let s_1_1: i64 = fn_state.datasize;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: mul s_1_2 s_1_0
        let s_1_3: i128 = ((s_1_2) * (s_1_0));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var n:i64
        let s_1_7: i64 = fn_state.n;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call V_read(s_1_8, s_1_6)
        let s_1_9: Bits = V_read(state, tracer, s_1_8, s_1_6);
        // D s_1_10: write-var operand <= s_1_9
        fn_state.operand = s_1_9;
        // C s_1_11: const #0s : i64
        let s_1_11: i64 = 0;
        // C s_1_12: const #1s : i
        let s_1_12: i128 = 1;
        // D s_1_13: read-var elements:i
        let s_1_13: i128 = fn_state.elements;
        // D s_1_14: sub s_1_13 s_1_12
        let s_1_14: i128 = ((s_1_13) - (s_1_12));
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: write-var gs#166838 <= s_1_15
        fn_state.gs_166838 = s_1_15;
        // D s_1_17: write-var e <= s_1_11
        fn_state.e = s_1_11;
        // N s_1_18: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#166838:i64
        let s_2_1: i64 = fn_state.gs_166838;
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
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var esizeshadow#1800:i64
        let s_3_1: i64 = fn_state.esizeshadow_1800;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // D s_3_7: read-var e:i64
        let s_3_7: i64 = fn_state.e;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: cast zx s_3_6 -> i
        let s_3_9: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_10: read-var operand:bv
        let s_3_10: Bits = fn_state.operand;
        // D s_3_11: call Elem_read(s_3_10, s_3_8, s_3_9)
        let s_3_11: Bits = Elem_read(state, tracer, s_3_10, s_3_8, s_3_9);
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (s_3_11.value() as i128);
        // D s_3_13: read-var shift:i
        let s_3_13: i128 = fn_state.shift;
        // D s_3_14: read-var round:u8
        let s_3_14: bool = fn_state.round;
        // D s_3_15: call RShr(s_3_12, s_3_13, s_3_14)
        let s_3_15: i128 = RShr(state, tracer, s_3_12, s_3_13, s_3_14);
        // D s_3_16: read-var esizeshadow#1800:i64
        let s_3_16: i64 = fn_state.esizeshadow_1800;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // C s_3_19: const #1s : i
        let s_3_19: i128 = 1;
        // D s_3_20: read-var esizeshadow#1800:i64
        let s_3_20: i64 = fn_state.esizeshadow_1800;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: sub s_3_21 s_3_19
        let s_3_22: i128 = ((s_3_21) - (s_3_19));
        // D s_3_23: cast reint s_3_22 -> i64
        let s_3_23: i64 = (s_3_22 as i64);
        // C s_3_24: const #0s : i
        let s_3_24: i128 = 0;
        // D s_3_25: cast zx s_3_23 -> i
        let s_3_25: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_26: call integer_subrange(s_3_15, s_3_25, s_3_24)
        let s_3_26: Bits = integer_subrange(state, tracer, s_3_15, s_3_25, s_3_24);
        // D s_3_27: read-var e:i64
        let s_3_27: i64 = fn_state.e;
        // D s_3_28: cast zx s_3_27 -> i
        let s_3_28: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_29: cast zx s_3_18 -> i
        let s_3_29: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_30: read-var result:bv
        let s_3_30: Bits = fn_state.result;
        // D s_3_31: call Elem_set(s_3_30, s_3_28, s_3_29, s_3_26)
        let s_3_31: Bits = Elem_set(state, tracer, s_3_30, s_3_28, s_3_29, s_3_26);
        // D s_3_32: write-var result <= s_3_31
        fn_state.result = s_3_31;
        // D s_3_33: read-var e:i64
        let s_3_33: i64 = fn_state.e;
        // C s_3_34: const #1s : i64
        let s_3_34: i64 = 1;
        // D s_3_35: add s_3_33 s_3_34
        let s_3_35: i64 = (s_3_33 + s_3_34);
        // D s_3_36: write-var e <= s_3_35
        fn_state.e = s_3_35;
        // N s_3_37: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasize:i64
        let s_4_0: i64 = fn_state.datasize;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: read-var part:i64
        let s_4_5: i64 = fn_state.part;
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: cast zx s_4_2 -> i
        let s_4_7: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_8: read-var result:bv
        let s_4_8: Bits = fn_state.result;
        // D s_4_9: call Vpart_set(s_4_4, s_4_6, s_4_7, s_4_8)
        let s_4_9: () = Vpart_set(state, tracer, s_4_4, s_4_6, s_4_7, s_4_8);
        // N s_4_10: return
        return;
    }
}
