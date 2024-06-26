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
use execute_aarch32_instrs_STRBT_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_STRBT_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    U: bool,
    Rn: u8,
    Rt: u8,
    imm12: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        shift_n: i128,
        m: i128,
        gs_302446: bool,
        t: i64,
        imm32: u32,
        gs_302447: bool,
        shift_t: u32,
        n: i64,
        add: bool,
        cond: u8,
        U: bool,
        Rn: u8,
        Rt: u8,
        imm12: u16,
    }
    let fn_state = FunctionState {
        cond,
        U,
        Rn,
        Rt,
        imm12,
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
        // D s_2_6: read-var Rt:u8
        let s_2_6: u8 = fn_state.Rt;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: write-var t <= s_2_9
        fn_state.t = s_2_9;
        // D s_2_11: read-var Rn:u8
        let s_2_11: u8 = fn_state.Rn;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 4u16);
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (s_2_12.value() as i128);
        // D s_2_14: cast reint s_2_13 -> i64
        let s_2_14: i64 = (s_2_13 as i64);
        // D s_2_15: write-var n <= s_2_14
        fn_state.n = s_2_14;
        // D s_2_16: read-var U:u8
        let s_2_16: bool = fn_state.U;
        // D s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 1u16);
        // C s_2_18: const #1u : u8
        let s_2_18: bool = true;
        // C s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 1u16);
        // D s_2_20: cmp-eq s_2_17 s_2_19
        let s_2_20: bool = ((s_2_17) == (s_2_19));
        // D s_2_21: write-var add <= s_2_20
        fn_state.add = s_2_20;
        // C s_2_22: const #32s : i
        let s_2_22: i128 = 32;
        // D s_2_23: read-var imm12:u12
        let s_2_23: u16 = fn_state.imm12;
        // D s_2_24: cast zx s_2_23 -> bv
        let s_2_24: Bits = Bits::new(s_2_23 as u128, 12u16);
        // D s_2_25: bits-cast zx s_2_24 -> bv length s_2_22
        let s_2_25: Bits = s_2_24.zero_extend(s_2_22);
        // D s_2_26: cast reint s_2_25 -> u32
        let s_2_26: u32 = (s_2_25.value() as u32);
        // D s_2_27: write-var imm32 <= s_2_26
        fn_state.imm32 = s_2_26;
        // C s_2_28: const #15s : i
        let s_2_28: i128 = 15;
        // D s_2_29: read-var t:i64
        let s_2_29: i64 = fn_state.t;
        // D s_2_30: cast zx s_2_29 -> i
        let s_2_30: i128 = (i128::try_from(s_2_29).unwrap());
        // D s_2_31: cmp-eq s_2_30 s_2_28
        let s_2_31: bool = ((s_2_30) == (s_2_28));
        // N s_2_32: branch s_2_31 b10 b3
        if s_2_31 {
            return block_10(state, tracer, fn_state);
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
        // D s_3_4: write-var gs#302446 <= s_3_3
        fn_state.gs_302446 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#302446:u8
        let s_4_0: bool = fn_state.gs_302446;
        // N s_4_1: branch s_4_0 b9 b5
        if s_4_0 {
            return block_9(state, tracer, fn_state);
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
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var t:i64
        let s_5_2: i64 = fn_state.t;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: write-var gs#302447 <= s_5_4
        fn_state.gs_302447 = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#302447:u8
        let s_6_0: bool = fn_state.gs_302447;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
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
        // D s_7_0: read-var shift_t:u32
        let s_7_0: u32 = fn_state.shift_t;
        // D s_7_1: read-var shift_n:i
        let s_7_1: i128 = fn_state.shift_n;
        // D s_7_2: read-var m:i
        let s_7_2: i128 = fn_state.m;
        // D s_7_3: read-var add:u8
        let s_7_3: bool = fn_state.add;
        // D s_7_4: read-var imm32:u32
        let s_7_4: u32 = fn_state.imm32;
        // D s_7_5: read-var n:i64
        let s_7_5: i64 = fn_state.n;
        // C s_7_6: const #1u : u8
        let s_7_6: bool = true;
        // C s_7_7: const #0u : u8
        let s_7_7: bool = false;
        // D s_7_8: read-var t:i64
        let s_7_8: i64 = fn_state.t;
        // D s_7_9: call execute_aarch32_instrs_STRBT_Op_A_txt(s_7_3, s_7_4, s_7_2, s_7_5, s_7_6, s_7_7, s_7_1, s_7_0, s_7_8)
        let s_7_9: () = execute_aarch32_instrs_STRBT_Op_A_txt(
            state,
            tracer,
            s_7_3,
            s_7_4,
            s_7_2,
            s_7_5,
            s_7_6,
            s_7_7,
            s_7_1,
            s_7_0,
            s_7_8,
        );
        // N s_7_10: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#302447 <= s_9_0
        fn_state.gs_302447 = s_9_0;
        // N s_9_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#302446 <= s_10_0
        fn_state.gs_302446 = s_10_0;
        // N s_10_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
