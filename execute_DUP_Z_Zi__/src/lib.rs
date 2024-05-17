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
use Z_set::*;
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use Zeros::*;
use replicate_bits_borealis_internal::*;
use common::*;
pub fn execute_DUP_Z_Zi__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    index: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element: Bits,
        esizeshadow_2983: i64,
        operand1: Bits,
        VLshadow_2984: i64,
        VLshadow_2985: i64,
        VL: i64,
        d: i64,
        esize: i64,
        index: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        index,
        n,
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
        // D s_0_3: write-var esizeshadow#2983 <= s_0_2
        fn_state.esizeshadow_2983 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2984 <= s_0_6
        fn_state.VLshadow_2984 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckSVEEnabled(s_0_8)
        let s_0_9: () = CheckSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2984:i64
        let s_1_0: i64 = fn_state.VLshadow_2984;
        // D s_1_1: write-var VLshadow#2985 <= s_1_0
        fn_state.VLshadow_2985 = s_1_0;
        // D s_1_2: read-var VLshadow#2985:i64
        let s_1_2: i64 = fn_state.VLshadow_2985;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#2983:i64
        let s_1_4: i64 = fn_state.esizeshadow_2983;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var VLshadow#2985:i64
        let s_1_8: i64 = fn_state.VLshadow_2985;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var n:i64
        let s_1_11: i64 = fn_state.n;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast zx s_1_10 -> i
        let s_1_13: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_14: call Z_read(s_1_12, s_1_13)
        let s_1_14: Bits = Z_read(state, tracer, s_1_12, s_1_13);
        // D s_1_15: write-var operand1 <= s_1_14
        fn_state.operand1 = s_1_14;
        // D s_1_16: read-var index:i64
        let s_1_16: i64 = fn_state.index;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_7 -> i
        let s_1_18: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_19: cmp-ge s_1_17 s_1_18
        let s_1_19: bool = ((s_1_17) >= (s_1_18));
        // N s_1_20: branch s_1_19 b4 b2
        if s_1_19 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var esizeshadow#2983:i64
        let s_2_0: i64 = fn_state.esizeshadow_2983;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: cast reint s_2_1 -> i64
        let s_2_2: i64 = (s_2_1 as i64);
        // D s_2_3: read-var index:i64
        let s_2_3: i64 = fn_state.index;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: cast zx s_2_2 -> i
        let s_2_5: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_6: read-var operand1:bv
        let s_2_6: Bits = fn_state.operand1;
        // D s_2_7: call Elem_read(s_2_6, s_2_4, s_2_5)
        let s_2_7: Bits = Elem_read(state, tracer, s_2_6, s_2_4, s_2_5);
        // D s_2_8: write-var element <= s_2_7
        fn_state.element = s_2_7;
        // N s_2_9: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var VLshadow#2985:i64
        let s_3_0: i64 = fn_state.VLshadow_2985;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var esizeshadow#2983:i64
        let s_3_2: i64 = fn_state.esizeshadow_2983;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: div s_3_1 s_3_3
        let s_3_4: i128 = ((s_3_1) / (s_3_3));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: read-var element:bv
        let s_3_7: Bits = fn_state.element;
        // D s_3_8: cast reint s_3_6 -> u64
        let s_3_8: u64 = (s_3_6 as u64);
        // D s_3_9: call replicate_bits_borealis_internal(s_3_7, s_3_8)
        let s_3_9: Bits = replicate_bits_borealis_internal(state, tracer, s_3_7, s_3_8);
        // D s_3_10: read-var VLshadow#2985:i64
        let s_3_10: i64 = fn_state.VLshadow_2985;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: cast reint s_3_11 -> i64
        let s_3_12: i64 = (s_3_11 as i64);
        // D s_3_13: read-var d:i64
        let s_3_13: i64 = fn_state.d;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: cast zx s_3_12 -> i
        let s_3_15: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_16: call Z_set(s_3_14, s_3_15, s_3_9)
        let s_3_16: () = Z_set(state, tracer, s_3_14, s_3_15, s_3_9);
        // N s_3_17: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var esizeshadow#2983:i64
        let s_4_0: i64 = fn_state.esizeshadow_2983;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call Zeros(s_4_1)
        let s_4_2: Bits = Zeros(state, tracer, s_4_1);
        // D s_4_3: write-var element <= s_4_2
        fn_state.element = s_4_2;
        // N s_4_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
