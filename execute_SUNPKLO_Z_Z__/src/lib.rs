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
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_SUNPKLO_Z_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    hi: bool,
    n: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        esizeshadow_3026: i64,
        e: i64,
        ga_284998: i64,
        hsize: i64,
        ga_284999: Bits,
        gs_197336: i64,
        elements: i64,
        VLshadow_3027: i64,
        VLshadow_3028: i64,
        element: Bits,
        result: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        hi: bool,
        n: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        hi,
        n,
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
        // D s_0_3: write-var esizeshadow#3026 <= s_0_2
        fn_state.esizeshadow_3026 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3027 <= s_0_6
        fn_state.VLshadow_3027 = s_0_6;
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
        // D s_1_0: read-var VLshadow#3027:i64
        let s_1_0: i64 = fn_state.VLshadow_3027;
        // D s_1_1: write-var VLshadow#3028 <= s_1_0
        fn_state.VLshadow_3028 = s_1_0;
        // D s_1_2: read-var VLshadow#3028:i64
        let s_1_2: i64 = fn_state.VLshadow_3028;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#3026:i64
        let s_1_4: i64 = fn_state.esizeshadow_3026;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: write-var elements <= s_1_7
        fn_state.elements = s_1_7;
        // C s_1_9: const #2s : i
        let s_1_9: i128 = 2;
        // D s_1_10: read-var esizeshadow#3026:i64
        let s_1_10: i64 = fn_state.esizeshadow_3026;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: div s_1_11 s_1_9
        let s_1_12: i128 = ((s_1_11) / (s_1_9));
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var hsize <= s_1_13
        fn_state.hsize = s_1_13;
        // D s_1_15: read-var VLshadow#3028:i64
        let s_1_15: i64 = fn_state.VLshadow_3028;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: read-var n:i64
        let s_1_18: i64 = fn_state.n;
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: cast zx s_1_17 -> i
        let s_1_20: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_21: call Z_read(s_1_19, s_1_20)
        let s_1_21: Bits = Z_read(state, tracer, s_1_19, s_1_20);
        // D s_1_22: write-var operand <= s_1_21
        fn_state.operand = s_1_21;
        // C s_1_23: const #0s : i64
        let s_1_23: i64 = 0;
        // C s_1_24: const #1s : i
        let s_1_24: i128 = 1;
        // D s_1_25: read-var elements:i64
        let s_1_25: i64 = fn_state.elements;
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_27: sub s_1_26 s_1_24
        let s_1_27: i128 = ((s_1_26) - (s_1_24));
        // D s_1_28: cast reint s_1_27 -> i64
        let s_1_28: i64 = (s_1_27 as i64);
        // D s_1_29: write-var gs#197336 <= s_1_28
        fn_state.gs_197336 = s_1_28;
        // D s_1_30: write-var e <= s_1_23
        fn_state.e = s_1_23;
        // N s_1_31: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#197336:i64
        let s_2_1: i64 = fn_state.gs_197336;
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
        // D s_3_0: read-var hi:u8
        let s_3_0: bool = fn_state.hi;
        // N s_3_1: branch s_3_0 b9 b4
        if s_3_0 {
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
        // D s_4_0: read-var hsize:i64
        let s_4_0: i64 = fn_state.hsize;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var e:i64
        let s_4_3: i64 = fn_state.e;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: read-var operand:bv
        let s_4_6: Bits = fn_state.operand;
        // D s_4_7: call Elem_read(s_4_6, s_4_4, s_4_5)
        let s_4_7: Bits = Elem_read(state, tracer, s_4_6, s_4_4, s_4_5);
        // D s_4_8: write-var element <= s_4_7
        fn_state.element = s_4_7;
        // N s_4_9: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#3026:i64
        let s_5_0: i64 = fn_state.esizeshadow_3026;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: write-var ga#284998 <= s_5_2
        fn_state.ga_284998 = s_5_2;
        // D s_5_4: read-var is_unsigned:u8
        let s_5_4: bool = fn_state.is_unsigned;
        // N s_5_5: branch s_5_4 b8 b6
        if s_5_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#3026:i64
        let s_6_0: i64 = fn_state.esizeshadow_3026;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var element:bv
        let s_6_2: Bits = fn_state.element;
        // D s_6_3: bits-cast sx s_6_2 -> bv length s_6_1
        let s_6_3: Bits = s_6_2.sign_extend(s_6_1);
        // D s_6_4: write-var ga#284999 <= s_6_3
        fn_state.ga_284999 = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var ga#284998:i64
        let s_7_2: i64 = fn_state.ga_284998;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var result:bv
        let s_7_4: Bits = fn_state.result;
        // D s_7_5: read-var ga#284999:bv
        let s_7_5: Bits = fn_state.ga_284999;
        // D s_7_6: call Elem_set(s_7_4, s_7_1, s_7_3, s_7_5)
        let s_7_6: Bits = Elem_set(state, tracer, s_7_4, s_7_1, s_7_3, s_7_5);
        // D s_7_7: write-var result <= s_7_6
        fn_state.result = s_7_6;
        // D s_7_8: read-var e:i64
        let s_7_8: i64 = fn_state.e;
        // C s_7_9: const #1s : i64
        let s_7_9: i64 = 1;
        // D s_7_10: add s_7_8 s_7_9
        let s_7_10: i64 = (s_7_8 + s_7_9);
        // D s_7_11: write-var e <= s_7_10
        fn_state.e = s_7_10;
        // N s_7_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esizeshadow#3026:i64
        let s_8_0: i64 = fn_state.esizeshadow_3026;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var element:bv
        let s_8_2: Bits = fn_state.element;
        // D s_8_3: bits-cast zx s_8_2 -> bv length s_8_1
        let s_8_3: Bits = s_8_2.zero_extend(s_8_1);
        // D s_8_4: write-var ga#284999 <= s_8_3
        fn_state.ga_284999 = s_8_3;
        // N s_8_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var e:i64
        let s_9_0: i64 = fn_state.e;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var elements:i64
        let s_9_2: i64 = fn_state.elements;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: add s_9_1 s_9_3
        let s_9_4: i128 = (s_9_1 + s_9_3);
        // D s_9_5: cast reint s_9_4 -> i64
        let s_9_5: i64 = (s_9_4 as i64);
        // D s_9_6: read-var hsize:i64
        let s_9_6: i64 = fn_state.hsize;
        // D s_9_7: cast zx s_9_6 -> i
        let s_9_7: i128 = (i128::try_from(s_9_6).unwrap());
        // D s_9_8: cast reint s_9_7 -> i64
        let s_9_8: i64 = (s_9_7 as i64);
        // D s_9_9: cast zx s_9_5 -> i
        let s_9_9: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_10: cast zx s_9_8 -> i
        let s_9_10: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_11: read-var operand:bv
        let s_9_11: Bits = fn_state.operand;
        // D s_9_12: call Elem_read(s_9_11, s_9_9, s_9_10)
        let s_9_12: Bits = Elem_read(state, tracer, s_9_11, s_9_9, s_9_10);
        // D s_9_13: write-var element <= s_9_12
        fn_state.element = s_9_12;
        // N s_9_14: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#3028:i64
        let s_10_0: i64 = fn_state.VLshadow_3028;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var d:i64
        let s_10_3: i64 = fn_state.d;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: read-var result:bv
        let s_10_6: Bits = fn_state.result;
        // D s_10_7: call Z_set(s_10_4, s_10_5, s_10_6)
        let s_10_7: () = Z_set(state, tracer, s_10_4, s_10_5, s_10_6);
        // N s_10_8: return
        return;
    }
}
