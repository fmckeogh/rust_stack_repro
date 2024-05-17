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
use GetNumEventCounters::*;
use u_get_MDCR_EL2_Type_HPMD::*;
use Halted::*;
use HavePMUv3p5::*;
use HaveRME::*;
use PMEVTYPER_read::*;
use u_get_SDER_Type_SUNIDEN::*;
use u_get_MDCR_EL2_Type_HCCD::*;
use u_get_MDCR_EL2_Type_HPME::*;
use u_get_HDCR_Type_HPME::*;
use PMUCounterIsHyp::*;
use u_get_MDCR_EL3_Type_SPME::*;
use u_get_PMCR_Type_E::*;
use HaveHPMDExt::*;
use HaveNoSecurePMUDisableOverride::*;
use PMCNTENSET_read::*;
use u_get_HDCR_Type_HPMFZO::*;
use u_get_PMCNTENSET_EL0_Type_F0::*;
use u_get_MDCR_EL2_Type_HPMFZO::*;
use u_get_SDER32_EL3_Type_SUNIDEN::*;
use SDER_read::*;
use u_get_PMCR_Type_DP::*;
use PMCCFILTR_read::*;
use HaveSecureEL2Ext::*;
use u_get_PMCR_EL0_Type_DP::*;
use HDCR_read::*;
use u_get_PMCR_EL0_Type_FZO::*;
use u_get_HDCR_Type_HPMD::*;
use CurrentSecurityState::*;
use u_get_MDCR_EL3_Type_MCCD::*;
use HavePMUv3p7::*;
use SDER32_EL3_read::*;
use u_get_PMCR_EL0_Type_E::*;
use HiLoPMUOverflow::*;
use HavePMUv3ICNTR::*;
use u__id::*;
use u_get_PMCR_Type_FZO::*;
use u_get_MDCR_EL3_Type_SCCD::*;
use PMCR_read::*;
use ELUsingAArch32::*;
use u_get_SDCR_Type_SPME::*;
use HaveAArch64::*;
use u_get_HDCR_Type_HCCD::*;
use ExternalSecureNoninvasiveDebugEnabled::*;
use u_get_MDCR_EL3_Type_MPMX::*;
use u_get_PMCNTENSET_Type_C::*;
use u_get_PMCNTENSET_EL0_Type_C::*;
use u_get_SDCR_Type_SCCD::*;
use common::*;
pub fn CountPMUEvents<T: Tracer>(state: &mut State, tracer: &T, idx: i64) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_2596: bool,
        prohibited: bool,
        RLU: bool,
        E: bool,
        gs_2542: bool,
        gs_2471: bool,
        gs_2532: bool,
        gs_2517: bool,
        gs_2480: bool,
        SH: bool,
        M: bool,
        RLK: bool,
        gs_2598: bool,
        gs_2495: bool,
        gs_2511: bool,
        gs_2491: bool,
        gs_2507: bool,
        gs_2498: bool,
        gs_2518: bool,
        gs_2493: bool,
        gs_2527: bool,
        ga_1477: ProductType5c790c8ef59cc8b2,
        enabled: bool,
        gs_2473: bool,
        NSH: bool,
        gs_2513: bool,
        ss: u32,
        frozen: bool,
        gs_2521: bool,
        filter: u32,
        resvd_for_el2: bool,
        gs_2494: bool,
        RLH: bool,
        num_counters: i128,
        gs_2599: bool,
        gs_2499: bool,
        FZ: bool,
        dpshadow_23: bool,
        gs_2504: bool,
        gs_2515: bool,
        gs_2489: bool,
        gs_2510: bool,
        gs_2514: bool,
        gs_2496: bool,
        P: bool,
        hccdshadow_25: bool,
        gs_2562: bool,
        sccdshadow_24: bool,
        filtered: bool,
        gs_2497: bool,
        ga_1483: ProductType700c18a878c5601b,
        gs_2500: bool,
        NSU: bool,
        U: bool,
        gs_2477: bool,
        gs_2512: bool,
        NSK: bool,
        debug: bool,
        filtershadow_21: u32,
        ga_1484: u8,
        ga_1399: ProductType700c18a878c5601b,
        gs_2565: bool,
        hpmdshadow_26: bool,
        ga_1471: ProductType700c18a878c5601b,
        gs_2546: bool,
        ssshadow_22: u32,
        gs_2472: bool,
        gs_2597: bool,
        gs_2505: bool,
        gs_2486: bool,
        gs_2481: bool,
        idx: i64,
    }
    let fn_state = FunctionState {
        idx,
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
        // S s_0_1: call GetNumEventCounters(s_0_0)
        let s_0_1: i128 = GetNumEventCounters(state, tracer, s_0_0);
        // D s_0_2: write-var num_counters <= s_0_1
        fn_state.num_counters = s_0_1;
        // D s_0_3: read-var idx:i64
        let s_0_3: i64 = fn_state.idx;
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #0u : u32
        let s_0_5: u32 = 0;
        // D s_0_6: read-reg s_0_5:i64
        let s_0_6: i64 = {
            let value = state.read_register::<i64>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (i128::try_from(s_0_6).unwrap());
        // D s_0_8: cmp-eq s_0_4 s_0_7
        let s_0_8: bool = ((s_0_4) == (s_0_7));
        // N s_0_9: branch s_0_8 b272 b1
        if s_0_8 {
            return block_272(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var idx:i64
        let s_1_0: i64 = fn_state.idx;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: read-var num_counters:i
        let s_1_2: i128 = fn_state.num_counters;
        // D s_1_3: cmp-lt s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) < (s_1_2));
        // D s_1_4: write-var gs#2471 <= s_1_3
        fn_state.gs_2471 = s_1_3;
        // N s_1_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#2471:u8
        let s_2_0: bool = fn_state.gs_2471;
        // N s_2_1: branch s_2_0 b271 b3
        if s_2_0 {
            return block_271(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var idx:i64
        let s_3_0: i64 = fn_state.idx;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // C s_3_2: const #8u : u32
        let s_3_2: u32 = 8;
        // D s_3_3: read-reg s_3_2:i64
        let s_3_3: i64 = {
            let value = state.read_register::<i64>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // N s_3_6: branch s_3_5 b270 b4
        if s_3_5 {
            return block_270(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#2472 <= s_4_0
        fn_state.gs_2472 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#2472:u8
        let s_5_0: bool = fn_state.gs_2472;
        // D s_5_1: write-var gs#2473 <= s_5_0
        fn_state.gs_2473 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#2473:u8
        let s_6_0: bool = fn_state.gs_2473;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // C s_6_2: const #() : ()
        let s_6_2: () = ();
        // S s_6_3: call Halted(s_6_2)
        let s_6_3: bool = Halted(state, tracer, s_6_2);
        // D s_6_4: write-var debug <= s_6_3
        fn_state.debug = s_6_3;
        // D s_6_5: read-var idx:i64
        let s_6_5: i64 = fn_state.idx;
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: call PMUCounterIsHyp(s_6_6)
        let s_6_7: bool = PMUCounterIsHyp(state, tracer, s_6_6);
        // D s_6_8: write-var resvd_for_el2 <= s_6_7
        fn_state.resvd_for_el2 = s_6_7;
        // C s_6_9: const #() : ()
        let s_6_9: () = ();
        // S s_6_10: call CurrentSecurityState(s_6_9)
        let s_6_10: u32 = CurrentSecurityState(state, tracer, s_6_9);
        // D s_6_11: write-var ss <= s_6_10
        fn_state.ss = s_6_10;
        // D s_6_12: read-var idx:i64
        let s_6_12: i64 = fn_state.idx;
        // D s_6_13: cast zx s_6_12 -> i
        let s_6_13: i128 = (i128::try_from(s_6_12).unwrap());
        // C s_6_14: const #8u : u32
        let s_6_14: u32 = 8;
        // D s_6_15: read-reg s_6_14:i64
        let s_6_15: i64 = {
            let value = state.read_register::<i64>(s_6_14 as isize);
            tracer.read_register(s_6_14 as isize, value);
            value
        };
        // D s_6_16: cast zx s_6_15 -> i
        let s_6_16: i128 = (i128::try_from(s_6_15).unwrap());
        // D s_6_17: cmp-eq s_6_13 s_6_16
        let s_6_17: bool = ((s_6_13) == (s_6_16));
        // D s_6_18: not s_6_17
        let s_6_18: bool = !s_6_17;
        // N s_6_19: branch s_6_18 b239 b7
        if s_6_18 {
            return block_239(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call HaveAArch64(s_7_0)
        let s_7_1: bool = HaveAArch64(state, tracer, s_7_0);
        // N s_7_2: assert s_7_1
        let s_7_2: () = assert!(s_7_1);
        // C s_7_3: const #21016u : u32
        let s_7_3: u32 = 21016;
        // D s_7_4: read-reg s_7_3:struct
        let s_7_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_3 as isize);
            tracer.read_register(s_7_3 as isize, value);
            value
        };
        // D s_7_5: call _get_PMCR_EL0_Type_E(s_7_4)
        let s_7_5: bool = u_get_PMCR_EL0_Type_E(state, tracer, s_7_4);
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 1u16);
        // C s_7_7: const #1u : u8
        let s_7_7: bool = true;
        // C s_7_8: cast zx s_7_7 -> bv
        let s_7_8: Bits = Bits::new(s_7_7 as u128, 1u16);
        // D s_7_9: cmp-eq s_7_6 s_7_8
        let s_7_9: bool = ((s_7_6) == (s_7_8));
        // N s_7_10: branch s_7_9 b238 b8
        if s_7_9 {
            return block_238(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#2477 <= s_8_0
        fn_state.gs_2477 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var gs#2477:u8
        let s_9_0: bool = fn_state.gs_2477;
        // D s_9_1: write-var enabled <= s_9_0
        fn_state.enabled = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var prohibited <= s_10_0
        fn_state.prohibited = s_10_0;
        // C s_10_2: const #424u : u32
        let s_10_2: u32 = 424;
        // D s_10_3: read-reg s_10_2:u8
        let s_10_3: u8 = {
            let value = state.read_register::<u8>(s_10_2 as isize);
            tracer.read_register(s_10_2 as isize, value);
            value
        };
        // C s_10_4: const #2u : u8
        let s_10_4: u8 = 2;
        // D s_10_5: cmp-lt s_10_3 s_10_4
        let s_10_5: bool = ((s_10_3) < (s_10_4));
        // N s_10_6: branch s_10_5 b237 b11
        if s_10_5 {
            return block_237(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#2493 <= s_11_0
        fn_state.gs_2493 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var gs#2493:u8
        let s_12_0: bool = fn_state.gs_2493;
        // N s_12_1: branch s_12_0 b217 b13
        if s_12_0 {
            return block_217(state, tracer, fn_state);
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
        // D s_14_0: read-var prohibited:u8
        let s_14_0: bool = fn_state.prohibited;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // N s_14_2: branch s_14_1 b216 b15
        if s_14_1 {
            return block_216(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#2494 <= s_15_0
        fn_state.gs_2494 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var gs#2494:u8
        let s_16_0: bool = fn_state.gs_2494;
        // N s_16_1: branch s_16_0 b215 b17
        if s_16_0 {
            return block_215(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#2495 <= s_17_0
        fn_state.gs_2495 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var gs#2495:u8
        let s_18_0: bool = fn_state.gs_2495;
        // N s_18_1: branch s_18_0 b214 b19
        if s_18_0 {
            return block_214(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#2496 <= s_19_0
        fn_state.gs_2496 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var gs#2496:u8
        let s_20_0: bool = fn_state.gs_2496;
        // N s_20_1: branch s_20_0 b207 b21
        if s_20_0 {
            return block_207(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var prohibited:u8
        let s_22_0: bool = fn_state.prohibited;
        // D s_22_1: not s_22_0
        let s_22_1: bool = !s_22_0;
        // N s_22_2: branch s_22_1 b206 b23
        if s_22_1 {
            return block_206(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#2497 <= s_23_0
        fn_state.gs_2497 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var gs#2497:u8
        let s_24_0: bool = fn_state.gs_2497;
        // N s_24_1: branch s_24_0 b205 b25
        if s_24_0 {
            return block_205(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#2498 <= s_25_0
        fn_state.gs_2498 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var gs#2498:u8
        let s_26_0: bool = fn_state.gs_2498;
        // N s_26_1: branch s_26_0 b204 b27
        if s_26_0 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#2499 <= s_27_0
        fn_state.gs_2499 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_28_0: read-var gs#2499:u8
        let s_28_0: bool = fn_state.gs_2499;
        // N s_28_1: branch s_28_0 b200 b29
        if s_28_0 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var prohibited:u8
        let s_30_0: bool = fn_state.prohibited;
        // N s_30_1: branch s_30_0 b199 b31
        if s_30_0 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#2500 <= s_31_0
        fn_state.gs_2500 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_32_0: read-var gs#2500:u8
        let s_32_0: bool = fn_state.gs_2500;
        // N s_32_1: branch s_32_0 b198 b33
        if s_32_0 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var frozen <= s_34_0
        fn_state.frozen = s_34_0;
        // C s_34_2: const #() : ()
        let s_34_2: () = ();
        // S s_34_3: call HavePMUv3p7(s_34_2)
        let s_34_3: bool = HavePMUv3p7(state, tracer, s_34_2);
        // N s_34_4: branch s_34_3 b185 b35
        if s_34_3 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_35_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_36_0: read-var prohibited:u8
        let s_36_0: bool = fn_state.prohibited;
        // N s_36_1: branch s_36_0 b184 b37
        if s_36_0 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_37_0: read-var frozen:u8
        let s_37_0: bool = fn_state.frozen;
        // D s_37_1: write-var gs#2512 <= s_37_0
        fn_state.gs_2512 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_38_0: read-var gs#2512:u8
        let s_38_0: bool = fn_state.gs_2512;
        // N s_38_1: branch s_38_0 b183 b39
        if s_38_0 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#2513 <= s_39_0
        fn_state.gs_2513 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_40_0: read-var gs#2513:u8
        let s_40_0: bool = fn_state.gs_2513;
        // N s_40_1: branch s_40_0 b176 b41
        if s_40_0 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_41_0: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call HavePMUv3p5(s_42_0)
        let s_42_1: bool = HavePMUv3p5(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b175 b43
        if s_42_1 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#2514 <= s_43_0
        fn_state.gs_2514 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_44_0: read-var gs#2514:u8
        let s_44_0: bool = fn_state.gs_2514;
        // N s_44_1: branch s_44_0 b153 b45
        if s_44_0 {
            return block_153(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_45_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call HavePMUv3p7(s_46_0)
        let s_46_1: bool = HavePMUv3p7(state, tracer, s_46_0);
        // N s_46_2: branch s_46_1 b152 b47
        if s_46_1 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#2515 <= s_47_0
        fn_state.gs_2515 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_48_0: read-var gs#2515:u8
        let s_48_0: bool = fn_state.gs_2515;
        // N s_48_1: branch s_48_0 b142 b49
        if s_48_0 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_49_0: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_50_0: read-var idx:i64
        let s_50_0: i64 = fn_state.idx;
        // D s_50_1: cast zx s_50_0 -> i
        let s_50_1: i128 = (i128::try_from(s_50_0).unwrap());
        // C s_50_2: const #8u : u32
        let s_50_2: u32 = 8;
        // D s_50_3: read-reg s_50_2:i64
        let s_50_3: i64 = {
            let value = state.read_register::<i64>(s_50_2 as isize);
            tracer.read_register(s_50_2 as isize, value);
            value
        };
        // D s_50_4: cast zx s_50_3 -> i
        let s_50_4: i128 = (i128::try_from(s_50_3).unwrap());
        // D s_50_5: cmp-eq s_50_1 s_50_4
        let s_50_5: bool = ((s_50_1) == (s_50_4));
        // D s_50_6: not s_50_5
        let s_50_6: bool = !s_50_5;
        // N s_50_7: branch s_50_6 b127 b51
        if s_50_6 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_51_0: const #104496u : u32
        let s_51_0: u32 = 104496;
        // D s_51_1: read-reg s_51_0:u64
        let s_51_1: u64 = {
            let value = state.read_register::<u64>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // C s_51_2: const #0s : i
        let s_51_2: i128 = 0;
        // D s_51_3: cast zx s_51_1 -> bv
        let s_51_3: Bits = Bits::new(s_51_1 as u128, 64u16);
        // C s_51_4: const #1s : i64
        let s_51_4: i64 = 1;
        // C s_51_5: cast zx s_51_4 -> i
        let s_51_5: i128 = (i128::try_from(s_51_4).unwrap());
        // C s_51_6: const #31s : i
        let s_51_6: i128 = 31;
        // C s_51_7: add s_51_6 s_51_5
        let s_51_7: i128 = (s_51_6 + s_51_5);
        // D s_51_8: bit-extract s_51_3 s_51_2 s_51_7
        let s_51_8: Bits = (Bits::new(
            ((s_51_3) >> (s_51_2)).value(),
            u16::try_from(s_51_7).unwrap(),
        ));
        // D s_51_9: cast reint s_51_8 -> u32
        let s_51_9: u32 = (s_51_8.value() as u32);
        // D s_51_10: write-var filter <= s_51_9
        fn_state.filter = s_51_9;
        // N s_51_11: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_52_0: read-var filter:u32
        let s_52_0: u32 = fn_state.filter;
        // D s_52_1: write-var filtershadow#21 <= s_52_0
        fn_state.filtershadow_21 = s_52_0;
        // C s_52_2: const #31s : i
        let s_52_2: i128 = 31;
        // D s_52_3: read-var filtershadow#21:u32
        let s_52_3: u32 = fn_state.filtershadow_21;
        // D s_52_4: cast zx s_52_3 -> bv
        let s_52_4: Bits = Bits::new(s_52_3 as u128, 32u16);
        // C s_52_5: const #1u : u64
        let s_52_5: u64 = 1;
        // D s_52_6: bit-extract s_52_4 s_52_2 s_52_5
        let s_52_6: Bits = (Bits::new(
            ((s_52_4) >> (s_52_2)).value(),
            u16::try_from(s_52_5).unwrap(),
        ));
        // D s_52_7: cast reint s_52_6 -> u8
        let s_52_7: bool = ((s_52_6.value()) != 0);
        // C s_52_8: const #0s : i
        let s_52_8: i128 = 0;
        // C s_52_9: const #0u : u64
        let s_52_9: u64 = 0;
        // D s_52_10: cast zx s_52_7 -> u64
        let s_52_10: u64 = (s_52_7 as u64);
        // C s_52_11: const #1u : u64
        let s_52_11: u64 = 1;
        // D s_52_12: and s_52_10 s_52_11
        let s_52_12: u64 = ((s_52_10) & (s_52_11));
        // D s_52_13: cmp-eq s_52_12 s_52_11
        let s_52_13: bool = ((s_52_12) == (s_52_11));
        // D s_52_14: lsl s_52_10 s_52_8
        let s_52_14: u64 = s_52_10 << s_52_8;
        // D s_52_15: or s_52_9 s_52_14
        let s_52_15: u64 = ((s_52_9) | (s_52_14));
        // D s_52_16: cmpl s_52_14
        let s_52_16: u64 = !s_52_14;
        // D s_52_17: and s_52_9 s_52_16
        let s_52_17: u64 = ((s_52_9) & (s_52_16));
        // D s_52_18: select s_52_13 s_52_15 s_52_17
        let s_52_18: u64 = if s_52_13 { s_52_15 } else { s_52_17 };
        // D s_52_19: cast trunc s_52_18 -> u8
        let s_52_19: bool = ((s_52_18) != 0);
        // D s_52_20: write-var P <= s_52_19
        fn_state.P = s_52_19;
        // C s_52_21: const #30s : i
        let s_52_21: i128 = 30;
        // D s_52_22: read-var filtershadow#21:u32
        let s_52_22: u32 = fn_state.filtershadow_21;
        // D s_52_23: cast zx s_52_22 -> bv
        let s_52_23: Bits = Bits::new(s_52_22 as u128, 32u16);
        // C s_52_24: const #1u : u64
        let s_52_24: u64 = 1;
        // D s_52_25: bit-extract s_52_23 s_52_21 s_52_24
        let s_52_25: Bits = (Bits::new(
            ((s_52_23) >> (s_52_21)).value(),
            u16::try_from(s_52_24).unwrap(),
        ));
        // D s_52_26: cast reint s_52_25 -> u8
        let s_52_26: bool = ((s_52_25.value()) != 0);
        // C s_52_27: const #0s : i
        let s_52_27: i128 = 0;
        // C s_52_28: const #0u : u64
        let s_52_28: u64 = 0;
        // D s_52_29: cast zx s_52_26 -> u64
        let s_52_29: u64 = (s_52_26 as u64);
        // C s_52_30: const #1u : u64
        let s_52_30: u64 = 1;
        // D s_52_31: and s_52_29 s_52_30
        let s_52_31: u64 = ((s_52_29) & (s_52_30));
        // D s_52_32: cmp-eq s_52_31 s_52_30
        let s_52_32: bool = ((s_52_31) == (s_52_30));
        // D s_52_33: lsl s_52_29 s_52_27
        let s_52_33: u64 = s_52_29 << s_52_27;
        // D s_52_34: or s_52_28 s_52_33
        let s_52_34: u64 = ((s_52_28) | (s_52_33));
        // D s_52_35: cmpl s_52_33
        let s_52_35: u64 = !s_52_33;
        // D s_52_36: and s_52_28 s_52_35
        let s_52_36: u64 = ((s_52_28) & (s_52_35));
        // D s_52_37: select s_52_32 s_52_34 s_52_36
        let s_52_37: u64 = if s_52_32 { s_52_34 } else { s_52_36 };
        // D s_52_38: cast trunc s_52_37 -> u8
        let s_52_38: bool = ((s_52_37) != 0);
        // D s_52_39: write-var U <= s_52_38
        fn_state.U = s_52_38;
        // C s_52_40: const #424u : u32
        let s_52_40: u32 = 424;
        // D s_52_41: read-reg s_52_40:u8
        let s_52_41: u8 = {
            let value = state.read_register::<u8>(s_52_40 as isize);
            tracer.read_register(s_52_40 as isize, value);
            value
        };
        // C s_52_42: const #2u : u8
        let s_52_42: u8 = 2;
        // D s_52_43: cmp-lt s_52_41 s_52_42
        let s_52_43: bool = ((s_52_41) < (s_52_42));
        // N s_52_44: branch s_52_43 b126 b53
        if s_52_43 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var NSK <= s_53_0
        fn_state.NSK = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_54_0: const #424u : u32
        let s_54_0: u32 = 424;
        // D s_54_1: read-reg s_54_0:u8
        let s_54_1: u8 = {
            let value = state.read_register::<u8>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // C s_54_2: const #2u : u8
        let s_54_2: u8 = 2;
        // D s_54_3: cmp-lt s_54_1 s_54_2
        let s_54_3: bool = ((s_54_1) < (s_54_2));
        // N s_54_4: branch s_54_3 b125 b55
        if s_54_3 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var NSU <= s_55_0
        fn_state.NSU = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_56_0: const #432u : u32
        let s_56_0: u32 = 432;
        // D s_56_1: read-reg s_56_0:u8
        let s_56_1: u8 = {
            let value = state.read_register::<u8>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // C s_56_2: const #2u : u8
        let s_56_2: u8 = 2;
        // D s_56_3: cmp-lt s_56_1 s_56_2
        let s_56_3: bool = ((s_56_1) < (s_56_2));
        // N s_56_4: branch s_56_3 b124 b57
        if s_56_3 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var NSH <= s_57_0
        fn_state.NSH = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_58_0: const #424u : u32
        let s_58_0: u32 = 424;
        // D s_58_1: read-reg s_58_0:u8
        let s_58_1: u8 = {
            let value = state.read_register::<u8>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // C s_58_2: const #2u : u8
        let s_58_2: u8 = 2;
        // D s_58_3: cmp-lt s_58_1 s_58_2
        let s_58_3: bool = ((s_58_1) < (s_58_2));
        // N s_58_4: branch s_58_3 b123 b59
        if s_58_3 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#2562 <= s_59_0
        fn_state.gs_2562 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_60_0: read-var gs#2562:u8
        let s_60_0: bool = fn_state.gs_2562;
        // N s_60_1: branch s_60_0 b122 b61
        if s_60_0 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var M <= s_61_0
        fn_state.M = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_62_0: const #424u : u32
        let s_62_0: u32 = 424;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // C s_62_2: const #2u : u8
        let s_62_2: u8 = 2;
        // D s_62_3: cmp-lt s_62_1 s_62_2
        let s_62_3: bool = ((s_62_1) < (s_62_2));
        // N s_62_4: branch s_62_3 b121 b63
        if s_62_3 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#2565 <= s_63_0
        fn_state.gs_2565 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_64_0: read-var gs#2565:u8
        let s_64_0: bool = fn_state.gs_2565;
        // N s_64_1: branch s_64_0 b120 b65
        if s_64_0 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var SH <= s_65_0
        fn_state.SH = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_66_0: const #() : ()
        let s_66_0: () = ();
        // S s_66_1: call HaveRME(s_66_0)
        let s_66_1: bool = HaveRME(state, tracer, s_66_0);
        // N s_66_2: branch s_66_1 b119 b67
        if s_66_1 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var RLK <= s_67_0
        fn_state.RLK = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_68_0: const #() : ()
        let s_68_0: () = ();
        // S s_68_1: call HaveRME(s_68_0)
        let s_68_1: bool = HaveRME(state, tracer, s_68_0);
        // N s_68_2: branch s_68_1 b118 b69
        if s_68_1 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var RLU <= s_69_0
        fn_state.RLU = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_70_0: const #() : ()
        let s_70_0: () = ();
        // S s_70_1: call HaveRME(s_70_0)
        let s_70_1: bool = HaveRME(state, tracer, s_70_0);
        // N s_70_2: branch s_70_1 b117 b71
        if s_70_1 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var RLH <= s_71_0
        fn_state.RLH = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call CurrentSecurityState(s_72_0)
        let s_72_1: u32 = CurrentSecurityState(state, tracer, s_72_0);
        // D s_72_2: write-var ssshadow#22 <= s_72_1
        fn_state.ssshadow_22 = s_72_1;
        // C s_72_3: const #16975u : u32
        let s_72_3: u32 = 16975;
        // D s_72_4: read-reg s_72_3:u8
        let s_72_4: u8 = {
            let value = state.read_register::<u8>(s_72_3 as isize);
            tracer.read_register(s_72_3 as isize, value);
            value
        };
        // D s_72_5: write-var ga#1484 <= s_72_4
        fn_state.ga_1484 = s_72_4;
        // D s_72_6: read-var ga#1484:u8
        let s_72_6: u8 = fn_state.ga_1484;
        // D s_72_7: cast zx s_72_6 -> bv
        let s_72_7: Bits = Bits::new(s_72_6 as u128, 2u16);
        // C s_72_8: const #448u : u32
        let s_72_8: u32 = 448;
        // D s_72_9: read-reg s_72_8:u8
        let s_72_9: u8 = {
            let value = state.read_register::<u8>(s_72_8 as isize);
            tracer.read_register(s_72_8 as isize, value);
            value
        };
        // D s_72_10: cast zx s_72_9 -> bv
        let s_72_10: Bits = Bits::new(s_72_9 as u128, 2u16);
        // D s_72_11: cmp-eq s_72_7 s_72_10
        let s_72_11: bool = ((s_72_7) == (s_72_10));
        // D s_72_12: not s_72_11
        let s_72_12: bool = !s_72_11;
        // N s_72_13: branch s_72_12 b94 b73
        if s_72_12 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_73_0: const #0u : u32
        let s_73_0: u32 = 0;
        // D s_73_1: read-var ssshadow#22:u32
        let s_73_1: u32 = fn_state.ssshadow_22;
        // D s_73_2: cmp-eq s_73_0 s_73_1
        let s_73_2: bool = ((s_73_0) == (s_73_1));
        // D s_73_3: not s_73_2
        let s_73_3: bool = !s_73_2;
        // N s_73_4: branch s_73_3 b89 b74
        if s_73_3 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_74_0: read-var U:u8
        let s_74_0: bool = fn_state.U;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 1u16);
        // D s_74_2: read-var NSU:u8
        let s_74_2: bool = fn_state.NSU;
        // D s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 1u16);
        // D s_74_4: cmp-ne s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) != (s_74_3));
        // D s_74_5: write-var filtered <= s_74_4
        fn_state.filtered = s_74_4;
        // N s_74_6: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_75_0: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_76_0: read-var debug:u8
        let s_76_0: bool = fn_state.debug;
        // D s_76_1: not s_76_0
        let s_76_1: bool = !s_76_0;
        // N s_76_2: branch s_76_1 b88 b77
        if s_76_1 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#2596 <= s_77_0
        fn_state.gs_2596 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_78_0: read-var gs#2596:u8
        let s_78_0: bool = fn_state.gs_2596;
        // N s_78_1: branch s_78_0 b87 b79
        if s_78_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_79_0: const #0u : u8
        let s_79_0: bool = false;
        // D s_79_1: write-var gs#2597 <= s_79_0
        fn_state.gs_2597 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_80_0: read-var gs#2597:u8
        let s_80_0: bool = fn_state.gs_2597;
        // N s_80_1: branch s_80_0 b86 b81
        if s_80_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#2598 <= s_81_0
        fn_state.gs_2598 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_82_0: read-var gs#2598:u8
        let s_82_0: bool = fn_state.gs_2598;
        // N s_82_1: branch s_82_0 b85 b83
        if s_82_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#2599 <= s_83_0
        fn_state.gs_2599 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_84_0: read-var gs#2599:u8
        let s_84_0: bool = fn_state.gs_2599;
        // N s_84_1: return s_84_0
        return s_84_0;
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_85_0: read-var frozen:u8
        let s_85_0: bool = fn_state.frozen;
        // D s_85_1: not s_85_0
        let s_85_1: bool = !s_85_0;
        // D s_85_2: write-var gs#2599 <= s_85_1
        fn_state.gs_2599 = s_85_1;
        // N s_85_3: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_86_0: read-var filtered:u8
        let s_86_0: bool = fn_state.filtered;
        // D s_86_1: not s_86_0
        let s_86_1: bool = !s_86_0;
        // D s_86_2: write-var gs#2598 <= s_86_1
        fn_state.gs_2598 = s_86_1;
        // N s_86_3: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_87_0: read-var prohibited:u8
        let s_87_0: bool = fn_state.prohibited;
        // D s_87_1: not s_87_0
        let s_87_1: bool = !s_87_0;
        // D s_87_2: write-var gs#2597 <= s_87_1
        fn_state.gs_2597 = s_87_1;
        // N s_87_3: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_88_0: read-var enabled:u8
        let s_88_0: bool = fn_state.enabled;
        // D s_88_1: write-var gs#2596 <= s_88_0
        fn_state.gs_2596 = s_88_0;
        // N s_88_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_89_0: const #3u : u32
        let s_89_0: u32 = 3;
        // D s_89_1: read-var ssshadow#22:u32
        let s_89_1: u32 = fn_state.ssshadow_22;
        // D s_89_2: cmp-eq s_89_0 s_89_1
        let s_89_2: bool = ((s_89_0) == (s_89_1));
        // D s_89_3: not s_89_2
        let s_89_3: bool = !s_89_2;
        // N s_89_4: branch s_89_3 b91 b90
        if s_89_3 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_90_0: read-var U:u8
        let s_90_0: bool = fn_state.U;
        // D s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 1u16);
        // C s_90_2: const #1u : u8
        let s_90_2: bool = true;
        // C s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // D s_90_4: cmp-eq s_90_1 s_90_3
        let s_90_4: bool = ((s_90_1) == (s_90_3));
        // D s_90_5: write-var filtered <= s_90_4
        fn_state.filtered = s_90_4;
        // N s_90_6: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_91_0: const #2u : u32
        let s_91_0: u32 = 2;
        // D s_91_1: read-var ssshadow#22:u32
        let s_91_1: u32 = fn_state.ssshadow_22;
        // D s_91_2: cmp-eq s_91_0 s_91_1
        let s_91_2: bool = ((s_91_0) == (s_91_1));
        // D s_91_3: not s_91_2
        let s_91_3: bool = !s_91_2;
        // N s_91_4: branch s_91_3 b93 b92
        if s_91_3 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_92_0: read-var U:u8
        let s_92_0: bool = fn_state.U;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // D s_92_2: read-var RLU:u8
        let s_92_2: bool = fn_state.RLU;
        // D s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-ne s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) != (s_92_3));
        // D s_92_5: write-var filtered <= s_92_4
        fn_state.filtered = s_92_4;
        // N s_92_6: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_93_0: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_94_0: read-var ga#1484:u8
        let s_94_0: u8 = fn_state.ga_1484;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 2u16);
        // C s_94_2: const #440u : u32
        let s_94_2: u32 = 440;
        // D s_94_3: read-reg s_94_2:u8
        let s_94_3: u8 = {
            let value = state.read_register::<u8>(s_94_2 as isize);
            tracer.read_register(s_94_2 as isize, value);
            value
        };
        // D s_94_4: cast zx s_94_3 -> bv
        let s_94_4: Bits = Bits::new(s_94_3 as u128, 2u16);
        // D s_94_5: cmp-eq s_94_1 s_94_4
        let s_94_5: bool = ((s_94_1) == (s_94_4));
        // D s_94_6: not s_94_5
        let s_94_6: bool = !s_94_5;
        // N s_94_7: branch s_94_6 b103 b95
        if s_94_6 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_95_0: const #0u : u32
        let s_95_0: u32 = 0;
        // D s_95_1: read-var ssshadow#22:u32
        let s_95_1: u32 = fn_state.ssshadow_22;
        // D s_95_2: cmp-eq s_95_0 s_95_1
        let s_95_2: bool = ((s_95_0) == (s_95_1));
        // D s_95_3: not s_95_2
        let s_95_3: bool = !s_95_2;
        // N s_95_4: branch s_95_3 b98 b96
        if s_95_3 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_96_0: read-var P:u8
        let s_96_0: bool = fn_state.P;
        // D s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 1u16);
        // D s_96_2: read-var NSK:u8
        let s_96_2: bool = fn_state.NSK;
        // D s_96_3: cast zx s_96_2 -> bv
        let s_96_3: Bits = Bits::new(s_96_2 as u128, 1u16);
        // D s_96_4: cmp-ne s_96_1 s_96_3
        let s_96_4: bool = ((s_96_1) != (s_96_3));
        // D s_96_5: write-var filtered <= s_96_4
        fn_state.filtered = s_96_4;
        // N s_96_6: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_97_0: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_98_0: const #3u : u32
        let s_98_0: u32 = 3;
        // D s_98_1: read-var ssshadow#22:u32
        let s_98_1: u32 = fn_state.ssshadow_22;
        // D s_98_2: cmp-eq s_98_0 s_98_1
        let s_98_2: bool = ((s_98_0) == (s_98_1));
        // D s_98_3: not s_98_2
        let s_98_3: bool = !s_98_2;
        // N s_98_4: branch s_98_3 b100 b99
        if s_98_3 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_99_0: read-var P:u8
        let s_99_0: bool = fn_state.P;
        // D s_99_1: cast zx s_99_0 -> bv
        let s_99_1: Bits = Bits::new(s_99_0 as u128, 1u16);
        // C s_99_2: const #1u : u8
        let s_99_2: bool = true;
        // C s_99_3: cast zx s_99_2 -> bv
        let s_99_3: Bits = Bits::new(s_99_2 as u128, 1u16);
        // D s_99_4: cmp-eq s_99_1 s_99_3
        let s_99_4: bool = ((s_99_1) == (s_99_3));
        // D s_99_5: write-var filtered <= s_99_4
        fn_state.filtered = s_99_4;
        // N s_99_6: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_100_0: const #2u : u32
        let s_100_0: u32 = 2;
        // D s_100_1: read-var ssshadow#22:u32
        let s_100_1: u32 = fn_state.ssshadow_22;
        // D s_100_2: cmp-eq s_100_0 s_100_1
        let s_100_2: bool = ((s_100_0) == (s_100_1));
        // D s_100_3: not s_100_2
        let s_100_3: bool = !s_100_2;
        // N s_100_4: branch s_100_3 b102 b101
        if s_100_3 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_101_0: read-var P:u8
        let s_101_0: bool = fn_state.P;
        // D s_101_1: cast zx s_101_0 -> bv
        let s_101_1: Bits = Bits::new(s_101_0 as u128, 1u16);
        // D s_101_2: read-var RLK:u8
        let s_101_2: bool = fn_state.RLK;
        // D s_101_3: cast zx s_101_2 -> bv
        let s_101_3: Bits = Bits::new(s_101_2 as u128, 1u16);
        // D s_101_4: cmp-ne s_101_1 s_101_3
        let s_101_4: bool = ((s_101_1) != (s_101_3));
        // D s_101_5: write-var filtered <= s_101_4
        fn_state.filtered = s_101_4;
        // N s_101_6: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_102_0: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_103_0: read-var ga#1484:u8
        let s_103_0: u8 = fn_state.ga_1484;
        // D s_103_1: cast zx s_103_0 -> bv
        let s_103_1: Bits = Bits::new(s_103_0 as u128, 2u16);
        // C s_103_2: const #432u : u32
        let s_103_2: u32 = 432;
        // D s_103_3: read-reg s_103_2:u8
        let s_103_3: u8 = {
            let value = state.read_register::<u8>(s_103_2 as isize);
            tracer.read_register(s_103_2 as isize, value);
            value
        };
        // D s_103_4: cast zx s_103_3 -> bv
        let s_103_4: Bits = Bits::new(s_103_3 as u128, 2u16);
        // D s_103_5: cmp-eq s_103_1 s_103_4
        let s_103_5: bool = ((s_103_1) == (s_103_4));
        // D s_103_6: not s_103_5
        let s_103_6: bool = !s_103_5;
        // N s_103_7: branch s_103_6 b112 b104
        if s_103_6 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_104_0: const #0u : u32
        let s_104_0: u32 = 0;
        // D s_104_1: read-var ssshadow#22:u32
        let s_104_1: u32 = fn_state.ssshadow_22;
        // D s_104_2: cmp-eq s_104_0 s_104_1
        let s_104_2: bool = ((s_104_0) == (s_104_1));
        // D s_104_3: not s_104_2
        let s_104_3: bool = !s_104_2;
        // N s_104_4: branch s_104_3 b107 b105
        if s_104_3 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_105_0: read-var NSH:u8
        let s_105_0: bool = fn_state.NSH;
        // D s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 1u16);
        // C s_105_2: const #0u : u8
        let s_105_2: bool = false;
        // C s_105_3: cast zx s_105_2 -> bv
        let s_105_3: Bits = Bits::new(s_105_2 as u128, 1u16);
        // D s_105_4: cmp-eq s_105_1 s_105_3
        let s_105_4: bool = ((s_105_1) == (s_105_3));
        // D s_105_5: write-var filtered <= s_105_4
        fn_state.filtered = s_105_4;
        // N s_105_6: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_106_0: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_107_0: const #3u : u32
        let s_107_0: u32 = 3;
        // D s_107_1: read-var ssshadow#22:u32
        let s_107_1: u32 = fn_state.ssshadow_22;
        // D s_107_2: cmp-eq s_107_0 s_107_1
        let s_107_2: bool = ((s_107_0) == (s_107_1));
        // D s_107_3: not s_107_2
        let s_107_3: bool = !s_107_2;
        // N s_107_4: branch s_107_3 b109 b108
        if s_107_3 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_108_0: read-var NSH:u8
        let s_108_0: bool = fn_state.NSH;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 1u16);
        // D s_108_2: read-var SH:u8
        let s_108_2: bool = fn_state.SH;
        // D s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 1u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: write-var filtered <= s_108_4
        fn_state.filtered = s_108_4;
        // N s_108_6: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_109_0: const #2u : u32
        let s_109_0: u32 = 2;
        // D s_109_1: read-var ssshadow#22:u32
        let s_109_1: u32 = fn_state.ssshadow_22;
        // D s_109_2: cmp-eq s_109_0 s_109_1
        let s_109_2: bool = ((s_109_0) == (s_109_1));
        // D s_109_3: not s_109_2
        let s_109_3: bool = !s_109_2;
        // N s_109_4: branch s_109_3 b111 b110
        if s_109_3 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_110_0: read-var NSH:u8
        let s_110_0: bool = fn_state.NSH;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 1u16);
        // D s_110_2: read-var RLH:u8
        let s_110_2: bool = fn_state.RLH;
        // D s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 1u16);
        // D s_110_4: cmp-eq s_110_1 s_110_3
        let s_110_4: bool = ((s_110_1) == (s_110_3));
        // D s_110_5: write-var filtered <= s_110_4
        fn_state.filtered = s_110_4;
        // N s_110_6: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_111_0: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_112_0: read-var ga#1484:u8
        let s_112_0: u8 = fn_state.ga_1484;
        // D s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 2u16);
        // C s_112_2: const #424u : u32
        let s_112_2: u32 = 424;
        // D s_112_3: read-reg s_112_2:u8
        let s_112_3: u8 = {
            let value = state.read_register::<u8>(s_112_2 as isize);
            tracer.read_register(s_112_2 as isize, value);
            value
        };
        // D s_112_4: cast zx s_112_3 -> bv
        let s_112_4: Bits = Bits::new(s_112_3 as u128, 2u16);
        // D s_112_5: cmp-eq s_112_1 s_112_4
        let s_112_5: bool = ((s_112_1) == (s_112_4));
        // D s_112_6: not s_112_5
        let s_112_6: bool = !s_112_5;
        // N s_112_7: branch s_112_6 b116 b113
        if s_112_6 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_113_0: const #() : ()
        let s_113_0: () = ();
        // S s_113_1: call HaveAArch64(s_113_0)
        let s_113_1: bool = HaveAArch64(state, tracer, s_113_0);
        // N s_113_2: branch s_113_1 b115 b114
        if s_113_1 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_114_0: read-var P:u8
        let s_114_0: bool = fn_state.P;
        // D s_114_1: cast zx s_114_0 -> bv
        let s_114_1: Bits = Bits::new(s_114_0 as u128, 1u16);
        // C s_114_2: const #1u : u8
        let s_114_2: bool = true;
        // C s_114_3: cast zx s_114_2 -> bv
        let s_114_3: Bits = Bits::new(s_114_2 as u128, 1u16);
        // D s_114_4: cmp-eq s_114_1 s_114_3
        let s_114_4: bool = ((s_114_1) == (s_114_3));
        // D s_114_5: write-var filtered <= s_114_4
        fn_state.filtered = s_114_4;
        // N s_114_6: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_115_0: read-var M:u8
        let s_115_0: bool = fn_state.M;
        // D s_115_1: cast zx s_115_0 -> bv
        let s_115_1: Bits = Bits::new(s_115_0 as u128, 1u16);
        // D s_115_2: read-var P:u8
        let s_115_2: bool = fn_state.P;
        // D s_115_3: cast zx s_115_2 -> bv
        let s_115_3: Bits = Bits::new(s_115_2 as u128, 1u16);
        // D s_115_4: cmp-ne s_115_1 s_115_3
        let s_115_4: bool = ((s_115_1) != (s_115_3));
        // D s_115_5: write-var filtered <= s_115_4
        fn_state.filtered = s_115_4;
        // N s_115_6: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_116_0: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_117_0: const #20s : i
        let s_117_0: i128 = 20;
        // D s_117_1: read-var filtershadow#21:u32
        let s_117_1: u32 = fn_state.filtershadow_21;
        // D s_117_2: cast zx s_117_1 -> bv
        let s_117_2: Bits = Bits::new(s_117_1 as u128, 32u16);
        // C s_117_3: const #1u : u64
        let s_117_3: u64 = 1;
        // D s_117_4: bit-extract s_117_2 s_117_0 s_117_3
        let s_117_4: Bits = (Bits::new(
            ((s_117_2) >> (s_117_0)).value(),
            u16::try_from(s_117_3).unwrap(),
        ));
        // D s_117_5: cast reint s_117_4 -> u8
        let s_117_5: bool = ((s_117_4.value()) != 0);
        // C s_117_6: const #0s : i
        let s_117_6: i128 = 0;
        // C s_117_7: const #0u : u64
        let s_117_7: u64 = 0;
        // D s_117_8: cast zx s_117_5 -> u64
        let s_117_8: u64 = (s_117_5 as u64);
        // C s_117_9: const #1u : u64
        let s_117_9: u64 = 1;
        // D s_117_10: and s_117_8 s_117_9
        let s_117_10: u64 = ((s_117_8) & (s_117_9));
        // D s_117_11: cmp-eq s_117_10 s_117_9
        let s_117_11: bool = ((s_117_10) == (s_117_9));
        // D s_117_12: lsl s_117_8 s_117_6
        let s_117_12: u64 = s_117_8 << s_117_6;
        // D s_117_13: or s_117_7 s_117_12
        let s_117_13: u64 = ((s_117_7) | (s_117_12));
        // D s_117_14: cmpl s_117_12
        let s_117_14: u64 = !s_117_12;
        // D s_117_15: and s_117_7 s_117_14
        let s_117_15: u64 = ((s_117_7) & (s_117_14));
        // D s_117_16: select s_117_11 s_117_13 s_117_15
        let s_117_16: u64 = if s_117_11 { s_117_13 } else { s_117_15 };
        // D s_117_17: cast trunc s_117_16 -> u8
        let s_117_17: bool = ((s_117_16) != 0);
        // D s_117_18: write-var RLH <= s_117_17
        fn_state.RLH = s_117_17;
        // N s_117_19: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_118_0: const #21s : i
        let s_118_0: i128 = 21;
        // D s_118_1: read-var filtershadow#21:u32
        let s_118_1: u32 = fn_state.filtershadow_21;
        // D s_118_2: cast zx s_118_1 -> bv
        let s_118_2: Bits = Bits::new(s_118_1 as u128, 32u16);
        // C s_118_3: const #1u : u64
        let s_118_3: u64 = 1;
        // D s_118_4: bit-extract s_118_2 s_118_0 s_118_3
        let s_118_4: Bits = (Bits::new(
            ((s_118_2) >> (s_118_0)).value(),
            u16::try_from(s_118_3).unwrap(),
        ));
        // D s_118_5: cast reint s_118_4 -> u8
        let s_118_5: bool = ((s_118_4.value()) != 0);
        // C s_118_6: const #0s : i
        let s_118_6: i128 = 0;
        // C s_118_7: const #0u : u64
        let s_118_7: u64 = 0;
        // D s_118_8: cast zx s_118_5 -> u64
        let s_118_8: u64 = (s_118_5 as u64);
        // C s_118_9: const #1u : u64
        let s_118_9: u64 = 1;
        // D s_118_10: and s_118_8 s_118_9
        let s_118_10: u64 = ((s_118_8) & (s_118_9));
        // D s_118_11: cmp-eq s_118_10 s_118_9
        let s_118_11: bool = ((s_118_10) == (s_118_9));
        // D s_118_12: lsl s_118_8 s_118_6
        let s_118_12: u64 = s_118_8 << s_118_6;
        // D s_118_13: or s_118_7 s_118_12
        let s_118_13: u64 = ((s_118_7) | (s_118_12));
        // D s_118_14: cmpl s_118_12
        let s_118_14: u64 = !s_118_12;
        // D s_118_15: and s_118_7 s_118_14
        let s_118_15: u64 = ((s_118_7) & (s_118_14));
        // D s_118_16: select s_118_11 s_118_13 s_118_15
        let s_118_16: u64 = if s_118_11 { s_118_13 } else { s_118_15 };
        // D s_118_17: cast trunc s_118_16 -> u8
        let s_118_17: bool = ((s_118_16) != 0);
        // D s_118_18: write-var RLU <= s_118_17
        fn_state.RLU = s_118_17;
        // N s_118_19: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_119_0: const #22s : i
        let s_119_0: i128 = 22;
        // D s_119_1: read-var filtershadow#21:u32
        let s_119_1: u32 = fn_state.filtershadow_21;
        // D s_119_2: cast zx s_119_1 -> bv
        let s_119_2: Bits = Bits::new(s_119_1 as u128, 32u16);
        // C s_119_3: const #1u : u64
        let s_119_3: u64 = 1;
        // D s_119_4: bit-extract s_119_2 s_119_0 s_119_3
        let s_119_4: Bits = (Bits::new(
            ((s_119_2) >> (s_119_0)).value(),
            u16::try_from(s_119_3).unwrap(),
        ));
        // D s_119_5: cast reint s_119_4 -> u8
        let s_119_5: bool = ((s_119_4.value()) != 0);
        // C s_119_6: const #0s : i
        let s_119_6: i128 = 0;
        // C s_119_7: const #0u : u64
        let s_119_7: u64 = 0;
        // D s_119_8: cast zx s_119_5 -> u64
        let s_119_8: u64 = (s_119_5 as u64);
        // C s_119_9: const #1u : u64
        let s_119_9: u64 = 1;
        // D s_119_10: and s_119_8 s_119_9
        let s_119_10: u64 = ((s_119_8) & (s_119_9));
        // D s_119_11: cmp-eq s_119_10 s_119_9
        let s_119_11: bool = ((s_119_10) == (s_119_9));
        // D s_119_12: lsl s_119_8 s_119_6
        let s_119_12: u64 = s_119_8 << s_119_6;
        // D s_119_13: or s_119_7 s_119_12
        let s_119_13: u64 = ((s_119_7) | (s_119_12));
        // D s_119_14: cmpl s_119_12
        let s_119_14: u64 = !s_119_12;
        // D s_119_15: and s_119_7 s_119_14
        let s_119_15: u64 = ((s_119_7) & (s_119_14));
        // D s_119_16: select s_119_11 s_119_13 s_119_15
        let s_119_16: u64 = if s_119_11 { s_119_13 } else { s_119_15 };
        // D s_119_17: cast trunc s_119_16 -> u8
        let s_119_17: bool = ((s_119_16) != 0);
        // D s_119_18: write-var RLK <= s_119_17
        fn_state.RLK = s_119_17;
        // N s_119_19: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_120_0: const #24s : i
        let s_120_0: i128 = 24;
        // D s_120_1: read-var filtershadow#21:u32
        let s_120_1: u32 = fn_state.filtershadow_21;
        // D s_120_2: cast zx s_120_1 -> bv
        let s_120_2: Bits = Bits::new(s_120_1 as u128, 32u16);
        // C s_120_3: const #1u : u64
        let s_120_3: u64 = 1;
        // D s_120_4: bit-extract s_120_2 s_120_0 s_120_3
        let s_120_4: Bits = (Bits::new(
            ((s_120_2) >> (s_120_0)).value(),
            u16::try_from(s_120_3).unwrap(),
        ));
        // D s_120_5: cast reint s_120_4 -> u8
        let s_120_5: bool = ((s_120_4.value()) != 0);
        // C s_120_6: const #0s : i
        let s_120_6: i128 = 0;
        // C s_120_7: const #0u : u64
        let s_120_7: u64 = 0;
        // D s_120_8: cast zx s_120_5 -> u64
        let s_120_8: u64 = (s_120_5 as u64);
        // C s_120_9: const #1u : u64
        let s_120_9: u64 = 1;
        // D s_120_10: and s_120_8 s_120_9
        let s_120_10: u64 = ((s_120_8) & (s_120_9));
        // D s_120_11: cmp-eq s_120_10 s_120_9
        let s_120_11: bool = ((s_120_10) == (s_120_9));
        // D s_120_12: lsl s_120_8 s_120_6
        let s_120_12: u64 = s_120_8 << s_120_6;
        // D s_120_13: or s_120_7 s_120_12
        let s_120_13: u64 = ((s_120_7) | (s_120_12));
        // D s_120_14: cmpl s_120_12
        let s_120_14: u64 = !s_120_12;
        // D s_120_15: and s_120_7 s_120_14
        let s_120_15: u64 = ((s_120_7) & (s_120_14));
        // D s_120_16: select s_120_11 s_120_13 s_120_15
        let s_120_16: u64 = if s_120_11 { s_120_13 } else { s_120_15 };
        // D s_120_17: cast trunc s_120_16 -> u8
        let s_120_17: bool = ((s_120_16) != 0);
        // D s_120_18: write-var SH <= s_120_17
        fn_state.SH = s_120_17;
        // N s_120_19: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_121_0: const #() : ()
        let s_121_0: () = ();
        // S s_121_1: call HaveSecureEL2Ext(s_121_0)
        let s_121_1: bool = HaveSecureEL2Ext(state, tracer, s_121_0);
        // D s_121_2: write-var gs#2565 <= s_121_1
        fn_state.gs_2565 = s_121_1;
        // N s_121_3: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_122_0: const #26s : i
        let s_122_0: i128 = 26;
        // D s_122_1: read-var filtershadow#21:u32
        let s_122_1: u32 = fn_state.filtershadow_21;
        // D s_122_2: cast zx s_122_1 -> bv
        let s_122_2: Bits = Bits::new(s_122_1 as u128, 32u16);
        // C s_122_3: const #1u : u64
        let s_122_3: u64 = 1;
        // D s_122_4: bit-extract s_122_2 s_122_0 s_122_3
        let s_122_4: Bits = (Bits::new(
            ((s_122_2) >> (s_122_0)).value(),
            u16::try_from(s_122_3).unwrap(),
        ));
        // D s_122_5: cast reint s_122_4 -> u8
        let s_122_5: bool = ((s_122_4.value()) != 0);
        // C s_122_6: const #0s : i
        let s_122_6: i128 = 0;
        // C s_122_7: const #0u : u64
        let s_122_7: u64 = 0;
        // D s_122_8: cast zx s_122_5 -> u64
        let s_122_8: u64 = (s_122_5 as u64);
        // C s_122_9: const #1u : u64
        let s_122_9: u64 = 1;
        // D s_122_10: and s_122_8 s_122_9
        let s_122_10: u64 = ((s_122_8) & (s_122_9));
        // D s_122_11: cmp-eq s_122_10 s_122_9
        let s_122_11: bool = ((s_122_10) == (s_122_9));
        // D s_122_12: lsl s_122_8 s_122_6
        let s_122_12: u64 = s_122_8 << s_122_6;
        // D s_122_13: or s_122_7 s_122_12
        let s_122_13: u64 = ((s_122_7) | (s_122_12));
        // D s_122_14: cmpl s_122_12
        let s_122_14: u64 = !s_122_12;
        // D s_122_15: and s_122_7 s_122_14
        let s_122_15: u64 = ((s_122_7) & (s_122_14));
        // D s_122_16: select s_122_11 s_122_13 s_122_15
        let s_122_16: u64 = if s_122_11 { s_122_13 } else { s_122_15 };
        // D s_122_17: cast trunc s_122_16 -> u8
        let s_122_17: bool = ((s_122_16) != 0);
        // D s_122_18: write-var M <= s_122_17
        fn_state.M = s_122_17;
        // N s_122_19: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_123_0: const #() : ()
        let s_123_0: () = ();
        // S s_123_1: call HaveAArch64(s_123_0)
        let s_123_1: bool = HaveAArch64(state, tracer, s_123_0);
        // D s_123_2: write-var gs#2562 <= s_123_1
        fn_state.gs_2562 = s_123_1;
        // N s_123_3: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_124_0: const #27s : i
        let s_124_0: i128 = 27;
        // D s_124_1: read-var filtershadow#21:u32
        let s_124_1: u32 = fn_state.filtershadow_21;
        // D s_124_2: cast zx s_124_1 -> bv
        let s_124_2: Bits = Bits::new(s_124_1 as u128, 32u16);
        // C s_124_3: const #1u : u64
        let s_124_3: u64 = 1;
        // D s_124_4: bit-extract s_124_2 s_124_0 s_124_3
        let s_124_4: Bits = (Bits::new(
            ((s_124_2) >> (s_124_0)).value(),
            u16::try_from(s_124_3).unwrap(),
        ));
        // D s_124_5: cast reint s_124_4 -> u8
        let s_124_5: bool = ((s_124_4.value()) != 0);
        // C s_124_6: const #0s : i
        let s_124_6: i128 = 0;
        // C s_124_7: const #0u : u64
        let s_124_7: u64 = 0;
        // D s_124_8: cast zx s_124_5 -> u64
        let s_124_8: u64 = (s_124_5 as u64);
        // C s_124_9: const #1u : u64
        let s_124_9: u64 = 1;
        // D s_124_10: and s_124_8 s_124_9
        let s_124_10: u64 = ((s_124_8) & (s_124_9));
        // D s_124_11: cmp-eq s_124_10 s_124_9
        let s_124_11: bool = ((s_124_10) == (s_124_9));
        // D s_124_12: lsl s_124_8 s_124_6
        let s_124_12: u64 = s_124_8 << s_124_6;
        // D s_124_13: or s_124_7 s_124_12
        let s_124_13: u64 = ((s_124_7) | (s_124_12));
        // D s_124_14: cmpl s_124_12
        let s_124_14: u64 = !s_124_12;
        // D s_124_15: and s_124_7 s_124_14
        let s_124_15: u64 = ((s_124_7) & (s_124_14));
        // D s_124_16: select s_124_11 s_124_13 s_124_15
        let s_124_16: u64 = if s_124_11 { s_124_13 } else { s_124_15 };
        // D s_124_17: cast trunc s_124_16 -> u8
        let s_124_17: bool = ((s_124_16) != 0);
        // D s_124_18: write-var NSH <= s_124_17
        fn_state.NSH = s_124_17;
        // N s_124_19: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_125_0: const #28s : i
        let s_125_0: i128 = 28;
        // D s_125_1: read-var filtershadow#21:u32
        let s_125_1: u32 = fn_state.filtershadow_21;
        // D s_125_2: cast zx s_125_1 -> bv
        let s_125_2: Bits = Bits::new(s_125_1 as u128, 32u16);
        // C s_125_3: const #1u : u64
        let s_125_3: u64 = 1;
        // D s_125_4: bit-extract s_125_2 s_125_0 s_125_3
        let s_125_4: Bits = (Bits::new(
            ((s_125_2) >> (s_125_0)).value(),
            u16::try_from(s_125_3).unwrap(),
        ));
        // D s_125_5: cast reint s_125_4 -> u8
        let s_125_5: bool = ((s_125_4.value()) != 0);
        // C s_125_6: const #0s : i
        let s_125_6: i128 = 0;
        // C s_125_7: const #0u : u64
        let s_125_7: u64 = 0;
        // D s_125_8: cast zx s_125_5 -> u64
        let s_125_8: u64 = (s_125_5 as u64);
        // C s_125_9: const #1u : u64
        let s_125_9: u64 = 1;
        // D s_125_10: and s_125_8 s_125_9
        let s_125_10: u64 = ((s_125_8) & (s_125_9));
        // D s_125_11: cmp-eq s_125_10 s_125_9
        let s_125_11: bool = ((s_125_10) == (s_125_9));
        // D s_125_12: lsl s_125_8 s_125_6
        let s_125_12: u64 = s_125_8 << s_125_6;
        // D s_125_13: or s_125_7 s_125_12
        let s_125_13: u64 = ((s_125_7) | (s_125_12));
        // D s_125_14: cmpl s_125_12
        let s_125_14: u64 = !s_125_12;
        // D s_125_15: and s_125_7 s_125_14
        let s_125_15: u64 = ((s_125_7) & (s_125_14));
        // D s_125_16: select s_125_11 s_125_13 s_125_15
        let s_125_16: u64 = if s_125_11 { s_125_13 } else { s_125_15 };
        // D s_125_17: cast trunc s_125_16 -> u8
        let s_125_17: bool = ((s_125_16) != 0);
        // D s_125_18: write-var NSU <= s_125_17
        fn_state.NSU = s_125_17;
        // N s_125_19: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_126_0: const #29s : i
        let s_126_0: i128 = 29;
        // D s_126_1: read-var filtershadow#21:u32
        let s_126_1: u32 = fn_state.filtershadow_21;
        // D s_126_2: cast zx s_126_1 -> bv
        let s_126_2: Bits = Bits::new(s_126_1 as u128, 32u16);
        // C s_126_3: const #1u : u64
        let s_126_3: u64 = 1;
        // D s_126_4: bit-extract s_126_2 s_126_0 s_126_3
        let s_126_4: Bits = (Bits::new(
            ((s_126_2) >> (s_126_0)).value(),
            u16::try_from(s_126_3).unwrap(),
        ));
        // D s_126_5: cast reint s_126_4 -> u8
        let s_126_5: bool = ((s_126_4.value()) != 0);
        // C s_126_6: const #0s : i
        let s_126_6: i128 = 0;
        // C s_126_7: const #0u : u64
        let s_126_7: u64 = 0;
        // D s_126_8: cast zx s_126_5 -> u64
        let s_126_8: u64 = (s_126_5 as u64);
        // C s_126_9: const #1u : u64
        let s_126_9: u64 = 1;
        // D s_126_10: and s_126_8 s_126_9
        let s_126_10: u64 = ((s_126_8) & (s_126_9));
        // D s_126_11: cmp-eq s_126_10 s_126_9
        let s_126_11: bool = ((s_126_10) == (s_126_9));
        // D s_126_12: lsl s_126_8 s_126_6
        let s_126_12: u64 = s_126_8 << s_126_6;
        // D s_126_13: or s_126_7 s_126_12
        let s_126_13: u64 = ((s_126_7) | (s_126_12));
        // D s_126_14: cmpl s_126_12
        let s_126_14: u64 = !s_126_12;
        // D s_126_15: and s_126_7 s_126_14
        let s_126_15: u64 = ((s_126_7) & (s_126_14));
        // D s_126_16: select s_126_11 s_126_13 s_126_15
        let s_126_16: u64 = if s_126_11 { s_126_13 } else { s_126_15 };
        // D s_126_17: cast trunc s_126_16 -> u8
        let s_126_17: bool = ((s_126_16) != 0);
        // D s_126_18: write-var NSK <= s_126_17
        fn_state.NSK = s_126_17;
        // N s_126_19: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_127_0: read-var idx:i64
        let s_127_0: i64 = fn_state.idx;
        // D s_127_1: cast zx s_127_0 -> i
        let s_127_1: i128 = (i128::try_from(s_127_0).unwrap());
        // C s_127_2: const #0u : u32
        let s_127_2: u32 = 0;
        // D s_127_3: read-reg s_127_2:i64
        let s_127_3: i64 = {
            let value = state.read_register::<i64>(s_127_2 as isize);
            tracer.read_register(s_127_2 as isize, value);
            value
        };
        // D s_127_4: cast zx s_127_3 -> i
        let s_127_4: i128 = (i128::try_from(s_127_3).unwrap());
        // D s_127_5: cmp-eq s_127_1 s_127_4
        let s_127_5: bool = ((s_127_1) == (s_127_4));
        // D s_127_6: not s_127_5
        let s_127_6: bool = !s_127_5;
        // N s_127_7: branch s_127_6 b132 b128
        if s_127_6 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_128_0: const #() : ()
        let s_128_0: () = ();
        // S s_128_1: call HaveAArch64(s_128_0)
        let s_128_1: bool = HaveAArch64(state, tracer, s_128_0);
        // N s_128_2: branch s_128_1 b131 b129
        if s_128_1 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_129_0: const #() : ()
        let s_129_0: () = ();
        // S s_129_1: call PMCCFILTR_read(s_129_0)
        let s_129_1: ProductType700c18a878c5601b = PMCCFILTR_read(
            state,
            tracer,
            s_129_0,
        );
        // D s_129_2: write-var ga#1471 <= s_129_1
        fn_state.ga_1471 = s_129_1;
        // D s_129_3: read-var ga#1471.0:struct
        let s_129_3: u32 = fn_state.ga_1471._0;
        // D s_129_4: write-var filter <= s_129_3
        fn_state.filter = s_129_3;
        // N s_129_5: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_130_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_131_0: const #15864u : u32
        let s_131_0: u32 = 15864;
        // D s_131_1: read-reg s_131_0:u64
        let s_131_1: u64 = {
            let value = state.read_register::<u64>(s_131_0 as isize);
            tracer.read_register(s_131_0 as isize, value);
            value
        };
        // C s_131_2: const #0s : i
        let s_131_2: i128 = 0;
        // D s_131_3: cast zx s_131_1 -> bv
        let s_131_3: Bits = Bits::new(s_131_1 as u128, 64u16);
        // C s_131_4: const #1s : i64
        let s_131_4: i64 = 1;
        // C s_131_5: cast zx s_131_4 -> i
        let s_131_5: i128 = (i128::try_from(s_131_4).unwrap());
        // C s_131_6: const #31s : i
        let s_131_6: i128 = 31;
        // C s_131_7: add s_131_6 s_131_5
        let s_131_7: i128 = (s_131_6 + s_131_5);
        // D s_131_8: bit-extract s_131_3 s_131_2 s_131_7
        let s_131_8: Bits = (Bits::new(
            ((s_131_3) >> (s_131_2)).value(),
            u16::try_from(s_131_7).unwrap(),
        ));
        // D s_131_9: cast reint s_131_8 -> u32
        let s_131_9: u32 = (s_131_8.value() as u32);
        // D s_131_10: write-var filter <= s_131_9
        fn_state.filter = s_131_9;
        // N s_131_11: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_132_0: const #() : ()
        let s_132_0: () = ();
        // S s_132_1: call HaveAArch64(s_132_0)
        let s_132_1: bool = HaveAArch64(state, tracer, s_132_0);
        // N s_132_2: branch s_132_1 b138 b133
        if s_132_1 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_133_0: read-var idx:i64
        let s_133_0: i64 = fn_state.idx;
        // D s_133_1: cast zx s_133_0 -> i
        let s_133_1: i128 = (i128::try_from(s_133_0).unwrap());
        // D s_133_2: call __id(s_133_1)
        let s_133_2: i128 = u__id(state, tracer, s_133_1);
        // D s_133_3: cast reint s_133_2 -> i64
        let s_133_3: i64 = (s_133_2 as i64);
        // C s_133_4: const #0s : i
        let s_133_4: i128 = 0;
        // D s_133_5: cast zx s_133_3 -> i
        let s_133_5: i128 = (i128::try_from(s_133_3).unwrap());
        // D s_133_6: cmp-le s_133_4 s_133_5
        let s_133_6: bool = ((s_133_4) <= (s_133_5));
        // N s_133_7: branch s_133_6 b137 b134
        if s_133_6 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_134_0: const #0u : u8
        let s_134_0: bool = false;
        // D s_134_1: write-var gs#2542 <= s_134_0
        fn_state.gs_2542 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_135_0: read-var gs#2542:u8
        let s_135_0: bool = fn_state.gs_2542;
        // N s_135_1: assert s_135_0
        let s_135_1: () = assert!(s_135_0);
        // D s_135_2: read-var idx:i64
        let s_135_2: i64 = fn_state.idx;
        // D s_135_3: call PMEVTYPER_read(s_135_2)
        let s_135_3: ProductType700c18a878c5601b = PMEVTYPER_read(
            state,
            tracer,
            s_135_2,
        );
        // D s_135_4: write-var ga#1483 <= s_135_3
        fn_state.ga_1483 = s_135_3;
        // D s_135_5: read-var ga#1483.0:struct
        let s_135_5: u32 = fn_state.ga_1483._0;
        // D s_135_6: write-var filter <= s_135_5
        fn_state.filter = s_135_5;
        // N s_135_7: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_136_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_137_0: read-var idx:i64
        let s_137_0: i64 = fn_state.idx;
        // D s_137_1: cast zx s_137_0 -> i
        let s_137_1: i128 = (i128::try_from(s_137_0).unwrap());
        // D s_137_2: call __id(s_137_1)
        let s_137_2: i128 = u__id(state, tracer, s_137_1);
        // D s_137_3: cast reint s_137_2 -> i64
        let s_137_3: i64 = (s_137_2 as i64);
        // C s_137_4: const #31s : i
        let s_137_4: i128 = 31;
        // D s_137_5: cast zx s_137_3 -> i
        let s_137_5: i128 = (i128::try_from(s_137_3).unwrap());
        // D s_137_6: cmp-lt s_137_5 s_137_4
        let s_137_6: bool = ((s_137_5) < (s_137_4));
        // D s_137_7: write-var gs#2542 <= s_137_6
        fn_state.gs_2542 = s_137_6;
        // N s_137_8: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_138_0: read-var idx:i64
        let s_138_0: i64 = fn_state.idx;
        // D s_138_1: cast zx s_138_0 -> i
        let s_138_1: i128 = (i128::try_from(s_138_0).unwrap());
        // D s_138_2: call __id(s_138_1)
        let s_138_2: i128 = u__id(state, tracer, s_138_1);
        // D s_138_3: cast reint s_138_2 -> i64
        let s_138_3: i64 = (s_138_2 as i64);
        // C s_138_4: const #0s : i
        let s_138_4: i128 = 0;
        // D s_138_5: cast zx s_138_3 -> i
        let s_138_5: i128 = (i128::try_from(s_138_3).unwrap());
        // D s_138_6: cmp-le s_138_4 s_138_5
        let s_138_6: bool = ((s_138_4) <= (s_138_5));
        // N s_138_7: branch s_138_6 b141 b139
        if s_138_6 {
            return block_141(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_139_0: const #0u : u8
        let s_139_0: bool = false;
        // D s_139_1: write-var gs#2546 <= s_139_0
        fn_state.gs_2546 = s_139_0;
        // N s_139_2: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_140_0: read-var gs#2546:u8
        let s_140_0: bool = fn_state.gs_2546;
        // N s_140_1: assert s_140_0
        let s_140_1: () = assert!(s_140_0);
        // C s_140_2: const #11208u : u32
        let s_140_2: u32 = 11208;
        // D s_140_3: read-reg s_140_2:[struct; 32]
        let s_140_3: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_140_2 as isize);
            tracer.read_register(s_140_2 as isize, value);
            value
        };
        // D s_140_4: read-var idx:i64
        let s_140_4: i64 = fn_state.idx;
        // D s_140_5: cast zx s_140_4 -> i
        let s_140_5: i128 = (i128::try_from(s_140_4).unwrap());
        // D s_140_6: read-element s_140_3[s_140_5]
        let s_140_6: ProductType5c790c8ef59cc8b2 = s_140_3[(s_140_5) as usize];
        // D s_140_7: write-var ga#1477 <= s_140_6
        fn_state.ga_1477 = s_140_6;
        // D s_140_8: read-var ga#1477.0:struct
        let s_140_8: u64 = fn_state.ga_1477._0;
        // C s_140_9: const #0s : i
        let s_140_9: i128 = 0;
        // D s_140_10: cast zx s_140_8 -> bv
        let s_140_10: Bits = Bits::new(s_140_8 as u128, 64u16);
        // C s_140_11: const #1s : i64
        let s_140_11: i64 = 1;
        // C s_140_12: cast zx s_140_11 -> i
        let s_140_12: i128 = (i128::try_from(s_140_11).unwrap());
        // C s_140_13: const #31s : i
        let s_140_13: i128 = 31;
        // C s_140_14: add s_140_13 s_140_12
        let s_140_14: i128 = (s_140_13 + s_140_12);
        // D s_140_15: bit-extract s_140_10 s_140_9 s_140_14
        let s_140_15: Bits = (Bits::new(
            ((s_140_10) >> (s_140_9)).value(),
            u16::try_from(s_140_14).unwrap(),
        ));
        // D s_140_16: cast reint s_140_15 -> u32
        let s_140_16: u32 = (s_140_15.value() as u32);
        // D s_140_17: write-var filter <= s_140_16
        fn_state.filter = s_140_16;
        // N s_140_18: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_141_0: read-var idx:i64
        let s_141_0: i64 = fn_state.idx;
        // D s_141_1: cast zx s_141_0 -> i
        let s_141_1: i128 = (i128::try_from(s_141_0).unwrap());
        // D s_141_2: call __id(s_141_1)
        let s_141_2: i128 = u__id(state, tracer, s_141_1);
        // D s_141_3: cast reint s_141_2 -> i64
        let s_141_3: i64 = (s_141_2 as i64);
        // C s_141_4: const #32s : i
        let s_141_4: i128 = 32;
        // D s_141_5: cast zx s_141_3 -> i
        let s_141_5: i128 = (i128::try_from(s_141_3).unwrap());
        // D s_141_6: cmp-lt s_141_5 s_141_4
        let s_141_6: bool = ((s_141_5) < (s_141_4));
        // D s_141_7: write-var gs#2546 <= s_141_6
        fn_state.gs_2546 = s_141_6;
        // N s_141_8: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_142_0: const #16975u : u32
        let s_142_0: u32 = 16975;
        // D s_142_1: read-reg s_142_0:u8
        let s_142_1: u8 = {
            let value = state.read_register::<u8>(s_142_0 as isize);
            tracer.read_register(s_142_0 as isize, value);
            value
        };
        // D s_142_2: cast zx s_142_1 -> bv
        let s_142_2: Bits = Bits::new(s_142_1 as u128, 2u16);
        // C s_142_3: const #424u : u32
        let s_142_3: u32 = 424;
        // D s_142_4: read-reg s_142_3:u8
        let s_142_4: u8 = {
            let value = state.read_register::<u8>(s_142_3 as isize);
            tracer.read_register(s_142_3 as isize, value);
            value
        };
        // D s_142_5: cast zx s_142_4 -> bv
        let s_142_5: Bits = Bits::new(s_142_4 as u128, 2u16);
        // D s_142_6: cmp-eq s_142_2 s_142_5
        let s_142_6: bool = ((s_142_2) == (s_142_5));
        // N s_142_7: branch s_142_6 b151 b143
        if s_142_6 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_143_0: const #0u : u8
        let s_143_0: bool = false;
        // D s_143_1: write-var gs#2517 <= s_143_0
        fn_state.gs_2517 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_144_0: read-var gs#2517:u8
        let s_144_0: bool = fn_state.gs_2517;
        // N s_144_1: branch s_144_0 b150 b145
        if s_144_0 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_145_0: const #0u : u8
        let s_145_0: bool = false;
        // D s_145_1: write-var gs#2518 <= s_145_0
        fn_state.gs_2518 = s_145_0;
        // N s_145_2: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_146_0: read-var gs#2518:u8
        let s_146_0: bool = fn_state.gs_2518;
        // N s_146_1: branch s_146_0 b149 b147
        if s_146_0 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_147_0: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_148_0: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_149_0: const #1u : u8
        let s_149_0: bool = true;
        // D s_149_1: write-var prohibited <= s_149_0
        fn_state.prohibited = s_149_0;
        // N s_149_2: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_150_0: const #22712u : u32
        let s_150_0: u32 = 22712;
        // D s_150_1: read-reg s_150_0:struct
        let s_150_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_150_0 as isize);
            tracer.read_register(s_150_0 as isize, value);
            value
        };
        // D s_150_2: call _get_MDCR_EL3_Type_MCCD(s_150_1)
        let s_150_2: bool = u_get_MDCR_EL3_Type_MCCD(state, tracer, s_150_1);
        // D s_150_3: cast zx s_150_2 -> bv
        let s_150_3: Bits = Bits::new(s_150_2 as u128, 1u16);
        // C s_150_4: const #1u : u8
        let s_150_4: bool = true;
        // C s_150_5: cast zx s_150_4 -> bv
        let s_150_5: Bits = Bits::new(s_150_4 as u128, 1u16);
        // D s_150_6: cmp-eq s_150_3 s_150_5
        let s_150_6: bool = ((s_150_3) == (s_150_5));
        // D s_150_7: write-var gs#2518 <= s_150_6
        fn_state.gs_2518 = s_150_6;
        // N s_150_8: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_151_0: const #() : ()
        let s_151_0: () = ();
        // S s_151_1: call HaveAArch64(s_151_0)
        let s_151_1: bool = HaveAArch64(state, tracer, s_151_0);
        // D s_151_2: write-var gs#2517 <= s_151_1
        fn_state.gs_2517 = s_151_1;
        // N s_151_3: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_152_0: read-var idx:i64
        let s_152_0: i64 = fn_state.idx;
        // D s_152_1: cast zx s_152_0 -> i
        let s_152_1: i128 = (i128::try_from(s_152_0).unwrap());
        // C s_152_2: const #0u : u32
        let s_152_2: u32 = 0;
        // D s_152_3: read-reg s_152_2:i64
        let s_152_3: i64 = {
            let value = state.read_register::<i64>(s_152_2 as isize);
            tracer.read_register(s_152_2 as isize, value);
            value
        };
        // D s_152_4: cast zx s_152_3 -> i
        let s_152_4: i128 = (i128::try_from(s_152_3).unwrap());
        // D s_152_5: cmp-eq s_152_1 s_152_4
        let s_152_5: bool = ((s_152_1) == (s_152_4));
        // D s_152_6: write-var gs#2515 <= s_152_5
        fn_state.gs_2515 = s_152_5;
        // N s_152_7: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_153_0: const #424u : u32
        let s_153_0: u32 = 424;
        // D s_153_1: read-reg s_153_0:u8
        let s_153_1: u8 = {
            let value = state.read_register::<u8>(s_153_0 as isize);
            tracer.read_register(s_153_0 as isize, value);
            value
        };
        // C s_153_2: const #2u : u8
        let s_153_2: u8 = 2;
        // D s_153_3: cmp-lt s_153_1 s_153_2
        let s_153_3: bool = ((s_153_1) < (s_153_2));
        // N s_153_4: branch s_153_3 b174 b154
        if s_153_3 {
            return block_174(state, tracer, fn_state);
        } else {
            return block_154(state, tracer, fn_state);
        };
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_154_0: const #0u : u8
        let s_154_0: bool = false;
        // D s_154_1: write-var gs#2521 <= s_154_0
        fn_state.gs_2521 = s_154_0;
        // N s_154_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_155_0: read-var gs#2521:u8
        let s_155_0: bool = fn_state.gs_2521;
        // N s_155_1: branch s_155_0 b167 b156
        if s_155_0 {
            return block_167(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_156_0: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_157_0: const #16975u : u32
        let s_157_0: u32 = 16975;
        // D s_157_1: read-reg s_157_0:u8
        let s_157_1: u8 = {
            let value = state.read_register::<u8>(s_157_0 as isize);
            tracer.read_register(s_157_0 as isize, value);
            value
        };
        // D s_157_2: cast zx s_157_1 -> bv
        let s_157_2: Bits = Bits::new(s_157_1 as u128, 2u16);
        // C s_157_3: const #432u : u32
        let s_157_3: u32 = 432;
        // D s_157_4: read-reg s_157_3:u8
        let s_157_4: u8 = {
            let value = state.read_register::<u8>(s_157_3 as isize);
            tracer.read_register(s_157_3 as isize, value);
            value
        };
        // D s_157_5: cast zx s_157_4 -> bv
        let s_157_5: Bits = Bits::new(s_157_4 as u128, 2u16);
        // D s_157_6: cmp-eq s_157_2 s_157_5
        let s_157_6: bool = ((s_157_2) == (s_157_5));
        // N s_157_7: branch s_157_6 b160 b158
        if s_157_6 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_158_0: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_159_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_160_0: const #() : ()
        let s_160_0: () = ();
        // S s_160_1: call HaveAArch64(s_160_0)
        let s_160_1: bool = HaveAArch64(state, tracer, s_160_0);
        // N s_160_2: branch s_160_1 b166 b161
        if s_160_1 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_161_0: const #() : ()
        let s_161_0: () = ();
        // S s_161_1: call HDCR_read(s_161_0)
        let s_161_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_161_0);
        // S s_161_2: call _get_HDCR_Type_HCCD(s_161_1)
        let s_161_2: bool = u_get_HDCR_Type_HCCD(state, tracer, s_161_1);
        // D s_161_3: write-var hccdshadow#25 <= s_161_2
        fn_state.hccdshadow_25 = s_161_2;
        // N s_161_4: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_162_0: read-var hccdshadow#25:u8
        let s_162_0: bool = fn_state.hccdshadow_25;
        // D s_162_1: cast zx s_162_0 -> bv
        let s_162_1: Bits = Bits::new(s_162_0 as u128, 1u16);
        // C s_162_2: const #1u : u8
        let s_162_2: bool = true;
        // C s_162_3: cast zx s_162_2 -> bv
        let s_162_3: Bits = Bits::new(s_162_2 as u128, 1u16);
        // D s_162_4: cmp-eq s_162_1 s_162_3
        let s_162_4: bool = ((s_162_1) == (s_162_3));
        // N s_162_5: branch s_162_4 b165 b163
        if s_162_4 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_163_0: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_164_0: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_165_0: const #1u : u8
        let s_165_0: bool = true;
        // D s_165_1: write-var prohibited <= s_165_0
        fn_state.prohibited = s_165_0;
        // N s_165_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_166_0: const #104880u : u32
        let s_166_0: u32 = 104880;
        // D s_166_1: read-reg s_166_0:struct
        let s_166_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_166_0 as isize);
            tracer.read_register(s_166_0 as isize, value);
            value
        };
        // D s_166_2: call _get_MDCR_EL2_Type_HCCD(s_166_1)
        let s_166_2: bool = u_get_MDCR_EL2_Type_HCCD(state, tracer, s_166_1);
        // D s_166_3: write-var hccdshadow#25 <= s_166_2
        fn_state.hccdshadow_25 = s_166_2;
        // N s_166_4: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_167_0: const #() : ()
        let s_167_0: () = ();
        // S s_167_1: call HaveAArch64(s_167_0)
        let s_167_1: bool = HaveAArch64(state, tracer, s_167_0);
        // N s_167_2: branch s_167_1 b173 b168
        if s_167_1 {
            return block_173(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_168_0: const #15048u : u32
        let s_168_0: u32 = 15048;
        // D s_168_1: read-reg s_168_0:struct
        let s_168_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_168_0 as isize);
            tracer.read_register(s_168_0 as isize, value);
            value
        };
        // D s_168_2: call _get_SDCR_Type_SCCD(s_168_1)
        let s_168_2: bool = u_get_SDCR_Type_SCCD(state, tracer, s_168_1);
        // D s_168_3: write-var sccdshadow#24 <= s_168_2
        fn_state.sccdshadow_24 = s_168_2;
        // N s_168_4: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_169_0: read-var sccdshadow#24:u8
        let s_169_0: bool = fn_state.sccdshadow_24;
        // D s_169_1: cast zx s_169_0 -> bv
        let s_169_1: Bits = Bits::new(s_169_0 as u128, 1u16);
        // C s_169_2: const #1u : u8
        let s_169_2: bool = true;
        // C s_169_3: cast zx s_169_2 -> bv
        let s_169_3: Bits = Bits::new(s_169_2 as u128, 1u16);
        // D s_169_4: cmp-eq s_169_1 s_169_3
        let s_169_4: bool = ((s_169_1) == (s_169_3));
        // N s_169_5: branch s_169_4 b172 b170
        if s_169_4 {
            return block_172(state, tracer, fn_state);
        } else {
            return block_170(state, tracer, fn_state);
        };
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_170_0: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_171_0: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_172_0: const #1u : u8
        let s_172_0: bool = true;
        // D s_172_1: write-var prohibited <= s_172_0
        fn_state.prohibited = s_172_0;
        // N s_172_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_173_0: const #22712u : u32
        let s_173_0: u32 = 22712;
        // D s_173_1: read-reg s_173_0:struct
        let s_173_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_173_0 as isize);
            tracer.read_register(s_173_0 as isize, value);
            value
        };
        // D s_173_2: call _get_MDCR_EL3_Type_SCCD(s_173_1)
        let s_173_2: bool = u_get_MDCR_EL3_Type_SCCD(state, tracer, s_173_1);
        // D s_173_3: write-var sccdshadow#24 <= s_173_2
        fn_state.sccdshadow_24 = s_173_2;
        // N s_173_4: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_174_0: read-var ss:u32
        let s_174_0: u32 = fn_state.ss;
        // C s_174_1: const #3u : u32
        let s_174_1: u32 = 3;
        // D s_174_2: cmp-eq s_174_0 s_174_1
        let s_174_2: bool = ((s_174_0) == (s_174_1));
        // D s_174_3: write-var gs#2521 <= s_174_2
        fn_state.gs_2521 = s_174_2;
        // N s_174_4: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_175_0: read-var idx:i64
        let s_175_0: i64 = fn_state.idx;
        // D s_175_1: cast zx s_175_0 -> i
        let s_175_1: i128 = (i128::try_from(s_175_0).unwrap());
        // C s_175_2: const #0u : u32
        let s_175_2: u32 = 0;
        // D s_175_3: read-reg s_175_2:i64
        let s_175_3: i64 = {
            let value = state.read_register::<i64>(s_175_2 as isize);
            tracer.read_register(s_175_2 as isize, value);
            value
        };
        // D s_175_4: cast zx s_175_3 -> i
        let s_175_4: i128 = (i128::try_from(s_175_3).unwrap());
        // D s_175_5: cmp-eq s_175_1 s_175_4
        let s_175_5: bool = ((s_175_1) == (s_175_4));
        // D s_175_6: write-var gs#2514 <= s_175_5
        fn_state.gs_2514 = s_175_5;
        // N s_175_7: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_176_0: const #() : ()
        let s_176_0: () = ();
        // S s_176_1: call HaveAArch64(s_176_0)
        let s_176_1: bool = HaveAArch64(state, tracer, s_176_0);
        // N s_176_2: branch s_176_1 b182 b177
        if s_176_1 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_177_0: const #() : ()
        let s_177_0: () = ();
        // S s_177_1: call PMCR_read(s_177_0)
        let s_177_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_177_0);
        // S s_177_2: call _get_PMCR_Type_DP(s_177_1)
        let s_177_2: bool = u_get_PMCR_Type_DP(state, tracer, s_177_1);
        // D s_177_3: write-var dpshadow#23 <= s_177_2
        fn_state.dpshadow_23 = s_177_2;
        // N s_177_4: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_178_0: read-var enabled:u8
        let s_178_0: bool = fn_state.enabled;
        // N s_178_1: branch s_178_0 b181 b179
        if s_178_0 {
            return block_181(state, tracer, fn_state);
        } else {
            return block_179(state, tracer, fn_state);
        };
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_179_0: const #0u : u8
        let s_179_0: bool = false;
        // D s_179_1: write-var gs#2527 <= s_179_0
        fn_state.gs_2527 = s_179_0;
        // N s_179_2: jump b180
        return block_180(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_180_0: read-var gs#2527:u8
        let s_180_0: bool = fn_state.gs_2527;
        // D s_180_1: write-var enabled <= s_180_0
        fn_state.enabled = s_180_0;
        // C s_180_2: const #0u : u8
        let s_180_2: bool = false;
        // D s_180_3: write-var prohibited <= s_180_2
        fn_state.prohibited = s_180_2;
        // C s_180_4: const #0u : u8
        let s_180_4: bool = false;
        // D s_180_5: write-var frozen <= s_180_4
        fn_state.frozen = s_180_4;
        // N s_180_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_181_0: read-var dpshadow#23:u8
        let s_181_0: bool = fn_state.dpshadow_23;
        // D s_181_1: cast zx s_181_0 -> bv
        let s_181_1: Bits = Bits::new(s_181_0 as u128, 1u16);
        // C s_181_2: const #0u : u8
        let s_181_2: bool = false;
        // C s_181_3: cast zx s_181_2 -> bv
        let s_181_3: Bits = Bits::new(s_181_2 as u128, 1u16);
        // D s_181_4: cmp-eq s_181_1 s_181_3
        let s_181_4: bool = ((s_181_1) == (s_181_3));
        // D s_181_5: write-var gs#2527 <= s_181_4
        fn_state.gs_2527 = s_181_4;
        // N s_181_6: jump b180
        return block_180(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_182_0: const #21016u : u32
        let s_182_0: u32 = 21016;
        // D s_182_1: read-reg s_182_0:struct
        let s_182_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_182_0 as isize);
            tracer.read_register(s_182_0 as isize, value);
            value
        };
        // D s_182_2: call _get_PMCR_EL0_Type_DP(s_182_1)
        let s_182_2: bool = u_get_PMCR_EL0_Type_DP(state, tracer, s_182_1);
        // D s_182_3: write-var dpshadow#23 <= s_182_2
        fn_state.dpshadow_23 = s_182_2;
        // N s_182_4: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_183_0: read-var idx:i64
        let s_183_0: i64 = fn_state.idx;
        // D s_183_1: cast zx s_183_0 -> i
        let s_183_1: i128 = (i128::try_from(s_183_0).unwrap());
        // C s_183_2: const #0u : u32
        let s_183_2: u32 = 0;
        // D s_183_3: read-reg s_183_2:i64
        let s_183_3: i64 = {
            let value = state.read_register::<i64>(s_183_2 as isize);
            tracer.read_register(s_183_2 as isize, value);
            value
        };
        // D s_183_4: cast zx s_183_3 -> i
        let s_183_4: i128 = (i128::try_from(s_183_3).unwrap());
        // D s_183_5: cmp-eq s_183_1 s_183_4
        let s_183_5: bool = ((s_183_1) == (s_183_4));
        // D s_183_6: write-var gs#2513 <= s_183_5
        fn_state.gs_2513 = s_183_5;
        // N s_183_7: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_184_0: const #1u : u8
        let s_184_0: bool = true;
        // D s_184_1: write-var gs#2512 <= s_184_0
        fn_state.gs_2512 = s_184_0;
        // N s_184_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_185_0: read-var resvd_for_el2:u8
        let s_185_0: bool = fn_state.resvd_for_el2;
        // N s_185_1: branch s_185_0 b194 b186
        if s_185_0 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_186(state, tracer, fn_state);
        };
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_186_0: const #() : ()
        let s_186_0: () = ();
        // S s_186_1: call HaveAArch64(s_186_0)
        let s_186_1: bool = HaveAArch64(state, tracer, s_186_0);
        // N s_186_2: branch s_186_1 b193 b187
        if s_186_1 {
            return block_193(state, tracer, fn_state);
        } else {
            return block_187(state, tracer, fn_state);
        };
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_187_0: const #() : ()
        let s_187_0: () = ();
        // S s_187_1: call PMCR_read(s_187_0)
        let s_187_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_187_0);
        // S s_187_2: call _get_PMCR_Type_FZO(s_187_1)
        let s_187_2: bool = u_get_PMCR_Type_FZO(state, tracer, s_187_1);
        // D s_187_3: write-var FZ <= s_187_2
        fn_state.FZ = s_187_2;
        // N s_187_4: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_188_0: jump b189
        return block_189(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_189_0: read-var FZ:u8
        let s_189_0: bool = fn_state.FZ;
        // D s_189_1: cast zx s_189_0 -> bv
        let s_189_1: Bits = Bits::new(s_189_0 as u128, 1u16);
        // C s_189_2: const #1u : u8
        let s_189_2: bool = true;
        // C s_189_3: cast zx s_189_2 -> bv
        let s_189_3: Bits = Bits::new(s_189_2 as u128, 1u16);
        // D s_189_4: cmp-eq s_189_1 s_189_3
        let s_189_4: bool = ((s_189_1) == (s_189_3));
        // N s_189_5: branch s_189_4 b192 b190
        if s_189_4 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_190(state, tracer, fn_state);
        };
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_190_0: const #0u : u8
        let s_190_0: bool = false;
        // D s_190_1: write-var gs#2532 <= s_190_0
        fn_state.gs_2532 = s_190_0;
        // N s_190_2: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_191_0: read-var gs#2532:u8
        let s_191_0: bool = fn_state.gs_2532;
        // D s_191_1: write-var frozen <= s_191_0
        fn_state.frozen = s_191_0;
        // N s_191_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_192_0: read-var resvd_for_el2:u8
        let s_192_0: bool = fn_state.resvd_for_el2;
        // D s_192_1: call HiLoPMUOverflow(s_192_0)
        let s_192_1: bool = HiLoPMUOverflow(state, tracer, s_192_0);
        // D s_192_2: write-var gs#2532 <= s_192_1
        fn_state.gs_2532 = s_192_1;
        // N s_192_3: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_193_0: const #21016u : u32
        let s_193_0: u32 = 21016;
        // D s_193_1: read-reg s_193_0:struct
        let s_193_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_193_0 as isize);
            tracer.read_register(s_193_0 as isize, value);
            value
        };
        // D s_193_2: call _get_PMCR_EL0_Type_FZO(s_193_1)
        let s_193_2: bool = u_get_PMCR_EL0_Type_FZO(state, tracer, s_193_1);
        // D s_193_3: write-var FZ <= s_193_2
        fn_state.FZ = s_193_2;
        // N s_193_4: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_194_0: const #() : ()
        let s_194_0: () = ();
        // S s_194_1: call HaveAArch64(s_194_0)
        let s_194_1: bool = HaveAArch64(state, tracer, s_194_0);
        // N s_194_2: branch s_194_1 b197 b195
        if s_194_1 {
            return block_197(state, tracer, fn_state);
        } else {
            return block_195(state, tracer, fn_state);
        };
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_195_0: const #() : ()
        let s_195_0: () = ();
        // S s_195_1: call HDCR_read(s_195_0)
        let s_195_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_195_0);
        // S s_195_2: call _get_HDCR_Type_HPMFZO(s_195_1)
        let s_195_2: bool = u_get_HDCR_Type_HPMFZO(state, tracer, s_195_1);
        // D s_195_3: write-var FZ <= s_195_2
        fn_state.FZ = s_195_2;
        // N s_195_4: jump b196
        return block_196(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_196_0: jump b189
        return block_189(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_197_0: const #104880u : u32
        let s_197_0: u32 = 104880;
        // D s_197_1: read-reg s_197_0:struct
        let s_197_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_197_0 as isize);
            tracer.read_register(s_197_0 as isize, value);
            value
        };
        // D s_197_2: call _get_MDCR_EL2_Type_HPMFZO(s_197_1)
        let s_197_2: bool = u_get_MDCR_EL2_Type_HPMFZO(state, tracer, s_197_1);
        // D s_197_3: write-var FZ <= s_197_2
        fn_state.FZ = s_197_2;
        // N s_197_4: jump b196
        return block_196(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_198_0: const #() : ()
        let s_198_0: () = ();
        // S s_198_1: call ExternalSecureNoninvasiveDebugEnabled(s_198_0)
        let s_198_1: bool = ExternalSecureNoninvasiveDebugEnabled(
            state,
            tracer,
            s_198_0,
        );
        // S s_198_2: not s_198_1
        let s_198_2: bool = !s_198_1;
        // D s_198_3: write-var prohibited <= s_198_2
        fn_state.prohibited = s_198_2;
        // N s_198_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_199_0: const #() : ()
        let s_199_0: () = ();
        // S s_199_1: call HaveNoSecurePMUDisableOverride(s_199_0)
        let s_199_1: bool = HaveNoSecurePMUDisableOverride(state, tracer, s_199_0);
        // S s_199_2: not s_199_1
        let s_199_2: bool = !s_199_1;
        // D s_199_3: write-var gs#2500 <= s_199_2
        fn_state.gs_2500 = s_199_2;
        // N s_199_4: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_200_0: const #() : ()
        let s_200_0: () = ();
        // S s_200_1: call HaveAArch64(s_200_0)
        let s_200_1: bool = HaveAArch64(state, tracer, s_200_0);
        // N s_200_2: branch s_200_1 b203 b201
        if s_200_1 {
            return block_203(state, tracer, fn_state);
        } else {
            return block_201(state, tracer, fn_state);
        };
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_201_0: const #() : ()
        let s_201_0: () = ();
        // S s_201_1: call HDCR_read(s_201_0)
        let s_201_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_201_0);
        // S s_201_2: call _get_HDCR_Type_HPMD(s_201_1)
        let s_201_2: bool = u_get_HDCR_Type_HPMD(state, tracer, s_201_1);
        // D s_201_3: write-var hpmdshadow#26 <= s_201_2
        fn_state.hpmdshadow_26 = s_201_2;
        // N s_201_4: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_202_0: read-var hpmdshadow#26:u8
        let s_202_0: bool = fn_state.hpmdshadow_26;
        // D s_202_1: cast zx s_202_0 -> bv
        let s_202_1: Bits = Bits::new(s_202_0 as u128, 1u16);
        // C s_202_2: const #1u : u8
        let s_202_2: bool = true;
        // C s_202_3: cast zx s_202_2 -> bv
        let s_202_3: Bits = Bits::new(s_202_2 as u128, 1u16);
        // D s_202_4: cmp-eq s_202_1 s_202_3
        let s_202_4: bool = ((s_202_1) == (s_202_3));
        // D s_202_5: write-var prohibited <= s_202_4
        fn_state.prohibited = s_202_4;
        // N s_202_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_203_0: const #104880u : u32
        let s_203_0: u32 = 104880;
        // D s_203_1: read-reg s_203_0:struct
        let s_203_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_203_0 as isize);
            tracer.read_register(s_203_0 as isize, value);
            value
        };
        // D s_203_2: call _get_MDCR_EL2_Type_HPMD(s_203_1)
        let s_203_2: bool = u_get_MDCR_EL2_Type_HPMD(state, tracer, s_203_1);
        // D s_203_3: write-var hpmdshadow#26 <= s_203_2
        fn_state.hpmdshadow_26 = s_203_2;
        // N s_203_4: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_204_0: read-var resvd_for_el2:u8
        let s_204_0: bool = fn_state.resvd_for_el2;
        // D s_204_1: not s_204_0
        let s_204_1: bool = !s_204_0;
        // D s_204_2: write-var gs#2499 <= s_204_1
        fn_state.gs_2499 = s_204_1;
        // N s_204_3: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_205_0: const #() : ()
        let s_205_0: () = ();
        // S s_205_1: call HaveHPMDExt(s_205_0)
        let s_205_1: bool = HaveHPMDExt(state, tracer, s_205_0);
        // D s_205_2: write-var gs#2498 <= s_205_1
        fn_state.gs_2498 = s_205_1;
        // N s_205_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_206_0: const #16975u : u32
        let s_206_0: u32 = 16975;
        // D s_206_1: read-reg s_206_0:u8
        let s_206_1: u8 = {
            let value = state.read_register::<u8>(s_206_0 as isize);
            tracer.read_register(s_206_0 as isize, value);
            value
        };
        // D s_206_2: cast zx s_206_1 -> bv
        let s_206_2: Bits = Bits::new(s_206_1 as u128, 2u16);
        // C s_206_3: const #432u : u32
        let s_206_3: u32 = 432;
        // D s_206_4: read-reg s_206_3:u8
        let s_206_4: u8 = {
            let value = state.read_register::<u8>(s_206_3 as isize);
            tracer.read_register(s_206_3 as isize, value);
            value
        };
        // D s_206_5: cast zx s_206_4 -> bv
        let s_206_5: Bits = Bits::new(s_206_4 as u128, 2u16);
        // D s_206_6: cmp-eq s_206_2 s_206_5
        let s_206_6: bool = ((s_206_2) == (s_206_5));
        // D s_206_7: write-var gs#2497 <= s_206_6
        fn_state.gs_2497 = s_206_6;
        // N s_206_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_207_0: const #22712u : u32
        let s_207_0: u32 = 22712;
        // D s_207_1: read-reg s_207_0:struct
        let s_207_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_207_0 as isize);
            tracer.read_register(s_207_0 as isize, value);
            value
        };
        // D s_207_2: call _get_MDCR_EL3_Type_MPMX(s_207_1)
        let s_207_2: bool = u_get_MDCR_EL3_Type_MPMX(state, tracer, s_207_1);
        // D s_207_3: cast zx s_207_2 -> bv
        let s_207_3: Bits = Bits::new(s_207_2 as u128, 1u16);
        // C s_207_4: const #1u : u8
        let s_207_4: bool = true;
        // C s_207_5: cast zx s_207_4 -> bv
        let s_207_5: Bits = Bits::new(s_207_4 as u128, 1u16);
        // D s_207_6: cmp-eq s_207_3 s_207_5
        let s_207_6: bool = ((s_207_3) == (s_207_5));
        // N s_207_7: branch s_207_6 b210 b208
        if s_207_6 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_208(state, tracer, fn_state);
        };
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_208_0: const #0u : u8
        let s_208_0: bool = false;
        // D s_208_1: write-var gs#2505 <= s_208_0
        fn_state.gs_2505 = s_208_0;
        // N s_208_2: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_209_0: read-var gs#2505:u8
        let s_209_0: bool = fn_state.gs_2505;
        // D s_209_1: write-var prohibited <= s_209_0
        fn_state.prohibited = s_209_0;
        // N s_209_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_210_0: const #22712u : u32
        let s_210_0: u32 = 22712;
        // D s_210_1: read-reg s_210_0:struct
        let s_210_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_210_0 as isize);
            tracer.read_register(s_210_0 as isize, value);
            value
        };
        // D s_210_2: call _get_MDCR_EL3_Type_SPME(s_210_1)
        let s_210_2: bool = u_get_MDCR_EL3_Type_SPME(state, tracer, s_210_1);
        // D s_210_3: cast zx s_210_2 -> bv
        let s_210_3: Bits = Bits::new(s_210_2 as u128, 1u16);
        // C s_210_4: const #0u : u8
        let s_210_4: bool = false;
        // C s_210_5: cast zx s_210_4 -> bv
        let s_210_5: Bits = Bits::new(s_210_4 as u128, 1u16);
        // D s_210_6: cmp-eq s_210_3 s_210_5
        let s_210_6: bool = ((s_210_3) == (s_210_5));
        // N s_210_7: branch s_210_6 b213 b211
        if s_210_6 {
            return block_213(state, tracer, fn_state);
        } else {
            return block_211(state, tracer, fn_state);
        };
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_211_0: read-var resvd_for_el2:u8
        let s_211_0: bool = fn_state.resvd_for_el2;
        // D s_211_1: not s_211_0
        let s_211_1: bool = !s_211_0;
        // D s_211_2: write-var gs#2504 <= s_211_1
        fn_state.gs_2504 = s_211_1;
        // N s_211_3: jump b212
        return block_212(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_212_0: read-var gs#2504:u8
        let s_212_0: bool = fn_state.gs_2504;
        // D s_212_1: write-var gs#2505 <= s_212_0
        fn_state.gs_2505 = s_212_0;
        // N s_212_2: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_213_0: const #1u : u8
        let s_213_0: bool = true;
        // D s_213_1: write-var gs#2504 <= s_213_0
        fn_state.gs_2504 = s_213_0;
        // N s_213_2: jump b212
        return block_212(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_214_0: const #() : ()
        let s_214_0: () = ();
        // S s_214_1: call HaveAArch64(s_214_0)
        let s_214_1: bool = HaveAArch64(state, tracer, s_214_0);
        // D s_214_2: write-var gs#2496 <= s_214_1
        fn_state.gs_2496 = s_214_1;
        // N s_214_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_215_0: const #16975u : u32
        let s_215_0: u32 = 16975;
        // D s_215_1: read-reg s_215_0:u8
        let s_215_1: u8 = {
            let value = state.read_register::<u8>(s_215_0 as isize);
            tracer.read_register(s_215_0 as isize, value);
            value
        };
        // D s_215_2: cast zx s_215_1 -> bv
        let s_215_2: Bits = Bits::new(s_215_1 as u128, 2u16);
        // C s_215_3: const #424u : u32
        let s_215_3: u32 = 424;
        // D s_215_4: read-reg s_215_3:u8
        let s_215_4: u8 = {
            let value = state.read_register::<u8>(s_215_3 as isize);
            tracer.read_register(s_215_3 as isize, value);
            value
        };
        // D s_215_5: cast zx s_215_4 -> bv
        let s_215_5: Bits = Bits::new(s_215_4 as u128, 2u16);
        // D s_215_6: cmp-eq s_215_2 s_215_5
        let s_215_6: bool = ((s_215_2) == (s_215_5));
        // D s_215_7: write-var gs#2495 <= s_215_6
        fn_state.gs_2495 = s_215_6;
        // N s_215_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_216_0: const #() : ()
        let s_216_0: () = ();
        // S s_216_1: call HavePMUv3p7(s_216_0)
        let s_216_1: bool = HavePMUv3p7(state, tracer, s_216_0);
        // D s_216_2: write-var gs#2494 <= s_216_1
        fn_state.gs_2494 = s_216_1;
        // N s_216_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_217_0: const #424u : u32
        let s_217_0: u32 = 424;
        // D s_217_1: read-reg s_217_0:u8
        let s_217_1: u8 = {
            let value = state.read_register::<u8>(s_217_0 as isize);
            tracer.read_register(s_217_0 as isize, value);
            value
        };
        // D s_217_2: call ELUsingAArch32(s_217_1)
        let s_217_2: bool = ELUsingAArch32(state, tracer, s_217_1);
        // D s_217_3: not s_217_2
        let s_217_3: bool = !s_217_2;
        // N s_217_4: branch s_217_3 b230 b218
        if s_217_3 {
            return block_230(state, tracer, fn_state);
        } else {
            return block_218(state, tracer, fn_state);
        };
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_218_0: const #15048u : u32
        let s_218_0: u32 = 15048;
        // D s_218_1: read-reg s_218_0:struct
        let s_218_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_218_0 as isize);
            tracer.read_register(s_218_0 as isize, value);
            value
        };
        // D s_218_2: call _get_SDCR_Type_SPME(s_218_1)
        let s_218_2: bool = u_get_SDCR_Type_SPME(state, tracer, s_218_1);
        // D s_218_3: cast zx s_218_2 -> bv
        let s_218_3: Bits = Bits::new(s_218_2 as u128, 1u16);
        // C s_218_4: const #0u : u8
        let s_218_4: bool = false;
        // C s_218_5: cast zx s_218_4 -> bv
        let s_218_5: Bits = Bits::new(s_218_4 as u128, 1u16);
        // D s_218_6: cmp-eq s_218_3 s_218_5
        let s_218_6: bool = ((s_218_3) == (s_218_5));
        // D s_218_7: write-var prohibited <= s_218_6
        fn_state.prohibited = s_218_6;
        // N s_218_8: jump b219
        return block_219(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_219_0: read-var prohibited:u8
        let s_219_0: bool = fn_state.prohibited;
        // N s_219_1: branch s_219_0 b229 b220
        if s_219_0 {
            return block_229(state, tracer, fn_state);
        } else {
            return block_220(state, tracer, fn_state);
        };
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_220_0: const #0u : u8
        let s_220_0: bool = false;
        // D s_220_1: write-var gs#2507 <= s_220_0
        fn_state.gs_2507 = s_220_0;
        // N s_220_2: jump b221
        return block_221(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_221_0: read-var gs#2507:u8
        let s_221_0: bool = fn_state.gs_2507;
        // N s_221_1: branch s_221_0 b224 b222
        if s_221_0 {
            return block_224(state, tracer, fn_state);
        } else {
            return block_222(state, tracer, fn_state);
        };
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_222_0: jump b223
        return block_223(state, tracer, fn_state);
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_223_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_224_0: const #424u : u32
        let s_224_0: u32 = 424;
        // D s_224_1: read-reg s_224_0:u8
        let s_224_1: u8 = {
            let value = state.read_register::<u8>(s_224_0 as isize);
            tracer.read_register(s_224_0 as isize, value);
            value
        };
        // D s_224_2: call ELUsingAArch32(s_224_1)
        let s_224_2: bool = ELUsingAArch32(state, tracer, s_224_1);
        // N s_224_3: branch s_224_2 b228 b225
        if s_224_2 {
            return block_228(state, tracer, fn_state);
        } else {
            return block_225(state, tracer, fn_state);
        };
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_225_0: const #440u : u32
        let s_225_0: u32 = 440;
        // D s_225_1: read-reg s_225_0:u8
        let s_225_1: u8 = {
            let value = state.read_register::<u8>(s_225_0 as isize);
            tracer.read_register(s_225_0 as isize, value);
            value
        };
        // D s_225_2: call ELUsingAArch32(s_225_1)
        let s_225_2: bool = ELUsingAArch32(state, tracer, s_225_1);
        // N s_225_3: branch s_225_2 b227 b226
        if s_225_2 {
            return block_227(state, tracer, fn_state);
        } else {
            return block_226(state, tracer, fn_state);
        };
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_226_0: jump b223
        return block_223(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_227_0: const #() : ()
        let s_227_0: () = ();
        // S s_227_1: call SDER32_EL3_read(s_227_0)
        let s_227_1: ProductType5c790c8ef59cc8b2 = SDER32_EL3_read(
            state,
            tracer,
            s_227_0,
        );
        // S s_227_2: call _get_SDER32_EL3_Type_SUNIDEN(s_227_1)
        let s_227_2: bool = u_get_SDER32_EL3_Type_SUNIDEN(state, tracer, s_227_1);
        // S s_227_3: cast zx s_227_2 -> bv
        let s_227_3: Bits = Bits::new(s_227_2 as u128, 1u16);
        // C s_227_4: const #0u : u8
        let s_227_4: bool = false;
        // C s_227_5: cast zx s_227_4 -> bv
        let s_227_5: Bits = Bits::new(s_227_4 as u128, 1u16);
        // S s_227_6: cmp-eq s_227_3 s_227_5
        let s_227_6: bool = ((s_227_3) == (s_227_5));
        // D s_227_7: write-var prohibited <= s_227_6
        fn_state.prohibited = s_227_6;
        // N s_227_8: jump b223
        return block_223(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_228_0: const #() : ()
        let s_228_0: () = ();
        // S s_228_1: call SDER_read(s_228_0)
        let s_228_1: ProductType700c18a878c5601b = SDER_read(state, tracer, s_228_0);
        // S s_228_2: call _get_SDER_Type_SUNIDEN(s_228_1)
        let s_228_2: bool = u_get_SDER_Type_SUNIDEN(state, tracer, s_228_1);
        // S s_228_3: cast zx s_228_2 -> bv
        let s_228_3: Bits = Bits::new(s_228_2 as u128, 1u16);
        // C s_228_4: const #0u : u8
        let s_228_4: bool = false;
        // C s_228_5: cast zx s_228_4 -> bv
        let s_228_5: Bits = Bits::new(s_228_4 as u128, 1u16);
        // S s_228_6: cmp-eq s_228_3 s_228_5
        let s_228_6: bool = ((s_228_3) == (s_228_5));
        // D s_228_7: write-var prohibited <= s_228_6
        fn_state.prohibited = s_228_6;
        // N s_228_8: jump b223
        return block_223(state, tracer, fn_state);
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_229_0: const #16975u : u32
        let s_229_0: u32 = 16975;
        // D s_229_1: read-reg s_229_0:u8
        let s_229_1: u8 = {
            let value = state.read_register::<u8>(s_229_0 as isize);
            tracer.read_register(s_229_0 as isize, value);
            value
        };
        // D s_229_2: cast zx s_229_1 -> bv
        let s_229_2: Bits = Bits::new(s_229_1 as u128, 2u16);
        // C s_229_3: const #448u : u32
        let s_229_3: u32 = 448;
        // D s_229_4: read-reg s_229_3:u8
        let s_229_4: u8 = {
            let value = state.read_register::<u8>(s_229_3 as isize);
            tracer.read_register(s_229_3 as isize, value);
            value
        };
        // D s_229_5: cast zx s_229_4 -> bv
        let s_229_5: Bits = Bits::new(s_229_4 as u128, 2u16);
        // D s_229_6: cmp-eq s_229_2 s_229_5
        let s_229_6: bool = ((s_229_2) == (s_229_5));
        // D s_229_7: write-var gs#2507 <= s_229_6
        fn_state.gs_2507 = s_229_6;
        // N s_229_8: jump b221
        return block_221(state, tracer, fn_state);
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_230_0: const #22712u : u32
        let s_230_0: u32 = 22712;
        // D s_230_1: read-reg s_230_0:struct
        let s_230_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_230_0 as isize);
            tracer.read_register(s_230_0 as isize, value);
            value
        };
        // D s_230_2: call _get_MDCR_EL3_Type_SPME(s_230_1)
        let s_230_2: bool = u_get_MDCR_EL3_Type_SPME(state, tracer, s_230_1);
        // D s_230_3: cast zx s_230_2 -> bv
        let s_230_3: Bits = Bits::new(s_230_2 as u128, 1u16);
        // C s_230_4: const #0u : u8
        let s_230_4: bool = false;
        // C s_230_5: cast zx s_230_4 -> bv
        let s_230_5: Bits = Bits::new(s_230_4 as u128, 1u16);
        // D s_230_6: cmp-eq s_230_3 s_230_5
        let s_230_6: bool = ((s_230_3) == (s_230_5));
        // N s_230_7: branch s_230_6 b236 b231
        if s_230_6 {
            return block_236(state, tracer, fn_state);
        } else {
            return block_231(state, tracer, fn_state);
        };
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_231_0: const #0u : u8
        let s_231_0: bool = false;
        // D s_231_1: write-var gs#2510 <= s_231_0
        fn_state.gs_2510 = s_231_0;
        // N s_231_2: jump b232
        return block_232(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_232_0: read-var gs#2510:u8
        let s_232_0: bool = fn_state.gs_2510;
        // N s_232_1: branch s_232_0 b235 b233
        if s_232_0 {
            return block_235(state, tracer, fn_state);
        } else {
            return block_233(state, tracer, fn_state);
        };
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_233_0: const #0u : u8
        let s_233_0: bool = false;
        // D s_233_1: write-var gs#2511 <= s_233_0
        fn_state.gs_2511 = s_233_0;
        // N s_233_2: jump b234
        return block_234(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_234_0: read-var gs#2511:u8
        let s_234_0: bool = fn_state.gs_2511;
        // D s_234_1: write-var prohibited <= s_234_0
        fn_state.prohibited = s_234_0;
        // N s_234_2: jump b219
        return block_219(state, tracer, fn_state);
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_235_0: const #22712u : u32
        let s_235_0: u32 = 22712;
        // D s_235_1: read-reg s_235_0:struct
        let s_235_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_235_0 as isize);
            tracer.read_register(s_235_0 as isize, value);
            value
        };
        // D s_235_2: call _get_MDCR_EL3_Type_MPMX(s_235_1)
        let s_235_2: bool = u_get_MDCR_EL3_Type_MPMX(state, tracer, s_235_1);
        // D s_235_3: cast zx s_235_2 -> bv
        let s_235_3: Bits = Bits::new(s_235_2 as u128, 1u16);
        // C s_235_4: const #0u : u8
        let s_235_4: bool = false;
        // C s_235_5: cast zx s_235_4 -> bv
        let s_235_5: Bits = Bits::new(s_235_4 as u128, 1u16);
        // D s_235_6: cmp-eq s_235_3 s_235_5
        let s_235_6: bool = ((s_235_3) == (s_235_5));
        // D s_235_7: write-var gs#2511 <= s_235_6
        fn_state.gs_2511 = s_235_6;
        // N s_235_8: jump b234
        return block_234(state, tracer, fn_state);
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_236_0: const #() : ()
        let s_236_0: () = ();
        // S s_236_1: call HavePMUv3p7(s_236_0)
        let s_236_1: bool = HavePMUv3p7(state, tracer, s_236_0);
        // D s_236_2: write-var gs#2510 <= s_236_1
        fn_state.gs_2510 = s_236_1;
        // N s_236_3: jump b232
        return block_232(state, tracer, fn_state);
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_237_0: read-var ss:u32
        let s_237_0: u32 = fn_state.ss;
        // C s_237_1: const #3u : u32
        let s_237_1: u32 = 3;
        // D s_237_2: cmp-eq s_237_0 s_237_1
        let s_237_2: bool = ((s_237_0) == (s_237_1));
        // D s_237_3: write-var gs#2493 <= s_237_2
        fn_state.gs_2493 = s_237_2;
        // N s_237_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_238_0: const #11736u : u32
        let s_238_0: u32 = 11736;
        // D s_238_1: read-reg s_238_0:struct
        let s_238_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_238_0 as isize);
            tracer.read_register(s_238_0 as isize, value);
            value
        };
        // D s_238_2: call _get_PMCNTENSET_EL0_Type_F0(s_238_1)
        let s_238_2: bool = u_get_PMCNTENSET_EL0_Type_F0(state, tracer, s_238_1);
        // D s_238_3: cast zx s_238_2 -> bv
        let s_238_3: Bits = Bits::new(s_238_2 as u128, 1u16);
        // C s_238_4: const #1u : u8
        let s_238_4: bool = true;
        // C s_238_5: cast zx s_238_4 -> bv
        let s_238_5: Bits = Bits::new(s_238_4 as u128, 1u16);
        // D s_238_6: cmp-eq s_238_3 s_238_5
        let s_238_6: bool = ((s_238_3) == (s_238_5));
        // D s_238_7: write-var gs#2477 <= s_238_6
        fn_state.gs_2477 = s_238_6;
        // N s_238_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_239_0: read-var idx:i64
        let s_239_0: i64 = fn_state.idx;
        // D s_239_1: cast zx s_239_0 -> i
        let s_239_1: i128 = (i128::try_from(s_239_0).unwrap());
        // C s_239_2: const #0u : u32
        let s_239_2: u32 = 0;
        // D s_239_3: read-reg s_239_2:i64
        let s_239_3: i64 = {
            let value = state.read_register::<i64>(s_239_2 as isize);
            tracer.read_register(s_239_2 as isize, value);
            value
        };
        // D s_239_4: cast zx s_239_3 -> i
        let s_239_4: i128 = (i128::try_from(s_239_3).unwrap());
        // D s_239_5: cmp-eq s_239_1 s_239_4
        let s_239_5: bool = ((s_239_1) == (s_239_4));
        // D s_239_6: not s_239_5
        let s_239_6: bool = !s_239_5;
        // N s_239_7: branch s_239_6 b249 b240
        if s_239_6 {
            return block_249(state, tracer, fn_state);
        } else {
            return block_240(state, tracer, fn_state);
        };
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_240_0: const #() : ()
        let s_240_0: () = ();
        // S s_240_1: call HaveAArch64(s_240_0)
        let s_240_1: bool = HaveAArch64(state, tracer, s_240_0);
        // N s_240_2: branch s_240_1 b245 b241
        if s_240_1 {
            return block_245(state, tracer, fn_state);
        } else {
            return block_241(state, tracer, fn_state);
        };
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_241_0: const #() : ()
        let s_241_0: () = ();
        // S s_241_1: call PMCR_read(s_241_0)
        let s_241_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_241_0);
        // S s_241_2: call _get_PMCR_Type_E(s_241_1)
        let s_241_2: bool = u_get_PMCR_Type_E(state, tracer, s_241_1);
        // S s_241_3: cast zx s_241_2 -> bv
        let s_241_3: Bits = Bits::new(s_241_2 as u128, 1u16);
        // C s_241_4: const #1u : u8
        let s_241_4: bool = true;
        // C s_241_5: cast zx s_241_4 -> bv
        let s_241_5: Bits = Bits::new(s_241_4 as u128, 1u16);
        // S s_241_6: cmp-eq s_241_3 s_241_5
        let s_241_6: bool = ((s_241_3) == (s_241_5));
        // N s_241_7: branch s_241_6 b244 b242
        if s_241_6 {
            return block_244(state, tracer, fn_state);
        } else {
            return block_242(state, tracer, fn_state);
        };
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_242_0: const #0u : u8
        let s_242_0: bool = false;
        // D s_242_1: write-var gs#2480 <= s_242_0
        fn_state.gs_2480 = s_242_0;
        // N s_242_2: jump b243
        return block_243(state, tracer, fn_state);
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_243_0: read-var gs#2480:u8
        let s_243_0: bool = fn_state.gs_2480;
        // D s_243_1: write-var enabled <= s_243_0
        fn_state.enabled = s_243_0;
        // N s_243_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_244_0: const #() : ()
        let s_244_0: () = ();
        // S s_244_1: call PMCNTENSET_read(s_244_0)
        let s_244_1: ProductType700c18a878c5601b = PMCNTENSET_read(
            state,
            tracer,
            s_244_0,
        );
        // S s_244_2: call _get_PMCNTENSET_Type_C(s_244_1)
        let s_244_2: bool = u_get_PMCNTENSET_Type_C(state, tracer, s_244_1);
        // S s_244_3: cast zx s_244_2 -> bv
        let s_244_3: Bits = Bits::new(s_244_2 as u128, 1u16);
        // C s_244_4: const #1u : u8
        let s_244_4: bool = true;
        // C s_244_5: cast zx s_244_4 -> bv
        let s_244_5: Bits = Bits::new(s_244_4 as u128, 1u16);
        // S s_244_6: cmp-eq s_244_3 s_244_5
        let s_244_6: bool = ((s_244_3) == (s_244_5));
        // D s_244_7: write-var gs#2480 <= s_244_6
        fn_state.gs_2480 = s_244_6;
        // N s_244_8: jump b243
        return block_243(state, tracer, fn_state);
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_245_0: const #21016u : u32
        let s_245_0: u32 = 21016;
        // D s_245_1: read-reg s_245_0:struct
        let s_245_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_245_0 as isize);
            tracer.read_register(s_245_0 as isize, value);
            value
        };
        // D s_245_2: call _get_PMCR_EL0_Type_E(s_245_1)
        let s_245_2: bool = u_get_PMCR_EL0_Type_E(state, tracer, s_245_1);
        // D s_245_3: cast zx s_245_2 -> bv
        let s_245_3: Bits = Bits::new(s_245_2 as u128, 1u16);
        // C s_245_4: const #1u : u8
        let s_245_4: bool = true;
        // C s_245_5: cast zx s_245_4 -> bv
        let s_245_5: Bits = Bits::new(s_245_4 as u128, 1u16);
        // D s_245_6: cmp-eq s_245_3 s_245_5
        let s_245_6: bool = ((s_245_3) == (s_245_5));
        // N s_245_7: branch s_245_6 b248 b246
        if s_245_6 {
            return block_248(state, tracer, fn_state);
        } else {
            return block_246(state, tracer, fn_state);
        };
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_246_0: const #0u : u8
        let s_246_0: bool = false;
        // D s_246_1: write-var gs#2481 <= s_246_0
        fn_state.gs_2481 = s_246_0;
        // N s_246_2: jump b247
        return block_247(state, tracer, fn_state);
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_247_0: read-var gs#2481:u8
        let s_247_0: bool = fn_state.gs_2481;
        // D s_247_1: write-var enabled <= s_247_0
        fn_state.enabled = s_247_0;
        // N s_247_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_248_0: const #11736u : u32
        let s_248_0: u32 = 11736;
        // D s_248_1: read-reg s_248_0:struct
        let s_248_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_248_0 as isize);
            tracer.read_register(s_248_0 as isize, value);
            value
        };
        // D s_248_2: call _get_PMCNTENSET_EL0_Type_C(s_248_1)
        let s_248_2: bool = u_get_PMCNTENSET_EL0_Type_C(state, tracer, s_248_1);
        // D s_248_3: cast zx s_248_2 -> bv
        let s_248_3: Bits = Bits::new(s_248_2 as u128, 1u16);
        // C s_248_4: const #1u : u8
        let s_248_4: bool = true;
        // C s_248_5: cast zx s_248_4 -> bv
        let s_248_5: Bits = Bits::new(s_248_4 as u128, 1u16);
        // D s_248_6: cmp-eq s_248_3 s_248_5
        let s_248_6: bool = ((s_248_3) == (s_248_5));
        // D s_248_7: write-var gs#2481 <= s_248_6
        fn_state.gs_2481 = s_248_6;
        // N s_248_8: jump b247
        return block_247(state, tracer, fn_state);
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_249_0: read-var resvd_for_el2:u8
        let s_249_0: bool = fn_state.resvd_for_el2;
        // N s_249_1: branch s_249_0 b266 b250
        if s_249_0 {
            return block_266(state, tracer, fn_state);
        } else {
            return block_250(state, tracer, fn_state);
        };
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_250_0: const #() : ()
        let s_250_0: () = ();
        // S s_250_1: call HaveAArch64(s_250_0)
        let s_250_1: bool = HaveAArch64(state, tracer, s_250_0);
        // N s_250_2: branch s_250_1 b265 b251
        if s_250_1 {
            return block_265(state, tracer, fn_state);
        } else {
            return block_251(state, tracer, fn_state);
        };
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_251_0: const #() : ()
        let s_251_0: () = ();
        // S s_251_1: call PMCR_read(s_251_0)
        let s_251_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_251_0);
        // S s_251_2: call _get_PMCR_Type_E(s_251_1)
        let s_251_2: bool = u_get_PMCR_Type_E(state, tracer, s_251_1);
        // D s_251_3: write-var E <= s_251_2
        fn_state.E = s_251_2;
        // N s_251_4: jump b252
        return block_252(state, tracer, fn_state);
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_252_0: jump b253
        return block_253(state, tracer, fn_state);
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_253_0: const #() : ()
        let s_253_0: () = ();
        // S s_253_1: call HaveAArch64(s_253_0)
        let s_253_1: bool = HaveAArch64(state, tracer, s_253_0);
        // N s_253_2: branch s_253_1 b261 b254
        if s_253_1 {
            return block_261(state, tracer, fn_state);
        } else {
            return block_254(state, tracer, fn_state);
        };
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_254_0: read-var idx:i64
        let s_254_0: i64 = fn_state.idx;
        // D s_254_1: cast zx s_254_0 -> i
        let s_254_1: i128 = (i128::try_from(s_254_0).unwrap());
        // D s_254_2: call __id(s_254_1)
        let s_254_2: i128 = u__id(state, tracer, s_254_1);
        // D s_254_3: cast reint s_254_2 -> i64
        let s_254_3: i64 = (s_254_2 as i64);
        // C s_254_4: const #0s : i
        let s_254_4: i128 = 0;
        // D s_254_5: cast zx s_254_3 -> i
        let s_254_5: i128 = (i128::try_from(s_254_3).unwrap());
        // D s_254_6: cmp-le s_254_4 s_254_5
        let s_254_6: bool = ((s_254_4) <= (s_254_5));
        // N s_254_7: branch s_254_6 b260 b255
        if s_254_6 {
            return block_260(state, tracer, fn_state);
        } else {
            return block_255(state, tracer, fn_state);
        };
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_255_0: const #0u : u8
        let s_255_0: bool = false;
        // D s_255_1: write-var gs#2486 <= s_255_0
        fn_state.gs_2486 = s_255_0;
        // N s_255_2: jump b256
        return block_256(state, tracer, fn_state);
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_256_0: read-var gs#2486:u8
        let s_256_0: bool = fn_state.gs_2486;
        // N s_256_1: assert s_256_0
        let s_256_1: () = assert!(s_256_0);
        // D s_256_2: read-var E:u8
        let s_256_2: bool = fn_state.E;
        // D s_256_3: cast zx s_256_2 -> bv
        let s_256_3: Bits = Bits::new(s_256_2 as u128, 1u16);
        // C s_256_4: const #1u : u8
        let s_256_4: bool = true;
        // C s_256_5: cast zx s_256_4 -> bv
        let s_256_5: Bits = Bits::new(s_256_4 as u128, 1u16);
        // D s_256_6: cmp-eq s_256_3 s_256_5
        let s_256_6: bool = ((s_256_3) == (s_256_5));
        // N s_256_7: branch s_256_6 b259 b257
        if s_256_6 {
            return block_259(state, tracer, fn_state);
        } else {
            return block_257(state, tracer, fn_state);
        };
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_257_0: const #0u : u8
        let s_257_0: bool = false;
        // D s_257_1: write-var gs#2489 <= s_257_0
        fn_state.gs_2489 = s_257_0;
        // N s_257_2: jump b258
        return block_258(state, tracer, fn_state);
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_258_0: read-var gs#2489:u8
        let s_258_0: bool = fn_state.gs_2489;
        // D s_258_1: write-var enabled <= s_258_0
        fn_state.enabled = s_258_0;
        // N s_258_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_259_0: const #() : ()
        let s_259_0: () = ();
        // S s_259_1: call PMCNTENSET_read(s_259_0)
        let s_259_1: ProductType700c18a878c5601b = PMCNTENSET_read(
            state,
            tracer,
            s_259_0,
        );
        // D s_259_2: write-var ga#1399 <= s_259_1
        fn_state.ga_1399 = s_259_1;
        // D s_259_3: read-var ga#1399.0:struct
        let s_259_3: u32 = fn_state.ga_1399._0;
        // D s_259_4: cast zx s_259_3 -> bv
        let s_259_4: Bits = Bits::new(s_259_3 as u128, 32u16);
        // D s_259_5: read-var idx:i64
        let s_259_5: i64 = fn_state.idx;
        // D s_259_6: cast zx s_259_5 -> i
        let s_259_6: i128 = (i128::try_from(s_259_5).unwrap());
        // C s_259_7: const #1u : u64
        let s_259_7: u64 = 1;
        // D s_259_8: bit-extract s_259_4 s_259_6 s_259_7
        let s_259_8: Bits = (Bits::new(
            ((s_259_4) >> (s_259_6)).value(),
            u16::try_from(s_259_7).unwrap(),
        ));
        // D s_259_9: cast reint s_259_8 -> u8
        let s_259_9: bool = ((s_259_8.value()) != 0);
        // C s_259_10: const #0s : i
        let s_259_10: i128 = 0;
        // C s_259_11: const #0u : u64
        let s_259_11: u64 = 0;
        // D s_259_12: cast zx s_259_9 -> u64
        let s_259_12: u64 = (s_259_9 as u64);
        // C s_259_13: const #1u : u64
        let s_259_13: u64 = 1;
        // D s_259_14: and s_259_12 s_259_13
        let s_259_14: u64 = ((s_259_12) & (s_259_13));
        // D s_259_15: cmp-eq s_259_14 s_259_13
        let s_259_15: bool = ((s_259_14) == (s_259_13));
        // D s_259_16: lsl s_259_12 s_259_10
        let s_259_16: u64 = s_259_12 << s_259_10;
        // D s_259_17: or s_259_11 s_259_16
        let s_259_17: u64 = ((s_259_11) | (s_259_16));
        // D s_259_18: cmpl s_259_16
        let s_259_18: u64 = !s_259_16;
        // D s_259_19: and s_259_11 s_259_18
        let s_259_19: u64 = ((s_259_11) & (s_259_18));
        // D s_259_20: select s_259_15 s_259_17 s_259_19
        let s_259_20: u64 = if s_259_15 { s_259_17 } else { s_259_19 };
        // D s_259_21: cast trunc s_259_20 -> u8
        let s_259_21: bool = ((s_259_20) != 0);
        // D s_259_22: cast zx s_259_21 -> bv
        let s_259_22: Bits = Bits::new(s_259_21 as u128, 1u16);
        // C s_259_23: const #1u : u8
        let s_259_23: bool = true;
        // C s_259_24: cast zx s_259_23 -> bv
        let s_259_24: Bits = Bits::new(s_259_23 as u128, 1u16);
        // D s_259_25: cmp-eq s_259_22 s_259_24
        let s_259_25: bool = ((s_259_22) == (s_259_24));
        // D s_259_26: write-var gs#2489 <= s_259_25
        fn_state.gs_2489 = s_259_25;
        // N s_259_27: jump b258
        return block_258(state, tracer, fn_state);
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_260_0: read-var idx:i64
        let s_260_0: i64 = fn_state.idx;
        // D s_260_1: cast zx s_260_0 -> i
        let s_260_1: i128 = (i128::try_from(s_260_0).unwrap());
        // D s_260_2: call __id(s_260_1)
        let s_260_2: i128 = u__id(state, tracer, s_260_1);
        // D s_260_3: cast reint s_260_2 -> i64
        let s_260_3: i64 = (s_260_2 as i64);
        // C s_260_4: const #32s : i
        let s_260_4: i128 = 32;
        // D s_260_5: cast zx s_260_3 -> i
        let s_260_5: i128 = (i128::try_from(s_260_3).unwrap());
        // D s_260_6: cmp-lt s_260_5 s_260_4
        let s_260_6: bool = ((s_260_5) < (s_260_4));
        // D s_260_7: write-var gs#2486 <= s_260_6
        fn_state.gs_2486 = s_260_6;
        // N s_260_8: jump b256
        return block_256(state, tracer, fn_state);
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_261_0: read-var E:u8
        let s_261_0: bool = fn_state.E;
        // D s_261_1: cast zx s_261_0 -> bv
        let s_261_1: Bits = Bits::new(s_261_0 as u128, 1u16);
        // C s_261_2: const #1u : u8
        let s_261_2: bool = true;
        // C s_261_3: cast zx s_261_2 -> bv
        let s_261_3: Bits = Bits::new(s_261_2 as u128, 1u16);
        // D s_261_4: cmp-eq s_261_1 s_261_3
        let s_261_4: bool = ((s_261_1) == (s_261_3));
        // N s_261_5: branch s_261_4 b264 b262
        if s_261_4 {
            return block_264(state, tracer, fn_state);
        } else {
            return block_262(state, tracer, fn_state);
        };
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_262_0: const #0u : u8
        let s_262_0: bool = false;
        // D s_262_1: write-var gs#2491 <= s_262_0
        fn_state.gs_2491 = s_262_0;
        // N s_262_2: jump b263
        return block_263(state, tracer, fn_state);
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_263_0: read-var gs#2491:u8
        let s_263_0: bool = fn_state.gs_2491;
        // D s_263_1: write-var enabled <= s_263_0
        fn_state.enabled = s_263_0;
        // N s_263_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_264_0: const #11736u : u32
        let s_264_0: u32 = 11736;
        // D s_264_1: read-reg s_264_0:u64
        let s_264_1: u64 = {
            let value = state.read_register::<u64>(s_264_0 as isize);
            tracer.read_register(s_264_0 as isize, value);
            value
        };
        // D s_264_2: cast zx s_264_1 -> bv
        let s_264_2: Bits = Bits::new(s_264_1 as u128, 64u16);
        // D s_264_3: read-var idx:i64
        let s_264_3: i64 = fn_state.idx;
        // D s_264_4: cast zx s_264_3 -> i
        let s_264_4: i128 = (i128::try_from(s_264_3).unwrap());
        // C s_264_5: const #1u : u64
        let s_264_5: u64 = 1;
        // D s_264_6: bit-extract s_264_2 s_264_4 s_264_5
        let s_264_6: Bits = (Bits::new(
            ((s_264_2) >> (s_264_4)).value(),
            u16::try_from(s_264_5).unwrap(),
        ));
        // D s_264_7: cast reint s_264_6 -> u8
        let s_264_7: bool = ((s_264_6.value()) != 0);
        // C s_264_8: const #0s : i
        let s_264_8: i128 = 0;
        // C s_264_9: const #0u : u64
        let s_264_9: u64 = 0;
        // D s_264_10: cast zx s_264_7 -> u64
        let s_264_10: u64 = (s_264_7 as u64);
        // C s_264_11: const #1u : u64
        let s_264_11: u64 = 1;
        // D s_264_12: and s_264_10 s_264_11
        let s_264_12: u64 = ((s_264_10) & (s_264_11));
        // D s_264_13: cmp-eq s_264_12 s_264_11
        let s_264_13: bool = ((s_264_12) == (s_264_11));
        // D s_264_14: lsl s_264_10 s_264_8
        let s_264_14: u64 = s_264_10 << s_264_8;
        // D s_264_15: or s_264_9 s_264_14
        let s_264_15: u64 = ((s_264_9) | (s_264_14));
        // D s_264_16: cmpl s_264_14
        let s_264_16: u64 = !s_264_14;
        // D s_264_17: and s_264_9 s_264_16
        let s_264_17: u64 = ((s_264_9) & (s_264_16));
        // D s_264_18: select s_264_13 s_264_15 s_264_17
        let s_264_18: u64 = if s_264_13 { s_264_15 } else { s_264_17 };
        // D s_264_19: cast trunc s_264_18 -> u8
        let s_264_19: bool = ((s_264_18) != 0);
        // D s_264_20: cast zx s_264_19 -> bv
        let s_264_20: Bits = Bits::new(s_264_19 as u128, 1u16);
        // C s_264_21: const #1u : u8
        let s_264_21: bool = true;
        // C s_264_22: cast zx s_264_21 -> bv
        let s_264_22: Bits = Bits::new(s_264_21 as u128, 1u16);
        // D s_264_23: cmp-eq s_264_20 s_264_22
        let s_264_23: bool = ((s_264_20) == (s_264_22));
        // D s_264_24: write-var gs#2491 <= s_264_23
        fn_state.gs_2491 = s_264_23;
        // N s_264_25: jump b263
        return block_263(state, tracer, fn_state);
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_265_0: const #21016u : u32
        let s_265_0: u32 = 21016;
        // D s_265_1: read-reg s_265_0:struct
        let s_265_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_265_0 as isize);
            tracer.read_register(s_265_0 as isize, value);
            value
        };
        // D s_265_2: call _get_PMCR_EL0_Type_E(s_265_1)
        let s_265_2: bool = u_get_PMCR_EL0_Type_E(state, tracer, s_265_1);
        // D s_265_3: write-var E <= s_265_2
        fn_state.E = s_265_2;
        // N s_265_4: jump b252
        return block_252(state, tracer, fn_state);
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_266_0: const #() : ()
        let s_266_0: () = ();
        // S s_266_1: call HaveAArch64(s_266_0)
        let s_266_1: bool = HaveAArch64(state, tracer, s_266_0);
        // N s_266_2: branch s_266_1 b269 b267
        if s_266_1 {
            return block_269(state, tracer, fn_state);
        } else {
            return block_267(state, tracer, fn_state);
        };
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_267_0: const #() : ()
        let s_267_0: () = ();
        // S s_267_1: call HDCR_read(s_267_0)
        let s_267_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_267_0);
        // S s_267_2: call _get_HDCR_Type_HPME(s_267_1)
        let s_267_2: bool = u_get_HDCR_Type_HPME(state, tracer, s_267_1);
        // D s_267_3: write-var E <= s_267_2
        fn_state.E = s_267_2;
        // N s_267_4: jump b268
        return block_268(state, tracer, fn_state);
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_268_0: jump b253
        return block_253(state, tracer, fn_state);
    }
    fn block_269<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_269_0: const #104880u : u32
        let s_269_0: u32 = 104880;
        // D s_269_1: read-reg s_269_0:struct
        let s_269_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_269_0 as isize);
            tracer.read_register(s_269_0 as isize, value);
            value
        };
        // D s_269_2: call _get_MDCR_EL2_Type_HPME(s_269_1)
        let s_269_2: bool = u_get_MDCR_EL2_Type_HPME(state, tracer, s_269_1);
        // D s_269_3: write-var E <= s_269_2
        fn_state.E = s_269_2;
        // N s_269_4: jump b268
        return block_268(state, tracer, fn_state);
    }
    fn block_270<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_270_0: const #() : ()
        let s_270_0: () = ();
        // S s_270_1: call HavePMUv3ICNTR(s_270_0)
        let s_270_1: bool = HavePMUv3ICNTR(state, tracer, s_270_0);
        // D s_270_2: write-var gs#2472 <= s_270_1
        fn_state.gs_2472 = s_270_1;
        // N s_270_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_271<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_271_0: const #1u : u8
        let s_271_0: bool = true;
        // D s_271_1: write-var gs#2473 <= s_271_0
        fn_state.gs_2473 = s_271_0;
        // N s_271_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_272<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_272_0: const #1u : u8
        let s_272_0: bool = true;
        // D s_272_1: write-var gs#2471 <= s_272_0
        fn_state.gs_2471 = s_272_0;
        // N s_272_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
