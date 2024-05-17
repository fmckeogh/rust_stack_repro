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
use u__id::*;
use Z_set::*;
use common::*;
pub fn execute_EXTQ_Z_ZI_Des<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    m: i64,
    position: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_4353: i64,
        s: i64,
        concat: u64,
        gs_220190: bool,
        result: Bits,
        VLshadow_4352: i64,
        operand1: Bits,
        gs_220182: i64,
        operand2: Bits,
        VL: i64,
        dn: i64,
        m: i64,
        position: i128,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        m,
        position,
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
        // D s_0_3: write-var VLshadow#4352 <= s_0_2
        fn_state.VLshadow_4352 = s_0_2;
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
        // D s_1_0: read-var VLshadow#4352:i64
        let s_1_0: i64 = fn_state.VLshadow_4352;
        // D s_1_1: write-var VLshadow#4353 <= s_1_0
        fn_state.VLshadow_4353 = s_1_0;
        // C s_1_2: const #128s : i
        let s_1_2: i128 = 128;
        // D s_1_3: read-var VLshadow#4353:i64
        let s_1_3: i64 = fn_state.VLshadow_4353;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#4353:i64
        let s_1_7: i64 = fn_state.VLshadow_4353;
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
        // D s_1_14: write-var operand1 <= s_1_13
        fn_state.operand1 = s_1_13;
        // D s_1_15: read-var VLshadow#4353:i64
        let s_1_15: i64 = fn_state.VLshadow_4353;
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
        // D s_1_28: write-var gs#220182 <= s_1_27
        fn_state.gs_220182 = s_1_27;
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
        // D s_2_1: read-var gs#220182:i64
        let s_2_1: i64 = fn_state.gs_220182;
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
        // C s_3_0: const #128s : i64
        let s_3_0: i64 = 128;
        // D s_3_1: read-var s:i64
        let s_3_1: i64 = fn_state.s;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // C s_3_3: cast zx s_3_0 -> i
        let s_3_3: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_4: read-var operand2:bv
        let s_3_4: Bits = fn_state.operand2;
        // D s_3_5: call Elem_read(s_3_4, s_3_2, s_3_3)
        let s_3_5: Bits = Elem_read(state, tracer, s_3_4, s_3_2, s_3_3);
        // D s_3_6: cast reint s_3_5 -> u128
        let s_3_6: u128 = (s_3_5.value() as u128);
        // C s_3_7: const #128s : i64
        let s_3_7: i64 = 128;
        // D s_3_8: read-var s:i64
        let s_3_8: i64 = fn_state.s;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // C s_3_10: cast zx s_3_7 -> i
        let s_3_10: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_11: read-var operand1:bv
        let s_3_11: Bits = fn_state.operand1;
        // D s_3_12: call Elem_read(s_3_11, s_3_9, s_3_10)
        let s_3_12: Bits = Elem_read(state, tracer, s_3_11, s_3_9, s_3_10);
        // D s_3_13: cast reint s_3_12 -> u128
        let s_3_13: u128 = (s_3_12.value() as u128);
        // D s_3_14: cast zx s_3_6 -> bv
        let s_3_14: Bits = Bits::new(s_3_6 as u128, 128u16);
        // D s_3_15: cast zx s_3_13 -> bv
        let s_3_15: Bits = Bits::new(s_3_13 as u128, 128u16);
        // D s_3_16: cast reint s_3_14 -> u128
        let s_3_16: u128 = (s_3_14.value() as u128);
        // D s_3_17: size-of s_3_14
        let s_3_17: u16 = s_3_14.length();
        // D s_3_18: cast reint s_3_15 -> u128
        let s_3_18: u128 = (s_3_15.value() as u128);
        // D s_3_19: size-of s_3_15
        let s_3_19: u16 = s_3_15.length();
        // D s_3_20: lsl s_3_16 s_3_19
        let s_3_20: u128 = s_3_16 << s_3_19;
        // D s_3_21: or s_3_20 s_3_18
        let s_3_21: u128 = ((s_3_20) | (s_3_18));
        // D s_3_22: add s_3_17 s_3_19
        let s_3_22: u16 = (s_3_17 + s_3_19);
        // D s_3_23: create-bits s_3_21 s_3_22
        let s_3_23: Bits = Bits::new(s_3_21, s_3_22);
        // D s_3_24: cast reint s_3_23 -> u256
        let s_3_24: u64 = (s_3_23.value() as u64);
        // D s_3_25: write-var concat <= s_3_24
        fn_state.concat = s_3_24;
        // D s_3_26: read-var position:i
        let s_3_26: i128 = fn_state.position;
        // D s_3_27: call __id(s_3_26)
        let s_3_27: i128 = u__id(state, tracer, s_3_26);
        // C s_3_28: const #0s : i
        let s_3_28: i128 = 0;
        // D s_3_29: cmp-le s_3_28 s_3_27
        let s_3_29: bool = ((s_3_28) <= (s_3_27));
        // N s_3_30: branch s_3_29 b6 b4
        if s_3_29 {
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
        // D s_4_1: write-var gs#220190 <= s_4_0
        fn_state.gs_220190 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#220190:u8
        let s_5_0: bool = fn_state.gs_220190;
        // N s_5_1: assert s_5_0
        let s_5_1: () = assert!(s_5_0);
        // C s_5_2: const #128s : i64
        let s_5_2: i64 = 128;
        // C s_5_3: const #128s : i
        let s_5_3: i128 = 128;
        // D s_5_4: read-var concat:u256
        let s_5_4: u64 = fn_state.concat;
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 256u16);
        // D s_5_6: read-var position:i
        let s_5_6: i128 = fn_state.position;
        // D s_5_7: bit-extract s_5_5 s_5_6 s_5_3
        let s_5_7: Bits = (Bits::new(
            ((s_5_5) >> (s_5_6)).value(),
            u16::try_from(s_5_3).unwrap(),
        ));
        // D s_5_8: cast reint s_5_7 -> u128
        let s_5_8: u128 = (s_5_7.value() as u128);
        // D s_5_9: read-var s:i64
        let s_5_9: i64 = fn_state.s;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // C s_5_11: cast zx s_5_2 -> i
        let s_5_11: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_12: cast zx s_5_8 -> bv
        let s_5_12: Bits = Bits::new(s_5_8 as u128, 128u16);
        // D s_5_13: read-var result:bv
        let s_5_13: Bits = fn_state.result;
        // D s_5_14: call Elem_set(s_5_13, s_5_10, s_5_11, s_5_12)
        let s_5_14: Bits = Elem_set(state, tracer, s_5_13, s_5_10, s_5_11, s_5_12);
        // D s_5_15: write-var result <= s_5_14
        fn_state.result = s_5_14;
        // D s_5_16: read-var s:i64
        let s_5_16: i64 = fn_state.s;
        // C s_5_17: const #1s : i64
        let s_5_17: i64 = 1;
        // D s_5_18: add s_5_16 s_5_17
        let s_5_18: i64 = (s_5_16 + s_5_17);
        // D s_5_19: write-var s <= s_5_18
        fn_state.s = s_5_18;
        // N s_5_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var position:i
        let s_6_0: i128 = fn_state.position;
        // D s_6_1: call __id(s_6_0)
        let s_6_1: i128 = u__id(state, tracer, s_6_0);
        // C s_6_2: const #127s : i
        let s_6_2: i128 = 127;
        // D s_6_3: add s_6_1 s_6_2
        let s_6_3: i128 = (s_6_1 + s_6_2);
        // C s_6_4: const #256s : i
        let s_6_4: i128 = 256;
        // D s_6_5: cmp-lt s_6_3 s_6_4
        let s_6_5: bool = ((s_6_3) < (s_6_4));
        // D s_6_6: write-var gs#220190 <= s_6_5
        fn_state.gs_220190 = s_6_5;
        // N s_6_7: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VLshadow#4353:i64
        let s_7_0: i64 = fn_state.VLshadow_4353;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var dn:i64
        let s_7_3: i64 = fn_state.dn;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast zx s_7_2 -> i
        let s_7_5: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_6: read-var result:bv
        let s_7_6: Bits = fn_state.result;
        // D s_7_7: call Z_set(s_7_4, s_7_5, s_7_6)
        let s_7_7: () = Z_set(state, tracer, s_7_4, s_7_5, s_7_6);
        // N s_7_8: return
        return;
    }
}
