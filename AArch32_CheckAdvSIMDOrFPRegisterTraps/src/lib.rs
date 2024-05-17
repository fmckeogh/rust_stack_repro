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
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TID0::*;
use ELUsingAArch32::*;
use HCR_read::*;
use u_get_HCR_EL2_Type_TID3::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HCR_Type_TID0::*;
use u_get_HCR_Type_TID3::*;
use AArch32_SystemAccessTrap::*;
use common::*;
pub fn AArch32_CheckAdvSIMDOrFPRegisterTraps<T: Tracer>(
    state: &mut State,
    tracer: &T,
    reg: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        tid0shadow_592: bool,
        gs_31789: bool,
        gs_31786: bool,
        gs_31785: bool,
        tid3shadow_593: bool,
        gs_31783: bool,
        gs_31788: bool,
        gs_31787: bool,
        reg: u8,
    }
    let fn_state = FunctionState {
        reg,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #440u : u32
        let s_0_3: u32 = 440;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b30 b1
        if s_0_6 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#31783 <= s_1_0
        fn_state.gs_31783 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#31783:u8
        let s_2_0: bool = fn_state.gs_31783;
        // N s_2_1: branch s_2_0 b4 b3
        if s_2_0 {
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
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #432u : u32
        let s_4_0: u32 = 432;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call ELUsingAArch32(s_4_1)
        let s_4_2: bool = ELUsingAArch32(state, tracer, s_4_1);
        // N s_4_3: branch s_4_2 b29 b5
        if s_4_2 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #102552u : u32
        let s_5_0: u32 = 102552;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_HCR_EL2_Type_TID0(s_5_1)
        let s_5_2: bool = u_get_HCR_EL2_Type_TID0(state, tracer, s_5_1);
        // D s_5_3: write-var tid0shadow#592 <= s_5_2
        fn_state.tid0shadow_592 = s_5_2;
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #432u : u32
        let s_6_0: u32 = 432;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call ELUsingAArch32(s_6_1)
        let s_6_2: bool = ELUsingAArch32(state, tracer, s_6_1);
        // N s_6_3: branch s_6_2 b28 b7
        if s_6_2 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #102552u : u32
        let s_7_0: u32 = 102552;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_HCR_EL2_Type_TID3(s_7_1)
        let s_7_2: bool = u_get_HCR_EL2_Type_TID3(state, tracer, s_7_1);
        // D s_7_3: write-var tid3shadow#593 <= s_7_2
        fn_state.tid3shadow_593 = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var tid0shadow#592:u8
        let s_8_0: bool = fn_state.tid0shadow_592;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #1u : u8
        let s_8_2: bool = true;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b27 b9
        if s_8_4 {
            return block_27(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#31785 <= s_9_0
        fn_state.gs_31785 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#31785:u8
        let s_10_0: bool = fn_state.gs_31785;
        // N s_10_1: branch s_10_0 b26 b11
        if s_10_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var tid3shadow#593:u8
        let s_11_0: bool = fn_state.tid3shadow_593;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #1u : u8
        let s_11_2: bool = true;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // N s_11_5: branch s_11_4 b19 b12
        if s_11_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#31788 <= s_12_0
        fn_state.gs_31788 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#31788:u8
        let s_13_0: bool = fn_state.gs_31788;
        // D s_13_1: write-var gs#31789 <= s_13_0
        fn_state.gs_31789 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#31789:u8
        let s_14_0: bool = fn_state.gs_31789;
        // N s_14_1: branch s_14_0 b16 b15
        if s_14_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
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
        // N s_16_3: branch s_16_2 b18 b17
        if s_16_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #8u : u8
        let s_17_0: u8 = 8;
        // C s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 4u16);
        // C s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (s_17_1.value() as i128);
        // C s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // C s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // C s_17_5: const #432u : u32
        let s_17_5: u32 = 432;
        // D s_17_6: read-reg s_17_5:u8
        let s_17_6: u8 = {
            let value = state.read_register::<u8>(s_17_5 as isize);
            tracer.read_register(s_17_5 as isize, value);
            value
        };
        // D s_17_7: call AArch64_AArch32SystemAccessTrap(s_17_6, s_17_4)
        let s_17_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_17_6, s_17_4);
        // N s_17_8: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #8u : u8
        let s_18_0: u8 = 8;
        // C s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 4u16);
        // C s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (s_18_1.value() as i128);
        // C s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // C s_18_5: const #400u : u32
        let s_18_5: u32 = 400;
        // D s_18_6: read-reg s_18_5:u8
        let s_18_6: u8 = {
            let value = state.read_register::<u8>(s_18_5 as isize);
            tracer.read_register(s_18_5 as isize, value);
            value
        };
        // D s_18_7: call AArch32_SystemAccessTrap(s_18_6, s_18_4)
        let s_18_7: () = AArch32_SystemAccessTrap(state, tracer, s_18_6, s_18_4);
        // N s_18_8: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var reg:u8
        let s_19_0: u8 = fn_state.reg;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 4u16);
        // C s_19_2: const #5u : u8
        let s_19_2: u8 = 5;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 4u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // N s_19_5: branch s_19_4 b25 b20
        if s_19_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var reg:u8
        let s_20_0: u8 = fn_state.reg;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 4u16);
        // C s_20_2: const #6u : u8
        let s_20_2: u8 = 6;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 4u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // N s_20_5: branch s_20_4 b24 b21
        if s_20_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var reg:u8
        let s_21_0: u8 = fn_state.reg;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 4u16);
        // C s_21_2: const #7u : u8
        let s_21_2: u8 = 7;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 4u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: write-var gs#31786 <= s_21_4
        fn_state.gs_31786 = s_21_4;
        // N s_21_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#31786:u8
        let s_22_0: bool = fn_state.gs_31786;
        // D s_22_1: write-var gs#31787 <= s_22_0
        fn_state.gs_31787 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#31787:u8
        let s_23_0: bool = fn_state.gs_31787;
        // D s_23_1: write-var gs#31788 <= s_23_0
        fn_state.gs_31788 = s_23_0;
        // N s_23_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#31786 <= s_24_0
        fn_state.gs_31786 = s_24_0;
        // N s_24_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#31787 <= s_25_0
        fn_state.gs_31787 = s_25_0;
        // N s_25_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#31789 <= s_26_0
        fn_state.gs_31789 = s_26_0;
        // N s_26_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var reg:u8
        let s_27_0: u8 = fn_state.reg;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 4u16);
        // C s_27_2: const #0u : u8
        let s_27_2: u8 = 0;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 4u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#31785 <= s_27_4
        fn_state.gs_31785 = s_27_4;
        // N s_27_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call HCR_read(s_28_0)
        let s_28_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_28_0);
        // S s_28_2: call _get_HCR_Type_TID3(s_28_1)
        let s_28_2: bool = u_get_HCR_Type_TID3(state, tracer, s_28_1);
        // D s_28_3: write-var tid3shadow#593 <= s_28_2
        fn_state.tid3shadow_593 = s_28_2;
        // N s_28_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call HCR_read(s_29_0)
        let s_29_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_29_0);
        // S s_29_2: call _get_HCR_Type_TID0(s_29_1)
        let s_29_2: bool = u_get_HCR_Type_TID0(state, tracer, s_29_1);
        // D s_29_3: write-var tid0shadow#592 <= s_29_2
        fn_state.tid0shadow_592 = s_29_2;
        // N s_29_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call EL2Enabled(s_30_0)
        let s_30_1: bool = EL2Enabled(state, tracer, s_30_0);
        // D s_30_2: write-var gs#31783 <= s_30_1
        fn_state.gs_31783 = s_30_1;
        // N s_30_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
