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
use LastInITBlock::*;
use ConditionPassed::*;
use execute_aarch32_instrs_LDR_l_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_LDR_l_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    Rt: u8,
    imm12: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        add: bool,
        t: i64,
        imm32: u32,
        gs_297786: bool,
        gs_297785: bool,
        U: bool,
        Rt: u8,
        imm12: u16,
    }
    let fn_state = FunctionState {
        U,
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
        // D s_2_0: read-var Rt:u8
        let s_2_0: u8 = fn_state.Rt;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var t <= s_2_3
        fn_state.t = s_2_3;
        // C s_2_5: const #32s : i
        let s_2_5: i128 = 32;
        // D s_2_6: read-var imm12:u12
        let s_2_6: u16 = fn_state.imm12;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 12u16);
        // D s_2_8: bits-cast zx s_2_7 -> bv length s_2_5
        let s_2_8: Bits = s_2_7.zero_extend(s_2_5);
        // D s_2_9: cast reint s_2_8 -> u32
        let s_2_9: u32 = (s_2_8.value() as u32);
        // D s_2_10: write-var imm32 <= s_2_9
        fn_state.imm32 = s_2_9;
        // D s_2_11: read-var U:u8
        let s_2_11: bool = fn_state.U;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 1u16);
        // C s_2_13: const #1u : u8
        let s_2_13: bool = true;
        // C s_2_14: cast zx s_2_13 -> bv
        let s_2_14: Bits = Bits::new(s_2_13 as u128, 1u16);
        // D s_2_15: cmp-eq s_2_12 s_2_14
        let s_2_15: bool = ((s_2_12) == (s_2_14));
        // D s_2_16: write-var add <= s_2_15
        fn_state.add = s_2_15;
        // C s_2_17: const #15s : i
        let s_2_17: i128 = 15;
        // D s_2_18: read-var t:i64
        let s_2_18: i64 = fn_state.t;
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (i128::try_from(s_2_18).unwrap());
        // D s_2_20: cmp-eq s_2_19 s_2_17
        let s_2_20: bool = ((s_2_19) == (s_2_17));
        // N s_2_21: branch s_2_20 b10 b3
        if s_2_20 {
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#297785 <= s_3_0
        fn_state.gs_297785 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#297785:u8
        let s_4_0: bool = fn_state.gs_297785;
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
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#297786 <= s_5_0
        fn_state.gs_297786 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#297786:u8
        let s_6_0: bool = fn_state.gs_297786;
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
        // D s_7_0: read-var add:u8
        let s_7_0: bool = fn_state.add;
        // D s_7_1: read-var imm32:u32
        let s_7_1: u32 = fn_state.imm32;
        // D s_7_2: read-var t:i64
        let s_7_2: i64 = fn_state.t;
        // D s_7_3: call execute_aarch32_instrs_LDR_l_Op_A_txt(s_7_0, s_7_1, s_7_2)
        let s_7_3: () = execute_aarch32_instrs_LDR_l_Op_A_txt(
            state,
            tracer,
            s_7_0,
            s_7_1,
            s_7_2,
        );
        // N s_7_4: return
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
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call LastInITBlock(s_9_0)
        let s_9_1: bool = LastInITBlock(state, tracer, s_9_0);
        // S s_9_2: not s_9_1
        let s_9_2: bool = !s_9_1;
        // D s_9_3: write-var gs#297786 <= s_9_2
        fn_state.gs_297786 = s_9_2;
        // N s_9_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call InITBlock(s_10_0)
        let s_10_1: bool = InITBlock(state, tracer, s_10_0);
        // D s_10_2: write-var gs#297785 <= s_10_1
        fn_state.gs_297785 = s_10_1;
        // N s_10_3: jump b4
        return block_4(state, tracer, fn_state);
    }
}
