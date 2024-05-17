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
use V_read::*;
use Elem_read::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_transfer_vector_permute_zip<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    esize: i64,
    m: i64,
    n: i64,
    pairs: i128,
    part: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        p: i64,
        base: i128,
        result: Bits,
        esizeshadow_2016: i64,
        operand1: Bits,
        datasizeshadow_2017: i64,
        gs_174432: i64,
        operand2: Bits,
        d: i64,
        datasize: i64,
        esize: i64,
        m: i64,
        n: i64,
        pairs: i128,
        part: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        esize,
        m,
        n,
        pairs,
        part,
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
        // D s_0_3: write-var esizeshadow#2016 <= s_0_2
        fn_state.esizeshadow_2016 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#2017 <= s_0_6
        fn_state.datasizeshadow_2017 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckFPAdvSIMDEnabled64(s_0_8)
        let s_0_9: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#2017:i64
        let s_1_0: i64 = fn_state.datasizeshadow_2017;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call V_read(s_1_4, s_1_2)
        let s_1_5: Bits = V_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: write-var operand1 <= s_1_5
        fn_state.operand1 = s_1_5;
        // D s_1_7: read-var datasizeshadow#2017:i64
        let s_1_7: i64 = fn_state.datasizeshadow_2017;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var m:i64
        let s_1_10: i64 = fn_state.m;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: call V_read(s_1_11, s_1_9)
        let s_1_12: Bits = V_read(state, tracer, s_1_11, s_1_9);
        // D s_1_13: write-var operand2 <= s_1_12
        fn_state.operand2 = s_1_12;
        // D s_1_14: read-var part:i64
        let s_1_14: i64 = fn_state.part;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: read-var pairs:i
        let s_1_16: i128 = fn_state.pairs;
        // D s_1_17: mul s_1_15 s_1_16
        let s_1_17: i128 = ((s_1_15) * (s_1_16));
        // D s_1_18: write-var base <= s_1_17
        fn_state.base = s_1_17;
        // C s_1_19: const #0s : i64
        let s_1_19: i64 = 0;
        // C s_1_20: const #1s : i
        let s_1_20: i128 = 1;
        // D s_1_21: read-var pairs:i
        let s_1_21: i128 = fn_state.pairs;
        // D s_1_22: sub s_1_21 s_1_20
        let s_1_22: i128 = ((s_1_21) - (s_1_20));
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: write-var gs#174432 <= s_1_23
        fn_state.gs_174432 = s_1_23;
        // D s_1_25: write-var p <= s_1_19
        fn_state.p = s_1_19;
        // N s_1_26: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var p:i64
        let s_2_0: i64 = fn_state.p;
        // D s_2_1: read-var gs#174432:i64
        let s_2_1: i64 = fn_state.gs_174432;
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
        // D s_3_1: read-var p:i64
        let s_3_1: i64 = fn_state.p;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // C s_3_4: const #0s : i
        let s_3_4: i128 = 0;
        // D s_3_5: add s_3_3 s_3_4
        let s_3_5: i128 = (s_3_3 + s_3_4);
        // D s_3_6: read-var esizeshadow#2016:i64
        let s_3_6: i64 = fn_state.esizeshadow_2016;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: read-var p:i64
        let s_3_9: i64 = fn_state.p;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: read-var base:i
        let s_3_11: i128 = fn_state.base;
        // D s_3_12: add s_3_11 s_3_10
        let s_3_12: i128 = (s_3_11 + s_3_10);
        // D s_3_13: read-var esizeshadow#2016:i64
        let s_3_13: i64 = fn_state.esizeshadow_2016;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: read-var operand1:bv
        let s_3_17: Bits = fn_state.operand1;
        // D s_3_18: call Elem_read(s_3_17, s_3_12, s_3_16)
        let s_3_18: Bits = Elem_read(state, tracer, s_3_17, s_3_12, s_3_16);
        // D s_3_19: cast zx s_3_8 -> i
        let s_3_19: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_20: read-var result:bv
        let s_3_20: Bits = fn_state.result;
        // D s_3_21: call Elem_set(s_3_20, s_3_5, s_3_19, s_3_18)
        let s_3_21: Bits = Elem_set(state, tracer, s_3_20, s_3_5, s_3_19, s_3_18);
        // D s_3_22: write-var result <= s_3_21
        fn_state.result = s_3_21;
        // C s_3_23: const #2s : i
        let s_3_23: i128 = 2;
        // D s_3_24: read-var p:i64
        let s_3_24: i64 = fn_state.p;
        // D s_3_25: cast zx s_3_24 -> i
        let s_3_25: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_26: mul s_3_23 s_3_25
        let s_3_26: i128 = ((s_3_23) * (s_3_25));
        // C s_3_27: const #1s : i
        let s_3_27: i128 = 1;
        // D s_3_28: add s_3_26 s_3_27
        let s_3_28: i128 = (s_3_26 + s_3_27);
        // D s_3_29: read-var esizeshadow#2016:i64
        let s_3_29: i64 = fn_state.esizeshadow_2016;
        // D s_3_30: cast zx s_3_29 -> i
        let s_3_30: i128 = (i128::try_from(s_3_29).unwrap());
        // D s_3_31: cast reint s_3_30 -> i64
        let s_3_31: i64 = (s_3_30 as i64);
        // D s_3_32: read-var p:i64
        let s_3_32: i64 = fn_state.p;
        // D s_3_33: cast zx s_3_32 -> i
        let s_3_33: i128 = (i128::try_from(s_3_32).unwrap());
        // D s_3_34: read-var base:i
        let s_3_34: i128 = fn_state.base;
        // D s_3_35: add s_3_34 s_3_33
        let s_3_35: i128 = (s_3_34 + s_3_33);
        // D s_3_36: read-var esizeshadow#2016:i64
        let s_3_36: i64 = fn_state.esizeshadow_2016;
        // D s_3_37: cast zx s_3_36 -> i
        let s_3_37: i128 = (i128::try_from(s_3_36).unwrap());
        // D s_3_38: cast reint s_3_37 -> i64
        let s_3_38: i64 = (s_3_37 as i64);
        // D s_3_39: cast zx s_3_38 -> i
        let s_3_39: i128 = (i128::try_from(s_3_38).unwrap());
        // D s_3_40: read-var operand2:bv
        let s_3_40: Bits = fn_state.operand2;
        // D s_3_41: call Elem_read(s_3_40, s_3_35, s_3_39)
        let s_3_41: Bits = Elem_read(state, tracer, s_3_40, s_3_35, s_3_39);
        // D s_3_42: cast zx s_3_31 -> i
        let s_3_42: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_43: read-var result:bv
        let s_3_43: Bits = fn_state.result;
        // D s_3_44: call Elem_set(s_3_43, s_3_28, s_3_42, s_3_41)
        let s_3_44: Bits = Elem_set(state, tracer, s_3_43, s_3_28, s_3_42, s_3_41);
        // D s_3_45: write-var result <= s_3_44
        fn_state.result = s_3_44;
        // D s_3_46: read-var p:i64
        let s_3_46: i64 = fn_state.p;
        // C s_3_47: const #1s : i64
        let s_3_47: i64 = 1;
        // D s_3_48: add s_3_46 s_3_47
        let s_3_48: i64 = (s_3_46 + s_3_47);
        // D s_3_49: write-var p <= s_3_48
        fn_state.p = s_3_48;
        // N s_3_50: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasizeshadow#2017:i64
        let s_4_0: i64 = fn_state.datasizeshadow_2017;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: read-var result:bv
        let s_4_5: Bits = fn_state.result;
        // D s_4_6: call V_set(s_4_4, s_4_2, s_4_5)
        let s_4_6: () = V_set(state, tracer, s_4_4, s_4_2, s_4_5);
        // N s_4_7: return
        return;
    }
}
