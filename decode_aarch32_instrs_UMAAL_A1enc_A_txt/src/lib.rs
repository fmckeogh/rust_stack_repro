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
use execute_aarch32_instrs_UMAAL_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_UMAAL_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    RdHi: u8,
    RdLo: u8,
    Rm: u8,
    Rn: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_304451: bool,
        dLo: i64,
        gs_304453: bool,
        dHi: i64,
        n: i64,
        gs_304449: bool,
        cond: u8,
        RdHi: u8,
        RdLo: u8,
        Rm: u8,
        Rn: u8,
    }
    let fn_state = FunctionState {
        cond,
        RdHi,
        RdLo,
        Rm,
        Rn,
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
        // D s_2_0: read-var cond:u8
        let s_2_0: u8 = fn_state.cond;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-ne s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) != (s_2_3));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // D s_2_6: read-var RdLo:u8
        let s_2_6: u8 = fn_state.RdLo;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: write-var dLo <= s_2_9
        fn_state.dLo = s_2_9;
        // D s_2_11: read-var RdHi:u8
        let s_2_11: u8 = fn_state.RdHi;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 4u16);
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (s_2_12.value() as i128);
        // D s_2_14: cast reint s_2_13 -> i64
        let s_2_14: i64 = (s_2_13 as i64);
        // D s_2_15: write-var dHi <= s_2_14
        fn_state.dHi = s_2_14;
        // D s_2_16: read-var Rn:u8
        let s_2_16: u8 = fn_state.Rn;
        // D s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 4u16);
        // D s_2_18: cast zx s_2_17 -> i
        let s_2_18: i128 = (s_2_17.value() as i128);
        // D s_2_19: cast reint s_2_18 -> i64
        let s_2_19: i64 = (s_2_18 as i64);
        // D s_2_20: write-var n <= s_2_19
        fn_state.n = s_2_19;
        // D s_2_21: read-var Rm:u8
        let s_2_21: u8 = fn_state.Rm;
        // D s_2_22: cast zx s_2_21 -> bv
        let s_2_22: Bits = Bits::new(s_2_21 as u128, 4u16);
        // D s_2_23: cast zx s_2_22 -> i
        let s_2_23: i128 = (s_2_22.value() as i128);
        // D s_2_24: cast reint s_2_23 -> i64
        let s_2_24: i64 = (s_2_23 as i64);
        // D s_2_25: write-var m <= s_2_24
        fn_state.m = s_2_24;
        // C s_2_26: const #15s : i
        let s_2_26: i128 = 15;
        // D s_2_27: read-var dLo:i64
        let s_2_27: i64 = fn_state.dLo;
        // D s_2_28: cast zx s_2_27 -> i
        let s_2_28: i128 = (i128::try_from(s_2_27).unwrap());
        // D s_2_29: cmp-eq s_2_28 s_2_26
        let s_2_29: bool = ((s_2_28) == (s_2_26));
        // N s_2_30: branch s_2_29 b15 b3
        if s_2_29 {
            return block_15(state, tracer, fn_state);
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
        // D s_3_1: read-var dHi:i64
        let s_3_1: i64 = fn_state.dHi;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // D s_3_4: write-var gs#304449 <= s_3_3
        fn_state.gs_304449 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#304449:u8
        let s_4_0: bool = fn_state.gs_304449;
        // N s_4_1: branch s_4_0 b14 b5
        if s_4_0 {
            return block_14(state, tracer, fn_state);
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
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // D s_5_4: write-var gs#304451 <= s_5_3
        fn_state.gs_304451 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#304451:u8
        let s_6_0: bool = fn_state.gs_304451;
        // N s_6_1: branch s_6_0 b13 b7
        if s_6_0 {
            return block_13(state, tracer, fn_state);
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
        // D s_7_1: read-var m:i64
        let s_7_1: i64 = fn_state.m;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_0
        let s_7_3: bool = ((s_7_2) == (s_7_0));
        // D s_7_4: write-var gs#304453 <= s_7_3
        fn_state.gs_304453 = s_7_3;
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#304453:u8
        let s_8_0: bool = fn_state.gs_304453;
        // N s_8_1: branch s_8_0 b12 b9
        if s_8_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var dHi:i64
        let s_9_0: i64 = fn_state.dHi;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var dLo:i64
        let s_9_2: i64 = fn_state.dLo;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var dHi:i64
        let s_10_0: i64 = fn_state.dHi;
        // D s_10_1: read-var dLo:i64
        let s_10_1: i64 = fn_state.dLo;
        // D s_10_2: read-var m:i64
        let s_10_2: i64 = fn_state.m;
        // D s_10_3: read-var n:i64
        let s_10_3: i64 = fn_state.n;
        // D s_10_4: call execute_aarch32_instrs_UMAAL_Op_A_txt(s_10_0, s_10_1, s_10_2, s_10_3)
        let s_10_4: () = execute_aarch32_instrs_UMAAL_Op_A_txt(
            state,
            tracer,
            s_10_0,
            s_10_1,
            s_10_2,
            s_10_3,
        );
        // N s_10_5: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#304453 <= s_13_0
        fn_state.gs_304453 = s_13_0;
        // N s_13_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#304451 <= s_14_0
        fn_state.gs_304451 = s_14_0;
        // N s_14_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#304449 <= s_15_0
        fn_state.gs_304449 = s_15_0;
        // N s_15_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
