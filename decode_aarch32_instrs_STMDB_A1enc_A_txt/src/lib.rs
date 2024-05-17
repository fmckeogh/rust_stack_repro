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
use BitCount::*;
use execute_aarch32_instrs_STMDB_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_STMDB_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    W: bool,
    Rn: u8,
    register_list: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_302244: bool,
        n: i64,
        wback: bool,
        registers: u16,
        cond: u8,
        W: bool,
        Rn: u8,
        register_list: u16,
    }
    let fn_state = FunctionState {
        cond,
        W,
        Rn,
        register_list,
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
        // D s_2_6: read-var Rn:u8
        let s_2_6: u8 = fn_state.Rn;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: write-var n <= s_2_9
        fn_state.n = s_2_9;
        // D s_2_11: read-var register_list:u16
        let s_2_11: u16 = fn_state.register_list;
        // D s_2_12: write-var registers <= s_2_11
        fn_state.registers = s_2_11;
        // D s_2_13: read-var W:u8
        let s_2_13: bool = fn_state.W;
        // D s_2_14: cast zx s_2_13 -> bv
        let s_2_14: Bits = Bits::new(s_2_13 as u128, 1u16);
        // C s_2_15: const #1u : u8
        let s_2_15: bool = true;
        // C s_2_16: cast zx s_2_15 -> bv
        let s_2_16: Bits = Bits::new(s_2_15 as u128, 1u16);
        // D s_2_17: cmp-eq s_2_14 s_2_16
        let s_2_17: bool = ((s_2_14) == (s_2_16));
        // D s_2_18: write-var wback <= s_2_17
        fn_state.wback = s_2_17;
        // C s_2_19: const #15s : i
        let s_2_19: i128 = 15;
        // D s_2_20: read-var n:i64
        let s_2_20: i64 = fn_state.n;
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
        // D s_3_0: read-var registers:u16
        let s_3_0: u16 = fn_state.registers;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 16u16);
        // D s_3_2: call BitCount(s_3_1)
        let s_3_2: i128 = BitCount(state, tracer, s_3_1);
        // C s_3_3: const #1s : i
        let s_3_3: i128 = 1;
        // D s_3_4: cmp-lt s_3_2 s_3_3
        let s_3_4: bool = ((s_3_2) < (s_3_3));
        // D s_3_5: write-var gs#302244 <= s_3_4
        fn_state.gs_302244 = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#302244:u8
        let s_4_0: bool = fn_state.gs_302244;
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
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: read-var registers:u16
        let s_5_1: u16 = fn_state.registers;
        // D s_5_2: read-var wback:u8
        let s_5_2: bool = fn_state.wback;
        // D s_5_3: call execute_aarch32_instrs_STMDB_Op_A_txt(s_5_0, s_5_1, s_5_2)
        let s_5_3: () = execute_aarch32_instrs_STMDB_Op_A_txt(
            state,
            tracer,
            s_5_0,
            s_5_1,
            s_5_2,
        );
        // N s_5_4: return
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
        // D s_7_1: write-var gs#302244 <= s_7_0
        fn_state.gs_302244 = s_7_0;
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
