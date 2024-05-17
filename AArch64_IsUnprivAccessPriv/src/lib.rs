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
use u_get_HCR_EL2_Type_E2H::*;
use HaveNVExt::*;
use HaveUAOExt::*;
use u_get_HCR_EL2_Type_NV::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use HaveVirtHostExt::*;
use u_get_HCR_EL2_Type_NV1::*;
use common::*;
pub fn AArch64_IsUnprivAccessPriv<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_20946: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_20949: bool,
        privileged: bool,
        gs_20952: bool,
        gs_20950: bool,
        gs_20956: bool,
        ga_16352: u8,
        gs_20946: (),
    }
    let fn_state = FunctionState {
        gs_20946,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: write-var ga#16352 <= s_0_1
        fn_state.ga_16352 = s_0_1;
        // D s_0_3: read-var ga#16352:u8
        let s_0_3: u8 = fn_state.ga_16352;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 2u16);
        // C s_0_5: const #448u : u32
        let s_0_5: u32 = 448;
        // D s_0_6: read-reg s_0_5:u8
        let s_0_6: u8 = {
            let value = state.read_register::<u8>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 2u16);
        // D s_0_8: cmp-eq s_0_4 s_0_7
        let s_0_8: bool = ((s_0_4) == (s_0_7));
        // D s_0_9: not s_0_8
        let s_0_9: bool = !s_0_8;
        // N s_0_10: branch s_0_9 b9 b1
        if s_0_9 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var privileged <= s_1_0
        fn_state.privileged = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call HaveUAOExt(s_2_0)
        let s_2_1: bool = HaveUAOExt(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b8 b3
        if s_2_1 {
            return block_8(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#20956 <= s_3_0
        fn_state.gs_20956 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#20956:u8
        let s_4_0: bool = fn_state.gs_20956;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
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
        // D s_6_0: read-var privileged:u8
        let s_6_0: bool = fn_state.privileged;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
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
        // C s_7_3: const #448u : u32
        let s_7_3: u32 = 448;
        // D s_7_4: read-reg s_7_3:u8
        let s_7_4: u8 = {
            let value = state.read_register::<u8>(s_7_3 as isize);
            tracer.read_register(s_7_3 as isize, value);
            value
        };
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 2u16);
        // D s_7_6: cmp-ne s_7_2 s_7_5
        let s_7_6: bool = ((s_7_2) != (s_7_5));
        // D s_7_7: write-var privileged <= s_7_6
        fn_state.privileged = s_7_6;
        // N s_7_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #16995u : u32
        let s_8_0: u32 = 16995;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: bool = {
            let value = state.read_register::<bool>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 1u16);
        // C s_8_3: const #1u : u8
        let s_8_3: bool = true;
        // C s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 1u16);
        // D s_8_5: cmp-eq s_8_2 s_8_4
        let s_8_5: bool = ((s_8_2) == (s_8_4));
        // D s_8_6: write-var gs#20956 <= s_8_5
        fn_state.gs_20956 = s_8_5;
        // N s_8_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var ga#16352:u8
        let s_9_0: u8 = fn_state.ga_16352;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 2u16);
        // C s_9_2: const #440u : u32
        let s_9_2: u32 = 440;
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
        // D s_9_6: not s_9_5
        let s_9_6: bool = !s_9_5;
        // N s_9_7: branch s_9_6 b17 b10
        if s_9_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call EL2Enabled(s_10_0)
        let s_10_1: bool = EL2Enabled(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b16 b11
        if s_10_1 {
            return block_16(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#20949 <= s_11_0
        fn_state.gs_20949 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var gs#20949:u8
        let s_12_0: bool = fn_state.gs_20949;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#20950 <= s_13_0
        fn_state.gs_20950 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var gs#20950:u8
        let s_14_0: bool = fn_state.gs_20950;
        // D s_14_1: write-var privileged <= s_14_0
        fn_state.privileged = s_14_0;
        // N s_14_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #102552u : u32
        let s_15_0: u32 = 102552;
        // D s_15_1: read-reg s_15_0:struct
        let s_15_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: call _get_HCR_EL2_Type_NV(s_15_1)
        let s_15_2: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_15_1);
        // C s_15_3: const #102552u : u32
        let s_15_3: u32 = 102552;
        // D s_15_4: read-reg s_15_3:struct
        let s_15_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_15_3 as isize);
            tracer.read_register(s_15_3 as isize, value);
            value
        };
        // D s_15_5: call _get_HCR_EL2_Type_NV1(s_15_4)
        let s_15_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_15_4);
        // D s_15_6: cast zx s_15_2 -> bv
        let s_15_6: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_7: cast zx s_15_5 -> bv
        let s_15_7: Bits = Bits::new(s_15_5 as u128, 1u16);
        // D s_15_8: cast reint s_15_6 -> u128
        let s_15_8: u128 = (s_15_6.value() as u128);
        // D s_15_9: size-of s_15_6
        let s_15_9: u16 = s_15_6.length();
        // D s_15_10: cast reint s_15_7 -> u128
        let s_15_10: u128 = (s_15_7.value() as u128);
        // D s_15_11: size-of s_15_7
        let s_15_11: u16 = s_15_7.length();
        // D s_15_12: lsl s_15_8 s_15_11
        let s_15_12: u128 = s_15_8 << s_15_11;
        // D s_15_13: or s_15_12 s_15_10
        let s_15_13: u128 = ((s_15_12) | (s_15_10));
        // D s_15_14: add s_15_9 s_15_11
        let s_15_14: u16 = (s_15_9 + s_15_11);
        // D s_15_15: create-bits s_15_13 s_15_14
        let s_15_15: Bits = Bits::new(s_15_13, s_15_14);
        // D s_15_16: cast reint s_15_15 -> u8
        let s_15_16: u8 = (s_15_15.value() as u8);
        // D s_15_17: cast zx s_15_16 -> bv
        let s_15_17: Bits = Bits::new(s_15_16 as u128, 2u16);
        // C s_15_18: const #3u : u8
        let s_15_18: u8 = 3;
        // C s_15_19: cast zx s_15_18 -> bv
        let s_15_19: Bits = Bits::new(s_15_18 as u128, 2u16);
        // D s_15_20: cmp-eq s_15_17 s_15_19
        let s_15_20: bool = ((s_15_17) == (s_15_19));
        // D s_15_21: write-var gs#20950 <= s_15_20
        fn_state.gs_20950 = s_15_20;
        // N s_15_22: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call HaveNVExt(s_16_0)
        let s_16_1: bool = HaveNVExt(state, tracer, s_16_0);
        // D s_16_2: write-var gs#20949 <= s_16_1
        fn_state.gs_20949 = s_16_1;
        // N s_16_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var ga#16352:u8
        let s_17_0: u8 = fn_state.ga_16352;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #432u : u32
        let s_17_2: u32 = 432;
        // D s_17_3: read-reg s_17_2:u8
        let s_17_3: u8 = {
            let value = state.read_register::<u8>(s_17_2 as isize);
            tracer.read_register(s_17_2 as isize, value);
            value
        };
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 2u16);
        // D s_17_5: cmp-eq s_17_1 s_17_4
        let s_17_5: bool = ((s_17_1) == (s_17_4));
        // D s_17_6: not s_17_5
        let s_17_6: bool = !s_17_5;
        // N s_17_7: branch s_17_6 b22 b18
        if s_17_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call HaveVirtHostExt(s_18_0)
        let s_18_1: bool = HaveVirtHostExt(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b21 b19
        if s_18_1 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#20952 <= s_19_0
        fn_state.gs_20952 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var gs#20952:u8
        let s_20_0: bool = fn_state.gs_20952;
        // D s_20_1: not s_20_0
        let s_20_1: bool = !s_20_0;
        // D s_20_2: write-var privileged <= s_20_1
        fn_state.privileged = s_20_1;
        // N s_20_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #102552u : u32
        let s_21_0: u32 = 102552;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_HCR_EL2_Type_E2H(s_21_1)
        let s_21_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_21_1);
        // C s_21_3: const #102552u : u32
        let s_21_3: u32 = 102552;
        // D s_21_4: read-reg s_21_3:struct
        let s_21_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_3 as isize);
            tracer.read_register(s_21_3 as isize, value);
            value
        };
        // D s_21_5: call _get_HCR_EL2_Type_TGE(s_21_4)
        let s_21_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_21_4);
        // D s_21_6: cast zx s_21_2 -> bv
        let s_21_6: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_7: cast zx s_21_5 -> bv
        let s_21_7: Bits = Bits::new(s_21_5 as u128, 1u16);
        // D s_21_8: cast reint s_21_6 -> u128
        let s_21_8: u128 = (s_21_6.value() as u128);
        // D s_21_9: size-of s_21_6
        let s_21_9: u16 = s_21_6.length();
        // D s_21_10: cast reint s_21_7 -> u128
        let s_21_10: u128 = (s_21_7.value() as u128);
        // D s_21_11: size-of s_21_7
        let s_21_11: u16 = s_21_7.length();
        // D s_21_12: lsl s_21_8 s_21_11
        let s_21_12: u128 = s_21_8 << s_21_11;
        // D s_21_13: or s_21_12 s_21_10
        let s_21_13: u128 = ((s_21_12) | (s_21_10));
        // D s_21_14: add s_21_9 s_21_11
        let s_21_14: u16 = (s_21_9 + s_21_11);
        // D s_21_15: create-bits s_21_13 s_21_14
        let s_21_15: Bits = Bits::new(s_21_13, s_21_14);
        // D s_21_16: cast reint s_21_15 -> u8
        let s_21_16: u8 = (s_21_15.value() as u8);
        // D s_21_17: cast zx s_21_16 -> bv
        let s_21_17: Bits = Bits::new(s_21_16 as u128, 2u16);
        // C s_21_18: const #3u : u8
        let s_21_18: u8 = 3;
        // C s_21_19: cast zx s_21_18 -> bv
        let s_21_19: Bits = Bits::new(s_21_18 as u128, 2u16);
        // D s_21_20: cmp-eq s_21_17 s_21_19
        let s_21_20: bool = ((s_21_17) == (s_21_19));
        // D s_21_21: write-var gs#20952 <= s_21_20
        fn_state.gs_20952 = s_21_20;
        // N s_21_22: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var ga#16352:u8
        let s_22_0: u8 = fn_state.ga_16352;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 2u16);
        // C s_22_2: const #424u : u32
        let s_22_2: u32 = 424;
        // D s_22_3: read-reg s_22_2:u8
        let s_22_3: u8 = {
            let value = state.read_register::<u8>(s_22_2 as isize);
            tracer.read_register(s_22_2 as isize, value);
            value
        };
        // D s_22_4: cast zx s_22_3 -> bv
        let s_22_4: Bits = Bits::new(s_22_3 as u128, 2u16);
        // D s_22_5: cmp-eq s_22_1 s_22_4
        let s_22_5: bool = ((s_22_1) == (s_22_4));
        // D s_22_6: not s_22_5
        let s_22_6: bool = !s_22_5;
        // N s_22_7: branch s_22_6 b24 b23
        if s_22_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var privileged <= s_23_0
        fn_state.privileged = s_23_0;
        // N s_23_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_24_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
