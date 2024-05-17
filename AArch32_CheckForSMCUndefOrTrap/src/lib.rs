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
use AArch64_CheckForSMCUndefOrTrap::*;
use HCR_read::*;
use ExceptionSyndrome::*;
use AArch32_TakeHypTrapException__1::*;
use ELUsingAArch32::*;
use u_get_HCR_Type_TSC::*;
use Zeros::*;
use EL2Enabled::*;
use common::*;
pub fn AArch32_CheckForSMCUndefOrTrap<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_31869: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_31872: bool,
        gs_31873: bool,
        gs_31870: bool,
        gs_31874: bool,
        gs_31869: (),
    }
    let fn_state = FunctionState {
        gs_31869,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #424u : u32
        let s_0_0: u32 = 424;
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
        // N s_0_5: branch s_0_4 b18 b1
        if s_0_4 {
            return block_18(state, tracer, fn_state);
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
        // D s_1_7: write-var gs#31870 <= s_1_6
        fn_state.gs_31870 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#31870:u8
        let s_2_0: bool = fn_state.gs_31870;
        // N s_2_1: branch s_2_0 b17 b3
        if s_2_0 {
            return block_17(state, tracer, fn_state);
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
        // S s_3_1: call EL2Enabled(s_3_0)
        let s_3_1: bool = EL2Enabled(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b16 b4
        if s_3_1 {
            return block_16(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#31872 <= s_4_0
        fn_state.gs_31872 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#31872:u8
        let s_5_0: bool = fn_state.gs_31872;
        // N s_5_1: branch s_5_0 b15 b6
        if s_5_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call EL2Enabled(s_6_0)
        let s_6_1: bool = EL2Enabled(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b14 b7
        if s_6_1 {
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
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#31873 <= s_7_0
        fn_state.gs_31873 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#31873:u8
        let s_8_0: bool = fn_state.gs_31873;
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
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#31874 <= s_9_0
        fn_state.gs_31874 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#31874:u8
        let s_10_0: bool = fn_state.gs_31874;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #14u : u32
        let s_12_0: u32 = 14;
        // S s_12_1: call ExceptionSyndrome(s_12_0)
        let s_12_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_12_0,
        );
        // S s_12_2: call AArch32_TakeHypTrapException__1(s_12_1)
        let s_12_2: () = AArch32_TakeHypTrapException__1(state, tracer, s_12_1);
        // N s_12_3: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call HCR_read(s_13_0)
        let s_13_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_13_0);
        // S s_13_2: call _get_HCR_Type_TSC(s_13_1)
        let s_13_2: bool = u_get_HCR_Type_TSC(state, tracer, s_13_1);
        // S s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // C s_13_4: const #1u : u8
        let s_13_4: bool = true;
        // C s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 1u16);
        // S s_13_6: cmp-eq s_13_3 s_13_5
        let s_13_6: bool = ((s_13_3) == (s_13_5));
        // D s_13_7: write-var gs#31874 <= s_13_6
        fn_state.gs_31874 = s_13_6;
        // N s_13_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #16975u : u32
        let s_14_0: u32 = 16975;
        // D s_14_1: read-reg s_14_0:u8
        let s_14_1: u8 = {
            let value = state.read_register::<u8>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 2u16);
        // C s_14_3: const #440u : u32
        let s_14_3: u32 = 440;
        // D s_14_4: read-reg s_14_3:u8
        let s_14_4: u8 = {
            let value = state.read_register::<u8>(s_14_3 as isize);
            tracer.read_register(s_14_3 as isize, value);
            value
        };
        // D s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 2u16);
        // D s_14_6: cmp-eq s_14_2 s_14_5
        let s_14_6: bool = ((s_14_2) == (s_14_5));
        // D s_14_7: write-var gs#31873 <= s_14_6
        fn_state.gs_31873 = s_14_6;
        // N s_14_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #16s : i
        let s_15_0: i128 = 16;
        // S s_15_1: call Zeros(s_15_0)
        let s_15_1: Bits = Zeros(state, tracer, s_15_0);
        // S s_15_2: cast reint s_15_1 -> u16
        let s_15_2: u16 = (s_15_1.value() as u16);
        // S s_15_3: call AArch64_CheckForSMCUndefOrTrap(s_15_2)
        let s_15_3: () = AArch64_CheckForSMCUndefOrTrap(state, tracer, s_15_2);
        // N s_15_4: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #432u : u32
        let s_16_0: u32 = 432;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call ELUsingAArch32(s_16_1)
        let s_16_2: bool = ELUsingAArch32(state, tracer, s_16_1);
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // D s_16_4: write-var gs#31872 <= s_16_3
        fn_state.gs_31872 = s_16_3;
        // N s_16_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#31870 <= s_18_0
        fn_state.gs_31870 = s_18_0;
        // N s_18_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
