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
use V_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use P_read::*;
use Elem_read::*;
use common::*;
pub fn execute_CLASTA_V_P_Z__<T: Tracer>(
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
        elements: i64,
        VLshadow_2942: i64,
        result: Bits,
        operand1: Bits,
        esizeshadow_2941: i64,
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
        // D s_0_3: write-var esizeshadow#2941 <= s_0_2
        fn_state.esizeshadow_2941 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2942 <= s_0_6
        fn_state.VLshadow_2942 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2942:i64
        let s_1_0: i64 = fn_state.VLshadow_2942;
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
        // D s_1_6: read-var esizeshadow#2941:i64
        let s_1_6: i64 = fn_state.esizeshadow_2941;
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
        // D s_1_17: read-var esizeshadow#2941:i64
        let s_1_17: i64 = fn_state.esizeshadow_2941;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // D s_1_20: read-var dn:i64
        let s_1_20: i64 = fn_state.dn;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: call V_read(s_1_21, s_1_19)
        let s_1_22: Bits = V_read(state, tracer, s_1_21, s_1_19);
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
        // D s_1_31: read-var esizeshadow#2941:i64
        let s_1_31: i64 = fn_state.esizeshadow_2941;
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
        // N s_1_38: branch s_1_37 b10 b2
        if s_1_37 {
            return block_10(state, tracer, fn_state);
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
        // N s_2_2: branch s_2_1 b6 b3
        if s_2_1 {
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
        // D s_4_0: read-var last:i
        let s_4_0: i128 = fn_state.last;
        // D s_4_1: read-var esizeshadow#2941:i64
        let s_4_1: i64 = fn_state.esizeshadow_2941;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: read-var operand2:bv
        let s_4_5: Bits = fn_state.operand2;
        // D s_4_6: call Elem_read(s_4_5, s_4_0, s_4_4)
        let s_4_6: Bits = Elem_read(state, tracer, s_4_5, s_4_0, s_4_4);
        // D s_4_7: write-var result <= s_4_6
        fn_state.result = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#2941:i64
        let s_5_0: i64 = fn_state.esizeshadow_2941;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var dn:i64
        let s_5_3: i64 = fn_state.dn;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: read-var result:bv
        let s_5_5: Bits = fn_state.result;
        // D s_5_6: call V_set(s_5_4, s_5_2, s_5_5)
        let s_5_6: () = V_set(state, tracer, s_5_4, s_5_2, s_5_5);
        // N s_5_7: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1s : i
        let s_6_0: i128 = 1;
        // D s_6_1: read-var last:i
        let s_6_1: i128 = fn_state.last;
        // D s_6_2: add s_6_1 s_6_0
        let s_6_2: i128 = (s_6_1 + s_6_0);
        // D s_6_3: write-var last <= s_6_2
        fn_state.last = s_6_2;
        // D s_6_4: read-var elements:i64
        let s_6_4: i64 = fn_state.elements;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: read-var last:i
        let s_6_6: i128 = fn_state.last;
        // D s_6_7: cmp-ge s_6_6 s_6_5
        let s_6_7: bool = ((s_6_6) >= (s_6_5));
        // N s_6_8: branch s_6_7 b9 b7
        if s_6_7 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
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
        // N s_8_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0s : i
        let s_9_0: i128 = 0;
        // D s_9_1: write-var last <= s_9_0
        fn_state.last = s_9_0;
        // N s_9_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esizeshadow#2941:i64
        let s_10_0: i64 = fn_state.esizeshadow_2941;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var operand1:bv
        let s_10_2: Bits = fn_state.operand1;
        // D s_10_3: bits-cast zx s_10_2 -> bv length s_10_1
        let s_10_3: Bits = s_10_2.zero_extend(s_10_1);
        // D s_10_4: write-var result <= s_10_3
        fn_state.result = s_10_3;
        // N s_10_5: jump b5
        return block_5(state, tracer, fn_state);
    }
}
