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
use EDSCR_write::*;
use Halted::*;
use u_update_EDSCR_Type_NS::*;
use HaveRME::*;
use u__UNKNOWN_bit::*;
use IsSecureEL2Enabled::*;
use CurrentSecurityState::*;
use u_update_EDSCR_Type_RW::*;
use u_get_SCRType_NS::*;
use SCR_GEN_read::*;
use ExternalSecureInvasiveDebugEnabled::*;
use u_update_EDSCR_Type_SDD::*;
use ExternalRootInvasiveDebugEnabled::*;
use u__UNKNOWN_bits::*;
use u_update_EDSCR_Type_EL::*;
use ELUsingAArch32::*;
use Bit::*;
use UsingAArch32::*;
use EDSCR_read::*;
use common::*;
pub fn UpdateEDSCRFields<T: Tracer>(state: &mut State, tracer: &T, gs_4461: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        RW: u8,
        ga_2874: bool,
        ga_2921: ProductType700c18a878c5601b,
        ssshadow_56: u32,
        gs_4519: bool,
        ga_2884: ProductType700c18a878c5601b,
        ga_2885: bool,
        ga_2947: bool,
        ga_2922: bool,
        gs_4521: bool,
        ga_2873: ProductType700c18a878c5601b,
        ga_2933: bool,
        ga_2925: bool,
        ga_2955: bool,
        gs_4520: bool,
        gs_4461: (),
    }
    let fn_state = FunctionState {
        gs_4461,
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
        // N s_0_3: branch s_0_2 b54 b1
        if s_0_2 {
            return block_54(state, tracer, fn_state);
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
        // S s_1_1: call EDSCR_read(s_1_0)
        let s_1_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_1_0);
        // C s_1_2: const #16975u : u32
        let s_1_2: u32 = 16975;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: u8 = {
            let value = state.read_register::<u8>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // D s_1_4: call _update_EDSCR_Type_EL(s_1_1, s_1_3)
        let s_1_4: ProductType700c18a878c5601b = u_update_EDSCR_Type_EL(
            state,
            tracer,
            s_1_1,
            s_1_3,
        );
        // D s_1_5: call EDSCR_write(s_1_4)
        let s_1_5: () = EDSCR_write(state, tracer, s_1_4);
        // C s_1_6: const #() : ()
        let s_1_6: () = ();
        // S s_1_7: call CurrentSecurityState(s_1_6)
        let s_1_7: u32 = CurrentSecurityState(state, tracer, s_1_6);
        // D s_1_8: write-var ssshadow#56 <= s_1_7
        fn_state.ssshadow_56 = s_1_7;
        // C s_1_9: const #() : ()
        let s_1_9: () = ();
        // S s_1_10: call HaveRME(s_1_9)
        let s_1_10: bool = HaveRME(state, tracer, s_1_9);
        // N s_1_11: branch s_1_10 b44 b2
        if s_1_10 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call EDSCR_read(s_2_0)
        let s_2_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_2_0);
        // D s_2_2: write-var ga#2921 <= s_2_1
        fn_state.ga_2921 = s_2_1;
        // D s_2_3: read-var ssshadow#56:u32
        let s_2_3: u32 = fn_state.ssshadow_56;
        // C s_2_4: const #3u : u32
        let s_2_4: u32 = 3;
        // D s_2_5: cmp-eq s_2_3 s_2_4
        let s_2_5: bool = ((s_2_3) == (s_2_4));
        // N s_2_6: branch s_2_5 b43 b3
        if s_2_5 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var ga#2922 <= s_3_0
        fn_state.ga_2922 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#2921:struct
        let s_4_0: ProductType700c18a878c5601b = fn_state.ga_2921;
        // D s_4_1: read-var ga#2922:u8
        let s_4_1: bool = fn_state.ga_2922;
        // D s_4_2: call _update_EDSCR_Type_NS(s_4_0, s_4_1)
        let s_4_2: ProductType700c18a878c5601b = u_update_EDSCR_Type_NS(
            state,
            tracer,
            s_4_0,
            s_4_1,
        );
        // D s_4_3: call EDSCR_write(s_4_2)
        let s_4_3: () = EDSCR_write(state, tracer, s_4_2);
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #440u : u32
        let s_5_0: u32 = 440;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call ELUsingAArch32(s_5_1)
        let s_5_2: bool = ELUsingAArch32(state, tracer, s_5_1);
        // N s_5_3: branch s_5_2 b42 b6
        if s_5_2 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var ga#2925 <= s_6_0
        fn_state.ga_2925 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#2925:u8
        let s_7_0: bool = fn_state.ga_2925;
        // D s_7_1: call Bit(s_7_0)
        let s_7_1: bool = Bit(state, tracer, s_7_0);
        // C s_7_2: const #1s : i
        let s_7_2: i128 = 1;
        // D s_7_3: read-var RW:u8
        let s_7_3: u8 = fn_state.RW;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 4u16);
        // C s_7_5: const #1u : u64
        let s_7_5: u64 = 1;
        // D s_7_6: bit-insert s_7_4 s_7_4 s_7_2 s_7_5
        let s_7_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_7_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_7_4.length(),
            );
            (s_7_4 & mask) | (s_7_4 << s_7_2)
        };
        // D s_7_7: cast reint s_7_6 -> u8
        let s_7_7: u8 = (s_7_6.value() as u8);
        // D s_7_8: write-var RW <= s_7_7
        fn_state.RW = s_7_7;
        // C s_7_9: const #16975u : u32
        let s_7_9: u32 = 16975;
        // D s_7_10: read-reg s_7_9:u8
        let s_7_10: u8 = {
            let value = state.read_register::<u8>(s_7_9 as isize);
            tracer.read_register(s_7_9 as isize, value);
            value
        };
        // D s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 2u16);
        // C s_7_12: const #448u : u32
        let s_7_12: u32 = 448;
        // D s_7_13: read-reg s_7_12:u8
        let s_7_13: u8 = {
            let value = state.read_register::<u8>(s_7_12 as isize);
            tracer.read_register(s_7_12 as isize, value);
            value
        };
        // D s_7_14: cast zx s_7_13 -> bv
        let s_7_14: Bits = Bits::new(s_7_13 as u128, 2u16);
        // D s_7_15: cmp-ne s_7_11 s_7_14
        let s_7_15: bool = ((s_7_11) != (s_7_14));
        // N s_7_16: branch s_7_15 b41 b8
        if s_7_15 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call UsingAArch32(s_8_0)
        let s_8_1: bool = UsingAArch32(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b40 b9
        if s_8_1 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var ga#2933 <= s_9_0
        fn_state.ga_2933 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#2933:u8
        let s_10_0: bool = fn_state.ga_2933;
        // D s_10_1: call Bit(s_10_0)
        let s_10_1: bool = Bit(state, tracer, s_10_0);
        // C s_10_2: const #0s : i
        let s_10_2: i128 = 0;
        // D s_10_3: read-var RW:u8
        let s_10_3: u8 = fn_state.RW;
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 4u16);
        // C s_10_5: const #1u : u64
        let s_10_5: u64 = 1;
        // D s_10_6: bit-insert s_10_4 s_10_4 s_10_2 s_10_5
        let s_10_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_10_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_10_4.length(),
            );
            (s_10_4 & mask) | (s_10_4 << s_10_2)
        };
        // D s_10_7: cast reint s_10_6 -> u8
        let s_10_7: u8 = (s_10_6.value() as u8);
        // D s_10_8: write-var RW <= s_10_7
        fn_state.RW = s_10_7;
        // N s_10_9: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #432u : u32
        let s_11_0: u32 = 432;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: u8 = {
            let value = state.read_register::<u8>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // C s_11_2: const #2u : u8
        let s_11_2: u8 = 2;
        // D s_11_3: cmp-lt s_11_1 s_11_2
        let s_11_3: bool = ((s_11_1) < (s_11_2));
        // D s_11_4: not s_11_3
        let s_11_4: bool = !s_11_3;
        // N s_11_5: branch s_11_4 b39 b12
        if s_11_4 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #424u : u32
        let s_12_0: u32 = 424;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // C s_12_2: const #2u : u8
        let s_12_2: u8 = 2;
        // D s_12_3: cmp-lt s_12_1 s_12_2
        let s_12_3: bool = ((s_12_1) < (s_12_2));
        // N s_12_4: branch s_12_3 b38 b13
        if s_12_3 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#4519 <= s_13_0
        fn_state.gs_4519 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#4519:u8
        let s_14_0: bool = fn_state.gs_4519;
        // N s_14_1: branch s_14_0 b37 b15
        if s_14_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#4520 <= s_15_0
        fn_state.gs_4520 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#4520:u8
        let s_16_0: bool = fn_state.gs_4520;
        // D s_16_1: write-var gs#4521 <= s_16_0
        fn_state.gs_4521 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#4521:u8
        let s_17_0: bool = fn_state.gs_4521;
        // N s_17_1: branch s_17_0 b36 b18
        if s_17_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #432u : u32
        let s_18_0: u32 = 432;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call ELUsingAArch32(s_18_1)
        let s_18_2: bool = ELUsingAArch32(state, tracer, s_18_1);
        // N s_18_3: branch s_18_2 b35 b19
        if s_18_2 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var ga#2947 <= s_19_0
        fn_state.ga_2947 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var ga#2947:u8
        let s_20_0: bool = fn_state.ga_2947;
        // D s_20_1: call Bit(s_20_0)
        let s_20_1: bool = Bit(state, tracer, s_20_0);
        // C s_20_2: const #2s : i
        let s_20_2: i128 = 2;
        // D s_20_3: read-var RW:u8
        let s_20_3: u8 = fn_state.RW;
        // D s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 4u16);
        // C s_20_5: const #1u : u64
        let s_20_5: u64 = 1;
        // D s_20_6: bit-insert s_20_4 s_20_4 s_20_2 s_20_5
        let s_20_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_20_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_20_4.length(),
            );
            (s_20_4 & mask) | (s_20_4 << s_20_2)
        };
        // D s_20_7: cast reint s_20_6 -> u8
        let s_20_7: u8 = (s_20_6.value() as u8);
        // D s_20_8: write-var RW <= s_20_7
        fn_state.RW = s_20_7;
        // N s_20_9: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #424u : u32
        let s_21_0: u32 = 424;
        // D s_21_1: read-reg s_21_0:u8
        let s_21_1: u8 = {
            let value = state.read_register::<u8>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // C s_21_2: const #2u : u8
        let s_21_2: u8 = 2;
        // D s_21_3: cmp-lt s_21_1 s_21_2
        let s_21_3: bool = ((s_21_1) < (s_21_2));
        // D s_21_4: not s_21_3
        let s_21_4: bool = !s_21_3;
        // N s_21_5: branch s_21_4 b34 b22
        if s_21_4 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #424u : u32
        let s_22_0: u32 = 424;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call ELUsingAArch32(s_22_1)
        let s_22_2: bool = ELUsingAArch32(state, tracer, s_22_1);
        // N s_22_3: branch s_22_2 b33 b23
        if s_22_2 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var ga#2955 <= s_23_0
        fn_state.ga_2955 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var ga#2955:u8
        let s_24_0: bool = fn_state.ga_2955;
        // D s_24_1: call Bit(s_24_0)
        let s_24_1: bool = Bit(state, tracer, s_24_0);
        // C s_24_2: const #3s : i
        let s_24_2: i128 = 3;
        // D s_24_3: read-var RW:u8
        let s_24_3: u8 = fn_state.RW;
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 4u16);
        // C s_24_5: const #1u : u64
        let s_24_5: u64 = 1;
        // D s_24_6: bit-insert s_24_4 s_24_4 s_24_2 s_24_5
        let s_24_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_24_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_24_4.length(),
            );
            (s_24_4 & mask) | (s_24_4 << s_24_2)
        };
        // D s_24_7: cast reint s_24_6 -> u8
        let s_24_7: u8 = (s_24_6.value() as u8);
        // D s_24_8: write-var RW <= s_24_7
        fn_state.RW = s_24_7;
        // N s_24_9: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #3s : i
        let s_25_0: i128 = 3;
        // D s_25_1: read-var RW:u8
        let s_25_1: u8 = fn_state.RW;
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 4u16);
        // C s_25_3: const #1u : u64
        let s_25_3: u64 = 1;
        // D s_25_4: bit-extract s_25_2 s_25_0 s_25_3
        let s_25_4: Bits = (Bits::new(
            ((s_25_2) >> (s_25_0)).value(),
            u16::try_from(s_25_3).unwrap(),
        ));
        // D s_25_5: cast reint s_25_4 -> u8
        let s_25_5: bool = ((s_25_4.value()) != 0);
        // C s_25_6: const #0s : i
        let s_25_6: i128 = 0;
        // C s_25_7: const #0u : u64
        let s_25_7: u64 = 0;
        // D s_25_8: cast zx s_25_5 -> u64
        let s_25_8: u64 = (s_25_5 as u64);
        // C s_25_9: const #1u : u64
        let s_25_9: u64 = 1;
        // D s_25_10: and s_25_8 s_25_9
        let s_25_10: u64 = ((s_25_8) & (s_25_9));
        // D s_25_11: cmp-eq s_25_10 s_25_9
        let s_25_11: bool = ((s_25_10) == (s_25_9));
        // D s_25_12: lsl s_25_8 s_25_6
        let s_25_12: u64 = s_25_8 << s_25_6;
        // D s_25_13: or s_25_7 s_25_12
        let s_25_13: u64 = ((s_25_7) | (s_25_12));
        // D s_25_14: cmpl s_25_12
        let s_25_14: u64 = !s_25_12;
        // D s_25_15: and s_25_7 s_25_14
        let s_25_15: u64 = ((s_25_7) & (s_25_14));
        // D s_25_16: select s_25_11 s_25_13 s_25_15
        let s_25_16: u64 = if s_25_11 { s_25_13 } else { s_25_15 };
        // D s_25_17: cast trunc s_25_16 -> u8
        let s_25_17: bool = ((s_25_16) != 0);
        // D s_25_18: cast zx s_25_17 -> bv
        let s_25_18: Bits = Bits::new(s_25_17 as u128, 1u16);
        // C s_25_19: const #0u : u8
        let s_25_19: bool = false;
        // C s_25_20: cast zx s_25_19 -> bv
        let s_25_20: Bits = Bits::new(s_25_19 as u128, 1u16);
        // D s_25_21: cmp-eq s_25_18 s_25_20
        let s_25_21: bool = ((s_25_18) == (s_25_20));
        // N s_25_22: branch s_25_21 b32 b26
        if s_25_21 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #2s : i
        let s_26_0: i128 = 2;
        // D s_26_1: read-var RW:u8
        let s_26_1: u8 = fn_state.RW;
        // D s_26_2: cast zx s_26_1 -> bv
        let s_26_2: Bits = Bits::new(s_26_1 as u128, 4u16);
        // C s_26_3: const #1u : u64
        let s_26_3: u64 = 1;
        // D s_26_4: bit-extract s_26_2 s_26_0 s_26_3
        let s_26_4: Bits = (Bits::new(
            ((s_26_2) >> (s_26_0)).value(),
            u16::try_from(s_26_3).unwrap(),
        ));
        // D s_26_5: cast reint s_26_4 -> u8
        let s_26_5: bool = ((s_26_4.value()) != 0);
        // C s_26_6: const #0s : i
        let s_26_6: i128 = 0;
        // C s_26_7: const #0u : u64
        let s_26_7: u64 = 0;
        // D s_26_8: cast zx s_26_5 -> u64
        let s_26_8: u64 = (s_26_5 as u64);
        // C s_26_9: const #1u : u64
        let s_26_9: u64 = 1;
        // D s_26_10: and s_26_8 s_26_9
        let s_26_10: u64 = ((s_26_8) & (s_26_9));
        // D s_26_11: cmp-eq s_26_10 s_26_9
        let s_26_11: bool = ((s_26_10) == (s_26_9));
        // D s_26_12: lsl s_26_8 s_26_6
        let s_26_12: u64 = s_26_8 << s_26_6;
        // D s_26_13: or s_26_7 s_26_12
        let s_26_13: u64 = ((s_26_7) | (s_26_12));
        // D s_26_14: cmpl s_26_12
        let s_26_14: u64 = !s_26_12;
        // D s_26_15: and s_26_7 s_26_14
        let s_26_15: u64 = ((s_26_7) & (s_26_14));
        // D s_26_16: select s_26_11 s_26_13 s_26_15
        let s_26_16: u64 = if s_26_11 { s_26_13 } else { s_26_15 };
        // D s_26_17: cast trunc s_26_16 -> u8
        let s_26_17: bool = ((s_26_16) != 0);
        // D s_26_18: cast zx s_26_17 -> bv
        let s_26_18: Bits = Bits::new(s_26_17 as u128, 1u16);
        // C s_26_19: const #0u : u8
        let s_26_19: bool = false;
        // C s_26_20: cast zx s_26_19 -> bv
        let s_26_20: Bits = Bits::new(s_26_19 as u128, 1u16);
        // D s_26_21: cmp-eq s_26_18 s_26_20
        let s_26_21: bool = ((s_26_18) == (s_26_20));
        // N s_26_22: branch s_26_21 b31 b27
        if s_26_21 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1s : i
        let s_27_0: i128 = 1;
        // D s_27_1: read-var RW:u8
        let s_27_1: u8 = fn_state.RW;
        // D s_27_2: cast zx s_27_1 -> bv
        let s_27_2: Bits = Bits::new(s_27_1 as u128, 4u16);
        // C s_27_3: const #1u : u64
        let s_27_3: u64 = 1;
        // D s_27_4: bit-extract s_27_2 s_27_0 s_27_3
        let s_27_4: Bits = (Bits::new(
            ((s_27_2) >> (s_27_0)).value(),
            u16::try_from(s_27_3).unwrap(),
        ));
        // D s_27_5: cast reint s_27_4 -> u8
        let s_27_5: bool = ((s_27_4.value()) != 0);
        // C s_27_6: const #0s : i
        let s_27_6: i128 = 0;
        // C s_27_7: const #0u : u64
        let s_27_7: u64 = 0;
        // D s_27_8: cast zx s_27_5 -> u64
        let s_27_8: u64 = (s_27_5 as u64);
        // C s_27_9: const #1u : u64
        let s_27_9: u64 = 1;
        // D s_27_10: and s_27_8 s_27_9
        let s_27_10: u64 = ((s_27_8) & (s_27_9));
        // D s_27_11: cmp-eq s_27_10 s_27_9
        let s_27_11: bool = ((s_27_10) == (s_27_9));
        // D s_27_12: lsl s_27_8 s_27_6
        let s_27_12: u64 = s_27_8 << s_27_6;
        // D s_27_13: or s_27_7 s_27_12
        let s_27_13: u64 = ((s_27_7) | (s_27_12));
        // D s_27_14: cmpl s_27_12
        let s_27_14: u64 = !s_27_12;
        // D s_27_15: and s_27_7 s_27_14
        let s_27_15: u64 = ((s_27_7) & (s_27_14));
        // D s_27_16: select s_27_11 s_27_13 s_27_15
        let s_27_16: u64 = if s_27_11 { s_27_13 } else { s_27_15 };
        // D s_27_17: cast trunc s_27_16 -> u8
        let s_27_17: bool = ((s_27_16) != 0);
        // D s_27_18: cast zx s_27_17 -> bv
        let s_27_18: Bits = Bits::new(s_27_17 as u128, 1u16);
        // C s_27_19: const #0u : u8
        let s_27_19: bool = false;
        // C s_27_20: cast zx s_27_19 -> bv
        let s_27_20: Bits = Bits::new(s_27_19 as u128, 1u16);
        // D s_27_21: cmp-eq s_27_18 s_27_20
        let s_27_21: bool = ((s_27_18) == (s_27_20));
        // N s_27_22: branch s_27_21 b30 b28
        if s_27_21 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call EDSCR_read(s_29_0)
        let s_29_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_29_0);
        // D s_29_2: read-var RW:u8
        let s_29_2: u8 = fn_state.RW;
        // D s_29_3: call _update_EDSCR_Type_RW(s_29_1, s_29_2)
        let s_29_3: ProductType700c18a878c5601b = u_update_EDSCR_Type_RW(
            state,
            tracer,
            s_29_1,
            s_29_2,
        );
        // D s_29_4: call EDSCR_write(s_29_3)
        let s_29_4: () = EDSCR_write(state, tracer, s_29_3);
        // N s_29_5: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call __UNKNOWN_bit(s_30_0)
        let s_30_1: bool = u__UNKNOWN_bit(state, tracer, s_30_0);
        // S s_30_2: call Bit(s_30_1)
        let s_30_2: bool = Bit(state, tracer, s_30_1);
        // C s_30_3: const #0s : i
        let s_30_3: i128 = 0;
        // D s_30_4: read-var RW:u8
        let s_30_4: u8 = fn_state.RW;
        // D s_30_5: cast zx s_30_4 -> bv
        let s_30_5: Bits = Bits::new(s_30_4 as u128, 4u16);
        // C s_30_6: const #1u : u64
        let s_30_6: u64 = 1;
        // D s_30_7: bit-insert s_30_5 s_30_5 s_30_3 s_30_6
        let s_30_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_30_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_30_5.length(),
            );
            (s_30_5 & mask) | (s_30_5 << s_30_3)
        };
        // D s_30_8: cast reint s_30_7 -> u8
        let s_30_8: u8 = (s_30_7.value() as u8);
        // D s_30_9: write-var RW <= s_30_8
        fn_state.RW = s_30_8;
        // N s_30_10: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #2s : i64
        let s_31_0: i64 = 2;
        // C s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // S s_31_2: call __UNKNOWN_bits(s_31_1)
        let s_31_2: Bits = u__UNKNOWN_bits(state, tracer, s_31_1);
        // S s_31_3: cast reint s_31_2 -> u8
        let s_31_3: u8 = (s_31_2.value() as u8);
        // C s_31_4: const #0s : i
        let s_31_4: i128 = 0;
        // D s_31_5: read-var RW:u8
        let s_31_5: u8 = fn_state.RW;
        // D s_31_6: cast zx s_31_5 -> bv
        let s_31_6: Bits = Bits::new(s_31_5 as u128, 4u16);
        // S s_31_7: cast zx s_31_3 -> bv
        let s_31_7: Bits = Bits::new(s_31_3 as u128, 2u16);
        // C s_31_8: const #1s : i
        let s_31_8: i128 = 1;
        // C s_31_9: const #1u : u64
        let s_31_9: u64 = 1;
        // C s_31_10: cast zx s_31_9 -> bv
        let s_31_10: Bits = Bits::new(s_31_9 as u128, 64u16);
        // C s_31_11: lsl s_31_10 s_31_8
        let s_31_11: Bits = s_31_10 << s_31_8;
        // C s_31_12: sub s_31_11 s_31_10
        let s_31_12: Bits = ((s_31_11) - (s_31_10));
        // S s_31_13: and s_31_7 s_31_12
        let s_31_13: Bits = ((s_31_7) & (s_31_12));
        // S s_31_14: lsl s_31_13 s_31_4
        let s_31_14: Bits = s_31_13 << s_31_4;
        // C s_31_15: lsl s_31_12 s_31_4
        let s_31_15: Bits = s_31_12 << s_31_4;
        // C s_31_16: cmpl s_31_15
        let s_31_16: Bits = !s_31_15;
        // D s_31_17: and s_31_6 s_31_16
        let s_31_17: Bits = ((s_31_6) & (s_31_16));
        // D s_31_18: or s_31_17 s_31_14
        let s_31_18: Bits = ((s_31_17) | (s_31_14));
        // D s_31_19: cast reint s_31_18 -> u8
        let s_31_19: u8 = (s_31_18.value() as u8);
        // D s_31_20: write-var RW <= s_31_19
        fn_state.RW = s_31_19;
        // N s_31_21: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #3s : i64
        let s_32_0: i64 = 3;
        // C s_32_1: cast zx s_32_0 -> i
        let s_32_1: i128 = (i128::try_from(s_32_0).unwrap());
        // S s_32_2: call __UNKNOWN_bits(s_32_1)
        let s_32_2: Bits = u__UNKNOWN_bits(state, tracer, s_32_1);
        // S s_32_3: cast reint s_32_2 -> u8
        let s_32_3: u8 = (s_32_2.value() as u8);
        // C s_32_4: const #0s : i
        let s_32_4: i128 = 0;
        // D s_32_5: read-var RW:u8
        let s_32_5: u8 = fn_state.RW;
        // D s_32_6: cast zx s_32_5 -> bv
        let s_32_6: Bits = Bits::new(s_32_5 as u128, 4u16);
        // S s_32_7: cast zx s_32_3 -> bv
        let s_32_7: Bits = Bits::new(s_32_3 as u128, 3u16);
        // C s_32_8: const #2s : i
        let s_32_8: i128 = 2;
        // C s_32_9: const #1u : u64
        let s_32_9: u64 = 1;
        // C s_32_10: cast zx s_32_9 -> bv
        let s_32_10: Bits = Bits::new(s_32_9 as u128, 64u16);
        // C s_32_11: lsl s_32_10 s_32_8
        let s_32_11: Bits = s_32_10 << s_32_8;
        // C s_32_12: sub s_32_11 s_32_10
        let s_32_12: Bits = ((s_32_11) - (s_32_10));
        // S s_32_13: and s_32_7 s_32_12
        let s_32_13: Bits = ((s_32_7) & (s_32_12));
        // S s_32_14: lsl s_32_13 s_32_4
        let s_32_14: Bits = s_32_13 << s_32_4;
        // C s_32_15: lsl s_32_12 s_32_4
        let s_32_15: Bits = s_32_12 << s_32_4;
        // C s_32_16: cmpl s_32_15
        let s_32_16: Bits = !s_32_15;
        // D s_32_17: and s_32_6 s_32_16
        let s_32_17: Bits = ((s_32_6) & (s_32_16));
        // D s_32_18: or s_32_17 s_32_14
        let s_32_18: Bits = ((s_32_17) | (s_32_14));
        // D s_32_19: cast reint s_32_18 -> u8
        let s_32_19: u8 = (s_32_18.value() as u8);
        // D s_32_20: write-var RW <= s_32_19
        fn_state.RW = s_32_19;
        // N s_32_21: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var ga#2955 <= s_33_0
        fn_state.ga_2955 = s_33_0;
        // N s_33_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #2s : i
        let s_34_0: i128 = 2;
        // D s_34_1: read-var RW:u8
        let s_34_1: u8 = fn_state.RW;
        // D s_34_2: cast zx s_34_1 -> bv
        let s_34_2: Bits = Bits::new(s_34_1 as u128, 4u16);
        // C s_34_3: const #1u : u64
        let s_34_3: u64 = 1;
        // D s_34_4: bit-extract s_34_2 s_34_0 s_34_3
        let s_34_4: Bits = (Bits::new(
            ((s_34_2) >> (s_34_0)).value(),
            u16::try_from(s_34_3).unwrap(),
        ));
        // D s_34_5: cast reint s_34_4 -> u8
        let s_34_5: bool = ((s_34_4.value()) != 0);
        // C s_34_6: const #0s : i
        let s_34_6: i128 = 0;
        // C s_34_7: const #0u : u64
        let s_34_7: u64 = 0;
        // D s_34_8: cast zx s_34_5 -> u64
        let s_34_8: u64 = (s_34_5 as u64);
        // C s_34_9: const #1u : u64
        let s_34_9: u64 = 1;
        // D s_34_10: and s_34_8 s_34_9
        let s_34_10: u64 = ((s_34_8) & (s_34_9));
        // D s_34_11: cmp-eq s_34_10 s_34_9
        let s_34_11: bool = ((s_34_10) == (s_34_9));
        // D s_34_12: lsl s_34_8 s_34_6
        let s_34_12: u64 = s_34_8 << s_34_6;
        // D s_34_13: or s_34_7 s_34_12
        let s_34_13: u64 = ((s_34_7) | (s_34_12));
        // D s_34_14: cmpl s_34_12
        let s_34_14: u64 = !s_34_12;
        // D s_34_15: and s_34_7 s_34_14
        let s_34_15: u64 = ((s_34_7) & (s_34_14));
        // D s_34_16: select s_34_11 s_34_13 s_34_15
        let s_34_16: u64 = if s_34_11 { s_34_13 } else { s_34_15 };
        // D s_34_17: cast trunc s_34_16 -> u8
        let s_34_17: bool = ((s_34_16) != 0);
        // D s_34_18: call Bit(s_34_17)
        let s_34_18: bool = Bit(state, tracer, s_34_17);
        // C s_34_19: const #3s : i
        let s_34_19: i128 = 3;
        // D s_34_20: read-var RW:u8
        let s_34_20: u8 = fn_state.RW;
        // D s_34_21: cast zx s_34_20 -> bv
        let s_34_21: Bits = Bits::new(s_34_20 as u128, 4u16);
        // C s_34_22: const #1u : u64
        let s_34_22: u64 = 1;
        // D s_34_23: bit-insert s_34_21 s_34_21 s_34_19 s_34_22
        let s_34_23: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_34_22 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_34_21.length(),
            );
            (s_34_21 & mask) | (s_34_21 << s_34_19)
        };
        // D s_34_24: cast reint s_34_23 -> u8
        let s_34_24: u8 = (s_34_23.value() as u8);
        // D s_34_25: write-var RW <= s_34_24
        fn_state.RW = s_34_24;
        // N s_34_26: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var ga#2947 <= s_35_0
        fn_state.ga_2947 = s_35_0;
        // N s_35_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1s : i
        let s_36_0: i128 = 1;
        // D s_36_1: read-var RW:u8
        let s_36_1: u8 = fn_state.RW;
        // D s_36_2: cast zx s_36_1 -> bv
        let s_36_2: Bits = Bits::new(s_36_1 as u128, 4u16);
        // C s_36_3: const #1u : u64
        let s_36_3: u64 = 1;
        // D s_36_4: bit-extract s_36_2 s_36_0 s_36_3
        let s_36_4: Bits = (Bits::new(
            ((s_36_2) >> (s_36_0)).value(),
            u16::try_from(s_36_3).unwrap(),
        ));
        // D s_36_5: cast reint s_36_4 -> u8
        let s_36_5: bool = ((s_36_4.value()) != 0);
        // C s_36_6: const #0s : i
        let s_36_6: i128 = 0;
        // C s_36_7: const #0u : u64
        let s_36_7: u64 = 0;
        // D s_36_8: cast zx s_36_5 -> u64
        let s_36_8: u64 = (s_36_5 as u64);
        // C s_36_9: const #1u : u64
        let s_36_9: u64 = 1;
        // D s_36_10: and s_36_8 s_36_9
        let s_36_10: u64 = ((s_36_8) & (s_36_9));
        // D s_36_11: cmp-eq s_36_10 s_36_9
        let s_36_11: bool = ((s_36_10) == (s_36_9));
        // D s_36_12: lsl s_36_8 s_36_6
        let s_36_12: u64 = s_36_8 << s_36_6;
        // D s_36_13: or s_36_7 s_36_12
        let s_36_13: u64 = ((s_36_7) | (s_36_12));
        // D s_36_14: cmpl s_36_12
        let s_36_14: u64 = !s_36_12;
        // D s_36_15: and s_36_7 s_36_14
        let s_36_15: u64 = ((s_36_7) & (s_36_14));
        // D s_36_16: select s_36_11 s_36_13 s_36_15
        let s_36_16: u64 = if s_36_11 { s_36_13 } else { s_36_15 };
        // D s_36_17: cast trunc s_36_16 -> u8
        let s_36_17: bool = ((s_36_16) != 0);
        // D s_36_18: call Bit(s_36_17)
        let s_36_18: bool = Bit(state, tracer, s_36_17);
        // C s_36_19: const #2s : i
        let s_36_19: i128 = 2;
        // D s_36_20: read-var RW:u8
        let s_36_20: u8 = fn_state.RW;
        // D s_36_21: cast zx s_36_20 -> bv
        let s_36_21: Bits = Bits::new(s_36_20 as u128, 4u16);
        // C s_36_22: const #1u : u64
        let s_36_22: u64 = 1;
        // D s_36_23: bit-insert s_36_21 s_36_21 s_36_19 s_36_22
        let s_36_23: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_36_22 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_36_21.length(),
            );
            (s_36_21 & mask) | (s_36_21 << s_36_19)
        };
        // D s_36_24: cast reint s_36_23 -> u8
        let s_36_24: u8 = (s_36_23.value() as u8);
        // D s_36_25: write-var RW <= s_36_24
        fn_state.RW = s_36_24;
        // N s_36_26: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call IsSecureEL2Enabled(s_37_0)
        let s_37_1: bool = IsSecureEL2Enabled(state, tracer, s_37_0);
        // S s_37_2: not s_37_1
        let s_37_2: bool = !s_37_1;
        // D s_37_3: write-var gs#4520 <= s_37_2
        fn_state.gs_4520 = s_37_2;
        // N s_37_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call SCR_GEN_read(s_38_0)
        let s_38_1: ProductType5c790c8ef59cc8b2 = SCR_GEN_read(state, tracer, s_38_0);
        // S s_38_2: call _get_SCRType_NS(s_38_1)
        let s_38_2: bool = u_get_SCRType_NS(state, tracer, s_38_1);
        // S s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // C s_38_4: const #0u : u8
        let s_38_4: bool = false;
        // C s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 1u16);
        // S s_38_6: cmp-eq s_38_3 s_38_5
        let s_38_6: bool = ((s_38_3) == (s_38_5));
        // D s_38_7: write-var gs#4519 <= s_38_6
        fn_state.gs_4519 = s_38_6;
        // N s_38_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#4521 <= s_39_0
        fn_state.gs_4521 = s_39_0;
        // N s_39_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var ga#2933 <= s_40_0
        fn_state.ga_2933 = s_40_0;
        // N s_40_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1s : i
        let s_41_0: i128 = 1;
        // D s_41_1: read-var RW:u8
        let s_41_1: u8 = fn_state.RW;
        // D s_41_2: cast zx s_41_1 -> bv
        let s_41_2: Bits = Bits::new(s_41_1 as u128, 4u16);
        // C s_41_3: const #1u : u64
        let s_41_3: u64 = 1;
        // D s_41_4: bit-extract s_41_2 s_41_0 s_41_3
        let s_41_4: Bits = (Bits::new(
            ((s_41_2) >> (s_41_0)).value(),
            u16::try_from(s_41_3).unwrap(),
        ));
        // D s_41_5: cast reint s_41_4 -> u8
        let s_41_5: bool = ((s_41_4.value()) != 0);
        // C s_41_6: const #0s : i
        let s_41_6: i128 = 0;
        // C s_41_7: const #0u : u64
        let s_41_7: u64 = 0;
        // D s_41_8: cast zx s_41_5 -> u64
        let s_41_8: u64 = (s_41_5 as u64);
        // C s_41_9: const #1u : u64
        let s_41_9: u64 = 1;
        // D s_41_10: and s_41_8 s_41_9
        let s_41_10: u64 = ((s_41_8) & (s_41_9));
        // D s_41_11: cmp-eq s_41_10 s_41_9
        let s_41_11: bool = ((s_41_10) == (s_41_9));
        // D s_41_12: lsl s_41_8 s_41_6
        let s_41_12: u64 = s_41_8 << s_41_6;
        // D s_41_13: or s_41_7 s_41_12
        let s_41_13: u64 = ((s_41_7) | (s_41_12));
        // D s_41_14: cmpl s_41_12
        let s_41_14: u64 = !s_41_12;
        // D s_41_15: and s_41_7 s_41_14
        let s_41_15: u64 = ((s_41_7) & (s_41_14));
        // D s_41_16: select s_41_11 s_41_13 s_41_15
        let s_41_16: u64 = if s_41_11 { s_41_13 } else { s_41_15 };
        // D s_41_17: cast trunc s_41_16 -> u8
        let s_41_17: bool = ((s_41_16) != 0);
        // D s_41_18: call Bit(s_41_17)
        let s_41_18: bool = Bit(state, tracer, s_41_17);
        // C s_41_19: const #0s : i
        let s_41_19: i128 = 0;
        // D s_41_20: read-var RW:u8
        let s_41_20: u8 = fn_state.RW;
        // D s_41_21: cast zx s_41_20 -> bv
        let s_41_21: Bits = Bits::new(s_41_20 as u128, 4u16);
        // C s_41_22: const #1u : u64
        let s_41_22: u64 = 1;
        // D s_41_23: bit-insert s_41_21 s_41_21 s_41_19 s_41_22
        let s_41_23: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_41_22 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_41_21.length(),
            );
            (s_41_21 & mask) | (s_41_21 << s_41_19)
        };
        // D s_41_24: cast reint s_41_23 -> u8
        let s_41_24: u8 = (s_41_23.value() as u8);
        // D s_41_25: write-var RW <= s_41_24
        fn_state.RW = s_41_24;
        // N s_41_26: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var ga#2925 <= s_42_0
        fn_state.ga_2925 = s_42_0;
        // N s_42_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var ga#2922 <= s_43_0
        fn_state.ga_2922 = s_43_0;
        // N s_43_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #3u : u32
        let s_44_0: u32 = 3;
        // D s_44_1: read-var ssshadow#56:u32
        let s_44_1: u32 = fn_state.ssshadow_56;
        // D s_44_2: cmp-eq s_44_0 s_44_1
        let s_44_2: bool = ((s_44_0) == (s_44_1));
        // D s_44_3: not s_44_2
        let s_44_3: bool = !s_44_2;
        // N s_44_4: branch s_44_3 b47 b45
        if s_44_3 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call EDSCR_read(s_45_0)
        let s_45_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_45_0);
        // S s_45_2: call EDSCR_write(s_45_1)
        let s_45_2: () = EDSCR_write(state, tracer, s_45_1);
        // N s_45_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u32
        let s_47_0: u32 = 0;
        // D s_47_1: read-var ssshadow#56:u32
        let s_47_1: u32 = fn_state.ssshadow_56;
        // D s_47_2: cmp-eq s_47_0 s_47_1
        let s_47_2: bool = ((s_47_0) == (s_47_1));
        // D s_47_3: not s_47_2
        let s_47_3: bool = !s_47_2;
        // N s_47_4: branch s_47_3 b49 b48
        if s_47_3 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EDSCR_read(s_48_0)
        let s_48_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_48_0);
        // S s_48_2: call EDSCR_write(s_48_1)
        let s_48_2: () = EDSCR_write(state, tracer, s_48_1);
        // N s_48_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u32
        let s_49_0: u32 = 1;
        // D s_49_1: read-var ssshadow#56:u32
        let s_49_1: u32 = fn_state.ssshadow_56;
        // D s_49_2: cmp-eq s_49_0 s_49_1
        let s_49_2: bool = ((s_49_0) == (s_49_1));
        // D s_49_3: not s_49_2
        let s_49_3: bool = !s_49_2;
        // N s_49_4: branch s_49_3 b51 b50
        if s_49_3 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call EDSCR_read(s_50_0)
        let s_50_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_50_0);
        // S s_50_2: call EDSCR_write(s_50_1)
        let s_50_2: () = EDSCR_write(state, tracer, s_50_1);
        // N s_50_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #2u : u32
        let s_51_0: u32 = 2;
        // D s_51_1: read-var ssshadow#56:u32
        let s_51_1: u32 = fn_state.ssshadow_56;
        // D s_51_2: cmp-eq s_51_0 s_51_1
        let s_51_2: bool = ((s_51_0) == (s_51_1));
        // D s_51_3: not s_51_2
        let s_51_3: bool = !s_51_2;
        // N s_51_4: branch s_51_3 b53 b52
        if s_51_3 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EDSCR_read(s_52_0)
        let s_52_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_52_0);
        // S s_52_2: call EDSCR_write(s_52_1)
        let s_52_2: () = EDSCR_write(state, tracer, s_52_1);
        // N s_52_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call EDSCR_read(s_54_0)
        let s_54_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_54_0);
        // C s_54_2: const #0u : u8
        let s_54_2: u8 = 0;
        // S s_54_3: call _update_EDSCR_Type_EL(s_54_1, s_54_2)
        let s_54_3: ProductType700c18a878c5601b = u_update_EDSCR_Type_EL(
            state,
            tracer,
            s_54_1,
            s_54_2,
        );
        // S s_54_4: call EDSCR_write(s_54_3)
        let s_54_4: () = EDSCR_write(state, tracer, s_54_3);
        // C s_54_5: const #() : ()
        let s_54_5: () = ();
        // S s_54_6: call HaveRME(s_54_5)
        let s_54_6: bool = HaveRME(state, tracer, s_54_5);
        // N s_54_7: branch s_54_6 b60 b55
        if s_54_6 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call EDSCR_read(s_55_0)
        let s_55_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_55_0);
        // D s_55_2: write-var ga#2884 <= s_55_1
        fn_state.ga_2884 = s_55_1;
        // C s_55_3: const #() : ()
        let s_55_3: () = ();
        // S s_55_4: call ExternalSecureInvasiveDebugEnabled(s_55_3)
        let s_55_4: bool = ExternalSecureInvasiveDebugEnabled(state, tracer, s_55_3);
        // N s_55_5: branch s_55_4 b59 b56
        if s_55_4 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var ga#2885 <= s_56_0
        fn_state.ga_2885 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var ga#2884:struct
        let s_57_0: ProductType700c18a878c5601b = fn_state.ga_2884;
        // D s_57_1: read-var ga#2885:u8
        let s_57_1: bool = fn_state.ga_2885;
        // D s_57_2: call _update_EDSCR_Type_SDD(s_57_0, s_57_1)
        let s_57_2: ProductType700c18a878c5601b = u_update_EDSCR_Type_SDD(
            state,
            tracer,
            s_57_0,
            s_57_1,
        );
        // D s_57_3: call EDSCR_write(s_57_2)
        let s_57_3: () = EDSCR_write(state, tracer, s_57_2);
        // C s_57_4: const #() : ()
        let s_57_4: () = ();
        // S s_57_5: call EDSCR_read(s_57_4)
        let s_57_5: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_57_4);
        // C s_57_6: const #() : ()
        let s_57_6: () = ();
        // S s_57_7: call __UNKNOWN_bit(s_57_6)
        let s_57_7: bool = u__UNKNOWN_bit(state, tracer, s_57_6);
        // S s_57_8: call _update_EDSCR_Type_NS(s_57_5, s_57_7)
        let s_57_8: ProductType700c18a878c5601b = u_update_EDSCR_Type_NS(
            state,
            tracer,
            s_57_5,
            s_57_7,
        );
        // S s_57_9: call EDSCR_write(s_57_8)
        let s_57_9: () = EDSCR_write(state, tracer, s_57_8);
        // N s_57_10: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call EDSCR_read(s_58_0)
        let s_58_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_58_0);
        // C s_58_2: const #15u : u8
        let s_58_2: u8 = 15;
        // S s_58_3: call _update_EDSCR_Type_RW(s_58_1, s_58_2)
        let s_58_3: ProductType700c18a878c5601b = u_update_EDSCR_Type_RW(
            state,
            tracer,
            s_58_1,
            s_58_2,
        );
        // S s_58_4: call EDSCR_write(s_58_3)
        let s_58_4: () = EDSCR_write(state, tracer, s_58_3);
        // N s_58_5: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var ga#2885 <= s_59_0
        fn_state.ga_2885 = s_59_0;
        // N s_59_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call EDSCR_read(s_60_0)
        let s_60_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_60_0);
        // D s_60_2: write-var ga#2873 <= s_60_1
        fn_state.ga_2873 = s_60_1;
        // C s_60_3: const #() : ()
        let s_60_3: () = ();
        // S s_60_4: call ExternalRootInvasiveDebugEnabled(s_60_3)
        let s_60_4: bool = ExternalRootInvasiveDebugEnabled(state, tracer, s_60_3);
        // N s_60_5: branch s_60_4 b63 b61
        if s_60_4 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var ga#2874 <= s_61_0
        fn_state.ga_2874 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var ga#2873:struct
        let s_62_0: ProductType700c18a878c5601b = fn_state.ga_2873;
        // D s_62_1: read-var ga#2874:u8
        let s_62_1: bool = fn_state.ga_2874;
        // D s_62_2: call _update_EDSCR_Type_SDD(s_62_0, s_62_1)
        let s_62_2: ProductType700c18a878c5601b = u_update_EDSCR_Type_SDD(
            state,
            tracer,
            s_62_0,
            s_62_1,
        );
        // D s_62_3: call EDSCR_write(s_62_2)
        let s_62_3: () = EDSCR_write(state, tracer, s_62_2);
        // C s_62_4: const #() : ()
        let s_62_4: () = ();
        // S s_62_5: call EDSCR_read(s_62_4)
        let s_62_5: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_62_4);
        // C s_62_6: const #2s : i64
        let s_62_6: i64 = 2;
        // C s_62_7: cast zx s_62_6 -> i
        let s_62_7: i128 = (i128::try_from(s_62_6).unwrap());
        // S s_62_8: call __UNKNOWN_bits(s_62_7)
        let s_62_8: Bits = u__UNKNOWN_bits(state, tracer, s_62_7);
        // S s_62_9: call EDSCR_write(s_62_5)
        let s_62_9: () = EDSCR_write(state, tracer, s_62_5);
        // N s_62_10: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var ga#2874 <= s_63_0
        fn_state.ga_2874 = s_63_0;
        // N s_63_2: jump b62
        return block_62(state, tracer, fn_state);
    }
}
