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
use u_get_TCR_EL3_Type_PnCH::*;
use u_get_TCR2_EL2_Type_PnCH::*;
use u_get_TCR2_EL1_Type_PnCH::*;
use ELUsingAArch32::*;
use IsTCR2EL1Enabled::*;
use IsTCR2EL2Enabled::*;
use S1TranslationRegime::*;
use IsD128Enabled::*;
use common::*;
pub fn ProtectionEnabled<T: Tracer>(state: &mut State, tracer: &T, el: u8) -> bool {
    #[derive(Default)]
    struct FunctionState {
        regime: u8,
        gs_20599: bool,
        gs_20597: bool,
        return_value: bool,
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
        // C s_0_1: const #2u : u8
        let s_0_1: u8 = 2;
        // D s_0_2: cmp-lt s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) < (s_0_1));
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // D s_0_4: read-var el:u8
        let s_0_4: u8 = fn_state.el;
        // D s_0_5: call S1TranslationRegime(s_0_4)
        let s_0_5: u8 = S1TranslationRegime(state, tracer, s_0_4);
        // D s_0_6: write-var regime <= s_0_5
        fn_state.regime = s_0_5;
        // D s_0_7: read-var regime:u8
        let s_0_7: u8 = fn_state.regime;
        // D s_0_8: call ELUsingAArch32(s_0_7)
        let s_0_8: bool = ELUsingAArch32(state, tracer, s_0_7);
        // D s_0_9: not s_0_8
        let s_0_9: bool = !s_0_8;
        // N s_0_10: assert s_0_9
        let s_0_10: () = assert!(s_0_9);
        // D s_0_11: read-var el:u8
        let s_0_11: u8 = fn_state.el;
        // D s_0_12: call IsD128Enabled(s_0_11)
        let s_0_12: bool = IsD128Enabled(state, tracer, s_0_11);
        // D s_0_13: not s_0_12
        let s_0_13: bool = !s_0_12;
        // N s_0_14: branch s_0_13 b3 b1
        if s_0_13 {
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
        // C s_1_0: const #1u : u8
        let s_1_0: bool = true;
        // D s_1_1: write-var return_value <= s_1_0
        fn_state.return_value = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var return_value:u8
        let s_2_0: bool = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var regime:u8
        let s_3_0: u8 = fn_state.regime;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #440u : u32
        let s_3_2: u32 = 440;
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
        // N s_3_7: branch s_3_6 b8 b4
        if s_3_6 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call IsTCR2EL1Enabled(s_4_0)
        let s_4_1: bool = IsTCR2EL1Enabled(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b7 b5
        if s_4_1 {
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
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#20597 <= s_5_0
        fn_state.gs_20597 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#20597:u8
        let s_6_0: bool = fn_state.gs_20597;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #14776u : u32
        let s_7_0: u32 = 14776;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_TCR2_EL1_Type_PnCH(s_7_1)
        let s_7_2: bool = u_get_TCR2_EL1_Type_PnCH(state, tracer, s_7_1);
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // C s_7_4: const #1u : u8
        let s_7_4: bool = true;
        // C s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 1u16);
        // D s_7_6: cmp-eq s_7_3 s_7_5
        let s_7_6: bool = ((s_7_3) == (s_7_5));
        // D s_7_7: write-var gs#20597 <= s_7_6
        fn_state.gs_20597 = s_7_6;
        // N s_7_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var regime:u8
        let s_8_0: u8 = fn_state.regime;
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
        // D s_8_6: not s_8_5
        let s_8_6: bool = !s_8_5;
        // N s_8_7: branch s_8_6 b13 b9
        if s_8_6 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call IsTCR2EL2Enabled(s_9_0)
        let s_9_1: bool = IsTCR2EL2Enabled(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b12 b10
        if s_9_1 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#20599 <= s_10_0
        fn_state.gs_20599 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var gs#20599:u8
        let s_11_0: bool = fn_state.gs_20599;
        // D s_11_1: write-var return_value <= s_11_0
        fn_state.return_value = s_11_0;
        // N s_11_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #15752u : u32
        let s_12_0: u32 = 15752;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_TCR2_EL2_Type_PnCH(s_12_1)
        let s_12_2: bool = u_get_TCR2_EL2_Type_PnCH(state, tracer, s_12_1);
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // C s_12_4: const #1u : u8
        let s_12_4: bool = true;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 1u16);
        // D s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // D s_12_7: write-var gs#20599 <= s_12_6
        fn_state.gs_20599 = s_12_6;
        // N s_12_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var regime:u8
        let s_13_0: u8 = fn_state.regime;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #424u : u32
        let s_13_2: u32 = 424;
        // D s_13_3: read-reg s_13_2:u8
        let s_13_3: u8 = {
            let value = state.read_register::<u8>(s_13_2 as isize);
            tracer.read_register(s_13_2 as isize, value);
            value
        };
        // D s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 2u16);
        // D s_13_5: cmp-eq s_13_1 s_13_4
        let s_13_5: bool = ((s_13_1) == (s_13_4));
        // D s_13_6: not s_13_5
        let s_13_6: bool = !s_13_5;
        // N s_13_7: branch s_13_6 b15 b14
        if s_13_6 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #10736u : u32
        let s_14_0: u32 = 10736;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_TCR_EL3_Type_PnCH(s_14_1)
        let s_14_2: bool = u_get_TCR_EL3_Type_PnCH(state, tracer, s_14_1);
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // C s_14_4: const #1u : u8
        let s_14_4: bool = true;
        // C s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // D s_14_6: cmp-eq s_14_3 s_14_5
        let s_14_6: bool = ((s_14_3) == (s_14_5));
        // D s_14_7: write-var return_value <= s_14_6
        fn_state.return_value = s_14_6;
        // N s_14_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var return_value <= s_15_0
        fn_state.return_value = s_15_0;
        // N s_15_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
