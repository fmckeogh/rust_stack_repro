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
use u_get_CPACR_EL1_Type_ZEN::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_CPTR_EL3_Type_EZ::*;
use IsInHost::*;
use ELUsingAArch32::*;
use u_get_CPTR_EL2_Type_ZEN::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_CPTR_EL2_Type_TZ::*;
use HaveVirtHostExt::*;
use EL2Enabled::*;
use common::*;
pub fn IsOriginalSVEEnabled<T: Tracer>(state: &mut State, tracer: &T, el: u8) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ga_2527: u8,
        gs_4065: bool,
        gs_4067: bool,
        return_value: bool,
        gs_4068: bool,
        ga_2536: u8,
        disabled: bool,
        gs_4073: bool,
        gs_4066: bool,
        gs_4069: bool,
        gs_4079: bool,
        el: u8,
    }
    let fn_state = FunctionState {
        el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var el:u8
        let s_0_0: u8 = fn_state.el;
        // D s_0_1: call ELUsingAArch32(s_0_0)
        let s_0_1: bool = ELUsingAArch32(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b53 b1
        if s_0_1 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var el:u8
        let s_1_0: u8 = fn_state.el;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #448u : u32
        let s_1_2: u32 = 448;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: u8 = {
            let value = state.read_register::<u8>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 2u16);
        // D s_1_5: cmp-eq s_1_1 s_1_4
        let s_1_5: bool = ((s_1_1) == (s_1_4));
        // N s_1_6: branch s_1_5 b52 b2
        if s_1_5 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var el:u8
        let s_2_0: u8 = fn_state.el;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #440u : u32
        let s_2_2: u32 = 440;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // D s_2_5: cmp-eq s_2_1 s_2_4
        let s_2_5: bool = ((s_2_1) == (s_2_4));
        // D s_2_6: write-var gs#4065 <= s_2_5
        fn_state.gs_4065 = s_2_5;
        // N s_2_7: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#4065:u8
        let s_3_0: bool = fn_state.gs_4065;
        // N s_3_1: branch s_3_0 b51 b4
        if s_3_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#4066 <= s_4_0
        fn_state.gs_4066 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#4066:u8
        let s_5_0: bool = fn_state.gs_4066;
        // N s_5_1: branch s_5_0 b43 b6
        if s_5_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var el:u8
        let s_7_0: u8 = fn_state.el;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #448u : u32
        let s_7_2: u32 = 448;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // D s_7_5: cmp-eq s_7_1 s_7_4
        let s_7_5: bool = ((s_7_1) == (s_7_4));
        // N s_7_6: branch s_7_5 b42 b8
        if s_7_5 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var el:u8
        let s_8_0: u8 = fn_state.el;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #440u : u32
        let s_8_2: u32 = 440;
        // D s_8_3: read-reg s_8_2:u8
        let s_8_3: u8 = {
            let value = state.read_register::<u8>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 2u16);
        // D s_8_5: cmp-eq s_8_1 s_8_4
        let s_8_5: bool = ((s_8_1) == (s_8_4));
        // N s_8_6: branch s_8_5 b41 b9
        if s_8_5 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var el:u8
        let s_9_0: u8 = fn_state.el;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 2u16);
        // C s_9_2: const #432u : u32
        let s_9_2: u32 = 432;
        // D s_9_3: read-reg s_9_2:u8
        let s_9_3: u8 = {
            let value = state.read_register::<u8>(s_9_2 as isize);
            tracer.read_register(s_9_2 as isize, value);
            value
        };
        // D s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 2u16);
        // D s_9_5: cmp-eq s_9_1 s_9_4
        let s_9_5: bool = ((s_9_1) == (s_9_4));
        // D s_9_6: write-var gs#4067 <= s_9_5
        fn_state.gs_4067 = s_9_5;
        // N s_9_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var gs#4067:u8
        let s_10_0: bool = fn_state.gs_4067;
        // D s_10_1: write-var gs#4068 <= s_10_0
        fn_state.gs_4068 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var gs#4068:u8
        let s_11_0: bool = fn_state.gs_4068;
        // N s_11_1: branch s_11_0 b40 b12
        if s_11_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#4069 <= s_12_0
        fn_state.gs_4069 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var gs#4069:u8
        let s_13_0: bool = fn_state.gs_4069;
        // N s_13_1: branch s_13_0 b22 b14
        if s_13_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #424u : u32
        let s_15_0: u32 = 424;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // C s_15_2: const #2u : u8
        let s_15_2: u8 = 2;
        // D s_15_3: cmp-lt s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) < (s_15_2));
        // N s_15_4: branch s_15_3 b19 b16
        if s_15_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var return_value:u8
        let s_18_0: bool = fn_state.return_value;
        // N s_18_1: return s_18_0
        return s_18_0;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #16840u : u32
        let s_19_0: u32 = 16840;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_CPTR_EL3_Type_EZ(s_19_1)
        let s_19_2: bool = u_get_CPTR_EL3_Type_EZ(state, tracer, s_19_1);
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // C s_19_4: const #0u : u8
        let s_19_4: bool = false;
        // C s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 1u16);
        // D s_19_6: cmp-eq s_19_3 s_19_5
        let s_19_6: bool = ((s_19_3) == (s_19_5));
        // N s_19_7: branch s_19_6 b21 b20
        if s_19_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_20_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var return_value <= s_21_0
        fn_state.return_value = s_21_0;
        // N s_21_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call HaveVirtHostExt(s_22_0)
        let s_22_1: bool = HaveVirtHostExt(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b39 b23
        if s_22_1 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#4073 <= s_23_0
        fn_state.gs_4073 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var gs#4073:u8
        let s_24_0: bool = fn_state.gs_4073;
        // N s_24_1: branch s_24_0 b28 b25
        if s_24_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #11088u : u32
        let s_25_0: u32 = 11088;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_CPTR_EL2_Type_TZ(s_25_1)
        let s_25_2: bool = u_get_CPTR_EL2_Type_TZ(state, tracer, s_25_1);
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // C s_25_4: const #1u : u8
        let s_25_4: bool = true;
        // C s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 1u16);
        // D s_25_6: cmp-eq s_25_3 s_25_5
        let s_25_6: bool = ((s_25_3) == (s_25_5));
        // N s_25_7: branch s_25_6 b27 b26
        if s_25_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_26_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var return_value <= s_27_0
        fn_state.return_value = s_27_0;
        // N s_27_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #11088u : u32
        let s_28_0: u32 = 11088;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_CPTR_EL2_Type_ZEN(s_28_1)
        let s_28_2: u8 = u_get_CPTR_EL2_Type_ZEN(state, tracer, s_28_1);
        // D s_28_3: write-var ga#2536 <= s_28_2
        fn_state.ga_2536 = s_28_2;
        // D s_28_4: read-var ga#2536:u8
        let s_28_4: u8 = fn_state.ga_2536;
        // C s_28_5: const #0s : i
        let s_28_5: i128 = 0;
        // D s_28_6: cast zx s_28_4 -> bv
        let s_28_6: Bits = Bits::new(s_28_4 as u128, 2u16);
        // C s_28_7: const #1s : i64
        let s_28_7: i64 = 1;
        // C s_28_8: cast zx s_28_7 -> i
        let s_28_8: i128 = (i128::try_from(s_28_7).unwrap());
        // C s_28_9: const #0s : i
        let s_28_9: i128 = 0;
        // C s_28_10: add s_28_9 s_28_8
        let s_28_10: i128 = (s_28_9 + s_28_8);
        // D s_28_11: bit-extract s_28_6 s_28_5 s_28_10
        let s_28_11: Bits = (Bits::new(
            ((s_28_6) >> (s_28_5)).value(),
            u16::try_from(s_28_10).unwrap(),
        ));
        // D s_28_12: cast reint s_28_11 -> u8
        let s_28_12: bool = ((s_28_11.value()) != 0);
        // D s_28_13: cast zx s_28_12 -> bv
        let s_28_13: Bits = Bits::new(s_28_12 as u128, 1u16);
        // C s_28_14: const #0u : u8
        let s_28_14: bool = false;
        // C s_28_15: cast zx s_28_14 -> bv
        let s_28_15: Bits = Bits::new(s_28_14 as u128, 1u16);
        // D s_28_16: cmp-eq s_28_13 s_28_15
        let s_28_16: bool = ((s_28_13) == (s_28_15));
        // D s_28_17: not s_28_16
        let s_28_17: bool = !s_28_16;
        // N s_28_18: branch s_28_17 b33 b29
        if s_28_17 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var disabled <= s_29_0
        fn_state.disabled = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var disabled:u8
        let s_30_0: bool = fn_state.disabled;
        // N s_30_1: branch s_30_0 b32 b31
        if s_30_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_31_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var return_value <= s_32_0
        fn_state.return_value = s_32_0;
        // N s_32_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_33_0: read-var ga#2536:u8
        let s_33_0: u8 = fn_state.ga_2536;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 2u16);
        // C s_33_2: const #1u : u8
        let s_33_2: u8 = 1;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 2u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: not s_33_4
        let s_33_5: bool = !s_33_4;
        // N s_33_6: branch s_33_5 b38 b34
        if s_33_5 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_34_0: read-var el:u8
        let s_34_0: u8 = fn_state.el;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 2u16);
        // C s_34_2: const #448u : u32
        let s_34_2: u32 = 448;
        // D s_34_3: read-reg s_34_2:u8
        let s_34_3: u8 = {
            let value = state.read_register::<u8>(s_34_2 as isize);
            tracer.read_register(s_34_2 as isize, value);
            value
        };
        // D s_34_4: cast zx s_34_3 -> bv
        let s_34_4: Bits = Bits::new(s_34_3 as u128, 2u16);
        // D s_34_5: cmp-eq s_34_1 s_34_4
        let s_34_5: bool = ((s_34_1) == (s_34_4));
        // N s_34_6: branch s_34_5 b37 b35
        if s_34_5 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#4079 <= s_35_0
        fn_state.gs_4079 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_36_0: read-var gs#4079:u8
        let s_36_0: bool = fn_state.gs_4079;
        // D s_36_1: write-var disabled <= s_36_0
        fn_state.disabled = s_36_0;
        // N s_36_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_37_0: const #102552u : u32
        let s_37_0: u32 = 102552;
        // D s_37_1: read-reg s_37_0:struct
        let s_37_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call _get_HCR_EL2_Type_TGE(s_37_1)
        let s_37_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_37_1);
        // D s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // C s_37_4: const #1u : u8
        let s_37_4: bool = true;
        // C s_37_5: cast zx s_37_4 -> bv
        let s_37_5: Bits = Bits::new(s_37_4 as u128, 1u16);
        // D s_37_6: cmp-eq s_37_3 s_37_5
        let s_37_6: bool = ((s_37_3) == (s_37_5));
        // D s_37_7: write-var gs#4079 <= s_37_6
        fn_state.gs_4079 = s_37_6;
        // N s_37_8: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var disabled <= s_38_0
        fn_state.disabled = s_38_0;
        // N s_38_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_39_0: const #102552u : u32
        let s_39_0: u32 = 102552;
        // D s_39_1: read-reg s_39_0:struct
        let s_39_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call _get_HCR_EL2_Type_E2H(s_39_1)
        let s_39_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_39_1);
        // D s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // C s_39_4: const #1u : u8
        let s_39_4: bool = true;
        // C s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // D s_39_6: cmp-eq s_39_3 s_39_5
        let s_39_6: bool = ((s_39_3) == (s_39_5));
        // D s_39_7: write-var gs#4073 <= s_39_6
        fn_state.gs_4073 = s_39_6;
        // N s_39_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call EL2Enabled(s_40_0)
        let s_40_1: bool = EL2Enabled(state, tracer, s_40_0);
        // D s_40_2: write-var gs#4069 <= s_40_1
        fn_state.gs_4069 = s_40_1;
        // N s_40_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#4067 <= s_41_0
        fn_state.gs_4067 = s_41_0;
        // N s_41_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#4068 <= s_42_0
        fn_state.gs_4068 = s_42_0;
        // N s_42_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_43_0: const #12088u : u32
        let s_43_0: u32 = 12088;
        // D s_43_1: read-reg s_43_0:struct
        let s_43_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: call _get_CPACR_EL1_Type_ZEN(s_43_1)
        let s_43_2: u8 = u_get_CPACR_EL1_Type_ZEN(state, tracer, s_43_1);
        // D s_43_3: write-var ga#2527 <= s_43_2
        fn_state.ga_2527 = s_43_2;
        // D s_43_4: read-var ga#2527:u8
        let s_43_4: u8 = fn_state.ga_2527;
        // C s_43_5: const #0s : i
        let s_43_5: i128 = 0;
        // D s_43_6: cast zx s_43_4 -> bv
        let s_43_6: Bits = Bits::new(s_43_4 as u128, 2u16);
        // C s_43_7: const #1s : i64
        let s_43_7: i64 = 1;
        // C s_43_8: cast zx s_43_7 -> i
        let s_43_8: i128 = (i128::try_from(s_43_7).unwrap());
        // C s_43_9: const #0s : i
        let s_43_9: i128 = 0;
        // C s_43_10: add s_43_9 s_43_8
        let s_43_10: i128 = (s_43_9 + s_43_8);
        // D s_43_11: bit-extract s_43_6 s_43_5 s_43_10
        let s_43_11: Bits = (Bits::new(
            ((s_43_6) >> (s_43_5)).value(),
            u16::try_from(s_43_10).unwrap(),
        ));
        // D s_43_12: cast reint s_43_11 -> u8
        let s_43_12: bool = ((s_43_11.value()) != 0);
        // D s_43_13: cast zx s_43_12 -> bv
        let s_43_13: Bits = Bits::new(s_43_12 as u128, 1u16);
        // C s_43_14: const #0u : u8
        let s_43_14: bool = false;
        // C s_43_15: cast zx s_43_14 -> bv
        let s_43_15: Bits = Bits::new(s_43_14 as u128, 1u16);
        // D s_43_16: cmp-eq s_43_13 s_43_15
        let s_43_16: bool = ((s_43_13) == (s_43_15));
        // D s_43_17: not s_43_16
        let s_43_17: bool = !s_43_16;
        // N s_43_18: branch s_43_17 b48 b44
        if s_43_17 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var disabled <= s_44_0
        fn_state.disabled = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_45_0: read-var disabled:u8
        let s_45_0: bool = fn_state.disabled;
        // N s_45_1: branch s_45_0 b47 b46
        if s_45_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_46_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var return_value <= s_47_0
        fn_state.return_value = s_47_0;
        // N s_47_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_48_0: read-var ga#2527:u8
        let s_48_0: u8 = fn_state.ga_2527;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 2u16);
        // C s_48_2: const #1u : u8
        let s_48_2: u8 = 1;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 2u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: not s_48_4
        let s_48_5: bool = !s_48_4;
        // N s_48_6: branch s_48_5 b50 b49
        if s_48_5 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_49_0: read-var el:u8
        let s_49_0: u8 = fn_state.el;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 2u16);
        // C s_49_2: const #448u : u32
        let s_49_2: u32 = 448;
        // D s_49_3: read-reg s_49_2:u8
        let s_49_3: u8 = {
            let value = state.read_register::<u8>(s_49_2 as isize);
            tracer.read_register(s_49_2 as isize, value);
            value
        };
        // D s_49_4: cast zx s_49_3 -> bv
        let s_49_4: Bits = Bits::new(s_49_3 as u128, 2u16);
        // D s_49_5: cmp-eq s_49_1 s_49_4
        let s_49_5: bool = ((s_49_1) == (s_49_4));
        // D s_49_6: write-var disabled <= s_49_5
        fn_state.disabled = s_49_5;
        // N s_49_7: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var disabled <= s_50_0
        fn_state.disabled = s_50_0;
        // N s_50_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call IsInHost(s_51_0)
        let s_51_1: bool = IsInHost(state, tracer, s_51_0);
        // S s_51_2: not s_51_1
        let s_51_2: bool = !s_51_1;
        // D s_51_3: write-var gs#4066 <= s_51_2
        fn_state.gs_4066 = s_51_2;
        // N s_51_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // D s_52_1: write-var gs#4065 <= s_52_0
        fn_state.gs_4065 = s_52_0;
        // N s_52_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var return_value <= s_53_0
        fn_state.return_value = s_53_0;
        // N s_53_2: jump b18
        return block_18(state, tracer, fn_state);
    }
}
