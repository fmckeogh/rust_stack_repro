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
use AArch32_CheckForSVCTrap::*;
use AArch32_CallSupervisor::*;
use common::*;
pub fn execute_aarch32_instrs_SVC_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm32: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
    }
    let fn_state = FunctionState {
        imm32,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var imm32:u32
        let s_0_1: u32 = fn_state.imm32;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 32u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #15s : i
        let s_0_5: i128 = 15;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_0 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u16
        let s_0_8: u16 = (s_0_7.value() as u16);
        // D s_0_9: call AArch32_CheckForSVCTrap(s_0_8)
        let s_0_9: () = AArch32_CheckForSVCTrap(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i
        let s_1_0: i128 = 0;
        // D s_1_1: read-var imm32:u32
        let s_1_1: u32 = fn_state.imm32;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 32u16);
        // C s_1_3: const #1s : i64
        let s_1_3: i64 = 1;
        // C s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // C s_1_5: const #15s : i
        let s_1_5: i128 = 15;
        // C s_1_6: add s_1_5 s_1_4
        let s_1_6: i128 = (s_1_5 + s_1_4);
        // D s_1_7: bit-extract s_1_2 s_1_0 s_1_6
        let s_1_7: Bits = (Bits::new(
            ((s_1_2) >> (s_1_0)).value(),
            u16::try_from(s_1_6).unwrap(),
        ));
        // D s_1_8: cast reint s_1_7 -> u16
        let s_1_8: u16 = (s_1_7.value() as u16);
        // D s_1_9: call AArch32_CallSupervisor(s_1_8)
        let s_1_9: () = AArch32_CallSupervisor(state, tracer, s_1_8);
        // N s_1_10: return
        return;
    }
}
