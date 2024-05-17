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
use execute_aarch32_instrs_MSR_i_Op_AS_txt::*;
use A32ExpandImm::*;
use common::*;
pub fn decode_aarch32_instrs_MSR_i_A1enc_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    R: bool,
    mask: u8,
    imm12: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_323520: bool,
        write_spsr: bool,
        imm32: u32,
        cond: u8,
        R: bool,
        mask: u8,
        imm12: u16,
    }
    let fn_state = FunctionState {
        cond,
        R,
        mask,
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
        // D s_2_6: read-var mask:u8
        let s_2_6: u8 = fn_state.mask;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // C s_2_8: const #0u : u8
        let s_2_8: u8 = 0;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 4u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // N s_2_11: branch s_2_10 b9 b3
        if s_2_10 {
            return block_9(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#323520 <= s_3_0
        fn_state.gs_323520 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#323520:u8
        let s_4_0: bool = fn_state.gs_323520;
        // N s_4_1: branch s_4_0 b8 b5
        if s_4_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var imm12:u12
        let s_5_0: u16 = fn_state.imm12;
        // D s_5_1: call A32ExpandImm(s_5_0)
        let s_5_1: u32 = A32ExpandImm(state, tracer, s_5_0);
        // D s_5_2: write-var imm32 <= s_5_1
        fn_state.imm32 = s_5_1;
        // D s_5_3: read-var R:u8
        let s_5_3: bool = fn_state.R;
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 1u16);
        // C s_5_5: const #1u : u8
        let s_5_5: bool = true;
        // C s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 1u16);
        // D s_5_7: cmp-eq s_5_4 s_5_6
        let s_5_7: bool = ((s_5_4) == (s_5_6));
        // D s_5_8: write-var write_spsr <= s_5_7
        fn_state.write_spsr = s_5_7;
        // D s_5_9: read-var mask:u8
        let s_5_9: u8 = fn_state.mask;
        // D s_5_10: cast zx s_5_9 -> bv
        let s_5_10: Bits = Bits::new(s_5_9 as u128, 4u16);
        // C s_5_11: const #0u : u8
        let s_5_11: u8 = 0;
        // C s_5_12: cast zx s_5_11 -> bv
        let s_5_12: Bits = Bits::new(s_5_11 as u128, 4u16);
        // D s_5_13: cmp-eq s_5_10 s_5_12
        let s_5_13: bool = ((s_5_10) == (s_5_12));
        // N s_5_14: branch s_5_13 b7 b6
        if s_5_13 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var imm32:u32
        let s_6_0: u32 = fn_state.imm32;
        // D s_6_1: read-var mask:u8
        let s_6_1: u8 = fn_state.mask;
        // D s_6_2: read-var write_spsr:u8
        let s_6_2: bool = fn_state.write_spsr;
        // D s_6_3: call execute_aarch32_instrs_MSR_i_Op_AS_txt(s_6_0, s_6_1, s_6_2)
        let s_6_3: () = execute_aarch32_instrs_MSR_i_Op_AS_txt(
            state,
            tracer,
            s_6_0,
            s_6_1,
            s_6_2,
        );
        // N s_6_4: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
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
        // D s_9_0: read-var R:u8
        let s_9_0: bool = fn_state.R;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // C s_9_2: const #0u : u8
        let s_9_2: bool = false;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: write-var gs#323520 <= s_9_4
        fn_state.gs_323520 = s_9_4;
        // N s_9_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
