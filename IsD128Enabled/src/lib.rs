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
use u_get_HCR_EL2_Type_E2H::*;
use Have128BitDescriptorExt::*;
use IsTCR2EL2Enabled::*;
use u_get_TCR2_EL1_Type_D128::*;
use IsTCR2EL1Enabled::*;
use ELIsInHost::*;
use common::*;
pub fn IsD128Enabled<T: Tracer>(state: &mut State, tracer: &T, el: u8) -> bool {
    #[derive(Default)]
    struct FunctionState {
        d128enabled: bool,
        gs_20585: bool,
        gs_20589: bool,
        gs_20590: bool,
        gs_20587: bool,
        gs_20586: bool,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call Have128BitDescriptorExt(s_0_0)
        let s_0_1: bool = Have128BitDescriptorExt(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b3 b1
        if s_0_1 {
            return block_3(state, tracer, fn_state);
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
        // D s_1_1: write-var d128enabled <= s_1_0
        fn_state.d128enabled = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var d128enabled:u8
        let s_2_0: bool = fn_state.d128enabled;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var el:u8
        let s_3_0: u8 = fn_state.el;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #448u : u32
        let s_3_2: u32 = 448;
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
        // D s_3_6: not s_3_5
        let s_3_6: bool = !s_3_5;
        // N s_3_7: branch s_3_6 b14 b4
        if s_3_6 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #448u : u32
        let s_4_0: u32 = 448;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call ELIsInHost(s_4_1)
        let s_4_2: bool = ELIsInHost(state, tracer, s_4_1);
        // D s_4_3: not s_4_2
        let s_4_3: bool = !s_4_2;
        // N s_4_4: branch s_4_3 b10 b5
        if s_4_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call IsTCR2EL2Enabled(s_5_0)
        let s_5_1: bool = IsTCR2EL2Enabled(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b9 b6
        if s_5_1 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#20585 <= s_6_0
        fn_state.gs_20585 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var gs#20585:u8
        let s_7_0: bool = fn_state.gs_20585;
        // D s_7_1: write-var d128enabled <= s_7_0
        fn_state.d128enabled = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_8_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #15752u : u32
        let s_9_0: u32 = 15752;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call _get_TCR2_EL2_Type_D128(s_9_1)
        let s_9_2: bool = u_get_TCR2_EL2_Type_D128(state, tracer, s_9_1);
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // C s_9_4: const #1u : u8
        let s_9_4: bool = true;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // D s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // D s_9_7: write-var gs#20585 <= s_9_6
        fn_state.gs_20585 = s_9_6;
        // N s_9_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call IsTCR2EL1Enabled(s_10_0)
        let s_10_1: bool = IsTCR2EL1Enabled(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b13 b11
        if s_10_1 {
            return block_13(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#20586 <= s_11_0
        fn_state.gs_20586 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var gs#20586:u8
        let s_12_0: bool = fn_state.gs_20586;
        // D s_12_1: write-var d128enabled <= s_12_0
        fn_state.d128enabled = s_12_0;
        // N s_12_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #14776u : u32
        let s_13_0: u32 = 14776;
        // D s_13_1: read-reg s_13_0:struct
        let s_13_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call _get_TCR2_EL1_Type_D128(s_13_1)
        let s_13_2: bool = u_get_TCR2_EL1_Type_D128(state, tracer, s_13_1);
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // C s_13_4: const #1u : u8
        let s_13_4: bool = true;
        // C s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 1u16);
        // D s_13_6: cmp-eq s_13_3 s_13_5
        let s_13_6: bool = ((s_13_3) == (s_13_5));
        // D s_13_7: write-var gs#20586 <= s_13_6
        fn_state.gs_20586 = s_13_6;
        // N s_13_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var el:u8
        let s_14_0: u8 = fn_state.el;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 2u16);
        // C s_14_2: const #440u : u32
        let s_14_2: u32 = 440;
        // D s_14_3: read-reg s_14_2:u8
        let s_14_3: u8 = {
            let value = state.read_register::<u8>(s_14_2 as isize);
            tracer.read_register(s_14_2 as isize, value);
            value
        };
        // D s_14_4: cast zx s_14_3 -> bv
        let s_14_4: Bits = Bits::new(s_14_3 as u128, 2u16);
        // D s_14_5: cmp-eq s_14_1 s_14_4
        let s_14_5: bool = ((s_14_1) == (s_14_4));
        // D s_14_6: not s_14_5
        let s_14_6: bool = !s_14_5;
        // N s_14_7: branch s_14_6 b19 b15
        if s_14_6 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call IsTCR2EL1Enabled(s_15_0)
        let s_15_1: bool = IsTCR2EL1Enabled(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b18 b16
        if s_15_1 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#20587 <= s_16_0
        fn_state.gs_20587 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var gs#20587:u8
        let s_17_0: bool = fn_state.gs_20587;
        // D s_17_1: write-var d128enabled <= s_17_0
        fn_state.d128enabled = s_17_0;
        // N s_17_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #14776u : u32
        let s_18_0: u32 = 14776;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_TCR2_EL1_Type_D128(s_18_1)
        let s_18_2: bool = u_get_TCR2_EL1_Type_D128(state, tracer, s_18_1);
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // C s_18_4: const #1u : u8
        let s_18_4: bool = true;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 1u16);
        // D s_18_6: cmp-eq s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) == (s_18_5));
        // D s_18_7: write-var gs#20587 <= s_18_6
        fn_state.gs_20587 = s_18_6;
        // N s_18_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var el:u8
        let s_19_0: u8 = fn_state.el;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 2u16);
        // C s_19_2: const #432u : u32
        let s_19_2: u32 = 432;
        // D s_19_3: read-reg s_19_2:u8
        let s_19_3: u8 = {
            let value = state.read_register::<u8>(s_19_2 as isize);
            tracer.read_register(s_19_2 as isize, value);
            value
        };
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 2u16);
        // D s_19_5: cmp-eq s_19_1 s_19_4
        let s_19_5: bool = ((s_19_1) == (s_19_4));
        // D s_19_6: not s_19_5
        let s_19_6: bool = !s_19_5;
        // N s_19_7: branch s_19_6 b27 b20
        if s_19_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call IsTCR2EL2Enabled(s_20_0)
        let s_20_1: bool = IsTCR2EL2Enabled(state, tracer, s_20_0);
        // N s_20_2: branch s_20_1 b26 b21
        if s_20_1 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#20589 <= s_21_0
        fn_state.gs_20589 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var gs#20589:u8
        let s_22_0: bool = fn_state.gs_20589;
        // N s_22_1: branch s_22_0 b25 b23
        if s_22_0 {
            return block_25(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#20590 <= s_23_0
        fn_state.gs_20590 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var gs#20590:u8
        let s_24_0: bool = fn_state.gs_20590;
        // D s_24_1: write-var d128enabled <= s_24_0
        fn_state.d128enabled = s_24_0;
        // N s_24_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #15752u : u32
        let s_25_0: u32 = 15752;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_TCR2_EL2_Type_D128(s_25_1)
        let s_25_2: bool = u_get_TCR2_EL2_Type_D128(state, tracer, s_25_1);
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // C s_25_4: const #1u : u8
        let s_25_4: bool = true;
        // C s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 1u16);
        // D s_25_6: cmp-eq s_25_3 s_25_5
        let s_25_6: bool = ((s_25_3) == (s_25_5));
        // D s_25_7: write-var gs#20590 <= s_25_6
        fn_state.gs_20590 = s_25_6;
        // N s_25_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #102552u : u32
        let s_26_0: u32 = 102552;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_HCR_EL2_Type_E2H(s_26_1)
        let s_26_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_26_1);
        // D s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // C s_26_4: const #1u : u8
        let s_26_4: bool = true;
        // C s_26_5: cast zx s_26_4 -> bv
        let s_26_5: Bits = Bits::new(s_26_4 as u128, 1u16);
        // D s_26_6: cmp-eq s_26_3 s_26_5
        let s_26_6: bool = ((s_26_3) == (s_26_5));
        // D s_26_7: write-var gs#20589 <= s_26_6
        fn_state.gs_20589 = s_26_6;
        // N s_26_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var el:u8
        let s_27_0: u8 = fn_state.el;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 2u16);
        // C s_27_2: const #424u : u32
        let s_27_2: u32 = 424;
        // D s_27_3: read-reg s_27_2:u8
        let s_27_3: u8 = {
            let value = state.read_register::<u8>(s_27_2 as isize);
            tracer.read_register(s_27_2 as isize, value);
            value
        };
        // D s_27_4: cast zx s_27_3 -> bv
        let s_27_4: Bits = Bits::new(s_27_3 as u128, 2u16);
        // D s_27_5: cmp-eq s_27_1 s_27_4
        let s_27_5: bool = ((s_27_1) == (s_27_4));
        // D s_27_6: not s_27_5
        let s_27_6: bool = !s_27_5;
        // N s_27_7: branch s_27_6 b29 b28
        if s_27_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #10736u : u32
        let s_28_0: u32 = 10736;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_TCR_EL3_Type_D128(s_28_1)
        let s_28_2: bool = u_get_TCR_EL3_Type_D128(state, tracer, s_28_1);
        // D s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // D s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var d128enabled <= s_28_6
        fn_state.d128enabled = s_28_6;
        // N s_28_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_29_0: jump b8
        return block_8(state, tracer, fn_state);
    }
}
