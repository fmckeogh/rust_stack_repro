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
use execute_aarch32_instrs_SSAT16_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_SSAT16_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rn: u8,
    Rd: u8,
    sat_imm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        saturate_to: i64,
        n: i64,
        gs_301838: bool,
        d: i64,
        Rn: u8,
        Rd: u8,
        sat_imm: u8,
    }
    let fn_state = FunctionState {
        Rn,
        Rd,
        sat_imm,
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
        // D s_2_5: read-var Rn:u8
        let s_2_5: u8 = fn_state.Rn;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 4u16);
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (s_2_6.value() as i128);
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // D s_2_9: write-var n <= s_2_8
        fn_state.n = s_2_8;
        // D s_2_10: read-var sat_imm:u8
        let s_2_10: u8 = fn_state.sat_imm;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 4u16);
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (s_2_11.value() as i128);
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // C s_2_14: const #1s : i
        let s_2_14: i128 = 1;
        // D s_2_15: cast zx s_2_13 -> i
        let s_2_15: i128 = (i128::try_from(s_2_13).unwrap());
        // D s_2_16: add s_2_15 s_2_14
        let s_2_16: i128 = (s_2_15 + s_2_14);
        // D s_2_17: cast reint s_2_16 -> i64
        let s_2_17: i64 = (s_2_16 as i64);
        // D s_2_18: write-var saturate_to <= s_2_17
        fn_state.saturate_to = s_2_17;
        // C s_2_19: const #15s : i
        let s_2_19: i128 = 15;
        // D s_2_20: read-var d:i64
        let s_2_20: i64 = fn_state.d;
        // D s_2_21: cast zx s_2_20 -> i
        let s_2_21: i128 = (i128::try_from(s_2_20).unwrap());
        // D s_2_22: cmp-eq s_2_21 s_2_19
        let s_2_22: bool = ((s_2_21) == (s_2_19));
        // N s_2_23: branch s_2_22 b7 b3
        if s_2_22 {
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
        // C s_3_0: const #15s : i
        let s_3_0: i128 = 15;
        // D s_3_1: read-var n:i64
        let s_3_1: i64 = fn_state.n;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // D s_3_4: write-var gs#301838 <= s_3_3
        fn_state.gs_301838 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#301838:u8
        let s_4_0: bool = fn_state.gs_301838;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var saturate_to:i64
        let s_5_0: i64 = fn_state.saturate_to;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var d:i64
        let s_5_3: i64 = fn_state.d;
        // D s_5_4: read-var n:i64
        let s_5_4: i64 = fn_state.n;
        // D s_5_5: call execute_aarch32_instrs_SSAT16_Op_A_txt(s_5_3, s_5_4, s_5_2)
        let s_5_5: () = execute_aarch32_instrs_SSAT16_Op_A_txt(
            state,
            tracer,
            s_5_3,
            s_5_4,
            s_5_2,
        );
        // N s_5_6: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#301838 <= s_7_0
        fn_state.gs_301838 = s_7_0;
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
