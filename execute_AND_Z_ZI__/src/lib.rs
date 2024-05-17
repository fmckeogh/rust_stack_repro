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
pub fn execute_AND_Z_ZI__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    imm: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        VLshadow_2862: i64,
        gs_194183: i64,
        result: Bits,
        VLshadow_2861: i64,
        VL: i64,
        dn: i64,
        imm: u64,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        imm,
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
        // D s_0_3: write-var VLshadow#2861 <= s_0_2
        fn_state.VLshadow_2861 = s_0_2;
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
        // D s_1_0: read-var VLshadow#2861:i64
        let s_1_0: i64 = fn_state.VLshadow_2861;
        // D s_1_1: write-var VLshadow#2862 <= s_1_0
        fn_state.VLshadow_2862 = s_1_0;
        // C s_1_2: const #64s : i
        let s_1_2: i128 = 64;
        // D s_1_3: read-var VLshadow#2862:i64
        let s_1_3: i64 = fn_state.VLshadow_2862;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2862:i64
        let s_1_7: i64 = fn_state.VLshadow_2862;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var dn:i64
        let s_1_10: i64 = fn_state.dn;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: cast zx s_1_9 -> i
        let s_1_12: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_13: call Z_read(s_1_11, s_1_12)
        let s_1_13: Bits = Z_read(state, tracer, s_1_11, s_1_12);
        // D s_1_14: write-var operand <= s_1_13
        fn_state.operand = s_1_13;
        // C s_1_15: const #0s : i64
        let s_1_15: i64 = 0;
        // C s_1_16: const #1s : i
        let s_1_16: i128 = 1;
        // D s_1_17: cast zx s_1_6 -> i
        let s_1_17: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_18: sub s_1_17 s_1_16
        let s_1_18: i128 = ((s_1_17) - (s_1_16));
        // D s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // D s_1_20: write-var gs#194183 <= s_1_19
        fn_state.gs_194183 = s_1_19;
        // D s_1_21: write-var e <= s_1_15
        fn_state.e = s_1_15;
        // N s_1_22: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#194183:i64
        let s_2_1: i64 = fn_state.gs_194183;
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
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: read-var e:i64
        let s_3_1: i64 = fn_state.e;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // C s_3_3: cast zx s_3_0 -> i
        let s_3_3: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_4: read-var operand:bv
        let s_3_4: Bits = fn_state.operand;
        // D s_3_5: call Elem_read(s_3_4, s_3_2, s_3_3)
        let s_3_5: Bits = Elem_read(state, tracer, s_3_4, s_3_2, s_3_3);
        // D s_3_6: cast reint s_3_5 -> u64
        let s_3_6: u64 = (s_3_5.value() as u64);
        // C s_3_7: const #64s : i64
        let s_3_7: i64 = 64;
        // D s_3_8: cast zx s_3_6 -> bv
        let s_3_8: Bits = Bits::new(s_3_6 as u128, 64u16);
        // D s_3_9: read-var imm:u64
        let s_3_9: u64 = fn_state.imm;
        // D s_3_10: cast zx s_3_9 -> bv
        let s_3_10: Bits = Bits::new(s_3_9 as u128, 64u16);
        // D s_3_11: and s_3_8 s_3_10
        let s_3_11: Bits = ((s_3_8) & (s_3_10));
        // D s_3_12: cast reint s_3_11 -> u64
        let s_3_12: u64 = (s_3_11.value() as u64);
        // D s_3_13: read-var e:i64
        let s_3_13: i64 = fn_state.e;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // C s_3_15: cast zx s_3_7 -> i
        let s_3_15: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_16: cast zx s_3_12 -> bv
        let s_3_16: Bits = Bits::new(s_3_12 as u128, 64u16);
        // D s_3_17: read-var result:bv
        let s_3_17: Bits = fn_state.result;
        // D s_3_18: call Elem_set(s_3_17, s_3_14, s_3_15, s_3_16)
        let s_3_18: Bits = Elem_set(state, tracer, s_3_17, s_3_14, s_3_15, s_3_16);
        // D s_3_19: write-var result <= s_3_18
        fn_state.result = s_3_18;
        // D s_3_20: read-var e:i64
        let s_3_20: i64 = fn_state.e;
        // C s_3_21: const #1s : i64
        let s_3_21: i64 = 1;
        // D s_3_22: add s_3_20 s_3_21
        let s_3_22: i64 = (s_3_20 + s_3_21);
        // D s_3_23: write-var e <= s_3_22
        fn_state.e = s_3_22;
        // N s_3_24: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2862:i64
        let s_4_0: i64 = fn_state.VLshadow_2862;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var dn:i64
        let s_4_3: i64 = fn_state.dn;
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
