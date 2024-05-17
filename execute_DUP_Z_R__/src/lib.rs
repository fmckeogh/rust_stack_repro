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
use X_read::*;
use Elem_set::*;
use CheckSVEEnabled::*;
use SP_read::*;
use Z_set::*;
use common::*;
pub fn execute_DUP_Z_R__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: u64,
        e: i64,
        gs_196728: i64,
        VLshadow_2990: i64,
        esizeshadow_2988: i64,
        VLshadow_2989: i64,
        elements: i64,
        result: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
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
        // D s_0_3: write-var esizeshadow#2988 <= s_0_2
        fn_state.esizeshadow_2988 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2989 <= s_0_6
        fn_state.VLshadow_2989 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2989:i64
        let s_1_0: i64 = fn_state.VLshadow_2989;
        // D s_1_1: write-var VLshadow#2990 <= s_1_0
        fn_state.VLshadow_2990 = s_1_0;
        // D s_1_2: read-var VLshadow#2990:i64
        let s_1_2: i64 = fn_state.VLshadow_2990;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#2988:i64
        let s_1_4: i64 = fn_state.esizeshadow_2988;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: write-var elements <= s_1_7
        fn_state.elements = s_1_7;
        // C s_1_9: const #31s : i
        let s_1_9: i128 = 31;
        // D s_1_10: read-var n:i64
        let s_1_10: i64 = fn_state.n;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: cmp-eq s_1_11 s_1_9
        let s_1_12: bool = ((s_1_11) == (s_1_9));
        // N s_1_13: branch s_1_12 b7 b2
        if s_1_12 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call X_read(s_2_2, s_2_0)
        let s_2_3: Bits = X_read(state, tracer, s_2_2, s_2_0);
        // D s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // D s_2_5: write-var operand <= s_2_4
        fn_state.operand = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
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
        // D s_3_6: write-var gs#196728 <= s_3_5
        fn_state.gs_196728 = s_3_5;
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
        // D s_4_1: read-var gs#196728:i64
        let s_4_1: i64 = fn_state.gs_196728;
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
        // D s_5_0: read-var esizeshadow#2988:i64
        let s_5_0: i64 = fn_state.esizeshadow_2988;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // C s_5_3: const #1s : i
        let s_5_3: i128 = 1;
        // D s_5_4: read-var esizeshadow#2988:i64
        let s_5_4: i64 = fn_state.esizeshadow_2988;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: sub s_5_5 s_5_3
        let s_5_6: i128 = ((s_5_5) - (s_5_3));
        // D s_5_7: cast reint s_5_6 -> i64
        let s_5_7: i64 = (s_5_6 as i64);
        // C s_5_8: const #0s : i
        let s_5_8: i128 = 0;
        // D s_5_9: read-var operand:u64
        let s_5_9: u64 = fn_state.operand;
        // D s_5_10: cast zx s_5_9 -> bv
        let s_5_10: Bits = Bits::new(s_5_9 as u128, 64u16);
        // D s_5_11: cast zx s_5_7 -> i
        let s_5_11: i128 = (i128::try_from(s_5_7).unwrap());
        // C s_5_12: const #1s : i64
        let s_5_12: i64 = 1;
        // C s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: sub s_5_11 s_5_8
        let s_5_14: i128 = ((s_5_11) - (s_5_8));
        // D s_5_15: add s_5_14 s_5_13
        let s_5_15: i128 = (s_5_14 + s_5_13);
        // D s_5_16: bit-extract s_5_10 s_5_8 s_5_15
        let s_5_16: Bits = (Bits::new(
            ((s_5_10) >> (s_5_8)).value(),
            u16::try_from(s_5_15).unwrap(),
        ));
        // D s_5_17: read-var e:i64
        let s_5_17: i64 = fn_state.e;
        // D s_5_18: cast zx s_5_17 -> i
        let s_5_18: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_19: cast zx s_5_2 -> i
        let s_5_19: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_20: read-var result:bv
        let s_5_20: Bits = fn_state.result;
        // D s_5_21: call Elem_set(s_5_20, s_5_18, s_5_19, s_5_16)
        let s_5_21: Bits = Elem_set(state, tracer, s_5_20, s_5_18, s_5_19, s_5_16);
        // D s_5_22: write-var result <= s_5_21
        fn_state.result = s_5_21;
        // D s_5_23: read-var e:i64
        let s_5_23: i64 = fn_state.e;
        // C s_5_24: const #1s : i64
        let s_5_24: i64 = 1;
        // D s_5_25: add s_5_23 s_5_24
        let s_5_25: i64 = (s_5_23 + s_5_24);
        // D s_5_26: write-var e <= s_5_25
        fn_state.e = s_5_25;
        // N s_5_27: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var VLshadow#2990:i64
        let s_6_0: i64 = fn_state.VLshadow_2990;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var d:i64
        let s_6_3: i64 = fn_state.d;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast zx s_6_2 -> i
        let s_6_5: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_6: read-var result:bv
        let s_6_6: Bits = fn_state.result;
        // D s_6_7: call Z_set(s_6_4, s_6_5, s_6_6)
        let s_6_7: () = Z_set(state, tracer, s_6_4, s_6_5, s_6_6);
        // N s_6_8: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call SP_read(s_7_0)
        let s_7_1: u64 = SP_read(state, tracer, s_7_0);
        // D s_7_2: write-var operand <= s_7_1
        fn_state.operand = s_7_1;
        // N s_7_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
