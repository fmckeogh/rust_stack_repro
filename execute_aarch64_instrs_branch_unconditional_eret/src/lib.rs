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
use ELR_read__1::*;
use AArch64_CheckForERetTrap::*;
use AuthIA::*;
use SP_read::*;
use AuthIB::*;
use return_exception::*;
use AArch64_ExceptionReturn::*;
use SPSR_read::*;
use common::*;
pub fn execute_aarch64_instrs_branch_unconditional_eret<T: Tracer>(
    state: &mut State,
    tracer: &T,
    pac: bool,
    use_key_a: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        target: u64,
        modifier: u64,
        pac: bool,
        use_key_a: bool,
    }
    let fn_state = FunctionState {
        pac,
        use_key_a,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var pac:u8
        let s_0_0: bool = fn_state.pac;
        // D s_0_1: read-var use_key_a:u8
        let s_0_1: bool = fn_state.use_key_a;
        // D s_0_2: call AArch64_CheckForERetTrap(s_0_0, s_0_1)
        let s_0_2: () = AArch64_CheckForERetTrap(state, tracer, s_0_0, s_0_1);
        // N s_0_3: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call ELR_read__1(s_1_0)
        let s_1_1: u64 = ELR_read__1(state, tracer, s_1_0);
        // D s_1_2: write-var target <= s_1_1
        fn_state.target = s_1_1;
        // C s_1_3: const #() : ()
        let s_1_3: () = ();
        // S s_1_4: call SP_read(s_1_3)
        let s_1_4: u64 = SP_read(state, tracer, s_1_3);
        // D s_1_5: write-var modifier <= s_1_4
        fn_state.modifier = s_1_4;
        // D s_1_6: read-var pac:u8
        let s_1_6: bool = fn_state.pac;
        // N s_1_7: branch s_1_6 b4 b2
        if s_1_6 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var target:u64
        let s_3_1: u64 = fn_state.target;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 64u16);
        // C s_3_3: const #1s : i64
        let s_3_3: i64 = 1;
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #55s : i
        let s_3_5: i128 = 55;
        // C s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: bit-extract s_3_2 s_3_0 s_3_6
        let s_3_7: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u56
        let s_3_8: u64 = (s_3_7.value() as u64);
        // D s_3_9: call return_exception(s_3_8)
        let s_3_9: () = return_exception(state, tracer, s_3_8);
        // C s_3_10: const #64s : i
        let s_3_10: i128 = 64;
        // S s_3_11: call SPSR_read(s_3_10)
        let s_3_11: Bits = SPSR_read(state, tracer, s_3_10);
        // S s_3_12: cast reint s_3_11 -> u64
        let s_3_12: u64 = (s_3_11.value() as u64);
        // D s_3_13: read-var target:u64
        let s_3_13: u64 = fn_state.target;
        // D s_3_14: call AArch64_ExceptionReturn(s_3_13, s_3_12)
        let s_3_14: () = AArch64_ExceptionReturn(state, tracer, s_3_13, s_3_12);
        // N s_3_15: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var use_key_a:u8
        let s_4_0: bool = fn_state.use_key_a;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var target:u64
        let s_5_0: u64 = fn_state.target;
        // D s_5_1: read-var modifier:u64
        let s_5_1: u64 = fn_state.modifier;
        // C s_5_2: const #1u : u8
        let s_5_2: bool = true;
        // D s_5_3: call AuthIB(s_5_0, s_5_1, s_5_2)
        let s_5_3: u64 = AuthIB(state, tracer, s_5_0, s_5_1, s_5_2);
        // D s_5_4: write-var target <= s_5_3
        fn_state.target = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var target:u64
        let s_7_0: u64 = fn_state.target;
        // D s_7_1: read-var modifier:u64
        let s_7_1: u64 = fn_state.modifier;
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // D s_7_3: call AuthIA(s_7_0, s_7_1, s_7_2)
        let s_7_3: u64 = AuthIA(state, tracer, s_7_0, s_7_1, s_7_2);
        // D s_7_4: write-var target <= s_7_3
        fn_state.target = s_7_3;
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b3
        return block_3(state, tracer, fn_state);
    }
}
