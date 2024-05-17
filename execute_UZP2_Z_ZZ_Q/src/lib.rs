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
use Zeros::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_UZP2_Z_ZZ_Q<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
    part: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        p: i64,
        gs_195878: i64,
        gs_195870: i64,
        u_3581: i64,
        pairs: i64,
        VLshadow_2929: i64,
        VLshadow_2930: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
        part: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        m,
        n,
        part,
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
        // D s_0_3: write-var VLshadow#2929 <= s_0_2
        fn_state.VLshadow_2929 = s_0_2;
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
        // D s_1_0: read-var VLshadow#2929:i64
        let s_1_0: i64 = fn_state.VLshadow_2929;
        // D s_1_1: write-var VLshadow#2930 <= s_1_0
        fn_state.VLshadow_2930 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esize:i64
        let s_1_3: i64 = fn_state.esize;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) * (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2930:i64
        let s_1_7: i64 = fn_state.VLshadow_2930;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: cmp-lt s_1_8 s_1_9
        let s_1_10: bool = ((s_1_8) < (s_1_9));
        // N s_1_11: branch s_1_10 b9 b2
        if s_1_10 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #2s : i
        let s_2_0: i128 = 2;
        // D s_2_1: read-var esize:i64
        let s_2_1: i64 = fn_state.esize;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: mul s_2_2 s_2_0
        let s_2_3: i128 = ((s_2_2) * (s_2_0));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: read-var VLshadow#2930:i64
        let s_2_5: i64 = fn_state.VLshadow_2930;
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_7: cast zx s_2_4 -> i
        let s_2_7: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_8: div s_2_6 s_2_7
        let s_2_8: i128 = ((s_2_6) / (s_2_7));
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: write-var pairs <= s_2_9
        fn_state.pairs = s_2_9;
        // D s_2_11: read-var VLshadow#2930:i64
        let s_2_11: i64 = fn_state.VLshadow_2930;
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // D s_2_14: read-var n:i64
        let s_2_14: i64 = fn_state.n;
        // D s_2_15: cast zx s_2_14 -> i
        let s_2_15: i128 = (i128::try_from(s_2_14).unwrap());
        // D s_2_16: cast zx s_2_13 -> i
        let s_2_16: i128 = (i128::try_from(s_2_13).unwrap());
        // D s_2_17: call Z_read(s_2_15, s_2_16)
        let s_2_17: Bits = Z_read(state, tracer, s_2_15, s_2_16);
        // D s_2_18: write-var operand1 <= s_2_17
        fn_state.operand1 = s_2_17;
        // D s_2_19: read-var VLshadow#2930:i64
        let s_2_19: i64 = fn_state.VLshadow_2930;
        // D s_2_20: cast zx s_2_19 -> i
        let s_2_20: i128 = (i128::try_from(s_2_19).unwrap());
        // D s_2_21: cast reint s_2_20 -> i64
        let s_2_21: i64 = (s_2_20 as i64);
        // D s_2_22: read-var m:i64
        let s_2_22: i64 = fn_state.m;
        // D s_2_23: cast zx s_2_22 -> i
        let s_2_23: i128 = (i128::try_from(s_2_22).unwrap());
        // D s_2_24: cast zx s_2_21 -> i
        let s_2_24: i128 = (i128::try_from(s_2_21).unwrap());
        // D s_2_25: call Z_read(s_2_23, s_2_24)
        let s_2_25: Bits = Z_read(state, tracer, s_2_23, s_2_24);
        // D s_2_26: write-var operand2 <= s_2_25
        fn_state.operand2 = s_2_25;
        // D s_2_27: read-var VLshadow#2930:i64
        let s_2_27: i64 = fn_state.VLshadow_2930;
        // D s_2_28: cast zx s_2_27 -> i
        let s_2_28: i128 = (i128::try_from(s_2_27).unwrap());
        // D s_2_29: call Zeros(s_2_28)
        let s_2_29: Bits = Zeros(state, tracer, s_2_28);
        // D s_2_30: write-var result <= s_2_29
        fn_state.result = s_2_29;
        // C s_2_31: const #0s : i64
        let s_2_31: i64 = 0;
        // C s_2_32: const #1s : i
        let s_2_32: i128 = 1;
        // D s_2_33: read-var pairs:i64
        let s_2_33: i64 = fn_state.pairs;
        // D s_2_34: cast zx s_2_33 -> i
        let s_2_34: i128 = (i128::try_from(s_2_33).unwrap());
        // D s_2_35: sub s_2_34 s_2_32
        let s_2_35: i128 = ((s_2_34) - (s_2_32));
        // D s_2_36: cast reint s_2_35 -> i64
        let s_2_36: i64 = (s_2_35 as i64);
        // D s_2_37: write-var gs#195870 <= s_2_36
        fn_state.gs_195870 = s_2_36;
        // D s_2_38: write-var p <= s_2_31
        fn_state.p = s_2_31;
        // N s_2_39: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var p:i64
        let s_3_0: i64 = fn_state.p;
        // D s_3_1: read-var gs#195870:i64
        let s_3_1: i64 = fn_state.gs_195870;
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
        // D s_4_0: read-var esize:i64
        let s_4_0: i64 = fn_state.esize;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // C s_4_3: const #2s : i
        let s_4_3: i128 = 2;
        // D s_4_4: read-var p:i64
        let s_4_4: i64 = fn_state.p;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: mul s_4_3 s_4_5
        let s_4_6: i128 = ((s_4_3) * (s_4_5));
        // D s_4_7: cast reint s_4_6 -> i64
        let s_4_7: i64 = (s_4_6 as i64);
        // D s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (i128::try_from(s_4_7).unwrap());
        // D s_4_9: read-var part:i64
        let s_4_9: i64 = fn_state.part;
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_11: add s_4_8 s_4_10
        let s_4_11: i128 = (s_4_8 + s_4_10);
        // D s_4_12: cast reint s_4_11 -> i64
        let s_4_12: i64 = (s_4_11 as i64);
        // D s_4_13: read-var esize:i64
        let s_4_13: i64 = fn_state.esize;
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (i128::try_from(s_4_13).unwrap());
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: cast zx s_4_12 -> i
        let s_4_16: i128 = (i128::try_from(s_4_12).unwrap());
        // D s_4_17: cast zx s_4_15 -> i
        let s_4_17: i128 = (i128::try_from(s_4_15).unwrap());
        // D s_4_18: read-var operand1:bv
        let s_4_18: Bits = fn_state.operand1;
        // D s_4_19: call Elem_read(s_4_18, s_4_16, s_4_17)
        let s_4_19: Bits = Elem_read(state, tracer, s_4_18, s_4_16, s_4_17);
        // D s_4_20: cast reint s_4_19 -> u128
        let s_4_20: u128 = (s_4_19.value() as u128);
        // D s_4_21: read-var p:i64
        let s_4_21: i64 = fn_state.p;
        // D s_4_22: cast zx s_4_21 -> i
        let s_4_22: i128 = (i128::try_from(s_4_21).unwrap());
        // D s_4_23: cast zx s_4_2 -> i
        let s_4_23: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_24: cast zx s_4_20 -> bv
        let s_4_24: Bits = Bits::new(s_4_20 as u128, 128u16);
        // D s_4_25: read-var result:bv
        let s_4_25: Bits = fn_state.result;
        // D s_4_26: call Elem_set(s_4_25, s_4_22, s_4_23, s_4_24)
        let s_4_26: Bits = Elem_set(state, tracer, s_4_25, s_4_22, s_4_23, s_4_24);
        // D s_4_27: write-var result <= s_4_26
        fn_state.result = s_4_26;
        // D s_4_28: read-var p:i64
        let s_4_28: i64 = fn_state.p;
        // C s_4_29: const #1s : i64
        let s_4_29: i64 = 1;
        // D s_4_30: add s_4_28 s_4_29
        let s_4_30: i64 = (s_4_28 + s_4_29);
        // D s_4_31: write-var p <= s_4_30
        fn_state.p = s_4_30;
        // N s_4_32: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i64
        let s_5_0: i64 = 0;
        // C s_5_1: const #1s : i
        let s_5_1: i128 = 1;
        // D s_5_2: read-var pairs:i64
        let s_5_2: i64 = fn_state.pairs;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: sub s_5_3 s_5_1
        let s_5_4: i128 = ((s_5_3) - (s_5_1));
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: write-var gs#195878 <= s_5_5
        fn_state.gs_195878 = s_5_5;
        // D s_5_7: write-var u#3581 <= s_5_0
        fn_state.u_3581 = s_5_0;
        // N s_5_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var u#3581:i64
        let s_6_0: i64 = fn_state.u_3581;
        // D s_6_1: read-var gs#195878:i64
        let s_6_1: i64 = fn_state.gs_195878;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b8 b7
        if s_6_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var pairs:i64
        let s_7_0: i64 = fn_state.pairs;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var u#3581:i64
        let s_7_2: i64 = fn_state.u_3581;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: add s_7_1 s_7_3
        let s_7_4: i128 = (s_7_1 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: read-var esize:i64
        let s_7_6: i64 = fn_state.esize;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: cast reint s_7_7 -> i64
        let s_7_8: i64 = (s_7_7 as i64);
        // C s_7_9: const #2s : i
        let s_7_9: i128 = 2;
        // D s_7_10: read-var u#3581:i64
        let s_7_10: i64 = fn_state.u_3581;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: mul s_7_9 s_7_11
        let s_7_12: i128 = ((s_7_9) * (s_7_11));
        // D s_7_13: cast reint s_7_12 -> i64
        let s_7_13: i64 = (s_7_12 as i64);
        // D s_7_14: cast zx s_7_13 -> i
        let s_7_14: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_15: read-var part:i64
        let s_7_15: i64 = fn_state.part;
        // D s_7_16: cast zx s_7_15 -> i
        let s_7_16: i128 = (i128::try_from(s_7_15).unwrap());
        // D s_7_17: add s_7_14 s_7_16
        let s_7_17: i128 = (s_7_14 + s_7_16);
        // D s_7_18: cast reint s_7_17 -> i64
        let s_7_18: i64 = (s_7_17 as i64);
        // D s_7_19: read-var esize:i64
        let s_7_19: i64 = fn_state.esize;
        // D s_7_20: cast zx s_7_19 -> i
        let s_7_20: i128 = (i128::try_from(s_7_19).unwrap());
        // D s_7_21: cast reint s_7_20 -> i64
        let s_7_21: i64 = (s_7_20 as i64);
        // D s_7_22: cast zx s_7_18 -> i
        let s_7_22: i128 = (i128::try_from(s_7_18).unwrap());
        // D s_7_23: cast zx s_7_21 -> i
        let s_7_23: i128 = (i128::try_from(s_7_21).unwrap());
        // D s_7_24: read-var operand2:bv
        let s_7_24: Bits = fn_state.operand2;
        // D s_7_25: call Elem_read(s_7_24, s_7_22, s_7_23)
        let s_7_25: Bits = Elem_read(state, tracer, s_7_24, s_7_22, s_7_23);
        // D s_7_26: cast reint s_7_25 -> u128
        let s_7_26: u128 = (s_7_25.value() as u128);
        // D s_7_27: cast zx s_7_5 -> i
        let s_7_27: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_28: cast zx s_7_8 -> i
        let s_7_28: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_29: cast zx s_7_26 -> bv
        let s_7_29: Bits = Bits::new(s_7_26 as u128, 128u16);
        // D s_7_30: read-var result:bv
        let s_7_30: Bits = fn_state.result;
        // D s_7_31: call Elem_set(s_7_30, s_7_27, s_7_28, s_7_29)
        let s_7_31: Bits = Elem_set(state, tracer, s_7_30, s_7_27, s_7_28, s_7_29);
        // D s_7_32: write-var result <= s_7_31
        fn_state.result = s_7_31;
        // D s_7_33: read-var u#3581:i64
        let s_7_33: i64 = fn_state.u_3581;
        // C s_7_34: const #1s : i64
        let s_7_34: i64 = 1;
        // D s_7_35: add s_7_33 s_7_34
        let s_7_35: i64 = (s_7_33 + s_7_34);
        // D s_7_36: write-var u#3581 <= s_7_35
        fn_state.u_3581 = s_7_35;
        // N s_7_37: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var VLshadow#2930:i64
        let s_8_0: i64 = fn_state.VLshadow_2930;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: read-var d:i64
        let s_8_3: i64 = fn_state.d;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cast zx s_8_2 -> i
        let s_8_5: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_6: read-var result:bv
        let s_8_6: Bits = fn_state.result;
        // D s_8_7: call Z_set(s_8_4, s_8_5, s_8_6)
        let s_8_7: () = Z_set(state, tracer, s_8_4, s_8_5, s_8_6);
        // N s_8_8: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
}
