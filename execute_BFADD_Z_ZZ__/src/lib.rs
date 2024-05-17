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
use Elem_set::*;
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use FPCR_read::*;
use BFAdd::*;
use common::*;
pub fn execute_BFADD_Z_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        VLshadow_2430: i64,
        gs_184388: i64,
        gs_733591: Bits,
        VLshadow_2429: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        VL: i64,
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        m,
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
        // D s_0_3: write-var VLshadow#2429 <= s_0_2
        fn_state.VLshadow_2429 = s_0_2;
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
        // D s_1_0: read-var VLshadow#2429:i64
        let s_1_0: i64 = fn_state.VLshadow_2429;
        // D s_1_1: write-var VLshadow#2430 <= s_1_0
        fn_state.VLshadow_2430 = s_1_0;
        // C s_1_2: const #16s : i
        let s_1_2: i128 = 16;
        // D s_1_3: read-var VLshadow#2430:i64
        let s_1_3: i64 = fn_state.VLshadow_2430;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2430:i64
        let s_1_7: i64 = fn_state.VLshadow_2430;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var n:i64
        let s_1_10: i64 = fn_state.n;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: cast zx s_1_9 -> i
        let s_1_12: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_13: call Z_read(s_1_11, s_1_12)
        let s_1_13: Bits = Z_read(state, tracer, s_1_11, s_1_12);
        // D s_1_14: write-var operand1 <= s_1_13
        fn_state.operand1 = s_1_13;
        // D s_1_15: read-var VLshadow#2430:i64
        let s_1_15: i64 = fn_state.VLshadow_2430;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: read-var m:i64
        let s_1_18: i64 = fn_state.m;
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: cast zx s_1_17 -> i
        let s_1_20: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_21: call Z_read(s_1_19, s_1_20)
        let s_1_21: Bits = Z_read(state, tracer, s_1_19, s_1_20);
        // D s_1_22: write-var operand2 <= s_1_21
        fn_state.operand2 = s_1_21;
        // C s_1_23: const #0s : i64
        let s_1_23: i64 = 0;
        // C s_1_24: const #1s : i
        let s_1_24: i128 = 1;
        // D s_1_25: cast zx s_1_6 -> i
        let s_1_25: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_26: sub s_1_25 s_1_24
        let s_1_26: i128 = ((s_1_25) - (s_1_24));
        // D s_1_27: cast reint s_1_26 -> i64
        let s_1_27: i64 = (s_1_26 as i64);
        // D s_1_28: write-var gs#184388 <= s_1_27
        fn_state.gs_184388 = s_1_27;
        // D s_1_29: write-var e <= s_1_23
        fn_state.e = s_1_23;
        // N s_1_30: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#184388:i64
        let s_2_1: i64 = fn_state.gs_184388;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b5 b3
        if s_2_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16s : i64
        let s_3_0: i64 = 16;
        // D s_3_1: read-var e:i64
        let s_3_1: i64 = fn_state.e;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // C s_3_3: cast zx s_3_0 -> i
        let s_3_3: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_4: read-var operand1:bv
        let s_3_4: Bits = fn_state.operand1;
        // D s_3_5: call Elem_read(s_3_4, s_3_2, s_3_3)
        let s_3_5: Bits = Elem_read(state, tracer, s_3_4, s_3_2, s_3_3);
        // D s_3_6: cast reint s_3_5 -> u16
        let s_3_6: u16 = (s_3_5.value() as u16);
        // C s_3_7: const #16s : i64
        let s_3_7: i64 = 16;
        // D s_3_8: read-var e:i64
        let s_3_8: i64 = fn_state.e;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // C s_3_10: cast zx s_3_7 -> i
        let s_3_10: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_11: read-var operand2:bv
        let s_3_11: Bits = fn_state.operand2;
        // D s_3_12: call Elem_read(s_3_11, s_3_9, s_3_10)
        let s_3_12: Bits = Elem_read(state, tracer, s_3_11, s_3_9, s_3_10);
        // D s_3_13: cast reint s_3_12 -> u16
        let s_3_13: u16 = (s_3_12.value() as u16);
        // C s_3_14: const #() : ()
        let s_3_14: () = ();
        // S s_3_15: call FPCR_read(s_3_14)
        let s_3_15: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_3_14);
        // D s_3_16: cast zx s_3_6 -> bv
        let s_3_16: Bits = Bits::new(s_3_6 as u128, 16u16);
        // D s_3_17: cast zx s_3_13 -> bv
        let s_3_17: Bits = Bits::new(s_3_13 as u128, 16u16);
        // D s_3_18: call BFAdd(s_3_16, s_3_17, s_3_15)
        let s_3_18: Bits = BFAdd(state, tracer, s_3_16, s_3_17, s_3_15);
        // D s_3_19: write-var gs#733591 <= s_3_18
        fn_state.gs_733591 = s_3_18;
        // N s_3_20: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#733591:bv
        let s_4_0: Bits = fn_state.gs_733591;
        // D s_4_1: cast reint s_4_0 -> u16
        let s_4_1: u16 = (s_4_0.value() as u16);
        // D s_4_2: read-var e:i64
        let s_4_2: i64 = fn_state.e;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // C s_4_4: const #16s : i64
        let s_4_4: i64 = 16;
        // C s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: cast zx s_4_1 -> bv
        let s_4_6: Bits = Bits::new(s_4_1 as u128, 16u16);
        // D s_4_7: read-var result:bv
        let s_4_7: Bits = fn_state.result;
        // D s_4_8: call Elem_set(s_4_7, s_4_3, s_4_5, s_4_6)
        let s_4_8: Bits = Elem_set(state, tracer, s_4_7, s_4_3, s_4_5, s_4_6);
        // D s_4_9: write-var result <= s_4_8
        fn_state.result = s_4_8;
        // D s_4_10: read-var e:i64
        let s_4_10: i64 = fn_state.e;
        // C s_4_11: const #1s : i64
        let s_4_11: i64 = 1;
        // D s_4_12: add s_4_10 s_4_11
        let s_4_12: i64 = (s_4_10 + s_4_11);
        // D s_4_13: write-var e <= s_4_12
        fn_state.e = s_4_12;
        // N s_4_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var VLshadow#2430:i64
        let s_5_0: i64 = fn_state.VLshadow_2430;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var d:i64
        let s_5_3: i64 = fn_state.d;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: read-var result:bv
        let s_5_6: Bits = fn_state.result;
        // D s_5_7: call Z_set(s_5_4, s_5_5, s_5_6)
        let s_5_7: () = Z_set(state, tracer, s_5_4, s_5_5, s_5_6);
        // N s_5_8: return
        return;
    }
}
