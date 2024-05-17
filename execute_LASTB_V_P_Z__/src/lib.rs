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
use LastActiveElement::*;
use V_set::*;
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use P_read::*;
use common::*;
pub fn execute_LASTB_V_P_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    isBefore: bool,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        VLshadow_3006: i64,
        esizeshadow_3005: i64,
        last: i128,
        elements: i64,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        isBefore: bool,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
        isBefore,
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
        // D s_0_3: write-var esizeshadow#3005 <= s_0_2
        fn_state.esizeshadow_3005 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3006 <= s_0_6
        fn_state.VLshadow_3006 = s_0_6;
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
        // D s_1_0: read-var VLshadow#3006:i64
        let s_1_0: i64 = fn_state.VLshadow_3006;
        // C s_1_1: const #8s : i
        let s_1_1: i128 = 8;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: div s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) / (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: cast zx s_1_0 -> i
        let s_1_5: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_6: read-var esizeshadow#3005:i64
        let s_1_6: i64 = fn_state.esizeshadow_3005;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: div s_1_5 s_1_7
        let s_1_8: i128 = ((s_1_5) / (s_1_7));
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: write-var elements <= s_1_9
        fn_state.elements = s_1_9;
        // D s_1_11: cast zx s_1_4 -> i
        let s_1_11: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: read-var g:i64
        let s_1_13: i64 = fn_state.g;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: cast zx s_1_12 -> i
        let s_1_15: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_16: call P_read(s_1_14, s_1_15)
        let s_1_16: Bits = P_read(state, tracer, s_1_14, s_1_15);
        // D s_1_17: cast zx s_1_0 -> i
        let s_1_17: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: read-var n:i64
        let s_1_19: i64 = fn_state.n;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast zx s_1_18 -> i
        let s_1_21: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_22: call Z_read(s_1_20, s_1_21)
        let s_1_22: Bits = Z_read(state, tracer, s_1_20, s_1_21);
        // D s_1_23: write-var operand <= s_1_22
        fn_state.operand = s_1_22;
        // D s_1_24: read-var esizeshadow#3005:i64
        let s_1_24: i64 = fn_state.esizeshadow_3005;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: call LastActiveElement(s_1_16, s_1_25)
        let s_1_26: i128 = LastActiveElement(state, tracer, s_1_16, s_1_25);
        // D s_1_27: write-var last <= s_1_26
        fn_state.last = s_1_26;
        // D s_1_28: read-var isBefore:u8
        let s_1_28: bool = fn_state.isBefore;
        // N s_1_29: branch s_1_28 b7 b2
        if s_1_28 {
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
        // C s_2_0: const #1s : i
        let s_2_0: i128 = 1;
        // D s_2_1: read-var last:i
        let s_2_1: i128 = fn_state.last;
        // D s_2_2: add s_2_1 s_2_0
        let s_2_2: i128 = (s_2_1 + s_2_0);
        // D s_2_3: write-var last <= s_2_2
        fn_state.last = s_2_2;
        // D s_2_4: read-var elements:i64
        let s_2_4: i64 = fn_state.elements;
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_6: read-var last:i
        let s_2_6: i128 = fn_state.last;
        // D s_2_7: cmp-ge s_2_6 s_2_5
        let s_2_7: bool = ((s_2_6) >= (s_2_5));
        // N s_2_8: branch s_2_7 b6 b3
        if s_2_7 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var last:i
        let s_5_0: i128 = fn_state.last;
        // D s_5_1: read-var esizeshadow#3005:i64
        let s_5_1: i64 = fn_state.esizeshadow_3005;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: read-var esizeshadow#3005:i64
        let s_5_4: i64 = fn_state.esizeshadow_3005;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: cast reint s_5_5 -> i64
        let s_5_6: i64 = (s_5_5 as i64);
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: read-var operand:bv
        let s_5_8: Bits = fn_state.operand;
        // D s_5_9: call Elem_read(s_5_8, s_5_0, s_5_7)
        let s_5_9: Bits = Elem_read(state, tracer, s_5_8, s_5_0, s_5_7);
        // D s_5_10: read-var d:i64
        let s_5_10: i64 = fn_state.d;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: call V_set(s_5_11, s_5_3, s_5_9)
        let s_5_12: () = V_set(state, tracer, s_5_11, s_5_3, s_5_9);
        // N s_5_13: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0s : i
        let s_6_0: i128 = 0;
        // D s_6_1: write-var last <= s_6_0
        fn_state.last = s_6_0;
        // N s_6_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0s : i
        let s_7_0: i128 = 0;
        // D s_7_1: read-var last:i
        let s_7_1: i128 = fn_state.last;
        // D s_7_2: cmp-lt s_7_1 s_7_0
        let s_7_2: bool = ((s_7_1) < (s_7_0));
        // N s_7_3: branch s_7_2 b10 b8
        if s_7_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1s : i
        let s_10_0: i128 = 1;
        // D s_10_1: read-var elements:i64
        let s_10_1: i64 = fn_state.elements;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: sub s_10_2 s_10_0
        let s_10_3: i128 = ((s_10_2) - (s_10_0));
        // D s_10_4: write-var last <= s_10_3
        fn_state.last = s_10_3;
        // N s_10_5: jump b9
        return block_9(state, tracer, fn_state);
    }
}
