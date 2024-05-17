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
use execute_aarch32_instrs_LDMIB_Op_A_txt::*;
use BitCount::*;
use common::*;
pub fn decode_aarch32_instrs_LDMIB_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    W: bool,
    Rn: u8,
    register_list: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        wback: bool,
        registers: u16,
        gs_296985: bool,
        gs_296983: bool,
        n: i64,
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
        // N s_2_23: branch s_2_22 b12 b3
        if s_2_22 {
            return block_12(state, tracer, fn_state);
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
        // D s_3_5: write-var gs#296983 <= s_3_4
        fn_state.gs_296983 = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#296983:u8
        let s_4_0: bool = fn_state.gs_296983;
        // N s_4_1: branch s_4_0 b11 b5
        if s_4_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var wback:u8
        let s_5_0: bool = fn_state.wback;
        // N s_5_1: branch s_5_0 b10 b6
        if s_5_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#296985 <= s_6_0
        fn_state.gs_296985 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#296985:u8
        let s_7_0: bool = fn_state.gs_296985;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var n:i64
        let s_8_0: i64 = fn_state.n;
        // D s_8_1: read-var registers:u16
        let s_8_1: u16 = fn_state.registers;
        // D s_8_2: read-var wback:u8
        let s_8_2: bool = fn_state.wback;
        // D s_8_3: call execute_aarch32_instrs_LDMIB_Op_A_txt(s_8_0, s_8_1, s_8_2)
        let s_8_3: () = execute_aarch32_instrs_LDMIB_Op_A_txt(
            state,
            tracer,
            s_8_0,
            s_8_1,
            s_8_2,
        );
        // N s_8_4: return
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
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var registers:u16
        let s_10_0: u16 = fn_state.registers;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 16u16);
        // D s_10_2: read-var n:i64
        let s_10_2: i64 = fn_state.n;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // C s_10_4: const #1u : u64
        let s_10_4: u64 = 1;
        // D s_10_5: bit-extract s_10_1 s_10_3 s_10_4
        let s_10_5: Bits = (Bits::new(
            ((s_10_1) >> (s_10_3)).value(),
            u16::try_from(s_10_4).unwrap(),
        ));
        // D s_10_6: cast reint s_10_5 -> u8
        let s_10_6: bool = ((s_10_5.value()) != 0);
        // C s_10_7: const #0s : i
        let s_10_7: i128 = 0;
        // C s_10_8: const #0u : u64
        let s_10_8: u64 = 0;
        // D s_10_9: cast zx s_10_6 -> u64
        let s_10_9: u64 = (s_10_6 as u64);
        // C s_10_10: const #1u : u64
        let s_10_10: u64 = 1;
        // D s_10_11: and s_10_9 s_10_10
        let s_10_11: u64 = ((s_10_9) & (s_10_10));
        // D s_10_12: cmp-eq s_10_11 s_10_10
        let s_10_12: bool = ((s_10_11) == (s_10_10));
        // D s_10_13: lsl s_10_9 s_10_7
        let s_10_13: u64 = s_10_9 << s_10_7;
        // D s_10_14: or s_10_8 s_10_13
        let s_10_14: u64 = ((s_10_8) | (s_10_13));
        // D s_10_15: cmpl s_10_13
        let s_10_15: u64 = !s_10_13;
        // D s_10_16: and s_10_8 s_10_15
        let s_10_16: u64 = ((s_10_8) & (s_10_15));
        // D s_10_17: select s_10_12 s_10_14 s_10_16
        let s_10_17: u64 = if s_10_12 { s_10_14 } else { s_10_16 };
        // D s_10_18: cast trunc s_10_17 -> u8
        let s_10_18: bool = ((s_10_17) != 0);
        // D s_10_19: cast zx s_10_18 -> bv
        let s_10_19: Bits = Bits::new(s_10_18 as u128, 1u16);
        // C s_10_20: const #1u : u8
        let s_10_20: bool = true;
        // C s_10_21: cast zx s_10_20 -> bv
        let s_10_21: Bits = Bits::new(s_10_20 as u128, 1u16);
        // D s_10_22: cmp-eq s_10_19 s_10_21
        let s_10_22: bool = ((s_10_19) == (s_10_21));
        // D s_10_23: write-var gs#296985 <= s_10_22
        fn_state.gs_296985 = s_10_22;
        // N s_10_24: jump b7
        return block_7(state, tracer, fn_state);
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
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#296983 <= s_12_0
        fn_state.gs_296983 = s_12_0;
        // N s_12_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
