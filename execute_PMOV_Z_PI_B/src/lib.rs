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
use P_read::*;
use PredicateElement::*;
use common::*;
pub fn execute_PMOV_Z_PI_B<T: Tracer>(
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
        gs_221195: i64,
        e: i64,
        VLshadow_4396: i64,
        elements: i64,
        result: Bits,
        VLshadow_4397: i64,
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
        // D s_0_3: write-var VLshadow#4396 <= s_0_2
        fn_state.VLshadow_4396 = s_0_2;
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
        // D s_1_0: read-var VLshadow#4396:i64
        let s_1_0: i64 = fn_state.VLshadow_4396;
        // D s_1_1: write-var VLshadow#4397 <= s_1_0
        fn_state.VLshadow_4397 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#4397:i64
        let s_1_3: i64 = fn_state.VLshadow_4397;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#4397:i64
        let s_1_7: i64 = fn_state.VLshadow_4397;
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
        // D s_1_21: read-var VLshadow#4397:i64
        let s_1_21: i64 = fn_state.VLshadow_4397;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: call Zeros(s_1_22)
        let s_1_23: Bits = Zeros(state, tracer, s_1_22);
        // D s_1_24: write-var result <= s_1_23
        fn_state.result = s_1_23;
        // C s_1_25: const #0s : i64
        let s_1_25: i64 = 0;
        // C s_1_26: const #1s : i
        let s_1_26: i128 = 1;
        // D s_1_27: read-var elements:i64
        let s_1_27: i64 = fn_state.elements;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: sub s_1_28 s_1_26
        let s_1_29: i128 = ((s_1_28) - (s_1_26));
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: write-var gs#221195 <= s_1_30
        fn_state.gs_221195 = s_1_30;
        // D s_1_32: write-var e <= s_1_25
        fn_state.e = s_1_25;
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
        // D s_2_1: read-var gs#221195:i64
        let s_2_1: i64 = fn_state.gs_221195;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var elements:i64
        let s_3_0: i64 = fn_state.elements;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var imm:i64
        let s_3_2: i64 = fn_state.imm;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: mul s_3_1 s_3_3
        let s_3_4: i128 = ((s_3_1) * (s_3_3));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: read-var e:i64
        let s_3_7: i64 = fn_state.e;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: add s_3_6 s_3_8
        let s_3_9: i128 = (s_3_6 + s_3_8);
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: read-var e:i64
        let s_3_11: i64 = fn_state.e;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: read-var esize:i64
        let s_3_13: i64 = fn_state.esize;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: read-var operand:bv
        let s_3_15: Bits = fn_state.operand;
        // D s_3_16: call PredicateElement(s_3_15, s_3_12, s_3_14)
        let s_3_16: bool = PredicateElement(state, tracer, s_3_15, s_3_12, s_3_14);
        // D s_3_17: call Bit(s_3_16)
        let s_3_17: bool = Bit(state, tracer, s_3_16);
        // D s_3_18: cast zx s_3_10 -> i
        let s_3_18: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_19: read-var result:bv
        let s_3_19: Bits = fn_state.result;
        // C s_3_20: const #1u : u64
        let s_3_20: u64 = 1;
        // D s_3_21: bit-insert s_3_19 s_3_19 s_3_18 s_3_20
        let s_3_21: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_3_20 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_3_19.length(),
            );
            (s_3_19 & mask) | (s_3_19 << s_3_18)
        };
        // D s_3_22: write-var result <= s_3_21
        fn_state.result = s_3_21;
        // D s_3_23: read-var e:i64
        let s_3_23: i64 = fn_state.e;
        // C s_3_24: const #1s : i64
        let s_3_24: i64 = 1;
        // D s_3_25: add s_3_23 s_3_24
        let s_3_25: i64 = (s_3_23 + s_3_24);
        // D s_3_26: write-var e <= s_3_25
        fn_state.e = s_3_25;
        // N s_3_27: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#4397:i64
        let s_4_0: i64 = fn_state.VLshadow_4397;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: read-var result:bv
        let s_4_6: Bits = fn_state.result;
        // D s_4_7: call Z_set(s_4_4, s_4_5, s_4_6)
        let s_4_7: () = Z_set(state, tracer, s_4_4, s_4_5, s_4_6);
        // N s_4_8: return
        return;
    }
}
