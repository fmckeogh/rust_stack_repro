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
use u__id::*;
use CheckSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_EXT_Z_ZI_Des<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dst: i64,
    esize: i64,
    position__arg: i64,
    s1: i64,
    s2: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_196453: bool,
        VLshadow_2971: i64,
        concat: Bits,
        positionshadow_2973: i128,
        VLshadow_2972: i64,
        operand1: Bits,
        operand2: Bits,
        position: i128,
        VL: i64,
        dst: i64,
        esize: i64,
        position__arg: i64,
        s1: i64,
        s2: i64,
    }
    let fn_state = FunctionState {
        VL,
        dst,
        esize,
        position__arg,
        s1,
        s2,
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
        // D s_0_3: write-var VLshadow#2971 <= s_0_2
        fn_state.VLshadow_2971 = s_0_2;
        // D s_0_4: read-var position__arg:i64
        let s_0_4: i64 = fn_state.position__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var position <= s_0_5
        fn_state.position = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call CheckSVEEnabled(s_0_7)
        let s_0_8: () = CheckSVEEnabled(state, tracer, s_0_7);
        // N s_0_9: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2971:i64
        let s_1_0: i64 = fn_state.VLshadow_2971;
        // D s_1_1: write-var VLshadow#2972 <= s_1_0
        fn_state.VLshadow_2972 = s_1_0;
        // D s_1_2: read-var VLshadow#2972:i64
        let s_1_2: i64 = fn_state.VLshadow_2972;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esize:i64
        let s_1_4: i64 = fn_state.esize;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var VLshadow#2972:i64
        let s_1_8: i64 = fn_state.VLshadow_2972;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var s1:i64
        let s_1_11: i64 = fn_state.s1;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast zx s_1_10 -> i
        let s_1_13: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_14: call Z_read(s_1_12, s_1_13)
        let s_1_14: Bits = Z_read(state, tracer, s_1_12, s_1_13);
        // D s_1_15: write-var operand1 <= s_1_14
        fn_state.operand1 = s_1_14;
        // D s_1_16: read-var VLshadow#2972:i64
        let s_1_16: i64 = fn_state.VLshadow_2972;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: read-var s2:i64
        let s_1_19: i64 = fn_state.s2;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast zx s_1_18 -> i
        let s_1_21: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_22: call Z_read(s_1_20, s_1_21)
        let s_1_22: Bits = Z_read(state, tracer, s_1_20, s_1_21);
        // D s_1_23: write-var operand2 <= s_1_22
        fn_state.operand2 = s_1_22;
        // D s_1_24: cast zx s_1_7 -> i
        let s_1_24: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_25: read-var position:i
        let s_1_25: i128 = fn_state.position;
        // D s_1_26: cmp-ge s_1_25 s_1_24
        let s_1_26: bool = ((s_1_25) >= (s_1_24));
        // N s_1_27: branch s_1_26 b7 b2
        if s_1_26 {
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
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #3s : i
        let s_3_0: i128 = 3;
        // D s_3_1: read-var position:i
        let s_3_1: i128 = fn_state.position;
        // D s_3_2: lsl s_3_1 s_3_0
        let s_3_2: i128 = s_3_1 << s_3_0;
        // D s_3_3: write-var positionshadow#2973 <= s_3_2
        fn_state.positionshadow_2973 = s_3_2;
        // D s_3_4: read-var operand2:bv
        let s_3_4: Bits = fn_state.operand2;
        // D s_3_5: read-var operand1:bv
        let s_3_5: Bits = fn_state.operand1;
        // D s_3_6: cast reint s_3_4 -> u128
        let s_3_6: u128 = (s_3_4.value() as u128);
        // D s_3_7: size-of s_3_4
        let s_3_7: u16 = s_3_4.length();
        // D s_3_8: cast reint s_3_5 -> u128
        let s_3_8: u128 = (s_3_5.value() as u128);
        // D s_3_9: size-of s_3_5
        let s_3_9: u16 = s_3_5.length();
        // D s_3_10: lsl s_3_6 s_3_9
        let s_3_10: u128 = s_3_6 << s_3_9;
        // D s_3_11: or s_3_10 s_3_8
        let s_3_11: u128 = ((s_3_10) | (s_3_8));
        // D s_3_12: add s_3_7 s_3_9
        let s_3_12: u16 = (s_3_7 + s_3_9);
        // D s_3_13: create-bits s_3_11 s_3_12
        let s_3_13: Bits = Bits::new(s_3_11, s_3_12);
        // D s_3_14: write-var concat <= s_3_13
        fn_state.concat = s_3_13;
        // D s_3_15: read-var positionshadow#2973:i
        let s_3_15: i128 = fn_state.positionshadow_2973;
        // D s_3_16: call __id(s_3_15)
        let s_3_16: i128 = u__id(state, tracer, s_3_15);
        // C s_3_17: const #0s : i
        let s_3_17: i128 = 0;
        // D s_3_18: cmp-le s_3_17 s_3_16
        let s_3_18: bool = ((s_3_17) <= (s_3_16));
        // N s_3_19: branch s_3_18 b6 b4
        if s_3_18 {
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
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#196453 <= s_4_0
        fn_state.gs_196453 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#196453:u8
        let s_5_0: bool = fn_state.gs_196453;
        // N s_5_1: assert s_5_0
        let s_5_1: () = assert!(s_5_0);
        // D s_5_2: read-var VLshadow#2972:i64
        let s_5_2: i64 = fn_state.VLshadow_2972;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: read-var VLshadow#2972:i64
        let s_5_5: i64 = fn_state.VLshadow_2972;
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: read-var positionshadow#2973:i
        let s_5_7: i128 = fn_state.positionshadow_2973;
        // D s_5_8: add s_5_7 s_5_6
        let s_5_8: i128 = (s_5_7 + s_5_6);
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // C s_5_10: const #1s : i
        let s_5_10: i128 = 1;
        // D s_5_11: cast zx s_5_9 -> i
        let s_5_11: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_12: sub s_5_11 s_5_10
        let s_5_12: i128 = ((s_5_11) - (s_5_10));
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_15: read-var concat:bv
        let s_5_15: Bits = fn_state.concat;
        // D s_5_16: read-var positionshadow#2973:i
        let s_5_16: i128 = fn_state.positionshadow_2973;
        // C s_5_17: const #1s : i64
        let s_5_17: i64 = 1;
        // C s_5_18: cast zx s_5_17 -> i
        let s_5_18: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_19: sub s_5_14 s_5_16
        let s_5_19: i128 = ((s_5_14) - (s_5_16));
        // D s_5_20: add s_5_19 s_5_18
        let s_5_20: i128 = (s_5_19 + s_5_18);
        // D s_5_21: bit-extract s_5_15 s_5_16 s_5_20
        let s_5_21: Bits = (Bits::new(
            ((s_5_15) >> (s_5_16)).value(),
            u16::try_from(s_5_20).unwrap(),
        ));
        // D s_5_22: read-var dst:i64
        let s_5_22: i64 = fn_state.dst;
        // D s_5_23: cast zx s_5_22 -> i
        let s_5_23: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_24: cast zx s_5_4 -> i
        let s_5_24: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_25: call Z_set(s_5_23, s_5_24, s_5_21)
        let s_5_25: () = Z_set(state, tracer, s_5_23, s_5_24, s_5_21);
        // N s_5_26: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var positionshadow#2973:i
        let s_6_0: i128 = fn_state.positionshadow_2973;
        // D s_6_1: call __id(s_6_0)
        let s_6_1: i128 = u__id(state, tracer, s_6_0);
        // D s_6_2: read-var VLshadow#2972:i64
        let s_6_2: i64 = fn_state.VLshadow_2972;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: call __id(s_6_3)
        let s_6_4: i128 = u__id(state, tracer, s_6_3);
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: add s_6_1 s_6_6
        let s_6_7: i128 = (s_6_1 + s_6_6);
        // C s_6_8: const #1s : i
        let s_6_8: i128 = 1;
        // D s_6_9: sub s_6_7 s_6_8
        let s_6_9: i128 = ((s_6_7) - (s_6_8));
        // D s_6_10: read-var VLshadow#2972:i64
        let s_6_10: i64 = fn_state.VLshadow_2972;
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_12: call __id(s_6_11)
        let s_6_12: i128 = u__id(state, tracer, s_6_11);
        // D s_6_13: cast reint s_6_12 -> i64
        let s_6_13: i64 = (s_6_12 as i64);
        // C s_6_14: const #2s : i
        let s_6_14: i128 = 2;
        // D s_6_15: cast zx s_6_13 -> i
        let s_6_15: i128 = (i128::try_from(s_6_13).unwrap());
        // D s_6_16: mul s_6_15 s_6_14
        let s_6_16: i128 = ((s_6_15) * (s_6_14));
        // D s_6_17: cast reint s_6_16 -> i64
        let s_6_17: i64 = (s_6_16 as i64);
        // D s_6_18: cast zx s_6_17 -> i
        let s_6_18: i128 = (i128::try_from(s_6_17).unwrap());
        // D s_6_19: cmp-lt s_6_9 s_6_18
        let s_6_19: bool = ((s_6_9) < (s_6_18));
        // D s_6_20: write-var gs#196453 <= s_6_19
        fn_state.gs_196453 = s_6_19;
        // N s_6_21: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0s : i
        let s_7_0: i128 = 0;
        // D s_7_1: write-var position <= s_7_0
        fn_state.position = s_7_0;
        // N s_7_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
