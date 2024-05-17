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
use u_get_HCR_EL2_Type_TGE::*;
use ProfilingBufferEnabled::*;
use u_get_PMSCR_EL1_Type_E1SPE::*;
use u_get_PMSCR_EL2_Type_E0HSPE::*;
use ProfilingBufferOwner::*;
use UsingAArch32::*;
use SecurityStateAtEL::*;
use HaveStatisticalProfiling::*;
use EL2Enabled::*;
use u_get_PMSCR_EL2_Type_E2SPE::*;
use u_get_PMSCR_EL1_Type_E0SPE::*;
use Unreachable::*;
use common::*;
pub fn StatisticalProfilingEnabled__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        tge_set: bool,
        gs_3608: bool,
        owning_ss: u32,
        return_value: bool,
        gs_3606: bool,
        owning_el: u8,
        gs_3599: bool,
        gs_3602: bool,
        gs_3600: bool,
        spe_bit: bool,
        gs_3607: bool,
        ga_2181: ProductTypec8897aad3eb4a29e,
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
        // S s_0_1: call HaveStatisticalProfiling(s_0_0)
        let s_0_1: bool = HaveStatisticalProfiling(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b35 b1
        if s_0_2 {
            return block_35(state, tracer, fn_state);
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
        // S s_1_1: call UsingAArch32(s_1_0)
        let s_1_1: bool = UsingAArch32(state, tracer, s_1_0);
        // D s_1_2: write-var gs#3599 <= s_1_1
        fn_state.gs_3599 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#3599:u8
        let s_2_0: bool = fn_state.gs_3599;
        // N s_2_1: branch s_2_0 b34 b3
        if s_2_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call ProfilingBufferEnabled(s_3_0)
        let s_3_1: bool = ProfilingBufferEnabled(state, tracer, s_3_0);
        // S s_3_2: not s_3_1
        let s_3_2: bool = !s_3_1;
        // D s_3_3: write-var gs#3600 <= s_3_2
        fn_state.gs_3600 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#3600:u8
        let s_4_0: bool = fn_state.gs_3600;
        // N s_4_1: branch s_4_0 b33 b5
        if s_4_0 {
            return block_33(state, tracer, fn_state);
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
        // S s_5_1: call EL2Enabled(s_5_0)
        let s_5_1: bool = EL2Enabled(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b32 b6
        if s_5_1 {
            return block_32(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#3602 <= s_6_0
        fn_state.gs_3602 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var gs#3602:u8
        let s_7_0: bool = fn_state.gs_3602;
        // D s_7_1: write-var tge_set <= s_7_0
        fn_state.tge_set = s_7_0;
        // C s_7_2: const #() : ()
        let s_7_2: () = ();
        // S s_7_3: call ProfilingBufferOwner(s_7_2)
        let s_7_3: ProductTypec8897aad3eb4a29e = ProfilingBufferOwner(
            state,
            tracer,
            s_7_2,
        );
        // D s_7_4: write-var ga#2181 <= s_7_3
        fn_state.ga_2181 = s_7_3;
        // D s_7_5: read-var ga#2181.0:struct
        let s_7_5: u32 = fn_state.ga_2181._0;
        // D s_7_6: read-var ga#2181.1:struct
        let s_7_6: u8 = fn_state.ga_2181._1;
        // D s_7_7: write-var owning_ss <= s_7_5
        fn_state.owning_ss = s_7_5;
        // D s_7_8: write-var owning_el <= s_7_6
        fn_state.owning_el = s_7_6;
        // D s_7_9: read-var owning_el:u8
        let s_7_9: u8 = fn_state.owning_el;
        // D s_7_10: cast zx s_7_9 -> bv
        let s_7_10: Bits = Bits::new(s_7_9 as u128, 2u16);
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (s_7_10.value() as i128);
        // D s_7_12: cast reint s_7_11 -> i64
        let s_7_12: i64 = (s_7_11 as i64);
        // D s_7_13: read-var el:u8
        let s_7_13: u8 = fn_state.el;
        // D s_7_14: cast zx s_7_13 -> bv
        let s_7_14: Bits = Bits::new(s_7_13 as u128, 2u16);
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (s_7_14.value() as i128);
        // D s_7_16: cast reint s_7_15 -> i64
        let s_7_16: i64 = (s_7_15 as i64);
        // D s_7_17: cast zx s_7_12 -> i
        let s_7_17: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_18: cast zx s_7_16 -> i
        let s_7_18: i128 = (i128::try_from(s_7_16).unwrap());
        // D s_7_19: cmp-lt s_7_17 s_7_18
        let s_7_19: bool = ((s_7_17) < (s_7_18));
        // N s_7_20: branch s_7_19 b31 b8
        if s_7_19 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var tge_set:u8
        let s_8_0: bool = fn_state.tge_set;
        // N s_8_1: branch s_8_0 b30 b9
        if s_8_0 {
            return block_30(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#3606 <= s_9_0
        fn_state.gs_3606 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var gs#3606:u8
        let s_10_0: bool = fn_state.gs_3606;
        // D s_10_1: write-var gs#3607 <= s_10_0
        fn_state.gs_3607 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var gs#3607:u8
        let s_11_0: bool = fn_state.gs_3607;
        // N s_11_1: branch s_11_0 b29 b12
        if s_11_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var el:u8
        let s_12_0: u8 = fn_state.el;
        // D s_12_1: call SecurityStateAtEL(s_12_0)
        let s_12_1: u32 = SecurityStateAtEL(state, tracer, s_12_0);
        // D s_12_2: read-var owning_ss:u32
        let s_12_2: u32 = fn_state.owning_ss;
        // D s_12_3: cmp-eq s_12_2 s_12_1
        let s_12_3: bool = ((s_12_2) == (s_12_1));
        // D s_12_4: write-var gs#3608 <= s_12_3
        fn_state.gs_3608 = s_12_3;
        // N s_12_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var gs#3608:u8
        let s_13_0: bool = fn_state.gs_3608;
        // N s_13_1: branch s_13_0 b28 b14
        if s_13_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
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
        // C s_14_2: const #424u : u32
        let s_14_2: u32 = 424;
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
        // N s_14_7: branch s_14_6 b18 b15
        if s_14_6 {
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
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call Unreachable(s_15_0)
        let s_15_1: () = Unreachable(state, tracer, s_15_0);
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var spe_bit:u8
        let s_16_0: bool = fn_state.spe_bit;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 1u16);
        // C s_16_2: const #1u : u8
        let s_16_2: bool = true;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: write-var return_value <= s_16_4
        fn_state.return_value = s_16_4;
        // N s_16_6: jump b17
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
        // D s_18_0: read-var el:u8
        let s_18_0: u8 = fn_state.el;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 2u16);
        // C s_18_2: const #432u : u32
        let s_18_2: u32 = 432;
        // D s_18_3: read-reg s_18_2:u8
        let s_18_3: u8 = {
            let value = state.read_register::<u8>(s_18_2 as isize);
            tracer.read_register(s_18_2 as isize, value);
            value
        };
        // D s_18_4: cast zx s_18_3 -> bv
        let s_18_4: Bits = Bits::new(s_18_3 as u128, 2u16);
        // D s_18_5: cmp-eq s_18_1 s_18_4
        let s_18_5: bool = ((s_18_1) == (s_18_4));
        // D s_18_6: not s_18_5
        let s_18_6: bool = !s_18_5;
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
        // C s_19_0: const #104928u : u32
        let s_19_0: u32 = 104928;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_PMSCR_EL2_Type_E2SPE(s_19_1)
        let s_19_2: bool = u_get_PMSCR_EL2_Type_E2SPE(state, tracer, s_19_1);
        // D s_19_3: write-var spe_bit <= s_19_2
        fn_state.spe_bit = s_19_2;
        // N s_19_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var el:u8
        let s_20_0: u8 = fn_state.el;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 2u16);
        // C s_20_2: const #440u : u32
        let s_20_2: u32 = 440;
        // D s_20_3: read-reg s_20_2:u8
        let s_20_3: u8 = {
            let value = state.read_register::<u8>(s_20_2 as isize);
            tracer.read_register(s_20_2 as isize, value);
            value
        };
        // D s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 2u16);
        // D s_20_5: cmp-eq s_20_1 s_20_4
        let s_20_5: bool = ((s_20_1) == (s_20_4));
        // D s_20_6: not s_20_5
        let s_20_6: bool = !s_20_5;
        // N s_20_7: branch s_20_6 b22 b21
        if s_20_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #21072u : u32
        let s_21_0: u32 = 21072;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_PMSCR_EL1_Type_E1SPE(s_21_1)
        let s_21_2: bool = u_get_PMSCR_EL1_Type_E1SPE(state, tracer, s_21_1);
        // D s_21_3: write-var spe_bit <= s_21_2
        fn_state.spe_bit = s_21_2;
        // N s_21_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var el:u8
        let s_22_0: u8 = fn_state.el;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 2u16);
        // C s_22_2: const #448u : u32
        let s_22_2: u32 = 448;
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
        // N s_22_7: branch s_22_6 b27 b23
        if s_22_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var tge_set:u8
        let s_23_0: bool = fn_state.tge_set;
        // N s_23_1: branch s_23_0 b26 b24
        if s_23_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #21072u : u32
        let s_24_0: u32 = 21072;
        // D s_24_1: read-reg s_24_0:struct
        let s_24_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call _get_PMSCR_EL1_Type_E0SPE(s_24_1)
        let s_24_2: bool = u_get_PMSCR_EL1_Type_E0SPE(state, tracer, s_24_1);
        // D s_24_3: write-var spe_bit <= s_24_2
        fn_state.spe_bit = s_24_2;
        // N s_24_4: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_25_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #104928u : u32
        let s_26_0: u32 = 104928;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_PMSCR_EL2_Type_E0HSPE(s_26_1)
        let s_26_2: bool = u_get_PMSCR_EL2_Type_E0HSPE(state, tracer, s_26_1);
        // D s_26_3: write-var spe_bit <= s_26_2
        fn_state.spe_bit = s_26_2;
        // N s_26_4: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_27_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var return_value <= s_28_0
        fn_state.return_value = s_28_0;
        // N s_28_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#3608 <= s_29_0
        fn_state.gs_3608 = s_29_0;
        // N s_29_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var owning_el:u8
        let s_30_0: u8 = fn_state.owning_el;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 2u16);
        // C s_30_2: const #440u : u32
        let s_30_2: u32 = 440;
        // D s_30_3: read-reg s_30_2:u8
        let s_30_3: u8 = {
            let value = state.read_register::<u8>(s_30_2 as isize);
            tracer.read_register(s_30_2 as isize, value);
            value
        };
        // D s_30_4: cast zx s_30_3 -> bv
        let s_30_4: Bits = Bits::new(s_30_3 as u128, 2u16);
        // D s_30_5: cmp-eq s_30_1 s_30_4
        let s_30_5: bool = ((s_30_1) == (s_30_4));
        // D s_30_6: write-var gs#3606 <= s_30_5
        fn_state.gs_3606 = s_30_5;
        // N s_30_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#3607 <= s_31_0
        fn_state.gs_3607 = s_31_0;
        // N s_31_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_32_0: const #102552u : u32
        let s_32_0: u32 = 102552;
        // D s_32_1: read-reg s_32_0:struct
        let s_32_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: call _get_HCR_EL2_Type_TGE(s_32_1)
        let s_32_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_32_1);
        // D s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // C s_32_4: const #1u : u8
        let s_32_4: bool = true;
        // C s_32_5: cast zx s_32_4 -> bv
        let s_32_5: Bits = Bits::new(s_32_4 as u128, 1u16);
        // D s_32_6: cmp-eq s_32_3 s_32_5
        let s_32_6: bool = ((s_32_3) == (s_32_5));
        // D s_32_7: write-var gs#3602 <= s_32_6
        fn_state.gs_3602 = s_32_6;
        // N s_32_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var return_value <= s_33_0
        fn_state.return_value = s_33_0;
        // N s_33_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#3600 <= s_34_0
        fn_state.gs_3600 = s_34_0;
        // N s_34_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#3599 <= s_35_0
        fn_state.gs_3599 = s_35_0;
        // N s_35_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
