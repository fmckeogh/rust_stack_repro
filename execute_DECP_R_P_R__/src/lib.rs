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
use X_read::*;
use CheckSVEEnabled::*;
use P_read::*;
use ActivePredicateElement::*;
use common::*;
pub fn execute_DECP_R_P_R__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_189905: i64,
        e: i64,
        count: i128,
        operand1: u64,
        operand2: Bits,
        VL: i64,
        dn: i64,
        esize: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        m,
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
        // D s_1_5: cast zx s_1_0 -> i
        let s_1_5: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_6: read-var esize:i64
        let s_1_6: i64 = fn_state.esize;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: div s_1_5 s_1_7
        let s_1_8: i128 = ((s_1_5) / (s_1_7));
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // C s_1_10: const #64s : i64
        let s_1_10: i64 = 64;
        // D s_1_11: read-var dn:i64
        let s_1_11: i64 = fn_state.dn;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call X_read(s_1_12, s_1_10)
        let s_1_13: Bits = X_read(state, tracer, s_1_12, s_1_10);
        // D s_1_14: cast reint s_1_13 -> u64
        let s_1_14: u64 = (s_1_13.value() as u64);
        // D s_1_15: write-var operand1 <= s_1_14
        fn_state.operand1 = s_1_14;
        // D s_1_16: cast zx s_1_4 -> i
        let s_1_16: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: read-var m:i64
        let s_1_18: i64 = fn_state.m;
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: cast zx s_1_17 -> i
        let s_1_20: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_21: call P_read(s_1_19, s_1_20)
        let s_1_21: Bits = P_read(state, tracer, s_1_19, s_1_20);
        // D s_1_22: write-var operand2 <= s_1_21
        fn_state.operand2 = s_1_21;
        // C s_1_23: const #0s : i
        let s_1_23: i128 = 0;
        // D s_1_24: write-var count <= s_1_23
        fn_state.count = s_1_23;
        // C s_1_25: const #0s : i64
        let s_1_25: i64 = 0;
        // C s_1_26: const #1s : i
        let s_1_26: i128 = 1;
        // D s_1_27: cast zx s_1_9 -> i
        let s_1_27: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_28: sub s_1_27 s_1_26
        let s_1_28: i128 = ((s_1_27) - (s_1_26));
        // D s_1_29: cast reint s_1_28 -> i64
        let s_1_29: i64 = (s_1_28 as i64);
        // D s_1_30: write-var gs#189905 <= s_1_29
        fn_state.gs_189905 = s_1_29;
        // D s_1_31: write-var e <= s_1_25
        fn_state.e = s_1_25;
        // N s_1_32: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#189905:i64
        let s_2_1: i64 = fn_state.gs_189905;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var esize:i64
        let s_3_2: i64 = fn_state.esize;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: read-var operand2:bv
        let s_3_4: Bits = fn_state.operand2;
        // D s_3_5: call ActivePredicateElement(s_3_4, s_3_1, s_3_3)
        let s_3_5: bool = ActivePredicateElement(state, tracer, s_3_4, s_3_1, s_3_3);
        // N s_3_6: branch s_3_5 b6 b4
        if s_3_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
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
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // C s_5_1: const #1s : i64
        let s_5_1: i64 = 1;
        // D s_5_2: add s_5_0 s_5_1
        let s_5_2: i64 = (s_5_0 + s_5_1);
        // D s_5_3: write-var e <= s_5_2
        fn_state.e = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1s : i
        let s_6_0: i128 = 1;
        // D s_6_1: read-var count:i
        let s_6_1: i128 = fn_state.count;
        // D s_6_2: add s_6_1 s_6_0
        let s_6_2: i128 = (s_6_1 + s_6_0);
        // D s_6_3: write-var count <= s_6_2
        fn_state.count = s_6_2;
        // N s_6_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var count:i
        let s_7_0: i128 = fn_state.count;
        // C s_7_1: const #64s : i64
        let s_7_1: i64 = 64;
        // D s_7_2: read-var operand1:u64
        let s_7_2: u64 = fn_state.operand1;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 64u16);
        // D s_7_4: cast cvt s_7_0 -> bv
        let s_7_4: Bits = Bits::new(s_7_0 as u128, 128);
        // D s_7_5: sub s_7_3 s_7_4
        let s_7_5: Bits = ((s_7_3) - (s_7_4));
        // D s_7_6: cast reint s_7_5 -> u64
        let s_7_6: u64 = (s_7_5.value() as u64);
        // D s_7_7: read-var dn:i64
        let s_7_7: i64 = fn_state.dn;
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: cast zx s_7_6 -> bv
        let s_7_9: Bits = Bits::new(s_7_6 as u128, 64u16);
        // D s_7_10: call X_set(s_7_8, s_7_1, s_7_9)
        let s_7_10: () = X_set(state, tracer, s_7_8, s_7_1, s_7_9);
        // N s_7_11: return
        return;
    }
}
