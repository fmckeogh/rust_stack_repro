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
use ELR_hyp_read::*;
use SynchronizeContext::*;
use DebugRestorePSR::*;
use R_read::*;
use Halted::*;
use AArch32_ExceptionReturn::*;
use SPSR_read::*;
use common::*;
pub fn execute_aarch32_instrs_ERET_Op_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_323201: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        new_pc_valueshadow_7884: u32,
        gs_323203: bool,
        gs_323201: (),
    }
    let fn_state = FunctionState {
        gs_323201,
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
        // S s_0_1: call Halted(s_0_0)
        let s_0_1: bool = Halted(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b6 b1
        if s_0_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16983u : u32
        let s_1_0: u32 = 16983;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 5u16);
        // C s_1_3: const #352u : u32
        let s_1_3: u32 = 352;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 5u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // N s_1_7: branch s_1_6 b5 b2
        if s_1_6 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #16983u : u32
        let s_2_0: u32 = 16983;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 5u16);
        // C s_2_3: const #416u : u32
        let s_2_3: u32 = 416;
        // D s_2_4: read-reg s_2_3:u8
        let s_2_4: u8 = {
            let value = state.read_register::<u8>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 5u16);
        // D s_2_6: cmp-eq s_2_2 s_2_5
        let s_2_6: bool = ((s_2_2) == (s_2_5));
        // N s_2_7: branch s_2_6 b4 b3
        if s_2_6 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call SynchronizeContext(s_3_0)
        let s_3_1: () = SynchronizeContext(state, tracer, s_3_0);
        // C s_3_2: const #() : ()
        let s_3_2: () = ();
        // S s_3_3: call DebugRestorePSR(s_3_2)
        let s_3_3: () = DebugRestorePSR(state, tracer, s_3_2);
        // N s_3_4: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #16983u : u32
        let s_6_0: u32 = 16983;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 5u16);
        // C s_6_3: const #352u : u32
        let s_6_3: u32 = 352;
        // D s_6_4: read-reg s_6_3:u8
        let s_6_4: u8 = {
            let value = state.read_register::<u8>(s_6_3 as isize);
            tracer.read_register(s_6_3 as isize, value);
            value
        };
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 5u16);
        // D s_6_6: cmp-eq s_6_2 s_6_5
        let s_6_6: bool = ((s_6_2) == (s_6_5));
        // N s_6_7: branch s_6_6 b14 b7
        if s_6_6 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16983u : u32
        let s_7_0: u32 = 16983;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 5u16);
        // C s_7_3: const #416u : u32
        let s_7_3: u32 = 416;
        // D s_7_4: read-reg s_7_3:u8
        let s_7_4: u8 = {
            let value = state.read_register::<u8>(s_7_3 as isize);
            tracer.read_register(s_7_3 as isize, value);
            value
        };
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 5u16);
        // D s_7_6: cmp-eq s_7_2 s_7_5
        let s_7_6: bool = ((s_7_2) == (s_7_5));
        // D s_7_7: write-var gs#323203 <= s_7_6
        fn_state.gs_323203 = s_7_6;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#323203:u8
        let s_8_0: bool = fn_state.gs_323203;
        // N s_8_1: branch s_8_0 b13 b9
        if s_8_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #16975u : u32
        let s_9_0: u32 = 16975;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 2u16);
        // C s_9_3: const #432u : u32
        let s_9_3: u32 = 432;
        // D s_9_4: read-reg s_9_3:u8
        let s_9_4: u8 = {
            let value = state.read_register::<u8>(s_9_3 as isize);
            tracer.read_register(s_9_3 as isize, value);
            value
        };
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 2u16);
        // D s_9_6: cmp-eq s_9_2 s_9_5
        let s_9_6: bool = ((s_9_2) == (s_9_5));
        // N s_9_7: branch s_9_6 b12 b10
        if s_9_6 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #14s : i
        let s_10_0: i128 = 14;
        // S s_10_1: call R_read(s_10_0)
        let s_10_1: u32 = R_read(state, tracer, s_10_0);
        // D s_10_2: write-var new_pc_valueshadow#7884 <= s_10_1
        fn_state.new_pc_valueshadow_7884 = s_10_1;
        // N s_10_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #32s : i
        let s_11_0: i128 = 32;
        // S s_11_1: call SPSR_read(s_11_0)
        let s_11_1: Bits = SPSR_read(state, tracer, s_11_0);
        // S s_11_2: cast reint s_11_1 -> u32
        let s_11_2: u32 = (s_11_1.value() as u32);
        // D s_11_3: read-var new_pc_valueshadow#7884:u32
        let s_11_3: u32 = fn_state.new_pc_valueshadow_7884;
        // D s_11_4: call AArch32_ExceptionReturn(s_11_3, s_11_2)
        let s_11_4: () = AArch32_ExceptionReturn(state, tracer, s_11_3, s_11_2);
        // N s_11_5: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call ELR_hyp_read(s_12_0)
        let s_12_1: u32 = ELR_hyp_read(state, tracer, s_12_0);
        // D s_12_2: write-var new_pc_valueshadow#7884 <= s_12_1
        fn_state.new_pc_valueshadow_7884 = s_12_1;
        // N s_12_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#323203 <= s_14_0
        fn_state.gs_323203 = s_14_0;
        // N s_14_2: jump b8
        return block_8(state, tracer, fn_state);
    }
}
