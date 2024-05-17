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
use ELUsingAArch32::*;
use HaveRME::*;
use ProfilingBufferOwner::*;
use u_get_SCR_EL3_Type_NSE::*;
use EffectiveSCR_EL3_NS::*;
use HaveStatisticalProfiling::*;
use u_get_PMBLIMITR_EL1_Type_E::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_PMBSR_EL1_Type_S::*;
use common::*;
pub fn ProfilingBufferEnabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_3582: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_3595: bool,
        state_match: bool,
        owning_ss: u32,
        gs_3596: bool,
        return_value: bool,
        owning_el: u8,
        state_bits: u8,
        gs_3597: bool,
        ga_2162: ProductTypec8897aad3eb4a29e,
        gs_3582: (),
    }
    let fn_state = FunctionState {
        gs_3582,
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
        // S s_0_1: call HaveStatisticalProfiling(s_0_0)
        let s_0_1: bool = HaveStatisticalProfiling(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b22 b1
        if s_0_2 {
            return block_22(state, tracer, fn_state);
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
        // S s_1_1: call ProfilingBufferOwner(s_1_0)
        let s_1_1: ProductTypec8897aad3eb4a29e = ProfilingBufferOwner(
            state,
            tracer,
            s_1_0,
        );
        // D s_1_2: write-var ga#2162 <= s_1_1
        fn_state.ga_2162 = s_1_1;
        // D s_1_3: read-var ga#2162.0:struct
        let s_1_3: u32 = fn_state.ga_2162._0;
        // D s_1_4: read-var ga#2162.1:struct
        let s_1_4: u8 = fn_state.ga_2162._1;
        // D s_1_5: write-var owning_ss <= s_1_3
        fn_state.owning_ss = s_1_3;
        // D s_1_6: write-var owning_el <= s_1_4
        fn_state.owning_el = s_1_4;
        // C s_1_7: const #() : ()
        let s_1_7: () = ();
        // S s_1_8: call HaveRME(s_1_7)
        let s_1_8: bool = HaveRME(state, tracer, s_1_7);
        // N s_1_9: branch s_1_8 b21 b2
        if s_1_8 {
            return block_21(state, tracer, fn_state);
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
        // D s_2_2: call _get_SCR_EL3_Type_NS(s_2_1)
        let s_2_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_2_1);
        // C s_2_3: const #0u : u8
        let s_2_3: bool = false;
        // C s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 1u16);
        // D s_2_5: cast zx s_2_2 -> bv
        let s_2_5: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_6: cast reint s_2_4 -> u128
        let s_2_6: u128 = (s_2_4.value() as u128);
        // D s_2_7: size-of s_2_4
        let s_2_7: u16 = s_2_4.length();
        // D s_2_8: cast reint s_2_5 -> u128
        let s_2_8: u128 = (s_2_5.value() as u128);
        // D s_2_9: size-of s_2_5
        let s_2_9: u16 = s_2_5.length();
        // D s_2_10: lsl s_2_6 s_2_9
        let s_2_10: u128 = s_2_6 << s_2_9;
        // D s_2_11: or s_2_10 s_2_8
        let s_2_11: u128 = ((s_2_10) | (s_2_8));
        // D s_2_12: add s_2_7 s_2_9
        let s_2_12: u16 = (s_2_7 + s_2_9);
        // D s_2_13: create-bits s_2_11 s_2_12
        let s_2_13: Bits = Bits::new(s_2_11, s_2_12);
        // D s_2_14: cast reint s_2_13 -> u8
        let s_2_14: u8 = (s_2_13.value() as u8);
        // D s_2_15: write-var state_bits <= s_2_14
        fn_state.state_bits = s_2_14;
        // N s_2_16: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #3u : u32
        let s_3_0: u32 = 3;
        // D s_3_1: read-var owning_ss:u32
        let s_3_1: u32 = fn_state.owning_ss;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b16 b4
        if s_3_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var state_bits:u8
        let s_4_0: u8 = fn_state.state_bits;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #0u : u8
        let s_4_2: u8 = 0;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: write-var state_match <= s_4_4
        fn_state.state_match = s_4_4;
        // N s_4_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var owning_el:u8
        let s_5_0: u8 = fn_state.owning_el;
        // D s_5_1: call ELUsingAArch32(s_5_0)
        let s_5_1: bool = ELUsingAArch32(state, tracer, s_5_0);
        // D s_5_2: not s_5_1
        let s_5_2: bool = !s_5_1;
        // N s_5_3: branch s_5_2 b15 b6
        if s_5_2 {
            return block_15(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#3595 <= s_6_0
        fn_state.gs_3595 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var gs#3595:u8
        let s_7_0: bool = fn_state.gs_3595;
        // N s_7_1: branch s_7_0 b14 b8
        if s_7_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#3596 <= s_8_0
        fn_state.gs_3596 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var gs#3596:u8
        let s_9_0: bool = fn_state.gs_3596;
        // N s_9_1: branch s_9_0 b13 b10
        if s_9_0 {
            return block_13(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#3597 <= s_10_0
        fn_state.gs_3597 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var gs#3597:u8
        let s_11_0: bool = fn_state.gs_3597;
        // D s_11_1: write-var return_value <= s_11_0
        fn_state.return_value = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var return_value:u8
        let s_12_0: bool = fn_state.return_value;
        // N s_12_1: return s_12_0
        return s_12_0;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #13704u : u32
        let s_13_0: u32 = 13704;
        // D s_13_1: read-reg s_13_0:struct
        let s_13_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call _get_PMBSR_EL1_Type_S(s_13_1)
        let s_13_2: bool = u_get_PMBSR_EL1_Type_S(state, tracer, s_13_1);
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // C s_13_4: const #0u : u8
        let s_13_4: bool = false;
        // C s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 1u16);
        // D s_13_6: cmp-eq s_13_3 s_13_5
        let s_13_6: bool = ((s_13_3) == (s_13_5));
        // D s_13_7: write-var gs#3597 <= s_13_6
        fn_state.gs_3597 = s_13_6;
        // N s_13_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #20480u : u32
        let s_14_0: u32 = 20480;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_PMBLIMITR_EL1_Type_E(s_14_1)
        let s_14_2: bool = u_get_PMBLIMITR_EL1_Type_E(state, tracer, s_14_1);
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // C s_14_4: const #1u : u8
        let s_14_4: bool = true;
        // C s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // D s_14_6: cmp-eq s_14_3 s_14_5
        let s_14_6: bool = ((s_14_3) == (s_14_5));
        // D s_14_7: write-var gs#3596 <= s_14_6
        fn_state.gs_3596 = s_14_6;
        // N s_14_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var state_match:u8
        let s_15_0: bool = fn_state.state_match;
        // D s_15_1: write-var gs#3595 <= s_15_0
        fn_state.gs_3595 = s_15_0;
        // N s_15_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #0u : u32
        let s_16_0: u32 = 0;
        // D s_16_1: read-var owning_ss:u32
        let s_16_1: u32 = fn_state.owning_ss;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // N s_16_4: branch s_16_3 b18 b17
        if s_16_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var state_bits:u8
        let s_17_0: u8 = fn_state.state_bits;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #1u : u8
        let s_17_2: u8 = 1;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 2u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: write-var state_match <= s_17_4
        fn_state.state_match = s_17_4;
        // N s_17_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #2u : u32
        let s_18_0: u32 = 2;
        // D s_18_1: read-var owning_ss:u32
        let s_18_1: u32 = fn_state.owning_ss;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // D s_18_3: not s_18_2
        let s_18_3: bool = !s_18_2;
        // N s_18_4: branch s_18_3 b20 b19
        if s_18_3 {
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
        // D s_19_0: read-var state_bits:u8
        let s_19_0: u8 = fn_state.state_bits;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 2u16);
        // C s_19_2: const #3u : u8
        let s_19_2: u8 = 3;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 2u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: write-var state_match <= s_19_4
        fn_state.state_match = s_19_4;
        // N s_19_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_20_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #90704u : u32
        let s_21_0: u32 = 90704;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_SCR_EL3_Type_NSE(s_21_1)
        let s_21_2: bool = u_get_SCR_EL3_Type_NSE(state, tracer, s_21_1);
        // C s_21_3: const #() : ()
        let s_21_3: () = ();
        // S s_21_4: call EffectiveSCR_EL3_NS(s_21_3)
        let s_21_4: bool = EffectiveSCR_EL3_NS(state, tracer, s_21_3);
        // D s_21_5: cast zx s_21_2 -> bv
        let s_21_5: Bits = Bits::new(s_21_2 as u128, 1u16);
        // S s_21_6: cast zx s_21_4 -> bv
        let s_21_6: Bits = Bits::new(s_21_4 as u128, 1u16);
        // D s_21_7: cast reint s_21_5 -> u128
        let s_21_7: u128 = (s_21_5.value() as u128);
        // D s_21_8: size-of s_21_5
        let s_21_8: u16 = s_21_5.length();
        // S s_21_9: cast reint s_21_6 -> u128
        let s_21_9: u128 = (s_21_6.value() as u128);
        // D s_21_10: size-of s_21_6
        let s_21_10: u16 = s_21_6.length();
        // D s_21_11: lsl s_21_7 s_21_10
        let s_21_11: u128 = s_21_7 << s_21_10;
        // D s_21_12: or s_21_11 s_21_9
        let s_21_12: u128 = ((s_21_11) | (s_21_9));
        // D s_21_13: add s_21_8 s_21_10
        let s_21_13: u16 = (s_21_8 + s_21_10);
        // D s_21_14: create-bits s_21_12 s_21_13
        let s_21_14: Bits = Bits::new(s_21_12, s_21_13);
        // D s_21_15: cast reint s_21_14 -> u8
        let s_21_15: u8 = (s_21_14.value() as u8);
        // D s_21_16: write-var state_bits <= s_21_15
        fn_state.state_bits = s_21_15;
        // N s_21_17: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var return_value <= s_22_0
        fn_state.return_value = s_22_0;
        // N s_22_2: jump b12
        return block_12(state, tracer, fn_state);
    }
}
