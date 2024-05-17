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
use u_get_SCR_EL3_Type_HCE::*;
use u_get_HCR_EL2_Type_HCD::*;
use EL2Enabled::*;
use AArch64_CallHypervisor::*;
use common::*;
pub fn execute_aarch64_instrs_system_exceptions_runtime_hvc<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_155039: bool,
        gs_155040: bool,
        gs_155038: bool,
        hvc_enable: bool,
        imm: u16,
    }
    let fn_state = FunctionState {
        imm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #432u : u32
        let s_0_0: u32 = 432;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #2u : u8
        let s_0_2: u8 = 2;
        // D s_0_3: cmp-lt s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) < (s_0_2));
        // D s_0_4: not s_0_3
        let s_0_4: bool = !s_0_3;
        // N s_0_5: branch s_0_4 b16 b1
        if s_0_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #448u : u32
        let s_1_3: u32 = 448;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // D s_1_7: write-var gs#155038 <= s_1_6
        fn_state.gs_155038 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#155038:u8
        let s_2_0: bool = fn_state.gs_155038;
        // N s_2_1: branch s_2_0 b15 b3
        if s_2_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16975u : u32
        let s_3_0: u32 = 16975;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 2u16);
        // C s_3_3: const #440u : u32
        let s_3_3: u32 = 440;
        // D s_3_4: read-reg s_3_3:u8
        let s_3_4: u8 = {
            let value = state.read_register::<u8>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // D s_3_6: cmp-eq s_3_2 s_3_5
        let s_3_6: bool = ((s_3_2) == (s_3_5));
        // N s_3_7: branch s_3_6 b14 b4
        if s_3_6 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#155039 <= s_4_0
        fn_state.gs_155039 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#155039:u8
        let s_5_0: bool = fn_state.gs_155039;
        // D s_5_1: write-var gs#155040 <= s_5_0
        fn_state.gs_155040 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#155040:u8
        let s_6_0: bool = fn_state.gs_155040;
        // N s_6_1: branch s_6_0 b13 b7
        if s_6_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #424u : u32
        let s_7_0: u32 = 424;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // C s_7_2: const #2u : u8
        let s_7_2: u8 = 2;
        // D s_7_3: cmp-lt s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) < (s_7_2));
        // N s_7_4: branch s_7_3 b12 b8
        if s_7_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #102552u : u32
        let s_8_0: u32 = 102552;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_HCR_EL2_Type_HCD(s_8_1)
        let s_8_2: bool = u_get_HCR_EL2_Type_HCD(state, tracer, s_8_1);
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: not s_8_3
        let s_8_4: Bits = !s_8_3;
        // D s_8_5: cast reint s_8_4 -> u8
        let s_8_5: bool = ((s_8_4.value()) != 0);
        // D s_8_6: write-var hvc_enable <= s_8_5
        fn_state.hvc_enable = s_8_5;
        // N s_8_7: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var hvc_enable:u8
        let s_9_0: bool = fn_state.hvc_enable;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // C s_9_2: const #0u : u8
        let s_9_2: bool = false;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var imm:u16
        let s_10_0: u16 = fn_state.imm;
        // D s_10_1: call AArch64_CallHypervisor(s_10_0)
        let s_10_1: () = AArch64_CallHypervisor(state, tracer, s_10_0);
        // N s_10_2: return
        return;
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
        // C s_12_0: const #90704u : u32
        let s_12_0: u32 = 90704;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_SCR_EL3_Type_HCE(s_12_1)
        let s_12_2: bool = u_get_SCR_EL3_Type_HCE(state, tracer, s_12_1);
        // D s_12_3: write-var hvc_enable <= s_12_2
        fn_state.hvc_enable = s_12_2;
        // N s_12_4: jump b9
        return block_9(state, tracer, fn_state);
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
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call EL2Enabled(s_14_0)
        let s_14_1: bool = EL2Enabled(state, tracer, s_14_0);
        // S s_14_2: not s_14_1
        let s_14_2: bool = !s_14_1;
        // D s_14_3: write-var gs#155039 <= s_14_2
        fn_state.gs_155039 = s_14_2;
        // N s_14_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#155040 <= s_15_0
        fn_state.gs_155040 = s_15_0;
        // N s_15_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#155038 <= s_16_0
        fn_state.gs_155038 = s_16_0;
        // N s_16_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
