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
use u_get_TCR_EL3_Type_D128::*;
use u_get_TCR2_EL2_Type_D128::*;
use u_get_HCR_EL2_Type_DC::*;
use Have128BitDescriptorExt::*;
use u_get_VTCR_EL2_Type_D128::*;
use u_get_TCR2_EL1_Type_D128::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_VM::*;
use common::*;
pub fn AArch64_isPARFormatD128<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    is_ATS1Ex: bool,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_29061: bool,
        gs_29060: bool,
        isPARFormatD128: bool,
        gs_29054: bool,
        regime: u32,
        is_ATS1Ex: bool,
    }
    let fn_state = FunctionState {
        regime,
        is_ATS1Ex,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var regime:u32
        let s_0_0: u32 = fn_state.regime;
        // C s_0_1: const #2u : u32
        let s_0_1: u32 = 2;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b21 b1
        if s_0_2 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call Have128BitDescriptorExt(s_1_0)
        let s_1_1: bool = Have128BitDescriptorExt(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // D s_1_3: write-var gs#29054 <= s_1_2
        fn_state.gs_29054 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#29054:u8
        let s_2_0: bool = fn_state.gs_29054;
        // N s_2_1: branch s_2_0 b20 b3
        if s_2_0 {
            return block_20(state, tracer, fn_state);
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
        // D s_3_1: write-var isPARFormatD128 <= s_3_0
        fn_state.isPARFormatD128 = s_3_0;
        // C s_3_2: const #0u : u32
        let s_3_2: u32 = 0;
        // D s_3_3: read-var regime:u32
        let s_3_3: u32 = fn_state.regime;
        // D s_3_4: cmp-eq s_3_2 s_3_3
        let s_3_4: bool = ((s_3_2) == (s_3_3));
        // D s_3_5: not s_3_4
        let s_3_5: bool = !s_3_4;
        // N s_3_6: branch s_3_5 b7 b4
        if s_3_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #10736u : u32
        let s_4_0: u32 = 10736;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call _get_TCR_EL3_Type_D128(s_4_1)
        let s_4_2: bool = u_get_TCR_EL3_Type_D128(state, tracer, s_4_1);
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // C s_4_4: const #1u : u8
        let s_4_4: bool = true;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // D s_4_6: cmp-eq s_4_3 s_4_5
        let s_4_6: bool = ((s_4_3) == (s_4_5));
        // D s_4_7: write-var isPARFormatD128 <= s_4_6
        fn_state.isPARFormatD128 = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
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
        // D s_6_0: read-var isPARFormatD128:u8
        let s_6_0: bool = fn_state.isPARFormatD128;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #3u : u32
        let s_7_0: u32 = 3;
        // D s_7_1: read-var regime:u32
        let s_7_1: u32 = fn_state.regime;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b9 b8
        if s_7_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #15752u : u32
        let s_8_0: u32 = 15752;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_TCR2_EL2_Type_D128(s_8_1)
        let s_8_2: bool = u_get_TCR2_EL2_Type_D128(state, tracer, s_8_1);
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // C s_8_4: const #1u : u8
        let s_8_4: bool = true;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // D s_8_6: cmp-eq s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) == (s_8_5));
        // D s_8_7: write-var isPARFormatD128 <= s_8_6
        fn_state.isPARFormatD128 = s_8_6;
        // N s_8_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #4u : u32
        let s_9_0: u32 = 4;
        // D s_9_1: read-var regime:u32
        let s_9_1: u32 = fn_state.regime;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b19 b10
        if s_9_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var is_ATS1Ex:u8
        let s_10_0: bool = fn_state.is_ATS1Ex;
        // N s_10_1: branch s_10_0 b18 b11
        if s_10_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call EL2Enabled(s_11_0)
        let s_11_1: bool = EL2Enabled(state, tracer, s_11_0);
        // S s_11_2: not s_11_1
        let s_11_2: bool = !s_11_1;
        // D s_11_3: write-var gs#29060 <= s_11_2
        fn_state.gs_29060 = s_11_2;
        // N s_11_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var gs#29060:u8
        let s_12_0: bool = fn_state.gs_29060;
        // N s_12_1: branch s_12_0 b17 b13
        if s_12_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #102552u : u32
        let s_13_0: u32 = 102552;
        // D s_13_1: read-reg s_13_0:struct
        let s_13_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call _get_HCR_EL2_Type_VM(s_13_1)
        let s_13_2: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_13_1);
        // C s_13_3: const #102552u : u32
        let s_13_3: u32 = 102552;
        // D s_13_4: read-reg s_13_3:struct
        let s_13_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_13_3 as isize);
            tracer.read_register(s_13_3 as isize, value);
            value
        };
        // D s_13_5: call _get_HCR_EL2_Type_DC(s_13_4)
        let s_13_5: bool = u_get_HCR_EL2_Type_DC(state, tracer, s_13_4);
        // D s_13_6: cast zx s_13_2 -> bv
        let s_13_6: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_7: cast zx s_13_5 -> bv
        let s_13_7: Bits = Bits::new(s_13_5 as u128, 1u16);
        // D s_13_8: cast reint s_13_6 -> u128
        let s_13_8: u128 = (s_13_6.value() as u128);
        // D s_13_9: size-of s_13_6
        let s_13_9: u16 = s_13_6.length();
        // D s_13_10: cast reint s_13_7 -> u128
        let s_13_10: u128 = (s_13_7.value() as u128);
        // D s_13_11: size-of s_13_7
        let s_13_11: u16 = s_13_7.length();
        // D s_13_12: lsl s_13_8 s_13_11
        let s_13_12: u128 = s_13_8 << s_13_11;
        // D s_13_13: or s_13_12 s_13_10
        let s_13_13: u128 = ((s_13_12) | (s_13_10));
        // D s_13_14: add s_13_9 s_13_11
        let s_13_14: u16 = (s_13_9 + s_13_11);
        // D s_13_15: create-bits s_13_13 s_13_14
        let s_13_15: Bits = Bits::new(s_13_13, s_13_14);
        // D s_13_16: cast reint s_13_15 -> u8
        let s_13_16: u8 = (s_13_15.value() as u8);
        // D s_13_17: cast zx s_13_16 -> bv
        let s_13_17: Bits = Bits::new(s_13_16 as u128, 2u16);
        // C s_13_18: const #0u : u8
        let s_13_18: u8 = 0;
        // C s_13_19: cast zx s_13_18 -> bv
        let s_13_19: Bits = Bits::new(s_13_18 as u128, 2u16);
        // D s_13_20: cmp-eq s_13_17 s_13_19
        let s_13_20: bool = ((s_13_17) == (s_13_19));
        // D s_13_21: write-var gs#29061 <= s_13_20
        fn_state.gs_29061 = s_13_20;
        // N s_13_22: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var gs#29061:u8
        let s_14_0: bool = fn_state.gs_29061;
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
    ) -> bool {
        // C s_15_0: const #15328u : u32
        let s_15_0: u32 = 15328;
        // D s_15_1: read-reg s_15_0:struct
        let s_15_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: call _get_VTCR_EL2_Type_D128(s_15_1)
        let s_15_2: bool = u_get_VTCR_EL2_Type_D128(state, tracer, s_15_1);
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // C s_15_4: const #1u : u8
        let s_15_4: bool = true;
        // C s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 1u16);
        // D s_15_6: cmp-eq s_15_3 s_15_5
        let s_15_6: bool = ((s_15_3) == (s_15_5));
        // D s_15_7: write-var isPARFormatD128 <= s_15_6
        fn_state.isPARFormatD128 = s_15_6;
        // N s_15_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #14776u : u32
        let s_16_0: u32 = 14776;
        // D s_16_1: read-reg s_16_0:struct
        let s_16_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call _get_TCR2_EL1_Type_D128(s_16_1)
        let s_16_2: bool = u_get_TCR2_EL1_Type_D128(state, tracer, s_16_1);
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // C s_16_4: const #1u : u8
        let s_16_4: bool = true;
        // C s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // D s_16_6: cmp-eq s_16_3 s_16_5
        let s_16_6: bool = ((s_16_3) == (s_16_5));
        // D s_16_7: write-var isPARFormatD128 <= s_16_6
        fn_state.isPARFormatD128 = s_16_6;
        // N s_16_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#29061 <= s_17_0
        fn_state.gs_29061 = s_17_0;
        // N s_17_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#29060 <= s_18_0
        fn_state.gs_29060 = s_18_0;
        // N s_18_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_19_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var isPARFormatD128 <= s_20_0
        fn_state.isPARFormatD128 = s_20_0;
        // N s_20_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#29054 <= s_21_0
        fn_state.gs_29054 = s_21_0;
        // N s_21_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
