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
use Zeros::*;
use CheckSVEEnabled::*;
use P_set::*;
use P_read::*;
use ActivePredicateElement::*;
use common::*;
pub fn execute_PSEL_P_PPi__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    imm: i64,
    m: i64,
    n: i64,
    v: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: Bits,
        operand1: Bits,
        PL: i64,
        VL: i64,
        d: i64,
        esize: i64,
        imm: i64,
        m: i64,
        n: i64,
        v: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        imm,
        m,
        n,
        v,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckSVEEnabled(s_0_0)
        let s_0_1: () = CheckSVEEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VL:i64
        let s_1_0: i64 = fn_state.VL;
        // C s_1_1: const #8s : i
        let s_1_1: i128 = 8;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: div s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) / (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: write-var PL <= s_1_4
        fn_state.PL = s_1_4;
        // D s_1_6: cast zx s_1_0 -> i
        let s_1_6: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_7: read-var esize:i64
        let s_1_7: i64 = fn_state.esize;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: div s_1_6 s_1_8
        let s_1_9: i128 = ((s_1_6) / (s_1_8));
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var PL:i64
        let s_1_11: i64 = fn_state.PL;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: read-var n:i64
        let s_1_14: i64 = fn_state.n;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast zx s_1_13 -> i
        let s_1_16: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_17: call P_read(s_1_15, s_1_16)
        let s_1_17: Bits = P_read(state, tracer, s_1_15, s_1_16);
        // D s_1_18: write-var operand1 <= s_1_17
        fn_state.operand1 = s_1_17;
        // D s_1_19: read-var PL:i64
        let s_1_19: i64 = fn_state.PL;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast reint s_1_20 -> i64
        let s_1_21: i64 = (s_1_20 as i64);
        // D s_1_22: read-var m:i64
        let s_1_22: i64 = fn_state.m;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: cast zx s_1_21 -> i
        let s_1_24: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_25: call P_read(s_1_23, s_1_24)
        let s_1_25: Bits = P_read(state, tracer, s_1_23, s_1_24);
        // C s_1_26: const #32s : i64
        let s_1_26: i64 = 32;
        // D s_1_27: read-var v:i64
        let s_1_27: i64 = fn_state.v;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: call X_read(s_1_28, s_1_26)
        let s_1_29: Bits = X_read(state, tracer, s_1_28, s_1_26);
        // D s_1_30: cast reint s_1_29 -> u32
        let s_1_30: u32 = (s_1_29.value() as u32);
        // D s_1_31: cast zx s_1_30 -> bv
        let s_1_31: Bits = Bits::new(s_1_30 as u128, 32u16);
        // D s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (s_1_31.value() as i128);
        // D s_1_33: cast reint s_1_32 -> i64
        let s_1_33: i64 = (s_1_32 as i64);
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: read-var imm:i64
        let s_1_35: i64 = fn_state.imm;
        // D s_1_36: cast zx s_1_35 -> i
        let s_1_36: i128 = (i128::try_from(s_1_35).unwrap());
        // D s_1_37: add s_1_34 s_1_36
        let s_1_37: i128 = (s_1_34 + s_1_36);
        // D s_1_38: cast reint s_1_37 -> i64
        let s_1_38: i64 = (s_1_37 as i64);
        // D s_1_39: cast zx s_1_38 -> i
        let s_1_39: i128 = (i128::try_from(s_1_38).unwrap());
        // D s_1_40: cast zx s_1_10 -> i
        let s_1_40: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_41: mod s_1_39 s_1_40
        let s_1_41: i128 = ((s_1_39) % (s_1_40));
        // D s_1_42: cast reint s_1_41 -> i64
        let s_1_42: i64 = (s_1_41 as i64);
        // D s_1_43: cast zx s_1_42 -> i
        let s_1_43: i128 = (i128::try_from(s_1_42).unwrap());
        // D s_1_44: read-var esize:i64
        let s_1_44: i64 = fn_state.esize;
        // D s_1_45: cast zx s_1_44 -> i
        let s_1_45: i128 = (i128::try_from(s_1_44).unwrap());
        // D s_1_46: call ActivePredicateElement(s_1_25, s_1_43, s_1_45)
        let s_1_46: bool = ActivePredicateElement(state, tracer, s_1_25, s_1_43, s_1_45);
        // N s_1_47: branch s_1_46 b4 b2
        if s_1_46 {
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
        // D s_2_0: read-var PL:i64
        let s_2_0: i64 = fn_state.PL;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var result <= s_2_2
        fn_state.result = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var PL:i64
        let s_3_0: i64 = fn_state.PL;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var d:i64
        let s_3_3: i64 = fn_state.d;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var result:bv
        let s_3_6: Bits = fn_state.result;
        // D s_3_7: call P_set(s_3_4, s_3_5, s_3_6)
        let s_3_7: () = P_set(state, tracer, s_3_4, s_3_5, s_3_6);
        // N s_3_8: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var operand1:bv
        let s_4_0: Bits = fn_state.operand1;
        // D s_4_1: write-var result <= s_4_0
        fn_state.result = s_4_0;
        // N s_4_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
