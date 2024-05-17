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
use HaveVirtHostExt::*;
use u_get_CPTR_EL2_Type_TFP::*;
use u_get_HCR_EL2_Type_E2H::*;
use EL2Enabled::*;
use AArch64_AdvSIMDFPAccessTrap::*;
use EL3SDDUndefPriority::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_CPTR_EL2_Type_FPEN::*;
use EL3SDDUndef::*;
use u_get_CPTR_EL3_Type_TFP::*;
use common::*;
pub fn AArch64_CheckFPAdvSIMDTrap<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_25055: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_25056: bool,
        disabled: bool,
        ga_19567: u8,
        gs_25057: bool,
        gs_25060: bool,
        gs_25070: bool,
        gs_25058: bool,
        gs_25059: bool,
        gs_25064: bool,
        gs_25055: (),
    }
    let fn_state = FunctionState {
        gs_25055,
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
        // N s_0_4: branch s_0_3 b45 b1
        if s_0_3 {
            return block_45(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#25056 <= s_1_0
        fn_state.gs_25056 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#25056:u8
        let s_2_0: bool = fn_state.gs_25056;
        // N s_2_1: branch s_2_0 b44 b3
        if s_2_0 {
            return block_44(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#25057 <= s_3_0
        fn_state.gs_25057 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#25057:u8
        let s_4_0: bool = fn_state.gs_25057;
        // N s_4_1: branch s_4_0 b43 b5
        if s_4_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16975u : u32
        let s_5_0: u32 = 16975;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 2u16);
        // C s_5_3: const #448u : u32
        let s_5_3: u32 = 448;
        // D s_5_4: read-reg s_5_3:u8
        let s_5_4: u8 = {
            let value = state.read_register::<u8>(s_5_3 as isize);
            tracer.read_register(s_5_3 as isize, value);
            value
        };
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 2u16);
        // D s_5_6: cmp-eq s_5_2 s_5_5
        let s_5_6: bool = ((s_5_2) == (s_5_5));
        // N s_5_7: branch s_5_6 b42 b6
        if s_5_6 {
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
        // C s_6_0: const #16975u : u32
        let s_6_0: u32 = 16975;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 2u16);
        // C s_6_3: const #440u : u32
        let s_6_3: u32 = 440;
        // D s_6_4: read-reg s_6_3:u8
        let s_6_4: u8 = {
            let value = state.read_register::<u8>(s_6_3 as isize);
            tracer.read_register(s_6_3 as isize, value);
            value
        };
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 2u16);
        // D s_6_6: cmp-eq s_6_2 s_6_5
        let s_6_6: bool = ((s_6_2) == (s_6_5));
        // N s_6_7: branch s_6_6 b41 b7
        if s_6_6 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16975u : u32
        let s_7_0: u32 = 16975;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 2u16);
        // C s_7_3: const #432u : u32
        let s_7_3: u32 = 432;
        // D s_7_4: read-reg s_7_3:u8
        let s_7_4: u8 = {
            let value = state.read_register::<u8>(s_7_3 as isize);
            tracer.read_register(s_7_3 as isize, value);
            value
        };
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 2u16);
        // D s_7_6: cmp-eq s_7_2 s_7_5
        let s_7_6: bool = ((s_7_2) == (s_7_5));
        // D s_7_7: write-var gs#25058 <= s_7_6
        fn_state.gs_25058 = s_7_6;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#25058:u8
        let s_8_0: bool = fn_state.gs_25058;
        // D s_8_1: write-var gs#25059 <= s_8_0
        fn_state.gs_25059 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#25059:u8
        let s_9_0: bool = fn_state.gs_25059;
        // N s_9_1: branch s_9_0 b40 b10
        if s_9_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#25060 <= s_10_0
        fn_state.gs_25060 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#25060:u8
        let s_11_0: bool = fn_state.gs_25060;
        // N s_11_1: branch s_11_0 b20 b12
        if s_11_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #424u : u32
        let s_13_0: u32 = 424;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // D s_13_3: cmp-lt s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) < (s_13_2));
        // N s_13_4: branch s_13_3 b15 b14
        if s_13_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #16840u : u32
        let s_15_0: u32 = 16840;
        // D s_15_1: read-reg s_15_0:struct
        let s_15_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: call _get_CPTR_EL3_Type_TFP(s_15_1)
        let s_15_2: bool = u_get_CPTR_EL3_Type_TFP(state, tracer, s_15_1);
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // C s_15_4: const #1u : u8
        let s_15_4: bool = true;
        // C s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 1u16);
        // D s_15_6: cmp-eq s_15_3 s_15_5
        let s_15_6: bool = ((s_15_3) == (s_15_5));
        // N s_15_7: branch s_15_6 b17 b16
        if s_15_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call EL3SDDUndef(s_17_0)
        let s_17_1: bool = EL3SDDUndef(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b19 b18
        if s_17_1 {
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
        // C s_18_0: const #424u : u32
        let s_18_0: u32 = 424;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call AArch64_AdvSIMDFPAccessTrap(s_18_1)
        let s_18_2: () = AArch64_AdvSIMDFPAccessTrap(state, tracer, s_18_1);
        // N s_18_3: return
        return;
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
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call HaveVirtHostExt(s_20_0)
        let s_20_1: bool = HaveVirtHostExt(state, tracer, s_20_0);
        // N s_20_2: branch s_20_1 b39 b21
        if s_20_1 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#25064 <= s_21_0
        fn_state.gs_25064 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#25064:u8
        let s_22_0: bool = fn_state.gs_25064;
        // N s_22_1: branch s_22_0 b27 b23
        if s_22_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #11088u : u32
        let s_23_0: u32 = 11088;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_CPTR_EL2_Type_TFP(s_23_1)
        let s_23_2: bool = u_get_CPTR_EL2_Type_TFP(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #1u : u8
        let s_23_4: bool = true;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // N s_23_7: branch s_23_6 b26 b24
        if s_23_6 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #432u : u32
        let s_26_0: u32 = 432;
        // D s_26_1: read-reg s_26_0:u8
        let s_26_1: u8 = {
            let value = state.read_register::<u8>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call AArch64_AdvSIMDFPAccessTrap(s_26_1)
        let s_26_2: () = AArch64_AdvSIMDFPAccessTrap(state, tracer, s_26_1);
        // N s_26_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
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
        // D s_27_3: write-var ga#19567 <= s_27_2
        fn_state.ga_19567 = s_27_2;
        // D s_27_4: read-var ga#19567:u8
        let s_27_4: u8 = fn_state.ga_19567;
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
        // N s_27_18: branch s_27_17 b33 b28
        if s_27_17 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var disabled <= s_28_0
        fn_state.disabled = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var disabled:u8
        let s_29_0: bool = fn_state.disabled;
        // N s_29_1: branch s_29_0 b32 b30
        if s_29_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #432u : u32
        let s_32_0: u32 = 432;
        // D s_32_1: read-reg s_32_0:u8
        let s_32_1: u8 = {
            let value = state.read_register::<u8>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: call AArch64_AdvSIMDFPAccessTrap(s_32_1)
        let s_32_2: () = AArch64_AdvSIMDFPAccessTrap(state, tracer, s_32_1);
        // N s_32_3: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var ga#19567:u8
        let s_33_0: u8 = fn_state.ga_19567;
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
    ) -> () {
        // C s_34_0: const #16975u : u32
        let s_34_0: u32 = 16975;
        // D s_34_1: read-reg s_34_0:u8
        let s_34_1: u8 = {
            let value = state.read_register::<u8>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: cast zx s_34_1 -> bv
        let s_34_2: Bits = Bits::new(s_34_1 as u128, 2u16);
        // C s_34_3: const #448u : u32
        let s_34_3: u32 = 448;
        // D s_34_4: read-reg s_34_3:u8
        let s_34_4: u8 = {
            let value = state.read_register::<u8>(s_34_3 as isize);
            tracer.read_register(s_34_3 as isize, value);
            value
        };
        // D s_34_5: cast zx s_34_4 -> bv
        let s_34_5: Bits = Bits::new(s_34_4 as u128, 2u16);
        // D s_34_6: cmp-eq s_34_2 s_34_5
        let s_34_6: bool = ((s_34_2) == (s_34_5));
        // N s_34_7: branch s_34_6 b37 b35
        if s_34_6 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#25070 <= s_35_0
        fn_state.gs_25070 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#25070:u8
        let s_36_0: bool = fn_state.gs_25070;
        // D s_36_1: write-var disabled <= s_36_0
        fn_state.disabled = s_36_0;
        // N s_36_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
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
        // D s_37_7: write-var gs#25070 <= s_37_6
        fn_state.gs_25070 = s_37_6;
        // N s_37_8: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var disabled <= s_38_0
        fn_state.disabled = s_38_0;
        // N s_38_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
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
        // D s_39_7: write-var gs#25064 <= s_39_6
        fn_state.gs_25064 = s_39_6;
        // N s_39_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call EL2Enabled(s_40_0)
        let s_40_1: bool = EL2Enabled(state, tracer, s_40_0);
        // D s_40_2: write-var gs#25060 <= s_40_1
        fn_state.gs_25060 = s_40_1;
        // N s_40_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#25058 <= s_41_0
        fn_state.gs_25058 = s_41_0;
        // N s_41_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#25059 <= s_42_0
        fn_state.gs_25059 = s_42_0;
        // N s_42_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: panic
        panic!("{:?}", ());
        // N s_43_1: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call EL3SDDUndefPriority(s_44_0)
        let s_44_1: bool = EL3SDDUndefPriority(state, tracer, s_44_0);
        // D s_44_2: write-var gs#25057 <= s_44_1
        fn_state.gs_25057 = s_44_1;
        // N s_44_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #16840u : u32
        let s_45_0: u32 = 16840;
        // D s_45_1: read-reg s_45_0:struct
        let s_45_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call _get_CPTR_EL3_Type_TFP(s_45_1)
        let s_45_2: bool = u_get_CPTR_EL3_Type_TFP(state, tracer, s_45_1);
        // D s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // C s_45_4: const #1u : u8
        let s_45_4: bool = true;
        // C s_45_5: cast zx s_45_4 -> bv
        let s_45_5: Bits = Bits::new(s_45_4 as u128, 1u16);
        // D s_45_6: cmp-eq s_45_3 s_45_5
        let s_45_6: bool = ((s_45_3) == (s_45_5));
        // D s_45_7: write-var gs#25056 <= s_45_6
        fn_state.gs_25056 = s_45_6;
        // N s_45_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
