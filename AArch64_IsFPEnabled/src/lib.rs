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
use u_get_CPTR_EL2_Type_FPEN::*;
use u_get_CPTR_EL2_Type_TFP::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_CPACR_EL1_Type_FPEN::*;
use IsInHost::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_CPTR_EL3_Type_TFP::*;
use HaveVirtHostExt::*;
use common::*;
pub fn AArch64_IsFPEnabled<T: Tracer>(state: &mut State, tracer: &T, el: u8) -> bool {
    #[derive(Default)]
    struct FunctionState {
        u_154: bool,
        ga_2444: u8,
        gs_3952: bool,
        gs_3955: bool,
        return_value: bool,
        disabled: bool,
        gs_3965: bool,
        ga_2435: u8,
        gs_3951: bool,
        gs_3953: bool,
        gs_3959: bool,
        gs_3954: bool,
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
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 2u16);
        // C s_0_2: const #448u : u32
        let s_0_2: u32 = 448;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 2u16);
        // D s_0_5: cmp-eq s_0_1 s_0_4
        let s_0_5: bool = ((s_0_1) == (s_0_4));
        // N s_0_6: branch s_0_5 b51 b1
        if s_0_5 {
            return block_51(state, tracer, fn_state);
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
        // C s_1_2: const #440u : u32
        let s_1_2: u32 = 440;
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
        // D s_1_6: write-var gs#3951 <= s_1_5
        fn_state.gs_3951 = s_1_5;
        // N s_1_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#3951:u8
        let s_2_0: bool = fn_state.gs_3951;
        // N s_2_1: branch s_2_0 b50 b3
        if s_2_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#3952 <= s_3_0
        fn_state.gs_3952 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#3952:u8
        let s_4_0: bool = fn_state.gs_3952;
        // N s_4_1: branch s_4_0 b42 b5
        if s_4_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var el:u8
        let s_6_0: u8 = fn_state.el;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #448u : u32
        let s_6_2: u32 = 448;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // D s_6_5: cmp-eq s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) == (s_6_4));
        // N s_6_6: branch s_6_5 b41 b7
        if s_6_5 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
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
        // C s_7_2: const #440u : u32
        let s_7_2: u32 = 440;
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
        // N s_7_6: branch s_7_5 b40 b8
        if s_7_5 {
            return block_40(state, tracer, fn_state);
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
        // C s_8_2: const #432u : u32
        let s_8_2: u32 = 432;
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
        // D s_8_6: write-var gs#3953 <= s_8_5
        fn_state.gs_3953 = s_8_5;
        // N s_8_7: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var gs#3953:u8
        let s_9_0: bool = fn_state.gs_3953;
        // D s_9_1: write-var gs#3954 <= s_9_0
        fn_state.gs_3954 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var gs#3954:u8
        let s_10_0: bool = fn_state.gs_3954;
        // N s_10_1: branch s_10_0 b39 b11
        if s_10_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#3955 <= s_11_0
        fn_state.gs_3955 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var gs#3955:u8
        let s_12_0: bool = fn_state.gs_3955;
        // N s_12_1: branch s_12_0 b21 b13
        if s_12_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #424u : u32
        let s_14_0: u32 = 424;
        // D s_14_1: read-reg s_14_0:u8
        let s_14_1: u8 = {
            let value = state.read_register::<u8>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // C s_14_2: const #2u : u8
        let s_14_2: u8 = 2;
        // D s_14_3: cmp-lt s_14_1 s_14_2
        let s_14_3: bool = ((s_14_1) < (s_14_2));
        // N s_14_4: branch s_14_3 b18 b15
        if s_14_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var return_value <= s_16_0
        fn_state.return_value = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var return_value:u8
        let s_17_0: bool = fn_state.return_value;
        // N s_17_1: return s_17_0
        return s_17_0;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #16840u : u32
        let s_18_0: u32 = 16840;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_CPTR_EL3_Type_TFP(s_18_1)
        let s_18_2: bool = u_get_CPTR_EL3_Type_TFP(state, tracer, s_18_1);
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // C s_18_4: const #1u : u8
        let s_18_4: bool = true;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 1u16);
        // D s_18_6: cmp-eq s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) == (s_18_5));
        // N s_18_7: branch s_18_6 b20 b19
        if s_18_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_19_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var return_value <= s_20_0
        fn_state.return_value = s_20_0;
        // N s_20_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call HaveVirtHostExt(s_21_0)
        let s_21_1: bool = HaveVirtHostExt(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b38 b22
        if s_21_1 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#3959 <= s_22_0
        fn_state.gs_3959 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var gs#3959:u8
        let s_23_0: bool = fn_state.gs_3959;
        // N s_23_1: branch s_23_0 b27 b24
        if s_23_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #11088u : u32
        let s_24_0: u32 = 11088;
        // D s_24_1: read-reg s_24_0:struct
        let s_24_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call _get_CPTR_EL2_Type_TFP(s_24_1)
        let s_24_2: bool = u_get_CPTR_EL2_Type_TFP(state, tracer, s_24_1);
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #1u : u8
        let s_24_4: bool = true;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // D s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // N s_24_7: branch s_24_6 b26 b25
        if s_24_6 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_25_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var return_value <= s_26_0
        fn_state.return_value = s_26_0;
        // N s_26_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #11088u : u32
        let s_27_0: u32 = 11088;
        // D s_27_1: read-reg s_27_0:struct
        let s_27_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call _get_CPTR_EL2_Type_FPEN(s_27_1)
        let s_27_2: u8 = u_get_CPTR_EL2_Type_FPEN(state, tracer, s_27_1);
        // D s_27_3: write-var ga#2444 <= s_27_2
        fn_state.ga_2444 = s_27_2;
        // D s_27_4: read-var ga#2444:u8
        let s_27_4: u8 = fn_state.ga_2444;
        // C s_27_5: const #0s : i
        let s_27_5: i128 = 0;
        // D s_27_6: cast zx s_27_4 -> bv
        let s_27_6: Bits = Bits::new(s_27_4 as u128, 2u16);
        // C s_27_7: const #1s : i64
        let s_27_7: i64 = 1;
        // C s_27_8: cast zx s_27_7 -> i
        let s_27_8: i128 = (i128::try_from(s_27_7).unwrap());
        // C s_27_9: const #0s : i
        let s_27_9: i128 = 0;
        // C s_27_10: add s_27_9 s_27_8
        let s_27_10: i128 = (s_27_9 + s_27_8);
        // D s_27_11: bit-extract s_27_6 s_27_5 s_27_10
        let s_27_11: Bits = (Bits::new(
            ((s_27_6) >> (s_27_5)).value(),
            u16::try_from(s_27_10).unwrap(),
        ));
        // D s_27_12: cast reint s_27_11 -> u8
        let s_27_12: bool = ((s_27_11.value()) != 0);
        // D s_27_13: cast zx s_27_12 -> bv
        let s_27_13: Bits = Bits::new(s_27_12 as u128, 1u16);
        // C s_27_14: const #0u : u8
        let s_27_14: bool = false;
        // C s_27_15: cast zx s_27_14 -> bv
        let s_27_15: Bits = Bits::new(s_27_14 as u128, 1u16);
        // D s_27_16: cmp-eq s_27_13 s_27_15
        let s_27_16: bool = ((s_27_13) == (s_27_15));
        // D s_27_17: not s_27_16
        let s_27_17: bool = !s_27_16;
        // N s_27_18: branch s_27_17 b32 b28
        if s_27_17 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var u#154 <= s_28_0
        fn_state.u_154 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_29_0: read-var u#154:u8
        let s_29_0: bool = fn_state.u_154;
        // N s_29_1: branch s_29_0 b31 b30
        if s_29_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_30_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var return_value <= s_31_0
        fn_state.return_value = s_31_0;
        // N s_31_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_32_0: read-var ga#2444:u8
        let s_32_0: u8 = fn_state.ga_2444;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 2u16);
        // C s_32_2: const #1u : u8
        let s_32_2: u8 = 1;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 2u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: not s_32_4
        let s_32_5: bool = !s_32_4;
        // N s_32_6: branch s_32_5 b37 b33
        if s_32_5 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_33_0: read-var el:u8
        let s_33_0: u8 = fn_state.el;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 2u16);
        // C s_33_2: const #448u : u32
        let s_33_2: u32 = 448;
        // D s_33_3: read-reg s_33_2:u8
        let s_33_3: u8 = {
            let value = state.read_register::<u8>(s_33_2 as isize);
            tracer.read_register(s_33_2 as isize, value);
            value
        };
        // D s_33_4: cast zx s_33_3 -> bv
        let s_33_4: Bits = Bits::new(s_33_3 as u128, 2u16);
        // D s_33_5: cmp-eq s_33_1 s_33_4
        let s_33_5: bool = ((s_33_1) == (s_33_4));
        // N s_33_6: branch s_33_5 b36 b34
        if s_33_5 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#3965 <= s_34_0
        fn_state.gs_3965 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_35_0: read-var gs#3965:u8
        let s_35_0: bool = fn_state.gs_3965;
        // D s_35_1: write-var u#154 <= s_35_0
        fn_state.u_154 = s_35_0;
        // N s_35_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_36_0: const #102552u : u32
        let s_36_0: u32 = 102552;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_HCR_EL2_Type_TGE(s_36_1)
        let s_36_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_36_1);
        // D s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // C s_36_4: const #1u : u8
        let s_36_4: bool = true;
        // C s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 1u16);
        // D s_36_6: cmp-eq s_36_3 s_36_5
        let s_36_6: bool = ((s_36_3) == (s_36_5));
        // D s_36_7: write-var gs#3965 <= s_36_6
        fn_state.gs_3965 = s_36_6;
        // N s_36_8: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var u#154 <= s_37_0
        fn_state.u_154 = s_37_0;
        // N s_37_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_38_0: const #102552u : u32
        let s_38_0: u32 = 102552;
        // D s_38_1: read-reg s_38_0:struct
        let s_38_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call _get_HCR_EL2_Type_E2H(s_38_1)
        let s_38_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_38_1);
        // D s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // C s_38_4: const #1u : u8
        let s_38_4: bool = true;
        // C s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 1u16);
        // D s_38_6: cmp-eq s_38_3 s_38_5
        let s_38_6: bool = ((s_38_3) == (s_38_5));
        // D s_38_7: write-var gs#3959 <= s_38_6
        fn_state.gs_3959 = s_38_6;
        // N s_38_8: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call EL2Enabled(s_39_0)
        let s_39_1: bool = EL2Enabled(state, tracer, s_39_0);
        // D s_39_2: write-var gs#3955 <= s_39_1
        fn_state.gs_3955 = s_39_1;
        // N s_39_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#3953 <= s_40_0
        fn_state.gs_3953 = s_40_0;
        // N s_40_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#3954 <= s_41_0
        fn_state.gs_3954 = s_41_0;
        // N s_41_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_42_0: const #12088u : u32
        let s_42_0: u32 = 12088;
        // D s_42_1: read-reg s_42_0:struct
        let s_42_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call _get_CPACR_EL1_Type_FPEN(s_42_1)
        let s_42_2: u8 = u_get_CPACR_EL1_Type_FPEN(state, tracer, s_42_1);
        // D s_42_3: write-var ga#2435 <= s_42_2
        fn_state.ga_2435 = s_42_2;
        // D s_42_4: read-var ga#2435:u8
        let s_42_4: u8 = fn_state.ga_2435;
        // C s_42_5: const #0s : i
        let s_42_5: i128 = 0;
        // D s_42_6: cast zx s_42_4 -> bv
        let s_42_6: Bits = Bits::new(s_42_4 as u128, 2u16);
        // C s_42_7: const #1s : i64
        let s_42_7: i64 = 1;
        // C s_42_8: cast zx s_42_7 -> i
        let s_42_8: i128 = (i128::try_from(s_42_7).unwrap());
        // C s_42_9: const #0s : i
        let s_42_9: i128 = 0;
        // C s_42_10: add s_42_9 s_42_8
        let s_42_10: i128 = (s_42_9 + s_42_8);
        // D s_42_11: bit-extract s_42_6 s_42_5 s_42_10
        let s_42_11: Bits = (Bits::new(
            ((s_42_6) >> (s_42_5)).value(),
            u16::try_from(s_42_10).unwrap(),
        ));
        // D s_42_12: cast reint s_42_11 -> u8
        let s_42_12: bool = ((s_42_11.value()) != 0);
        // D s_42_13: cast zx s_42_12 -> bv
        let s_42_13: Bits = Bits::new(s_42_12 as u128, 1u16);
        // C s_42_14: const #0u : u8
        let s_42_14: bool = false;
        // C s_42_15: cast zx s_42_14 -> bv
        let s_42_15: Bits = Bits::new(s_42_14 as u128, 1u16);
        // D s_42_16: cmp-eq s_42_13 s_42_15
        let s_42_16: bool = ((s_42_13) == (s_42_15));
        // D s_42_17: not s_42_16
        let s_42_17: bool = !s_42_16;
        // N s_42_18: branch s_42_17 b47 b43
        if s_42_17 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var disabled <= s_43_0
        fn_state.disabled = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_44_0: read-var disabled:u8
        let s_44_0: bool = fn_state.disabled;
        // N s_44_1: branch s_44_0 b46 b45
        if s_44_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_45_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var return_value <= s_46_0
        fn_state.return_value = s_46_0;
        // N s_46_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_47_0: read-var ga#2435:u8
        let s_47_0: u8 = fn_state.ga_2435;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 2u16);
        // C s_47_2: const #1u : u8
        let s_47_2: u8 = 1;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 2u16);
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // D s_47_5: not s_47_4
        let s_47_5: bool = !s_47_4;
        // N s_47_6: branch s_47_5 b49 b48
        if s_47_5 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_48_0: read-var el:u8
        let s_48_0: u8 = fn_state.el;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 2u16);
        // C s_48_2: const #448u : u32
        let s_48_2: u32 = 448;
        // D s_48_3: read-reg s_48_2:u8
        let s_48_3: u8 = {
            let value = state.read_register::<u8>(s_48_2 as isize);
            tracer.read_register(s_48_2 as isize, value);
            value
        };
        // D s_48_4: cast zx s_48_3 -> bv
        let s_48_4: Bits = Bits::new(s_48_3 as u128, 2u16);
        // D s_48_5: cmp-eq s_48_1 s_48_4
        let s_48_5: bool = ((s_48_1) == (s_48_4));
        // D s_48_6: write-var disabled <= s_48_5
        fn_state.disabled = s_48_5;
        // N s_48_7: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var disabled <= s_49_0
        fn_state.disabled = s_49_0;
        // N s_49_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call IsInHost(s_50_0)
        let s_50_1: bool = IsInHost(state, tracer, s_50_0);
        // S s_50_2: not s_50_1
        let s_50_2: bool = !s_50_1;
        // D s_50_3: write-var gs#3952 <= s_50_2
        fn_state.gs_3952 = s_50_2;
        // N s_50_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_51_0: const #1u : u8
        let s_51_0: bool = true;
        // D s_51_1: write-var gs#3951 <= s_51_0
        fn_state.gs_3951 = s_51_0;
        // N s_51_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
