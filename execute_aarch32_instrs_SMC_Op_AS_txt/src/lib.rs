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
use u_get_SCR_EL3_Type_SMD::*;
use ELUsingAArch32::*;
use u_get_SCR_Type_SCD::*;
use AArch32_TakeSMCException::*;
use EndOfInstruction::*;
use AArch32_CheckForSMCUndefOrTrap::*;
use AArch64_CallSecureMonitor::*;
use Zeros::*;
use CurrentSecurityState::*;
use ConstrainUnpredictable::*;
use common::*;
pub fn execute_aarch32_instrs_SMC_Op_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_323567: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        cshadow_7906: u32,
        gs_323571: bool,
        gs_323567: (),
    }
    let fn_state = FunctionState {
        gs_323567,
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
        // S s_0_1: call AArch32_CheckForSMCUndefOrTrap(s_0_0)
        let s_0_1: () = AArch32_CheckForSMCUndefOrTrap(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #424u : u32
        let s_1_0: u32 = 424;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call ELUsingAArch32(s_1_1)
        let s_1_2: bool = ELUsingAArch32(state, tracer, s_1_1);
        // D s_1_3: not s_1_2
        let s_1_3: bool = !s_1_2;
        // N s_1_4: branch s_1_3 b17 b2
        if s_1_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #20920u : u32
        let s_2_0: u32 = 20920;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_SCR_Type_SCD(s_2_1)
        let s_2_2: bool = u_get_SCR_Type_SCD(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #1u : u8
        let s_2_4: bool = true;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b7 b3
        if s_2_6 {
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
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #424u : u32
        let s_4_0: u32 = 424;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call ELUsingAArch32(s_4_1)
        let s_4_2: bool = ELUsingAArch32(state, tracer, s_4_1);
        // D s_4_3: not s_4_2
        let s_4_3: bool = !s_4_2;
        // N s_4_4: branch s_4_3 b6 b5
        if s_4_3 {
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call AArch32_TakeSMCException(s_5_0)
        let s_5_1: () = AArch32_TakeSMCException(state, tracer, s_5_0);
        // N s_5_2: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #16s : i
        let s_6_0: i128 = 16;
        // S s_6_1: call Zeros(s_6_0)
        let s_6_1: Bits = Zeros(state, tracer, s_6_0);
        // S s_6_2: cast reint s_6_1 -> u16
        let s_6_2: u16 = (s_6_1.value() as u16);
        // S s_6_3: call AArch64_CallSecureMonitor(s_6_2)
        let s_6_3: () = AArch64_CallSecureMonitor(state, tracer, s_6_2);
        // N s_6_4: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call CurrentSecurityState(s_7_0)
        let s_7_1: u32 = CurrentSecurityState(state, tracer, s_7_0);
        // C s_7_2: const #3u : u32
        let s_7_2: u32 = 3;
        // S s_7_3: cmp-eq s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) == (s_7_2));
        // N s_7_4: branch s_7_3 b10 b8
        if s_7_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
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
        // C s_10_0: const #48u : u32
        let s_10_0: u32 = 48;
        // S s_10_1: call ConstrainUnpredictable(s_10_0)
        let s_10_1: u32 = ConstrainUnpredictable(state, tracer, s_10_0);
        // D s_10_2: write-var cshadow#7906 <= s_10_1
        fn_state.cshadow_7906 = s_10_1;
        // D s_10_3: read-var cshadow#7906:u32
        let s_10_3: u32 = fn_state.cshadow_7906;
        // C s_10_4: const #4u : u32
        let s_10_4: u32 = 4;
        // D s_10_5: cmp-eq s_10_3 s_10_4
        let s_10_5: bool = ((s_10_3) == (s_10_4));
        // N s_10_6: branch s_10_5 b16 b11
        if s_10_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var cshadow#7906:u32
        let s_11_0: u32 = fn_state.cshadow_7906;
        // C s_11_1: const #2u : u32
        let s_11_1: u32 = 2;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: write-var gs#323571 <= s_11_2
        fn_state.gs_323571 = s_11_2;
        // N s_11_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#323571:u8
        let s_12_0: bool = fn_state.gs_323571;
        // N s_12_1: assert s_12_0
        let s_12_1: () = assert!(s_12_0);
        // D s_12_2: read-var cshadow#7906:u32
        let s_12_2: u32 = fn_state.cshadow_7906;
        // C s_12_3: const #4u : u32
        let s_12_3: u32 = 4;
        // D s_12_4: cmp-eq s_12_2 s_12_3
        let s_12_4: bool = ((s_12_2) == (s_12_3));
        // N s_12_5: branch s_12_4 b15 b13
        if s_12_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call EndOfInstruction(s_15_0)
        let s_15_1: () = EndOfInstruction(state, tracer, s_15_0);
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#323571 <= s_16_0
        fn_state.gs_323571 = s_16_0;
        // N s_16_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #90704u : u32
        let s_17_0: u32 = 90704;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_SCR_EL3_Type_SMD(s_17_1)
        let s_17_2: bool = u_get_SCR_EL3_Type_SMD(state, tracer, s_17_1);
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // C s_17_4: const #1u : u8
        let s_17_4: bool = true;
        // C s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 1u16);
        // D s_17_6: cmp-eq s_17_3 s_17_5
        let s_17_6: bool = ((s_17_3) == (s_17_5));
        // N s_17_7: branch s_17_6 b19 b18
        if s_17_6 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
}
