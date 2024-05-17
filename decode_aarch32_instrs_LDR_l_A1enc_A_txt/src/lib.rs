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
use execute_aarch32_instrs_LDR_l_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDR_l_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    P: bool,
    U: bool,
    W: bool,
    Rt: u8,
    imm12: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        imm32: u32,
        gs_297771: bool,
        gs_297766: bool,
        add: bool,
        cond: u8,
        P: bool,
        U: bool,
        W: bool,
        Rt: u8,
        imm12: u16,
    }
    let fn_state = FunctionState {
        cond,
        P,
        U,
        W,
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
        // D s_2_6: read-var P:u8
        let s_2_6: bool = fn_state.P;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 1u16);
        // C s_2_8: const #0u : u8
        let s_2_8: bool = false;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 1u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // N s_2_11: branch s_2_10 b12 b3
        if s_2_10 {
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#297766 <= s_3_0
        fn_state.gs_297766 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#297766:u8
        let s_4_0: bool = fn_state.gs_297766;
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
        // D s_5_0: read-var Rt:u8
        let s_5_0: u8 = fn_state.Rt;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: write-var t <= s_5_3
        fn_state.t = s_5_3;
        // C s_5_5: const #32s : i
        let s_5_5: i128 = 32;
        // D s_5_6: read-var imm12:u12
        let s_5_6: u16 = fn_state.imm12;
        // D s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 12u16);
        // D s_5_8: bits-cast zx s_5_7 -> bv length s_5_5
        let s_5_8: Bits = s_5_7.zero_extend(s_5_5);
        // D s_5_9: cast reint s_5_8 -> u32
        let s_5_9: u32 = (s_5_8.value() as u32);
        // D s_5_10: write-var imm32 <= s_5_9
        fn_state.imm32 = s_5_9;
        // D s_5_11: read-var U:u8
        let s_5_11: bool = fn_state.U;
        // D s_5_12: cast zx s_5_11 -> bv
        let s_5_12: Bits = Bits::new(s_5_11 as u128, 1u16);
        // C s_5_13: const #1u : u8
        let s_5_13: bool = true;
        // C s_5_14: cast zx s_5_13 -> bv
        let s_5_14: Bits = Bits::new(s_5_13 as u128, 1u16);
        // D s_5_15: cmp-eq s_5_12 s_5_14
        let s_5_15: bool = ((s_5_12) == (s_5_14));
        // D s_5_16: write-var add <= s_5_15
        fn_state.add = s_5_15;
        // D s_5_17: read-var P:u8
        let s_5_17: bool = fn_state.P;
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 1u16);
        // C s_5_19: const #0u : u8
        let s_5_19: bool = false;
        // C s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 1u16);
        // D s_5_21: cmp-eq s_5_18 s_5_20
        let s_5_21: bool = ((s_5_18) == (s_5_20));
        // N s_5_22: branch s_5_21 b10 b6
        if s_5_21 {
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
        // D s_6_0: read-var W:u8
        let s_6_0: bool = fn_state.W;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: write-var gs#297771 <= s_6_4
        fn_state.gs_297771 = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#297771:u8
        let s_7_0: bool = fn_state.gs_297771;
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
        // D s_8_0: read-var add:u8
        let s_8_0: bool = fn_state.add;
        // D s_8_1: read-var imm32:u32
        let s_8_1: u32 = fn_state.imm32;
        // D s_8_2: read-var t:i64
        let s_8_2: i64 = fn_state.t;
        // D s_8_3: call execute_aarch32_instrs_LDR_l_Op_A_txt(s_8_0, s_8_1, s_8_2)
        let s_8_3: () = execute_aarch32_instrs_LDR_l_Op_A_txt(
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
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#297771 <= s_10_0
        fn_state.gs_297771 = s_10_0;
        // N s_10_2: jump b7
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
        // D s_12_0: read-var W:u8
        let s_12_0: bool = fn_state.W;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #1u : u8
        let s_12_2: bool = true;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var gs#297766 <= s_12_4
        fn_state.gs_297766 = s_12_4;
        // N s_12_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
