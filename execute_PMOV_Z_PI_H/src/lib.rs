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
use Bit::*;
use Zeros::*;
use CheckSVEEnabled::*;
use Z_read::*;
use P_read::*;
use PredicateElement::*;
use common::*;
pub fn execute_PMOV_Z_PI_H<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    imm: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        VLshadow_4398: i64,
        e: i64,
        gs_221244: i64,
        elements: i64,
        result: Bits,
        VLshadow_4399: i64,
        VL: i64,
        d: i64,
        esize: i64,
        imm: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        imm,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#4398 <= s_0_2
        fn_state.VLshadow_4398 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckSVEEnabled(s_0_4)
        let s_0_5: () = CheckSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#4398:i64
        let s_1_0: i64 = fn_state.VLshadow_4398;
        // D s_1_1: write-var VLshadow#4399 <= s_1_0
        fn_state.VLshadow_4399 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#4399:i64
        let s_1_3: i64 = fn_state.VLshadow_4399;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#4399:i64
        let s_1_7: i64 = fn_state.VLshadow_4399;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esize:i64
        let s_1_9: i64 = fn_state.esize;
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
        // D s_1_16: read-var n:i64
        let s_1_16: i64 = fn_state.n;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call P_read(s_1_17, s_1_18)
        let s_1_19: Bits = P_read(state, tracer, s_1_17, s_1_18);
        // D s_1_20: write-var operand <= s_1_19
        fn_state.operand = s_1_19;
        // C s_1_21: const #0s : i
        let s_1_21: i128 = 0;
        // D s_1_22: read-var imm:i64
        let s_1_22: i64 = fn_state.imm;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: cmp-eq s_1_23 s_1_21
        let s_1_24: bool = ((s_1_23) == (s_1_21));
        // N s_1_25: branch s_1_24 b7 b2
        if s_1_24 {
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
        // D s_2_0: read-var VLshadow#4399:i64
        let s_2_0: i64 = fn_state.VLshadow_4399;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: cast reint s_2_1 -> i64
        let s_2_2: i64 = (s_2_1 as i64);
        // D s_2_3: read-var d:i64
        let s_2_3: i64 = fn_state.d;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: cast zx s_2_2 -> i
        let s_2_5: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_6: call Z_read(s_2_4, s_2_5)
        let s_2_6: Bits = Z_read(state, tracer, s_2_4, s_2_5);
        // D s_2_7: write-var result <= s_2_6
        fn_state.result = s_2_6;
        // N s_2_8: jump b3
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
        // D s_3_6: write-var gs#221244 <= s_3_5
        fn_state.gs_221244 = s_3_5;
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
        // D s_4_1: read-var gs#221244:i64
        let s_4_1: i64 = fn_state.gs_221244;
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
        // D s_5_0: read-var elements:i64
        let s_5_0: i64 = fn_state.elements;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var imm:i64
        let s_5_2: i64 = fn_state.imm;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: mul s_5_1 s_5_3
        let s_5_4: i128 = ((s_5_1) * (s_5_3));
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: read-var e:i64
        let s_5_7: i64 = fn_state.e;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: add s_5_6 s_5_8
        let s_5_9: i128 = (s_5_6 + s_5_8);
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: read-var e:i64
        let s_5_11: i64 = fn_state.e;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: read-var esize:i64
        let s_5_13: i64 = fn_state.esize;
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_15: read-var operand:bv
        let s_5_15: Bits = fn_state.operand;
        // D s_5_16: call PredicateElement(s_5_15, s_5_12, s_5_14)
        let s_5_16: bool = PredicateElement(state, tracer, s_5_15, s_5_12, s_5_14);
        // D s_5_17: call Bit(s_5_16)
        let s_5_17: bool = Bit(state, tracer, s_5_16);
        // D s_5_18: cast zx s_5_10 -> i
        let s_5_18: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_19: read-var result:bv
        let s_5_19: Bits = fn_state.result;
        // C s_5_20: const #1u : u64
        let s_5_20: u64 = 1;
        // D s_5_21: bit-insert s_5_19 s_5_19 s_5_18 s_5_20
        let s_5_21: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_5_20 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_5_19.length(),
            );
            (s_5_19 & mask) | (s_5_19 << s_5_18)
        };
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
        // D s_6_0: read-var VLshadow#4399:i64
        let s_6_0: i64 = fn_state.VLshadow_4399;
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
        // D s_7_0: read-var VLshadow#4399:i64
        let s_7_0: i64 = fn_state.VLshadow_4399;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call Zeros(s_7_1)
        let s_7_2: Bits = Zeros(state, tracer, s_7_1);
        // D s_7_3: write-var result <= s_7_2
        fn_state.result = s_7_2;
        // N s_7_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
