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
use asl_Int::*;
use X_read::*;
use SatQ::*;
use CheckSVEEnabled::*;
use P_read::*;
use ActivePredicateElement::*;
use common::*;
pub fn execute_UQINCP_R_P_R_UW<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    m: i64,
    ssize: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        ga_280607: u64,
        count: i128,
        gs_190006: i64,
        result: Bits,
        operand1: Bits,
        gs_740859: ProductTypef506aa96a892fe52,
        operand2: Bits,
        VL: i64,
        dn: i64,
        esize: i64,
        m: i64,
        ssize: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        m,
        ssize,
        is_unsigned,
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
        // D s_1_10: read-var ssize:i64
        let s_1_10: i64 = fn_state.ssize;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: read-var dn:i64
        let s_1_13: i64 = fn_state.dn;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: call X_read(s_1_14, s_1_12)
        let s_1_15: Bits = X_read(state, tracer, s_1_14, s_1_12);
        // D s_1_16: write-var operand1 <= s_1_15
        fn_state.operand1 = s_1_15;
        // D s_1_17: cast zx s_1_4 -> i
        let s_1_17: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: read-var m:i64
        let s_1_19: i64 = fn_state.m;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast zx s_1_18 -> i
        let s_1_21: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_22: call P_read(s_1_20, s_1_21)
        let s_1_22: Bits = P_read(state, tracer, s_1_20, s_1_21);
        // D s_1_23: write-var operand2 <= s_1_22
        fn_state.operand2 = s_1_22;
        // C s_1_24: const #0s : i
        let s_1_24: i128 = 0;
        // D s_1_25: write-var count <= s_1_24
        fn_state.count = s_1_24;
        // C s_1_26: const #0s : i64
        let s_1_26: i64 = 0;
        // C s_1_27: const #1s : i
        let s_1_27: i128 = 1;
        // D s_1_28: cast zx s_1_9 -> i
        let s_1_28: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_29: sub s_1_28 s_1_27
        let s_1_29: i128 = ((s_1_28) - (s_1_27));
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: write-var gs#190006 <= s_1_30
        fn_state.gs_190006 = s_1_30;
        // D s_1_32: write-var e <= s_1_26
        fn_state.e = s_1_26;
        // N s_1_33: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#190006:i64
        let s_2_1: i64 = fn_state.gs_190006;
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
        // D s_7_1: read-var operand1:bv
        let s_7_1: Bits = fn_state.operand1;
        // D s_7_2: read-var is_unsigned:u8
        let s_7_2: bool = fn_state.is_unsigned;
        // D s_7_3: call asl_Int(s_7_1, s_7_2)
        let s_7_3: i128 = asl_Int(state, tracer, s_7_1, s_7_2);
        // D s_7_4: add s_7_3 s_7_0
        let s_7_4: i128 = (s_7_3 + s_7_0);
        // D s_7_5: read-var ssize:i64
        let s_7_5: i64 = fn_state.ssize;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: cast reint s_7_6 -> i64
        let s_7_7: i64 = (s_7_6 as i64);
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: read-var is_unsigned:u8
        let s_7_9: bool = fn_state.is_unsigned;
        // D s_7_10: call SatQ(s_7_4, s_7_8, s_7_9)
        let s_7_10: ProductTypef506aa96a892fe52 = SatQ(
            state,
            tracer,
            s_7_4,
            s_7_8,
            s_7_9,
        );
        // D s_7_11: write-var gs#740859 <= s_7_10
        fn_state.gs_740859 = s_7_10;
        // D s_7_12: read-var gs#740859.0:struct
        let s_7_12: Bits = fn_state.gs_740859._0;
        // D s_7_13: cast reint s_7_12 -> u32
        let s_7_13: u32 = (s_7_12.value() as u32);
        // D s_7_14: cast zx s_7_13 -> bv
        let s_7_14: Bits = Bits::new(s_7_13 as u128, 32u16);
        // D s_7_15: write-var result <= s_7_14
        fn_state.result = s_7_14;
        // D s_7_16: read-var is_unsigned:u8
        let s_7_16: bool = fn_state.is_unsigned;
        // N s_7_17: branch s_7_16 b10 b8
        if s_7_16 {
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
        // C s_8_0: const #64s : i
        let s_8_0: i128 = 64;
        // D s_8_1: read-var result:bv
        let s_8_1: Bits = fn_state.result;
        // D s_8_2: bits-cast sx s_8_1 -> bv length s_8_0
        let s_8_2: Bits = s_8_1.sign_extend(s_8_0);
        // D s_8_3: cast reint s_8_2 -> u64
        let s_8_3: u64 = (s_8_2.value() as u64);
        // D s_8_4: write-var ga#280607 <= s_8_3
        fn_state.ga_280607 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var dn:i64
        let s_9_0: i64 = fn_state.dn;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var ga#280607:u64
        let s_9_2: u64 = fn_state.ga_280607;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 64u16);
        // C s_9_4: const #64s : i64
        let s_9_4: i64 = 64;
        // D s_9_5: call X_set(s_9_1, s_9_4, s_9_3)
        let s_9_5: () = X_set(state, tracer, s_9_1, s_9_4, s_9_3);
        // N s_9_6: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #64s : i
        let s_10_0: i128 = 64;
        // D s_10_1: read-var result:bv
        let s_10_1: Bits = fn_state.result;
        // D s_10_2: bits-cast zx s_10_1 -> bv length s_10_0
        let s_10_2: Bits = s_10_1.zero_extend(s_10_0);
        // D s_10_3: cast reint s_10_2 -> u64
        let s_10_3: u64 = (s_10_2.value() as u64);
        // D s_10_4: write-var ga#280607 <= s_10_3
        fn_state.ga_280607 = s_10_3;
        // N s_10_5: jump b9
        return block_9(state, tracer, fn_state);
    }
}
