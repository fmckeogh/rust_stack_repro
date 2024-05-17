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
use ConditionPassed::*;
use execute_aarch32_instrs_STLEXD_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_STLEXD_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rn: u8,
    Rt: u8,
    Rt2: u8,
    Rd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_325003: bool,
        t: i64,
        t2: i64,
        gs_325002: bool,
        gs_324999: bool,
        gs_324997: bool,
        gs_325001: bool,
        n: i64,
        d: i64,
        Rn: u8,
        Rt: u8,
        Rt2: u8,
        Rd: u8,
    }
    let fn_state = FunctionState {
        Rn,
        Rt,
        Rt2,
        Rd,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Rd:u8
        let s_2_0: u8 = fn_state.Rd;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var d <= s_2_3
        fn_state.d = s_2_3;
        // D s_2_5: read-var Rt:u8
        let s_2_5: u8 = fn_state.Rt;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 4u16);
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (s_2_6.value() as i128);
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // D s_2_9: write-var t <= s_2_8
        fn_state.t = s_2_8;
        // D s_2_10: read-var Rt2:u8
        let s_2_10: u8 = fn_state.Rt2;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 4u16);
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (s_2_11.value() as i128);
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // D s_2_14: write-var t2 <= s_2_13
        fn_state.t2 = s_2_13;
        // D s_2_15: read-var Rn:u8
        let s_2_15: u8 = fn_state.Rn;
        // D s_2_16: cast zx s_2_15 -> bv
        let s_2_16: Bits = Bits::new(s_2_15 as u128, 4u16);
        // D s_2_17: cast zx s_2_16 -> i
        let s_2_17: i128 = (s_2_16.value() as i128);
        // D s_2_18: cast reint s_2_17 -> i64
        let s_2_18: i64 = (s_2_17 as i64);
        // D s_2_19: write-var n <= s_2_18
        fn_state.n = s_2_18;
        // C s_2_20: const #15s : i
        let s_2_20: i128 = 15;
        // D s_2_21: read-var d:i64
        let s_2_21: i64 = fn_state.d;
        // D s_2_22: cast zx s_2_21 -> i
        let s_2_22: i128 = (i128::try_from(s_2_21).unwrap());
        // D s_2_23: cmp-eq s_2_22 s_2_20
        let s_2_23: bool = ((s_2_22) == (s_2_20));
        // N s_2_24: branch s_2_23 b21 b3
        if s_2_23 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #15s : i
        let s_3_0: i128 = 15;
        // D s_3_1: read-var t:i64
        let s_3_1: i64 = fn_state.t;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // D s_3_4: write-var gs#324997 <= s_3_3
        fn_state.gs_324997 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#324997:u8
        let s_4_0: bool = fn_state.gs_324997;
        // N s_4_1: branch s_4_0 b20 b5
        if s_4_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #15s : i
        let s_5_0: i128 = 15;
        // D s_5_1: read-var t2:i64
        let s_5_1: i64 = fn_state.t2;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // D s_5_4: write-var gs#324999 <= s_5_3
        fn_state.gs_324999 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#324999:u8
        let s_6_0: bool = fn_state.gs_324999;
        // N s_6_1: branch s_6_0 b19 b7
        if s_6_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #15s : i
        let s_7_0: i128 = 15;
        // D s_7_1: read-var n:i64
        let s_7_1: i64 = fn_state.n;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_0
        let s_7_3: bool = ((s_7_2) == (s_7_0));
        // D s_7_4: write-var gs#325001 <= s_7_3
        fn_state.gs_325001 = s_7_3;
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#325001:u8
        let s_8_0: bool = fn_state.gs_325001;
        // N s_8_1: branch s_8_0 b18 b9
        if s_8_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var d:i64
        let s_9_0: i64 = fn_state.d;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var n:i64
        let s_9_2: i64 = fn_state.n;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // N s_9_5: branch s_9_4 b17 b10
        if s_9_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var d:i64
        let s_10_0: i64 = fn_state.d;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var t:i64
        let s_10_2: i64 = fn_state.t;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: write-var gs#325002 <= s_10_4
        fn_state.gs_325002 = s_10_4;
        // N s_10_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#325002:u8
        let s_11_0: bool = fn_state.gs_325002;
        // N s_11_1: branch s_11_0 b16 b12
        if s_11_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var d:i64
        let s_12_0: i64 = fn_state.d;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var t2:i64
        let s_12_2: i64 = fn_state.t2;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var gs#325003 <= s_12_4
        fn_state.gs_325003 = s_12_4;
        // N s_12_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#325003:u8
        let s_13_0: bool = fn_state.gs_325003;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var d:i64
        let s_14_0: i64 = fn_state.d;
        // D s_14_1: read-var n:i64
        let s_14_1: i64 = fn_state.n;
        // D s_14_2: read-var t:i64
        let s_14_2: i64 = fn_state.t;
        // D s_14_3: read-var t2:i64
        let s_14_3: i64 = fn_state.t2;
        // D s_14_4: call execute_aarch32_instrs_STLEXD_Op_A_txt(s_14_0, s_14_1, s_14_2, s_14_3)
        let s_14_4: () = execute_aarch32_instrs_STLEXD_Op_A_txt(
            state,
            tracer,
            s_14_0,
            s_14_1,
            s_14_2,
            s_14_3,
        );
        // N s_14_5: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#325003 <= s_16_0
        fn_state.gs_325003 = s_16_0;
        // N s_16_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#325002 <= s_17_0
        fn_state.gs_325002 = s_17_0;
        // N s_17_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#325001 <= s_19_0
        fn_state.gs_325001 = s_19_0;
        // N s_19_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#324999 <= s_20_0
        fn_state.gs_324999 = s_20_0;
        // N s_20_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#324997 <= s_21_0
        fn_state.gs_324997 = s_21_0;
        // N s_21_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
