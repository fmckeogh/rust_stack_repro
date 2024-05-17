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
use AESSubBytes::*;
use Z_set::*;
use Elem_set::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use AESShiftRows::*;
use common::*;
pub fn execute_AESE_Z_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_217836: i64,
        s: i64,
        VLshadow_4251: i64,
        VLshadow_4250: i64,
        result: Bits,
        VL: i64,
        dn: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        m,
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
        // D s_0_3: write-var VLshadow#4250 <= s_0_2
        fn_state.VLshadow_4250 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckNonStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#4250:i64
        let s_1_0: i64 = fn_state.VLshadow_4250;
        // D s_1_1: write-var VLshadow#4251 <= s_1_0
        fn_state.VLshadow_4251 = s_1_0;
        // C s_1_2: const #128s : i
        let s_1_2: i128 = 128;
        // D s_1_3: read-var VLshadow#4251:i64
        let s_1_3: i64 = fn_state.VLshadow_4251;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#4251:i64
        let s_1_7: i64 = fn_state.VLshadow_4251;
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
        // D s_1_14: read-var VLshadow#4251:i64
        let s_1_14: i64 = fn_state.VLshadow_4251;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: read-var m:i64
        let s_1_17: i64 = fn_state.m;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: cast zx s_1_16 -> i
        let s_1_19: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_20: call Z_read(s_1_18, s_1_19)
        let s_1_20: Bits = Z_read(state, tracer, s_1_18, s_1_19);
        // D s_1_21: xor s_1_13 s_1_20
        let s_1_21: Bits = ((s_1_13) ^ (s_1_20));
        // D s_1_22: write-var result <= s_1_21
        fn_state.result = s_1_21;
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
        // D s_1_28: write-var gs#217836 <= s_1_27
        fn_state.gs_217836 = s_1_27;
        // D s_1_29: write-var s <= s_1_23
        fn_state.s = s_1_23;
        // N s_1_30: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var s:i64
        let s_2_0: i64 = fn_state.s;
        // D s_2_1: read-var gs#217836:i64
        let s_2_1: i64 = fn_state.gs_217836;
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
        // C s_3_0: const #128s : i64
        let s_3_0: i64 = 128;
        // C s_3_1: const #128s : i64
        let s_3_1: i64 = 128;
        // D s_3_2: read-var s:i64
        let s_3_2: i64 = fn_state.s;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // C s_3_4: cast zx s_3_1 -> i
        let s_3_4: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_5: read-var result:bv
        let s_3_5: Bits = fn_state.result;
        // D s_3_6: call Elem_read(s_3_5, s_3_3, s_3_4)
        let s_3_6: Bits = Elem_read(state, tracer, s_3_5, s_3_3, s_3_4);
        // D s_3_7: cast reint s_3_6 -> u128
        let s_3_7: u128 = (s_3_6.value() as u128);
        // D s_3_8: call AESShiftRows(s_3_7)
        let s_3_8: u128 = AESShiftRows(state, tracer, s_3_7);
        // D s_3_9: call AESSubBytes(s_3_8)
        let s_3_9: u128 = AESSubBytes(state, tracer, s_3_8);
        // D s_3_10: read-var s:i64
        let s_3_10: i64 = fn_state.s;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // C s_3_12: cast zx s_3_0 -> i
        let s_3_12: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_13: cast zx s_3_9 -> bv
        let s_3_13: Bits = Bits::new(s_3_9 as u128, 128u16);
        // D s_3_14: read-var result:bv
        let s_3_14: Bits = fn_state.result;
        // D s_3_15: call Elem_set(s_3_14, s_3_11, s_3_12, s_3_13)
        let s_3_15: Bits = Elem_set(state, tracer, s_3_14, s_3_11, s_3_12, s_3_13);
        // D s_3_16: write-var result <= s_3_15
        fn_state.result = s_3_15;
        // D s_3_17: read-var s:i64
        let s_3_17: i64 = fn_state.s;
        // C s_3_18: const #1s : i64
        let s_3_18: i64 = 1;
        // D s_3_19: add s_3_17 s_3_18
        let s_3_19: i64 = (s_3_17 + s_3_18);
        // D s_3_20: write-var s <= s_3_19
        fn_state.s = s_3_19;
        // N s_3_21: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#4251:i64
        let s_4_0: i64 = fn_state.VLshadow_4251;
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
