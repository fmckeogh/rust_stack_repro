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
use u_get_SCTLRType_SA0::*;
use SCTLR_read__1::*;
use SP_read::*;
use AArch64_SPAlignmentFault::*;
use u_get_SCTLRType_SA::*;
use Align_bits::*;
use common::*;
pub fn CheckSPAlignment<T: Tracer>(state: &mut State, tracer: &T, gs_15817: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_15819: bool,
        stack_align_check: bool,
        sp: u64,
        gs_15817: (),
    }
    let fn_state = FunctionState {
        gs_15817,
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
        // S s_0_1: call SP_read(s_0_0)
        let s_0_1: u64 = SP_read(state, tracer, s_0_0);
        // D s_0_2: write-var sp <= s_0_1
        fn_state.sp = s_0_1;
        // C s_0_3: const #16975u : u32
        let s_0_3: u32 = 16975;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // C s_0_6: const #448u : u32
        let s_0_6: u32 = 448;
        // D s_0_7: read-reg s_0_6:u8
        let s_0_7: u8 = {
            let value = state.read_register::<u8>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 2u16);
        // D s_0_9: cmp-eq s_0_5 s_0_8
        let s_0_9: bool = ((s_0_5) == (s_0_8));
        // N s_0_10: branch s_0_9 b8 b1
        if s_0_9 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call SCTLR_read__1(s_1_0)
        let s_1_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_1_0);
        // S s_1_2: call _get_SCTLRType_SA(s_1_1)
        let s_1_2: bool = u_get_SCTLRType_SA(state, tracer, s_1_1);
        // S s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // C s_1_4: const #0u : u8
        let s_1_4: bool = false;
        // C s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 1u16);
        // S s_1_6: cmp-ne s_1_3 s_1_5
        let s_1_6: bool = ((s_1_3) != (s_1_5));
        // D s_1_7: write-var stack_align_check <= s_1_6
        fn_state.stack_align_check = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var stack_align_check:u8
        let s_2_0: bool = fn_state.stack_align_check;
        // N s_2_1: branch s_2_0 b7 b3
        if s_2_0 {
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#15819 <= s_3_0
        fn_state.gs_15819 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#15819:u8
        let s_4_0: bool = fn_state.gs_15819;
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
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call AArch64_SPAlignmentFault(s_6_0)
        let s_6_1: () = AArch64_SPAlignmentFault(state, tracer, s_6_0);
        // N s_6_2: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16s : i
        let s_7_0: i128 = 16;
        // D s_7_1: read-var sp:u64
        let s_7_1: u64 = fn_state.sp;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 64u16);
        // D s_7_3: call Align_bits(s_7_2, s_7_0)
        let s_7_3: Bits = Align_bits(state, tracer, s_7_2, s_7_0);
        // D s_7_4: cast reint s_7_3 -> u64
        let s_7_4: u64 = (s_7_3.value() as u64);
        // D s_7_5: read-var sp:u64
        let s_7_5: u64 = fn_state.sp;
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 64u16);
        // D s_7_7: cast zx s_7_4 -> bv
        let s_7_7: Bits = Bits::new(s_7_4 as u128, 64u16);
        // D s_7_8: cmp-ne s_7_6 s_7_7
        let s_7_8: bool = ((s_7_6) != (s_7_7));
        // D s_7_9: write-var gs#15819 <= s_7_8
        fn_state.gs_15819 = s_7_8;
        // N s_7_10: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call SCTLR_read__1(s_8_0)
        let s_8_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_8_0);
        // S s_8_2: call _get_SCTLRType_SA0(s_8_1)
        let s_8_2: bool = u_get_SCTLRType_SA0(state, tracer, s_8_1);
        // S s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // C s_8_4: const #0u : u8
        let s_8_4: bool = false;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // S s_8_6: cmp-ne s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) != (s_8_5));
        // D s_8_7: write-var stack_align_check <= s_8_6
        fn_state.stack_align_check = s_8_6;
        // N s_8_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
