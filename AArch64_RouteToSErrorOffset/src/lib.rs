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
use IsSCTLR2EL2Enabled::*;
use u_get_SCR_EL3_Type_EASE::*;
use HaveDoubleFault2Ext::*;
use u_get_SCTLR2_EL2_Type_EASE::*;
use u_get_SCTLR2_EL1_Type_EASE::*;
use IsSCTLR2EL1Enabled::*;
use HaveDoubleFaultExt::*;
use common::*;
pub fn AArch64_RouteToSErrorOffset<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target_el: u8,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_9779: bool,
        return_value: bool,
        gs_9781: bool,
        ease_bit: bool,
        target_el: u8,
    }
    let fn_state = FunctionState {
        target_el,
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
        // S s_0_1: call HaveDoubleFaultExt(s_0_0)
        let s_0_1: bool = HaveDoubleFaultExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b20 b1
        if s_0_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var target_el:u8
        let s_1_0: u8 = fn_state.target_el;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #424u : u32
        let s_1_2: u32 = 424;
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
        // D s_1_6: not s_1_5
        let s_1_6: bool = !s_1_5;
        // N s_1_7: branch s_1_6 b5 b2
        if s_1_6 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #90704u : u32
        let s_2_0: u32 = 90704;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_SCR_EL3_Type_EASE(s_2_1)
        let s_2_2: bool = u_get_SCR_EL3_Type_EASE(state, tracer, s_2_1);
        // D s_2_3: write-var ease_bit <= s_2_2
        fn_state.ease_bit = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var ease_bit:u8
        let s_3_0: bool = fn_state.ease_bit;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: write-var return_value <= s_3_4
        fn_state.return_value = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var return_value:u8
        let s_4_0: bool = fn_state.return_value;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var target_el:u8
        let s_5_0: u8 = fn_state.target_el;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #432u : u32
        let s_5_2: u32 = 432;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // D s_5_6: not s_5_5
        let s_5_6: bool = !s_5_5;
        // N s_5_7: branch s_5_6 b12 b6
        if s_5_6 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call HaveDoubleFault2Ext(s_6_0)
        let s_6_1: bool = HaveDoubleFault2Ext(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b11 b7
        if s_6_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#9779 <= s_7_0
        fn_state.gs_9779 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#9779:u8
        let s_8_0: bool = fn_state.gs_9779;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var ease_bit <= s_9_0
        fn_state.ease_bit = s_9_0;
        // N s_9_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #102680u : u32
        let s_10_0: u32 = 102680;
        // D s_10_1: read-reg s_10_0:struct
        let s_10_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call _get_SCTLR2_EL2_Type_EASE(s_10_1)
        let s_10_2: bool = u_get_SCTLR2_EL2_Type_EASE(state, tracer, s_10_1);
        // D s_10_3: write-var ease_bit <= s_10_2
        fn_state.ease_bit = s_10_2;
        // N s_10_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call IsSCTLR2EL2Enabled(s_11_0)
        let s_11_1: bool = IsSCTLR2EL2Enabled(state, tracer, s_11_0);
        // D s_11_2: write-var gs#9779 <= s_11_1
        fn_state.gs_9779 = s_11_1;
        // N s_11_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var target_el:u8
        let s_12_0: u8 = fn_state.target_el;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 2u16);
        // C s_12_2: const #440u : u32
        let s_12_2: u32 = 440;
        // D s_12_3: read-reg s_12_2:u8
        let s_12_3: u8 = {
            let value = state.read_register::<u8>(s_12_2 as isize);
            tracer.read_register(s_12_2 as isize, value);
            value
        };
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 2u16);
        // D s_12_5: cmp-eq s_12_1 s_12_4
        let s_12_5: bool = ((s_12_1) == (s_12_4));
        // D s_12_6: not s_12_5
        let s_12_6: bool = !s_12_5;
        // N s_12_7: branch s_12_6 b19 b13
        if s_12_6 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call HaveDoubleFault2Ext(s_13_0)
        let s_13_1: bool = HaveDoubleFault2Ext(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b18 b14
        if s_13_1 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#9781 <= s_14_0
        fn_state.gs_9781 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var gs#9781:u8
        let s_15_0: bool = fn_state.gs_9781;
        // N s_15_1: branch s_15_0 b17 b16
        if s_15_0 {
            return block_17(state, tracer, fn_state);
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
        // D s_16_1: write-var ease_bit <= s_16_0
        fn_state.ease_bit = s_16_0;
        // N s_16_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #11720u : u32
        let s_17_0: u32 = 11720;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_SCTLR2_EL1_Type_EASE(s_17_1)
        let s_17_2: bool = u_get_SCTLR2_EL1_Type_EASE(state, tracer, s_17_1);
        // D s_17_3: write-var ease_bit <= s_17_2
        fn_state.ease_bit = s_17_2;
        // N s_17_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call IsSCTLR2EL1Enabled(s_18_0)
        let s_18_1: bool = IsSCTLR2EL1Enabled(state, tracer, s_18_0);
        // D s_18_2: write-var gs#9781 <= s_18_1
        fn_state.gs_9781 = s_18_1;
        // N s_18_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_19_0: jump b3
        return block_3(state, tracer, fn_state);
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
        // N s_20_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
