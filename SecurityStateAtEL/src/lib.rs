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
use HaveSecureEL2Ext::*;
use HaveRME::*;
use ELUsingAArch32::*;
use SecureOnlyImplementation::*;
use u_get_SCR_EL3_Type_NSE::*;
use EffectiveSCR_EL3_NS::*;
use EL2Enabled::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_SCR_Type_NS::*;
use Unreachable::*;
use common::*;
pub fn SecurityStateAtEL<T: Tracer>(state: &mut State, tracer: &T, EL: u8) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_2069: bool,
        return_value: u32,
        ga_1071: u32,
        effective_nse_nsshadow_20: u8,
        ga_1074: u32,
        EL: u8,
    }
    let fn_state = FunctionState {
        EL,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveRME(s_0_0)
        let s_0_1: bool = HaveRME(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b21 b1
        if s_0_1 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_2_0: const #424u : u32
        let s_2_0: u32 = 424;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // C s_2_2: const #2u : u8
        let s_2_2: u8 = 2;
        // D s_2_3: cmp-lt s_2_1 s_2_2
        let s_2_3: bool = ((s_2_1) < (s_2_2));
        // D s_2_4: not s_2_3
        let s_2_4: bool = !s_2_3;
        // N s_2_5: branch s_2_4 b18 b3
        if s_2_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_3_0: read-var EL:u8
        let s_3_0: u8 = fn_state.EL;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #424u : u32
        let s_3_2: u32 = 424;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // N s_3_6: branch s_3_5 b17 b4
        if s_3_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_4_0: read-var EL:u8
        let s_4_0: u8 = fn_state.EL;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #432u : u32
        let s_4_2: u32 = 432;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-ne s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) != (s_4_4));
        // N s_4_6: branch s_4_5 b16 b5
        if s_4_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call EL2Enabled(s_5_0)
        let s_5_1: bool = EL2Enabled(state, tracer, s_5_0);
        // D s_5_2: write-var gs#2069 <= s_5_1
        fn_state.gs_2069 = s_5_1;
        // N s_5_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_6_0: read-var gs#2069:u8
        let s_6_0: bool = fn_state.gs_2069;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // C s_6_2: const #424u : u32
        let s_6_2: u32 = 424;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: call ELUsingAArch32(s_6_3)
        let s_6_4: bool = ELUsingAArch32(state, tracer, s_6_3);
        // D s_6_5: not s_6_4
        let s_6_5: bool = !s_6_4;
        // N s_6_6: branch s_6_5 b12 b7
        if s_6_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_7_0: const #20920u : u32
        let s_7_0: u32 = 20920;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_SCR_Type_NS(s_7_1)
        let s_7_2: bool = u_get_SCR_Type_NS(state, tracer, s_7_1);
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // C s_7_4: const #1u : u8
        let s_7_4: bool = true;
        // C s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 1u16);
        // D s_7_6: cmp-eq s_7_3 s_7_5
        let s_7_6: bool = ((s_7_3) == (s_7_5));
        // N s_7_7: branch s_7_6 b11 b8
        if s_7_6 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_8_0: const #3u : u32
        let s_8_0: u32 = 3;
        // D s_8_1: write-var ga#1074 <= s_8_0
        fn_state.ga_1074 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_9_0: read-var ga#1074:u32
        let s_9_0: u32 = fn_state.ga_1074;
        // D s_9_1: write-var return_value <= s_9_0
        fn_state.return_value = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_10_0: read-var return_value:u32
        let s_10_0: u32 = fn_state.return_value;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_11_0: const #0u : u32
        let s_11_0: u32 = 0;
        // D s_11_1: write-var ga#1074 <= s_11_0
        fn_state.ga_1074 = s_11_0;
        // N s_11_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_12_0: const #90704u : u32
        let s_12_0: u32 = 90704;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_SCR_EL3_Type_NS(s_12_1)
        let s_12_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_12_1);
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // C s_12_4: const #1u : u8
        let s_12_4: bool = true;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 1u16);
        // D s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // N s_12_7: branch s_12_6 b15 b13
        if s_12_6 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_13_0: const #3u : u32
        let s_13_0: u32 = 3;
        // D s_13_1: write-var ga#1071 <= s_13_0
        fn_state.ga_1071 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_14_0: read-var ga#1071:u32
        let s_14_0: u32 = fn_state.ga_1071;
        // D s_14_1: write-var return_value <= s_14_0
        fn_state.return_value = s_14_0;
        // N s_14_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_15_0: const #0u : u32
        let s_15_0: u32 = 0;
        // D s_15_1: write-var ga#1071 <= s_15_0
        fn_state.ga_1071 = s_15_0;
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#2069 <= s_16_0
        fn_state.gs_2069 = s_16_0;
        // N s_16_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_17_0: const #3u : u32
        let s_17_0: u32 = 3;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call SecureOnlyImplementation(s_18_0)
        let s_18_1: bool = SecureOnlyImplementation(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b20 b19
        if s_18_1 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_19_0: const #0u : u32
        let s_19_0: u32 = 0;
        // D s_19_1: write-var return_value <= s_19_0
        fn_state.return_value = s_19_0;
        // N s_19_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_20_0: const #3u : u32
        let s_20_0: u32 = 3;
        // D s_20_1: write-var return_value <= s_20_0
        fn_state.return_value = s_20_0;
        // N s_20_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_21_0: read-var EL:u8
        let s_21_0: u8 = fn_state.EL;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 2u16);
        // C s_21_2: const #424u : u32
        let s_21_2: u32 = 424;
        // D s_21_3: read-reg s_21_2:u8
        let s_21_3: u8 = {
            let value = state.read_register::<u8>(s_21_2 as isize);
            tracer.read_register(s_21_2 as isize, value);
            value
        };
        // D s_21_4: cast zx s_21_3 -> bv
        let s_21_4: Bits = Bits::new(s_21_3 as u128, 2u16);
        // D s_21_5: cmp-eq s_21_1 s_21_4
        let s_21_5: bool = ((s_21_1) == (s_21_4));
        // N s_21_6: branch s_21_5 b32 b22
        if s_21_5 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_22_0: const #90704u : u32
        let s_22_0: u32 = 90704;
        // D s_22_1: read-reg s_22_0:struct
        let s_22_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call _get_SCR_EL3_Type_NSE(s_22_1)
        let s_22_2: bool = u_get_SCR_EL3_Type_NSE(state, tracer, s_22_1);
        // C s_22_3: const #() : ()
        let s_22_3: () = ();
        // S s_22_4: call EffectiveSCR_EL3_NS(s_22_3)
        let s_22_4: bool = EffectiveSCR_EL3_NS(state, tracer, s_22_3);
        // D s_22_5: cast zx s_22_2 -> bv
        let s_22_5: Bits = Bits::new(s_22_2 as u128, 1u16);
        // S s_22_6: cast zx s_22_4 -> bv
        let s_22_6: Bits = Bits::new(s_22_4 as u128, 1u16);
        // D s_22_7: cast reint s_22_5 -> u128
        let s_22_7: u128 = (s_22_5.value() as u128);
        // D s_22_8: size-of s_22_5
        let s_22_8: u16 = s_22_5.length();
        // S s_22_9: cast reint s_22_6 -> u128
        let s_22_9: u128 = (s_22_6.value() as u128);
        // D s_22_10: size-of s_22_6
        let s_22_10: u16 = s_22_6.length();
        // D s_22_11: lsl s_22_7 s_22_10
        let s_22_11: u128 = s_22_7 << s_22_10;
        // D s_22_12: or s_22_11 s_22_9
        let s_22_12: u128 = ((s_22_11) | (s_22_9));
        // D s_22_13: add s_22_8 s_22_10
        let s_22_13: u16 = (s_22_8 + s_22_10);
        // D s_22_14: create-bits s_22_12 s_22_13
        let s_22_14: Bits = Bits::new(s_22_12, s_22_13);
        // D s_22_15: cast reint s_22_14 -> u8
        let s_22_15: u8 = (s_22_14.value() as u8);
        // D s_22_16: write-var effective_nse_nsshadow#20 <= s_22_15
        fn_state.effective_nse_nsshadow_20 = s_22_15;
        // D s_22_17: read-var effective_nse_nsshadow#20:u8
        let s_22_17: u8 = fn_state.effective_nse_nsshadow_20;
        // D s_22_18: cast zx s_22_17 -> bv
        let s_22_18: Bits = Bits::new(s_22_17 as u128, 2u16);
        // C s_22_19: const #0u : u8
        let s_22_19: u8 = 0;
        // C s_22_20: cast zx s_22_19 -> bv
        let s_22_20: Bits = Bits::new(s_22_19 as u128, 2u16);
        // D s_22_21: cmp-eq s_22_18 s_22_20
        let s_22_21: bool = ((s_22_18) == (s_22_20));
        // D s_22_22: not s_22_21
        let s_22_22: bool = !s_22_21;
        // N s_22_23: branch s_22_22 b27 b23
        if s_22_22 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call HaveSecureEL2Ext(s_23_0)
        let s_23_1: bool = HaveSecureEL2Ext(state, tracer, s_23_0);
        // N s_23_2: branch s_23_1 b26 b24
        if s_23_1 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call Unreachable(s_24_0)
        let s_24_1: () = Unreachable(state, tracer, s_24_0);
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_25_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_26_0: const #3u : u32
        let s_26_0: u32 = 3;
        // D s_26_1: write-var return_value <= s_26_0
        fn_state.return_value = s_26_0;
        // N s_26_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_27_0: read-var effective_nse_nsshadow#20:u8
        let s_27_0: u8 = fn_state.effective_nse_nsshadow_20;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 2u16);
        // C s_27_2: const #1u : u8
        let s_27_2: u8 = 1;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 2u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: not s_27_4
        let s_27_5: bool = !s_27_4;
        // N s_27_6: branch s_27_5 b29 b28
        if s_27_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_28_0: const #0u : u32
        let s_28_0: u32 = 0;
        // D s_28_1: write-var return_value <= s_28_0
        fn_state.return_value = s_28_0;
        // N s_28_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_29_0: read-var effective_nse_nsshadow#20:u8
        let s_29_0: u8 = fn_state.effective_nse_nsshadow_20;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 2u16);
        // C s_29_2: const #3u : u8
        let s_29_2: u8 = 3;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 2u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: not s_29_4
        let s_29_5: bool = !s_29_4;
        // N s_29_6: branch s_29_5 b31 b30
        if s_29_5 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_30_0: const #2u : u32
        let s_30_0: u32 = 2;
        // D s_30_1: write-var return_value <= s_30_0
        fn_state.return_value = s_30_0;
        // N s_30_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call Unreachable(s_31_0)
        let s_31_1: () = Unreachable(state, tracer, s_31_0);
        // N s_31_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_32_0: const #1u : u32
        let s_32_0: u32 = 1;
        // D s_32_1: write-var return_value <= s_32_0
        fn_state.return_value = s_32_0;
        // N s_32_2: jump b10
        return block_10(state, tracer, fn_state);
    }
}
