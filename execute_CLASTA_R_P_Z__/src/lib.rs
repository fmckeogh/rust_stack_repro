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
use X_set::*;
use u__id::*;
use CheckSVEEnabled::*;
use P_read::*;
use LastActiveElement::*;
use X_read::*;
use Elem_read::*;
use Z_read::*;
use common::*;
pub fn execute_CLASTA_R_P_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    csize: i64,
    dn: i64,
    esize: i64,
    g: i64,
    isBefore: bool,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_195938: bool,
        last: i128,
        VLshadow_2933: i64,
        lastshadow_2935: i128,
        csizeshadow_2932: i64,
        elements: i64,
        gs_195934: bool,
        result: Bits,
        operand1: Bits,
        esizeshadow_2931: i64,
        operand2: Bits,
        VL: i64,
        csize: i64,
        dn: i64,
        esize: i64,
        g: i64,
        isBefore: bool,
        m: i64,
    }
    let fn_state = FunctionState {
        VL,
        csize,
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
        // D s_0_3: write-var esizeshadow#2931 <= s_0_2
        fn_state.esizeshadow_2931 = s_0_2;
        // D s_0_4: read-var csize:i64
        let s_0_4: i64 = fn_state.csize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var csizeshadow#2932 <= s_0_6
        fn_state.csizeshadow_2932 = s_0_6;
        // D s_0_8: read-var VL:i64
        let s_0_8: i64 = fn_state.VL;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: cast reint s_0_9 -> i64
        let s_0_10: i64 = (s_0_9 as i64);
        // D s_0_11: write-var VLshadow#2933 <= s_0_10
        fn_state.VLshadow_2933 = s_0_10;
        // C s_0_12: const #() : ()
        let s_0_12: () = ();
        // S s_0_13: call CheckSVEEnabled(s_0_12)
        let s_0_13: () = CheckSVEEnabled(state, tracer, s_0_12);
        // N s_0_14: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2933:i64
        let s_1_0: i64 = fn_state.VLshadow_2933;
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
        // D s_1_6: read-var esizeshadow#2931:i64
        let s_1_6: i64 = fn_state.esizeshadow_2931;
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
        // D s_1_17: read-var esizeshadow#2931:i64
        let s_1_17: i64 = fn_state.esizeshadow_2931;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // D s_1_20: read-var dn:i64
        let s_1_20: i64 = fn_state.dn;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: call X_read(s_1_21, s_1_19)
        let s_1_22: Bits = X_read(state, tracer, s_1_21, s_1_19);
        // D s_1_23: write-var operand1 <= s_1_22
        fn_state.operand1 = s_1_22;
        // D s_1_24: cast zx s_1_0 -> i
        let s_1_24: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: read-var m:i64
        let s_1_26: i64 = fn_state.m;
        // D s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_28: cast zx s_1_25 -> i
        let s_1_28: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_29: call Z_read(s_1_27, s_1_28)
        let s_1_29: Bits = Z_read(state, tracer, s_1_27, s_1_28);
        // D s_1_30: write-var operand2 <= s_1_29
        fn_state.operand2 = s_1_29;
        // D s_1_31: read-var esizeshadow#2931:i64
        let s_1_31: i64 = fn_state.esizeshadow_2931;
        // D s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (i128::try_from(s_1_31).unwrap());
        // D s_1_33: call LastActiveElement(s_1_16, s_1_32)
        let s_1_33: i128 = LastActiveElement(state, tracer, s_1_16, s_1_32);
        // D s_1_34: write-var last <= s_1_33
        fn_state.last = s_1_33;
        // C s_1_35: const #0s : i
        let s_1_35: i128 = 0;
        // D s_1_36: read-var last:i
        let s_1_36: i128 = fn_state.last;
        // D s_1_37: cmp-lt s_1_36 s_1_35
        let s_1_37: bool = ((s_1_36) < (s_1_35));
        // N s_1_38: branch s_1_37 b13 b2
        if s_1_37 {
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
        // D s_4_1: write-var lastshadow#2935 <= s_4_0
        fn_state.lastshadow_2935 = s_4_0;
        // D s_4_2: read-var esizeshadow#2931:i64
        let s_4_2: i64 = fn_state.esizeshadow_2931;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: call __id(s_4_3)
        let s_4_4: i128 = u__id(state, tracer, s_4_3);
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // D s_4_7: cast zx s_4_5 -> i
        let s_4_7: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_8: cmp-ge s_4_7 s_4_6
        let s_4_8: bool = ((s_4_7) >= (s_4_6));
        // N s_4_9: branch s_4_8 b8 b5
        if s_4_8 {
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
        // D s_5_1: write-var gs#195934 <= s_5_0
        fn_state.gs_195934 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#195934:u8
        let s_6_0: bool = fn_state.gs_195934;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // D s_6_2: read-var esizeshadow#2931:i64
        let s_6_2: i64 = fn_state.esizeshadow_2931;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: read-var operand2:bv
        let s_6_6: Bits = fn_state.operand2;
        // D s_6_7: read-var lastshadow#2935:i
        let s_6_7: i128 = fn_state.lastshadow_2935;
        // D s_6_8: call Elem_read(s_6_6, s_6_7, s_6_5)
        let s_6_8: Bits = Elem_read(state, tracer, s_6_6, s_6_7, s_6_5);
        // D s_6_9: read-var csizeshadow#2932:i64
        let s_6_9: i64 = fn_state.csizeshadow_2932;
        // D s_6_10: cast zx s_6_9 -> i
        let s_6_10: i128 = (i128::try_from(s_6_9).unwrap());
        // D s_6_11: bits-cast zx s_6_8 -> bv length s_6_10
        let s_6_11: Bits = s_6_8.zero_extend(s_6_10);
        // D s_6_12: write-var result <= s_6_11
        fn_state.result = s_6_11;
        // N s_6_13: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var csizeshadow#2932:i64
        let s_7_0: i64 = fn_state.csizeshadow_2932;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var dn:i64
        let s_7_3: i64 = fn_state.dn;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: read-var result:bv
        let s_7_5: Bits = fn_state.result;
        // D s_7_6: call X_set(s_7_4, s_7_2, s_7_5)
        let s_7_6: () = X_set(state, tracer, s_7_4, s_7_2, s_7_5);
        // N s_7_7: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var csizeshadow#2932:i64
        let s_8_0: i64 = fn_state.csizeshadow_2932;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call __id(s_8_1)
        let s_8_2: i128 = u__id(state, tracer, s_8_1);
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var esizeshadow#2931:i64
        let s_8_4: i64 = fn_state.esizeshadow_2931;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: call __id(s_8_5)
        let s_8_6: i128 = u__id(state, tracer, s_8_5);
        // D s_8_7: cast reint s_8_6 -> i64
        let s_8_7: i64 = (s_8_6 as i64);
        // D s_8_8: cast zx s_8_3 -> i
        let s_8_8: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_9: cast zx s_8_7 -> i
        let s_8_9: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_10: cmp-ge s_8_8 s_8_9
        let s_8_10: bool = ((s_8_8) >= (s_8_9));
        // D s_8_11: write-var gs#195934 <= s_8_10
        fn_state.gs_195934 = s_8_10;
        // N s_8_12: jump b6
        return block_6(state, tracer, fn_state);
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
        // D s_13_0: read-var esizeshadow#2931:i64
        let s_13_0: i64 = fn_state.esizeshadow_2931;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call __id(s_13_1)
        let s_13_2: i128 = u__id(state, tracer, s_13_1);
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: const #0s : i
        let s_13_4: i128 = 0;
        // D s_13_5: cast zx s_13_3 -> i
        let s_13_5: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_6: cmp-ge s_13_5 s_13_4
        let s_13_6: bool = ((s_13_5) >= (s_13_4));
        // N s_13_7: branch s_13_6 b16 b14
        if s_13_6 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#195938 <= s_14_0
        fn_state.gs_195938 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#195938:u8
        let s_15_0: bool = fn_state.gs_195938;
        // N s_15_1: assert s_15_0
        let s_15_1: () = assert!(s_15_0);
        // D s_15_2: read-var csizeshadow#2932:i64
        let s_15_2: i64 = fn_state.csizeshadow_2932;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: read-var operand1:bv
        let s_15_4: Bits = fn_state.operand1;
        // D s_15_5: bits-cast zx s_15_4 -> bv length s_15_3
        let s_15_5: Bits = s_15_4.zero_extend(s_15_3);
        // D s_15_6: write-var result <= s_15_5
        fn_state.result = s_15_5;
        // N s_15_7: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var csizeshadow#2932:i64
        let s_16_0: i64 = fn_state.csizeshadow_2932;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: call __id(s_16_1)
        let s_16_2: i128 = u__id(state, tracer, s_16_1);
        // D s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // D s_16_4: read-var esizeshadow#2931:i64
        let s_16_4: i64 = fn_state.esizeshadow_2931;
        // D s_16_5: cast zx s_16_4 -> i
        let s_16_5: i128 = (i128::try_from(s_16_4).unwrap());
        // D s_16_6: call __id(s_16_5)
        let s_16_6: i128 = u__id(state, tracer, s_16_5);
        // D s_16_7: cast reint s_16_6 -> i64
        let s_16_7: i64 = (s_16_6 as i64);
        // D s_16_8: cast zx s_16_3 -> i
        let s_16_8: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_9: cast zx s_16_7 -> i
        let s_16_9: i128 = (i128::try_from(s_16_7).unwrap());
        // D s_16_10: cmp-ge s_16_8 s_16_9
        let s_16_10: bool = ((s_16_8) >= (s_16_9));
        // D s_16_11: write-var gs#195938 <= s_16_10
        fn_state.gs_195938 = s_16_10;
        // N s_16_12: jump b15
        return block_15(state, tracer, fn_state);
    }
}
