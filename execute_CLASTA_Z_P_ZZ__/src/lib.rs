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
use Elem_set::*;
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use P_read::*;
use Z_set::*;
use common::*;
pub fn execute_CLASTA_Z_P_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    g: i64,
    isBefore: bool,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        last: i128,
        e: i64,
        VLshadow_2950: i64,
        gs_196146: i64,
        elements: i64,
        lastshadow_2952: i128,
        result: Bits,
        operand1: Bits,
        VLshadow_2951: i64,
        esizeshadow_2949: i64,
        operand2: Bits,
        VL: i64,
        dn: i64,
        esize: i64,
        g: i64,
        isBefore: bool,
        m: i64,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        g,
        isBefore,
        m,
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
        // D s_0_3: write-var esizeshadow#2949 <= s_0_2
        fn_state.esizeshadow_2949 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2950 <= s_0_6
        fn_state.VLshadow_2950 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2950:i64
        let s_1_0: i64 = fn_state.VLshadow_2950;
        // D s_1_1: write-var VLshadow#2951 <= s_1_0
        fn_state.VLshadow_2951 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2951:i64
        let s_1_3: i64 = fn_state.VLshadow_2951;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2951:i64
        let s_1_7: i64 = fn_state.VLshadow_2951;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#2949:i64
        let s_1_9: i64 = fn_state.esizeshadow_2949;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var elements <= s_1_12
        fn_state.elements = s_1_12;
        // D s_1_14: cast zx s_1_6 -> i
        let s_1_14: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var g:i64
        let s_1_16: i64 = fn_state.g;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call P_read(s_1_17, s_1_18)
        let s_1_19: Bits = P_read(state, tracer, s_1_17, s_1_18);
        // D s_1_20: read-var VLshadow#2951:i64
        let s_1_20: i64 = fn_state.VLshadow_2951;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: read-var dn:i64
        let s_1_23: i64 = fn_state.dn;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast zx s_1_22 -> i
        let s_1_25: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_26: call Z_read(s_1_24, s_1_25)
        let s_1_26: Bits = Z_read(state, tracer, s_1_24, s_1_25);
        // D s_1_27: write-var operand1 <= s_1_26
        fn_state.operand1 = s_1_26;
        // D s_1_28: read-var VLshadow#2951:i64
        let s_1_28: i64 = fn_state.VLshadow_2951;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: read-var m:i64
        let s_1_31: i64 = fn_state.m;
        // D s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (i128::try_from(s_1_31).unwrap());
        // D s_1_33: cast zx s_1_30 -> i
        let s_1_33: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_34: call Z_read(s_1_32, s_1_33)
        let s_1_34: Bits = Z_read(state, tracer, s_1_32, s_1_33);
        // D s_1_35: write-var operand2 <= s_1_34
        fn_state.operand2 = s_1_34;
        // D s_1_36: read-var esizeshadow#2949:i64
        let s_1_36: i64 = fn_state.esizeshadow_2949;
        // D s_1_37: cast zx s_1_36 -> i
        let s_1_37: i128 = (i128::try_from(s_1_36).unwrap());
        // D s_1_38: call LastActiveElement(s_1_19, s_1_37)
        let s_1_38: i128 = LastActiveElement(state, tracer, s_1_19, s_1_37);
        // D s_1_39: write-var last <= s_1_38
        fn_state.last = s_1_38;
        // C s_1_40: const #0s : i
        let s_1_40: i128 = 0;
        // D s_1_41: read-var last:i
        let s_1_41: i128 = fn_state.last;
        // D s_1_42: cmp-lt s_1_41 s_1_40
        let s_1_42: bool = ((s_1_41) < (s_1_40));
        // N s_1_43: branch s_1_42 b13 b2
        if s_1_42 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var isBefore:u8
        let s_2_0: bool = fn_state.isBefore;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b9 b3
        if s_2_1 {
            return block_9(state, tracer, fn_state);
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
        // D s_4_0: read-var last:i
        let s_4_0: i128 = fn_state.last;
        // D s_4_1: write-var lastshadow#2952 <= s_4_0
        fn_state.lastshadow_2952 = s_4_0;
        // C s_4_2: const #0s : i64
        let s_4_2: i64 = 0;
        // C s_4_3: const #1s : i
        let s_4_3: i128 = 1;
        // D s_4_4: read-var elements:i64
        let s_4_4: i64 = fn_state.elements;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: sub s_4_5 s_4_3
        let s_4_6: i128 = ((s_4_5) - (s_4_3));
        // D s_4_7: cast reint s_4_6 -> i64
        let s_4_7: i64 = (s_4_6 as i64);
        // D s_4_8: write-var gs#196146 <= s_4_7
        fn_state.gs_196146 = s_4_7;
        // D s_4_9: write-var e <= s_4_2
        fn_state.e = s_4_2;
        // N s_4_10: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // D s_5_1: read-var gs#196146:i64
        let s_5_1: i64 = fn_state.gs_196146;
        // D s_5_2: cmp-gt s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) > (s_5_1));
        // N s_5_3: branch s_5_2 b7 b6
        if s_5_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#2949:i64
        let s_6_0: i64 = fn_state.esizeshadow_2949;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var esizeshadow#2949:i64
        let s_6_3: i64 = fn_state.esizeshadow_2949;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: read-var operand2:bv
        let s_6_7: Bits = fn_state.operand2;
        // D s_6_8: read-var lastshadow#2952:i
        let s_6_8: i128 = fn_state.lastshadow_2952;
        // D s_6_9: call Elem_read(s_6_7, s_6_8, s_6_6)
        let s_6_9: Bits = Elem_read(state, tracer, s_6_7, s_6_8, s_6_6);
        // D s_6_10: read-var e:i64
        let s_6_10: i64 = fn_state.e;
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_12: cast zx s_6_2 -> i
        let s_6_12: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_13: read-var result:bv
        let s_6_13: Bits = fn_state.result;
        // D s_6_14: call Elem_set(s_6_13, s_6_11, s_6_12, s_6_9)
        let s_6_14: Bits = Elem_set(state, tracer, s_6_13, s_6_11, s_6_12, s_6_9);
        // D s_6_15: write-var result <= s_6_14
        fn_state.result = s_6_14;
        // D s_6_16: read-var e:i64
        let s_6_16: i64 = fn_state.e;
        // C s_6_17: const #1s : i64
        let s_6_17: i64 = 1;
        // D s_6_18: add s_6_16 s_6_17
        let s_6_18: i64 = (s_6_16 + s_6_17);
        // D s_6_19: write-var e <= s_6_18
        fn_state.e = s_6_18;
        // N s_6_20: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var VLshadow#2951:i64
        let s_8_0: i64 = fn_state.VLshadow_2951;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: read-var dn:i64
        let s_8_3: i64 = fn_state.dn;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cast zx s_8_2 -> i
        let s_8_5: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_6: read-var result:bv
        let s_8_6: Bits = fn_state.result;
        // D s_8_7: call Z_set(s_8_4, s_8_5, s_8_6)
        let s_8_7: () = Z_set(state, tracer, s_8_4, s_8_5, s_8_6);
        // N s_8_8: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1s : i
        let s_9_0: i128 = 1;
        // D s_9_1: read-var last:i
        let s_9_1: i128 = fn_state.last;
        // D s_9_2: add s_9_1 s_9_0
        let s_9_2: i128 = (s_9_1 + s_9_0);
        // D s_9_3: write-var last <= s_9_2
        fn_state.last = s_9_2;
        // D s_9_4: read-var elements:i64
        let s_9_4: i64 = fn_state.elements;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: read-var last:i
        let s_9_6: i128 = fn_state.last;
        // D s_9_7: cmp-ge s_9_6 s_9_5
        let s_9_7: bool = ((s_9_6) >= (s_9_5));
        // N s_9_8: branch s_9_7 b12 b10
        if s_9_7 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: write-var last <= s_12_0
        fn_state.last = s_12_0;
        // N s_12_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var operand1:bv
        let s_13_0: Bits = fn_state.operand1;
        // D s_13_1: write-var result <= s_13_0
        fn_state.result = s_13_0;
        // N s_13_2: jump b8
        return block_8(state, tracer, fn_state);
    }
}
