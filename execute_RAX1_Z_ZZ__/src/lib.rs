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
use HaveSME2p1::*;
use CheckSVEEnabled::*;
use ROL::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_RAX1_Z_ZZ__<T: Tracer>(
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
        result: Bits,
        gs_217955: i64,
        operand1: Bits,
        VLshadow_4256: i64,
        VLshadow_4257: i64,
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
        // D s_0_3: write-var VLshadow#4256 <= s_0_2
        fn_state.VLshadow_4256 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call HaveSME2p1(s_0_4)
        let s_0_5: bool = HaveSME2p1(state, tracer, s_0_4);
        // N s_0_6: branch s_0_5 b6 b1
        if s_0_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call CheckNonStreamingSVEEnabled(s_1_0)
        let s_1_1: () = CheckNonStreamingSVEEnabled(state, tracer, s_1_0);
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#4256:i64
        let s_2_0: i64 = fn_state.VLshadow_4256;
        // D s_2_1: write-var VLshadow#4257 <= s_2_0
        fn_state.VLshadow_4257 = s_2_0;
        // C s_2_2: const #64s : i
        let s_2_2: i128 = 64;
        // D s_2_3: read-var VLshadow#4257:i64
        let s_2_3: i64 = fn_state.VLshadow_4257;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: div s_2_4 s_2_2
        let s_2_5: i128 = ((s_2_4) / (s_2_2));
        // D s_2_6: cast reint s_2_5 -> i64
        let s_2_6: i64 = (s_2_5 as i64);
        // D s_2_7: read-var VLshadow#4257:i64
        let s_2_7: i64 = fn_state.VLshadow_4257;
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: read-var n:i64
        let s_2_10: i64 = fn_state.n;
        // D s_2_11: cast zx s_2_10 -> i
        let s_2_11: i128 = (i128::try_from(s_2_10).unwrap());
        // D s_2_12: cast zx s_2_9 -> i
        let s_2_12: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_13: call Z_read(s_2_11, s_2_12)
        let s_2_13: Bits = Z_read(state, tracer, s_2_11, s_2_12);
        // D s_2_14: write-var operand1 <= s_2_13
        fn_state.operand1 = s_2_13;
        // D s_2_15: read-var VLshadow#4257:i64
        let s_2_15: i64 = fn_state.VLshadow_4257;
        // D s_2_16: cast zx s_2_15 -> i
        let s_2_16: i128 = (i128::try_from(s_2_15).unwrap());
        // D s_2_17: cast reint s_2_16 -> i64
        let s_2_17: i64 = (s_2_16 as i64);
        // D s_2_18: read-var m:i64
        let s_2_18: i64 = fn_state.m;
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (i128::try_from(s_2_18).unwrap());
        // D s_2_20: cast zx s_2_17 -> i
        let s_2_20: i128 = (i128::try_from(s_2_17).unwrap());
        // D s_2_21: call Z_read(s_2_19, s_2_20)
        let s_2_21: Bits = Z_read(state, tracer, s_2_19, s_2_20);
        // D s_2_22: write-var operand2 <= s_2_21
        fn_state.operand2 = s_2_21;
        // C s_2_23: const #0s : i64
        let s_2_23: i64 = 0;
        // C s_2_24: const #1s : i
        let s_2_24: i128 = 1;
        // D s_2_25: cast zx s_2_6 -> i
        let s_2_25: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_26: sub s_2_25 s_2_24
        let s_2_26: i128 = ((s_2_25) - (s_2_24));
        // D s_2_27: cast reint s_2_26 -> i64
        let s_2_27: i64 = (s_2_26 as i64);
        // D s_2_28: write-var gs#217955 <= s_2_27
        fn_state.gs_217955 = s_2_27;
        // D s_2_29: write-var e <= s_2_23
        fn_state.e = s_2_23;
        // N s_2_30: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: read-var gs#217955:i64
        let s_3_1: i64 = fn_state.gs_217955;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b5 b4
        if s_3_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #64s : i64
        let s_4_0: i64 = 64;
        // D s_4_1: read-var e:i64
        let s_4_1: i64 = fn_state.e;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // C s_4_3: cast zx s_4_0 -> i
        let s_4_3: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_4: read-var operand1:bv
        let s_4_4: Bits = fn_state.operand1;
        // D s_4_5: call Elem_read(s_4_4, s_4_2, s_4_3)
        let s_4_5: Bits = Elem_read(state, tracer, s_4_4, s_4_2, s_4_3);
        // D s_4_6: cast reint s_4_5 -> u64
        let s_4_6: u64 = (s_4_5.value() as u64);
        // C s_4_7: const #64s : i64
        let s_4_7: i64 = 64;
        // D s_4_8: read-var e:i64
        let s_4_8: i64 = fn_state.e;
        // D s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (i128::try_from(s_4_8).unwrap());
        // C s_4_10: cast zx s_4_7 -> i
        let s_4_10: i128 = (i128::try_from(s_4_7).unwrap());
        // D s_4_11: read-var operand2:bv
        let s_4_11: Bits = fn_state.operand2;
        // D s_4_12: call Elem_read(s_4_11, s_4_9, s_4_10)
        let s_4_12: Bits = Elem_read(state, tracer, s_4_11, s_4_9, s_4_10);
        // D s_4_13: cast reint s_4_12 -> u64
        let s_4_13: u64 = (s_4_12.value() as u64);
        // C s_4_14: const #64s : i64
        let s_4_14: i64 = 64;
        // C s_4_15: const #1s : i
        let s_4_15: i128 = 1;
        // D s_4_16: cast zx s_4_13 -> bv
        let s_4_16: Bits = Bits::new(s_4_13 as u128, 64u16);
        // D s_4_17: call ROL(s_4_16, s_4_15)
        let s_4_17: Bits = ROL(state, tracer, s_4_16, s_4_15);
        // D s_4_18: cast reint s_4_17 -> u64
        let s_4_18: u64 = (s_4_17.value() as u64);
        // D s_4_19: cast zx s_4_6 -> bv
        let s_4_19: Bits = Bits::new(s_4_6 as u128, 64u16);
        // D s_4_20: cast zx s_4_18 -> bv
        let s_4_20: Bits = Bits::new(s_4_18 as u128, 64u16);
        // D s_4_21: xor s_4_19 s_4_20
        let s_4_21: Bits = ((s_4_19) ^ (s_4_20));
        // D s_4_22: cast reint s_4_21 -> u64
        let s_4_22: u64 = (s_4_21.value() as u64);
        // D s_4_23: read-var e:i64
        let s_4_23: i64 = fn_state.e;
        // D s_4_24: cast zx s_4_23 -> i
        let s_4_24: i128 = (i128::try_from(s_4_23).unwrap());
        // C s_4_25: cast zx s_4_14 -> i
        let s_4_25: i128 = (i128::try_from(s_4_14).unwrap());
        // D s_4_26: cast zx s_4_22 -> bv
        let s_4_26: Bits = Bits::new(s_4_22 as u128, 64u16);
        // D s_4_27: read-var result:bv
        let s_4_27: Bits = fn_state.result;
        // D s_4_28: call Elem_set(s_4_27, s_4_24, s_4_25, s_4_26)
        let s_4_28: Bits = Elem_set(state, tracer, s_4_27, s_4_24, s_4_25, s_4_26);
        // D s_4_29: write-var result <= s_4_28
        fn_state.result = s_4_28;
        // D s_4_30: read-var e:i64
        let s_4_30: i64 = fn_state.e;
        // C s_4_31: const #1s : i64
        let s_4_31: i64 = 1;
        // D s_4_32: add s_4_30 s_4_31
        let s_4_32: i64 = (s_4_30 + s_4_31);
        // D s_4_33: write-var e <= s_4_32
        fn_state.e = s_4_32;
        // N s_4_34: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var VLshadow#4257:i64
        let s_5_0: i64 = fn_state.VLshadow_4257;
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
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call CheckSVEEnabled(s_6_0)
        let s_6_1: () = CheckSVEEnabled(state, tracer, s_6_0);
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
